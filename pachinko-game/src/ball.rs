//! Ball, Pin, and the physics step for the playfield.
//!
//! Per intent C-1: this lives in pachinko-game, NOT pachinko-core. The math
//! layer stays pure. Per PRD-002 F-2: ball entity has position, velocity,
//! lifecycle; pins are circles; collision is 2D circle-circle elastic.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BallState {
    InFlight,
    InChucker,
    Lost,
}

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub r: f32,
    pub state: BallState,
    pub age_ms: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Pin {
    pub x: f32,
    pub y: f32,
    pub r: f32,
}

/// Result of one physics step: ball indices that transitioned to InChucker
/// this frame. The caller maps each entry to a `session.pull_chucker()` call.
#[derive(Debug, Default)]
pub struct StepResult {
    pub chucker_entries: u32,
}

pub struct Playfield {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub chucker_cx: f32,
    pub chucker_cy: f32,
    pub chucker_r: f32,
    /// Vertical bounds of the pin-field zone (below LCD, above chucker).
    pub pin_zone_top: f32,
    pub pin_zone_bottom: f32,
    /// LCD rect (for the reel display) — physics doesn't collide with it,
    /// but render draws it.
    pub lcd_x: f32,
    pub lcd_y: f32,
    pub lcd_w: f32,
    pub lcd_h: f32,
}

const GRAVITY_PX_PER_S2: f32 = 1300.0;
const WALL_RESTITUTION: f32 = 0.55;
const PIN_RESTITUTION: f32 = 0.78;
const PIN_RANDOM_KICK: f32 = 18.0; // tiny lateral jitter on pin hit (deterministic via age)

impl Ball {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32) -> Self {
        Self { x, y, vx, vy, r: 6.0, state: BallState::InFlight, age_ms: 0.0 }
    }
}

/// Step all balls by `dt` seconds. Mutates each ball's position/velocity/state.
/// Returns aggregate counts.
pub fn step(balls: &mut [Ball], pins: &[Pin], pf: &Playfield, dt: f32) -> StepResult {
    let mut r = StepResult::default();
    for b in balls.iter_mut() {
        if b.state != BallState::InFlight { continue; }
        b.age_ms += dt * 1000.0;
        // Gravity
        b.vy += GRAVITY_PX_PER_S2 * dt;
        // Integrate
        b.x += b.vx * dt;
        b.y += b.vy * dt;

        // Walls
        if b.x - b.r < pf.left {
            b.x = pf.left + b.r;
            b.vx = -b.vx * WALL_RESTITUTION;
        }
        if b.x + b.r > pf.right {
            b.x = pf.right - b.r;
            b.vx = -b.vx * WALL_RESTITUTION;
        }
        // Allow the ball to enter above pf.top (the launch arc carries it over the top)
        // but reflect off ceiling if it actually goes too high.
        if b.y - b.r < pf.top - 80.0 {
            b.y = pf.top - 80.0 + b.r;
            b.vy = -b.vy * WALL_RESTITUTION;
        }

        // Pin collisions (naive O(n_balls * n_pins); ~4k ops/frame at 50 balls, 80 pins)
        for p in pins {
            let dx = b.x - p.x;
            let dy = b.y - p.y;
            let dist2 = dx * dx + dy * dy;
            let min_d = b.r + p.r;
            if dist2 < min_d * min_d && dist2 > 0.0001 {
                let dist = dist2.sqrt();
                let nx = dx / dist;
                let ny = dy / dist;
                // Push the ball out of overlap
                let overlap = min_d - dist;
                b.x += nx * overlap;
                b.y += ny * overlap;
                // Reflect velocity about normal
                let dot = b.vx * nx + b.vy * ny;
                b.vx = (b.vx - 2.0 * dot * nx) * PIN_RESTITUTION;
                b.vy = (b.vy - 2.0 * dot * ny) * PIN_RESTITUTION;
                // Deterministic micro-kick: bias laterally based on ball.age so two
                // balls hitting the same pin don't fall in identical paths.
                let kick = ((b.age_ms * 0.137).sin()) * PIN_RANDOM_KICK;
                b.vx += kick;
            }
        }

        // Chucker entry: a circle test against the chucker cup
        let cdx = b.x - pf.chucker_cx;
        let cdy = b.y - pf.chucker_cy;
        if cdx * cdx + cdy * cdy < (pf.chucker_r + b.r * 0.4).powi(2) {
            b.state = BallState::InChucker;
            r.chucker_entries += 1;
            continue;
        }

        // Lost off the bottom
        if b.y - b.r > pf.bottom {
            b.state = BallState::Lost;
        }
    }
    r
}

/// Remove terminal-state balls from the vec; keeps the active set bounded.
pub fn prune(balls: &mut Vec<Ball>) {
    balls.retain(|b| b.state == BallState::InFlight);
}
