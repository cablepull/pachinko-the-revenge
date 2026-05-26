//! Cabinet rendering. macroquad immediate-mode draws.
//!
//! Visual grammar (per pachinko-expertise skill §5):
//! - High color saturation, primary-heavy
//! - Layered density (bezel, LCD, attacker, data lamp, knob)
//! - Mixed-script overlays (MVP: romaji; JP text deferred — see feedback)
//! - Snap-to-keyframe animation easing
//! - Screen shake on confirmed reach

use macroquad::prelude::*;
use pachinko_core::coordinator::CabinetState;
use pachinko_core::outcome::ReachTier;

use crate::ball::{Ball, BallState, Pin, Playfield};
use crate::playfield;

pub struct RenderState {
    pub reel_offsets: [f32; 3],
    pub reel_targets: [Option<u8>; 3],
    /// Rotation speed in digits/sec. Zero = stopped.
    pub reel_speed: [f32; 3],
    /// Countdown (sec) until this reel snaps to a target. <= 0 means already stopped.
    pub reel_stop_at: [f32; 3],
    pub shake_t: f32,
    pub flash_t: f32,
    pub fanfare_t: f32,
    pub reach_t: f32,
    pub round_t: f32,
    pub particles: Vec<Particle>,
    pub overlay_text: String,
    pub overlay_t: f32,
    pub last_reach_tier: Option<ReachTier>,
    pub current_state_label: String,
}

#[derive(Clone, Copy)]
pub struct Particle {
    pub x: f32, pub y: f32,
    pub vx: f32, pub vy: f32,
    pub life: f32,
    pub color: Color,
}

impl RenderState {
    pub fn new() -> Self {
        Self {
            reel_offsets: [0.0; 3],
            reel_targets: [Some(7), Some(3), Some(1)], // attractive initial pose
            reel_speed: [0.0; 3],
            reel_stop_at: [0.0; 3],
            shake_t: 0.0,
            flash_t: 0.0,
            fanfare_t: 0.0,
            reach_t: 0.0,
            round_t: 0.0,
            particles: Vec::new(),
            overlay_text: String::new(),
            overlay_t: 0.0,
            last_reach_tier: None,
            current_state_label: "BASE".into(),
        }
    }

    /// Start a spin animation. Stagger the stops in [t0, t1, t2] seconds.
    pub fn start_spin(&mut self, stops_in: [f32; 3]) {
        self.reel_targets = [None; 3];
        self.reel_speed = [22.0, 19.0, 17.0];
        self.reel_stop_at = stops_in;
    }

    /// Snap all reels immediately to the given digits (used on direct hit jackpot).
    pub fn snap_reels(&mut self, digits: [u8; 3]) {
        self.reel_targets = [Some(digits[0]), Some(digits[1]), Some(digits[2])];
        self.reel_speed = [0.0; 3];
        self.reel_stop_at = [0.0; 3];
    }

    pub fn flash(&mut self, dur: f32) { self.flash_t = dur; }
    pub fn shake(&mut self, dur: f32) { self.shake_t = dur; }
    pub fn show_overlay(&mut self, text: impl Into<String>, dur: f32) {
        self.overlay_text = text.into();
        self.overlay_t = dur;
    }

    pub fn spawn_jackpot_particles(&mut self, cx: f32, cy: f32) {
        let rng_seed = (cx as u64) ^ (cy as u64) ^ 0xCAFE_F00D;
        let mut s = rng_seed;
        for _ in 0..120 {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let angle = ((s >> 33) as f32 / u32::MAX as f32) * std::f32::consts::TAU;
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let speed = 200.0 + ((s >> 33) as f32 / u32::MAX as f32) * 350.0;
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let hue = (s >> 33) as f32 / u32::MAX as f32;
            let col = if hue < 0.4 { GOLD } else if hue < 0.7 { ORANGE } else if hue < 0.9 { RED } else { YELLOW };
            self.particles.push(Particle {
                x: cx, y: cy,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                life: 1.4,
                color: col,
            });
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.shake_t = (self.shake_t - dt).max(0.0);
        self.flash_t = (self.flash_t - dt).max(0.0);
        self.fanfare_t = (self.fanfare_t - dt).max(0.0);
        self.reach_t = (self.reach_t - dt).max(0.0);
        self.round_t = (self.round_t - dt).max(0.0);
        self.overlay_t = (self.overlay_t - dt).max(0.0);

        // Reels — advance offset while spinning, snap when countdown expires.
        for i in 0..3 {
            if self.reel_targets[i].is_none() && self.reel_speed[i] > 0.0 {
                self.reel_offsets[i] += self.reel_speed[i] * dt;
                self.reel_stop_at[i] -= dt;
                if self.reel_stop_at[i] <= 0.0 {
                    // Snap to whichever digit the offset lands near. Add `i` to
                    // de-correlate so reels don't all land on the same digit
                    // unless explicitly snapped via snap_reels().
                    let digit = ((self.reel_offsets[i] as i32 + (i as i32) * 3).rem_euclid(10)) as u8;
                    self.reel_targets[i] = Some(digit);
                    self.reel_speed[i] = 0.0;
                    self.reel_stop_at[i] = 0.0;
                }
            }
        }

        let g = 350.0;
        for p in self.particles.iter_mut() {
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vy += g * dt;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);
    }
}

pub fn draw_cabinet(
    rs: &RenderState,
    cab_state: CabinetState,
    kakuhen_remaining: u32,
    spins_since_jp: u32,
    last_jp_history: &[u32],
    unlocked_chapter: u32,
    balls_won: u64,
    total_jackpots: u32,
    pf: &Playfield,
    pins: &[Pin],
    balls: &[Ball],
    balls_fired: u32,
    balls_returned: u32,
    launcher_active: bool,
) {
    clear_background(Color::from_rgba(8, 6, 16, 255));

    let sw = screen_width();
    let sh = screen_height();

    let shake_x = if rs.shake_t > 0.0 { (rs.shake_t * 47.0).sin() * 8.0 * rs.shake_t.min(1.0) } else { 0.0 };
    let shake_y = if rs.shake_t > 0.0 { (rs.shake_t * 59.0).cos() * 8.0 * rs.shake_t.min(1.0) } else { 0.0 };

    // Background tint by state
    let tint = state_tint(cab_state);
    draw_rectangle(0.0, 0.0, sw, sh, tint);

    // Layered radial-ish glow
    for i in (0..6).rev() {
        let alpha = (i as f32 / 6.0) * 0.18;
        let c = Color::new(tint.r, tint.g, tint.b, alpha);
        draw_rectangle(0.0, 0.0, sw, sh, c);
    }

    // Cabinet bezel (gold) — match playfield builder's expected dimensions
    let cx = sw * 0.5 + shake_x;
    let cy = sh * 0.5 + shake_y;
    let cab_w = sw * 0.72;
    let cab_h = sh * 0.82;
    let cab_x = cx - cab_w * 0.5;
    let cab_y = cy - cab_h * 0.5;
    draw_rectangle(cab_x - 14.0, cab_y - 14.0, cab_w + 28.0, cab_h + 28.0, Color::from_rgba(60, 20, 20, 255));
    draw_rectangle(cab_x - 8.0, cab_y - 8.0, cab_w + 16.0, cab_h + 16.0, Color::from_rgba(230, 180, 50, 255));
    draw_rectangle(cab_x, cab_y, cab_w, cab_h, Color::from_rgba(20, 12, 28, 255));

    // LCD screen area — now compact (top quarter of cabinet) to make room
    // for the ball-and-pin playfield below.
    let lcd_x = pf.lcd_x;
    let lcd_y = pf.lcd_y;
    let lcd_w = pf.lcd_w;
    let lcd_h = pf.lcd_h;
    let lcd_bg = lcd_color(cab_state, rs);
    draw_rectangle(lcd_x, lcd_y, lcd_w, lcd_h, lcd_bg);
    draw_rectangle_lines(lcd_x, lcd_y, lcd_w, lcd_h, 4.0, Color::from_rgba(255, 200, 0, 255));

    // Title overlay strip
    // (Japanese chars deferred: macroquad's default font has no CJK glyphs and
    //  embedding Noto Sans JP would 10x the WASM size. TODO: subset font.)
    if cab_state == CabinetState::KakuhenBase || cab_state == CabinetState::KakuhenReach {
        draw_rectangle(lcd_x, lcd_y, lcd_w, 30.0, Color::from_rgba(255, 50, 80, 200));
        draw_text("ST KAKUHEN MODE  ::  CHANCE TIME", lcd_x + 10.0, lcd_y + 22.0, 22.0, WHITE);
        draw_text(&format!("ST {kakuhen_remaining:>3}"), lcd_x + lcd_w - 110.0, lcd_y + 22.0, 22.0, WHITE);
    } else {
        draw_rectangle(lcd_x, lcd_y, lcd_w, 30.0, Color::from_rgba(40, 30, 60, 180));
        draw_text("PACHINKO  ::  THE REVENGE", lcd_x + 10.0, lcd_y + 22.0, 22.0, GOLD);
        // chapter shown in the top-left session-stats line and announced via overlay
        // on advance — no need to duplicate it here under the data lamp.
        let _ = unlocked_chapter;
    }

    // Reels (3, equally spaced) — slightly tighter now that the LCD is shorter
    let reel_count = 3;
    let reel_w = lcd_w * 0.20;
    let reel_h = lcd_h * 0.62;
    let reel_gap = (lcd_w - reel_w * reel_count as f32) / (reel_count as f32 + 1.0);
    let reel_y = lcd_y + 36.0;
    for i in 0..reel_count {
        let rx = lcd_x + reel_gap + (reel_w + reel_gap) * i as f32;
        draw_rectangle(rx, reel_y, reel_w, reel_h, WHITE);
        draw_rectangle_lines(rx, reel_y, reel_w, reel_h, 3.0, BLACK);
        // Spinning numerals
        draw_reel(rx, reel_y, reel_w, reel_h, rs.reel_offsets[i], rs.reel_targets[i]);
    }

    // Reach banner
    if rs.reach_t > 0.0 {
        let tier_text = match rs.last_reach_tier {
            Some(ReachTier::Calm) => "REACH . . .",
            Some(ReachTier::Mid) => "REACH !!",
            Some(ReachTier::Premium) => "PREMIUM REACH !!!",
            Some(ReachTier::Confirmed) => "<<  IT  ENDS  TONIGHT  >>",
            None => "",
        };
        let col = match rs.last_reach_tier {
            Some(ReachTier::Calm) => Color::from_rgba(180, 180, 220, 255),
            Some(ReachTier::Mid) => YELLOW,
            Some(ReachTier::Premium) => ORANGE,
            Some(ReachTier::Confirmed) => Color::from_rgba(255, 80, 80, 255),
            None => WHITE,
        };
        let size = match rs.last_reach_tier {
            Some(ReachTier::Confirmed) => 64.0,
            Some(ReachTier::Premium) => 44.0,
            Some(ReachTier::Mid) => 36.0,
            _ => 28.0,
        };
        let yoff = lcd_y + lcd_h * 0.82;
        let m = measure_text(tier_text, None, size as u16, 1.0);
        draw_text(tier_text, lcd_x + (lcd_w - m.width) * 0.5, yoff, size, col);
    }

    // Speed lines on premium/confirmed reach
    if rs.reach_t > 0.0 && matches!(rs.last_reach_tier, Some(ReachTier::Premium) | Some(ReachTier::Confirmed)) {
        let intensity = rs.reach_t.min(1.0);
        let lines = if matches!(rs.last_reach_tier, Some(ReachTier::Confirmed)) { 60 } else { 30 };
        for i in 0..lines {
            let mut s = (i as u64).wrapping_mul(6364136223846793005).wrapping_add(rs.reach_t as u64);
            let nx1 = (s % 7919) as f32 / 7919.0;
            s = s.wrapping_mul(2862933555777941757);
            let nx2 = (s % 7919) as f32 / 7919.0;
            let x = lcd_x + nx1 * lcd_w;
            let y = lcd_y + nx2 * lcd_h;
            let len = 60.0 * intensity;
            draw_line(x, y, x + len, y, 2.0, Color::new(1.0, 1.0, 1.0, 0.4 * intensity));
        }
    }

    // ---- PIN FIELD ----
    // Draw the playfield backdrop first (a darker rectangle behind the pins).
    let pf_left = pf.left;
    let pf_right = pf.right;
    let pf_top = pf.pin_zone_top - 6.0;
    let pf_bottom = pf.pin_zone_bottom + 36.0;
    draw_rectangle(
        pf_left, pf_top,
        pf_right - pf_left, pf_bottom - pf_top,
        Color::from_rgba(12, 8, 22, 200),
    );
    // Pins (small steel dots)
    for p in pins {
        // Subtle highlight + dark base for a 3D feel
        draw_circle(p.x, p.y, p.r + 0.5, Color::from_rgba(20, 20, 22, 255));
        draw_circle(p.x, p.y, p.r, Color::from_rgba(200, 200, 210, 255));
        draw_circle(p.x - p.r * 0.3, p.y - p.r * 0.3, p.r * 0.4, Color::from_rgba(255, 255, 255, 220));
    }

    // ---- BALLS ----
    for b in balls {
        if b.state != BallState::InFlight { continue; }
        // Shadow
        draw_circle(b.x + 1.5, b.y + 2.0, b.r, Color::new(0.0, 0.0, 0.0, 0.4));
        // Body — steel highlight
        draw_circle(b.x, b.y, b.r, Color::from_rgba(220, 220, 230, 255));
        draw_circle(b.x - b.r * 0.35, b.y - b.r * 0.35, b.r * 0.35, Color::from_rgba(255, 255, 255, 230));
    }

    // ---- LAUNCH CHUTE (right side) + KNOB ----
    let (chute_x, chute_y, chute_w, chute_h) = playfield::launcher_chute_rect(pf);
    draw_rectangle(chute_x, chute_y, chute_w, chute_h, Color::from_rgba(40, 28, 18, 255));
    draw_rectangle_lines(chute_x, chute_y, chute_w, chute_h, 2.0, Color::from_rgba(120, 80, 30, 255));
    // Knob: a gold dial just below the chute
    let knob_cx = chute_x + chute_w * 0.5;
    let knob_cy = chute_y + chute_h + 28.0;
    let knob_r = 22.0;
    let knob_color = if launcher_active { Color::from_rgba(255, 220, 80, 255) } else { Color::from_rgba(210, 170, 30, 255) };
    draw_circle(knob_cx, knob_cy, knob_r + 3.0, Color::from_rgba(40, 28, 18, 255));
    draw_circle(knob_cx, knob_cy, knob_r, knob_color);
    // Pointer indicator (rotates when active)
    let pointer_angle: f32 = if launcher_active { -0.6 } else { -1.2 };
    let px = knob_cx + pointer_angle.cos() * knob_r * 0.7;
    let py = knob_cy + pointer_angle.sin() * knob_r * 0.7;
    draw_line(knob_cx, knob_cy, px, py, 3.0, BLACK);
    draw_text("KNOB", knob_cx - 18.0, knob_cy + knob_r + 16.0, 14.0, Color::from_rgba(200, 160, 80, 255));

    // ---- ATTACKER (jackpot door) — narrower band below the chucker zone ----
    let att_y = cab_y + cab_h * 0.80;
    let att_h = cab_h * 0.11;
    let att_x = lcd_x;
    let att_w = lcd_w;
    let attacker_open = matches!(cab_state, CabinetState::JackpotRound | CabinetState::BetweenRounds);
    if attacker_open {
        draw_rectangle(att_x, att_y, att_w, att_h, Color::from_rgba(255, 200, 60, 255));
        draw_text("OPEN  !!  ATTACKER  !!  OPEN", att_x + 16.0, att_y + att_h * 0.62, 28.0, RED);
    } else {
        draw_rectangle(att_x, att_y, att_w, att_h, Color::from_rgba(40, 20, 30, 255));
        draw_rectangle_lines(att_x, att_y, att_w, att_h, 3.0, Color::from_rgba(120, 80, 30, 255));
        draw_text("- attacker closed -", att_x + att_w * 0.3, att_y + att_h * 0.62, 18.0, Color::from_rgba(160, 100, 80, 255));
    }

    // ---- CHUCKER (gold cup) — where balls land to trigger reels ----
    draw_circle(pf.chucker_cx, pf.chucker_cy + 2.0, pf.chucker_r + 2.0, Color::from_rgba(60, 40, 8, 255));
    draw_circle(pf.chucker_cx, pf.chucker_cy, pf.chucker_r, Color::from_rgba(255, 215, 0, 255));
    draw_circle_lines(pf.chucker_cx, pf.chucker_cy, pf.chucker_r, 2.0, Color::from_rgba(120, 80, 0, 255));
    draw_text("HESO", pf.chucker_cx - 20.0, pf.chucker_cy + 5.0, 16.0, BLACK);

    // Data lamp HUD (top right). Made taller to fit BALLS FIRED/RETURNED.
    draw_data_lamp(
        sw - 260.0 - 8.0, 8.0, 260.0, 226.0,
        spins_since_jp, last_jp_history,
        cab_state == CabinetState::KakuhenBase || cab_state == CabinetState::KakuhenReach,
        kakuhen_remaining, balls_won, total_jackpots,
        balls_fired, balls_returned,
    );

    // Particles (jackpot confetti)
    for p in &rs.particles {
        let a = (p.life / 1.4).clamp(0.0, 1.0);
        let c = Color::new(p.color.r, p.color.g, p.color.b, a);
        draw_circle(p.x + shake_x, p.y + shake_y, 5.0, c);
    }

    // Full-screen flash overlay
    if rs.flash_t > 0.0 {
        let a = (rs.flash_t / 0.3).clamp(0.0, 1.0) * 0.8;
        draw_rectangle(0.0, 0.0, sw, sh, Color::new(1.0, 1.0, 0.7, a));
    }

    // Overlay text (FEVER!!, big jackpot banner)
    if rs.overlay_t > 0.0 {
        let m = measure_text(&rs.overlay_text, None, 96, 1.0);
        let bx = (sw - m.width) * 0.5;
        let by = sh * 0.45;
        draw_rectangle(bx - 20.0, by - 80.0, m.width + 40.0, 110.0, Color::new(0.0, 0.0, 0.0, 0.65));
        draw_text(&rs.overlay_text, bx, by, 96.0, GOLD);
    }

    // Bottom HUD: knob hint
    draw_text("[SPACE / CLICK]  pull chucker     [R]  reset session", 14.0, sh - 14.0, 20.0, Color::new(1.0, 1.0, 1.0, 0.75));

    // State debug strip (small)
    draw_text(&format!("state: {:?}", cab_state), 14.0, 22.0, 18.0, Color::new(0.5, 0.7, 1.0, 0.8));
}

fn state_tint(s: CabinetState) -> Color {
    match s {
        CabinetState::Base => Color::from_rgba(20, 14, 38, 255),
        CabinetState::Reach => Color::from_rgba(55, 18, 42, 255),
        CabinetState::JackpotRound | CabinetState::BetweenRounds => Color::from_rgba(80, 60, 10, 255),
        CabinetState::KakuhenBase => Color::from_rgba(50, 12, 30, 255),
        CabinetState::KakuhenReach => Color::from_rgba(80, 14, 36, 255),
    }
}

fn lcd_color(s: CabinetState, rs: &RenderState) -> Color {
    let base = match s {
        CabinetState::Base => Color::from_rgba(12, 18, 30, 255),
        CabinetState::Reach => Color::from_rgba(40, 14, 30, 255),
        CabinetState::JackpotRound | CabinetState::BetweenRounds => {
            // Flashing gold during jackpot
            let phase = (get_time() as f32 * 5.0).sin() * 0.5 + 0.5;
            Color::from_rgba(180, 130 + (phase * 60.0) as u8, 30, 255)
        }
        CabinetState::KakuhenBase => Color::from_rgba(40, 12, 28, 255),
        CabinetState::KakuhenReach => Color::from_rgba(80, 14, 30, 255),
    };
    if rs.reach_t > 0.0 && matches!(rs.last_reach_tier, Some(ReachTier::Confirmed)) {
        // Cycle to deep red on confirmed
        let p = (get_time() as f32 * 4.0).sin() * 0.5 + 0.5;
        Color::new(0.6 + p * 0.4, 0.06, 0.1, 1.0)
    } else {
        base
    }
}

fn draw_reel(x: f32, y: f32, w: f32, h: f32, offset: f32, target: Option<u8>) {
    // Classic 3-row pachinko reel: top / middle (result) / bottom.
    // Heights are derived from the rect so digits always fit; text never leaks.
    let row_h = h / 3.0;
    let center_y = y + h * 0.5;
    let scroll = if target.is_some() { target.unwrap() as f32 } else { offset };
    let digit_size = (row_h * 0.78) as u16;
    for k in -1i32..=1 {
        // Use a fractional scroll so spinning reels smoothly slide between digits.
        let frac = scroll - scroll.floor();
        let digit_idx = (((scroll.floor() as i32) + k + 10).rem_euclid(10)) as u8;
        let dy = center_y + (k as f32) * row_h - row_h * 0.5
               + if target.is_none() { -frac * row_h } else { 0.0 };
        if k == 0 {
            draw_rectangle(x + 4.0, y + h * 0.5 - row_h * 0.5 + 2.0,
                           w - 8.0, row_h - 4.0,
                           Color::from_rgba(255, 240, 200, 255));
        }
        // Strict clip: text vertical center must be inside the rect, else skip.
        let text_y = dy + row_h * 0.78;
        if text_y < y + row_h * 0.3 || text_y > y + h { continue; }
        let txt = format!("{digit_idx}");
        let m = measure_text(&txt, None, digit_size, 1.0);
        let c = if digit_idx == 7 { Color::from_rgba(200, 30, 30, 255) } else { Color::from_rgba(20, 20, 30, 255) };
        draw_text(&txt, x + (w - m.width) * 0.5, text_y, digit_size as f32, c);
    }
    // Bezel cap top + bottom — covers any straggler pixels just outside the rect.
    draw_rectangle(x - 4.0, y - 6.0, w + 8.0, 6.0, Color::from_rgba(20, 12, 28, 255));
    draw_rectangle(x - 4.0, y + h, w + 8.0, 6.0, Color::from_rgba(20, 12, 28, 255));
}

fn draw_data_lamp(x: f32, y: f32, w: f32, h: f32, spins_since: u32, history: &[u32], in_kakuhen: bool, remaining: u32, balls_won: u64, total_jackpots: u32, balls_fired: u32, balls_returned: u32) {
    draw_rectangle(x, y, w, h, Color::from_rgba(30, 20, 60, 230));
    draw_rectangle_lines(x, y, w, h, 2.0, GOLD);
    draw_text("DATA  LAMP", x + 8.0, y + 22.0, 18.0, GOLD);
    draw_text("v0.2", x + w - 36.0, y + 22.0, 14.0, Color::new(1.0, 1.0, 1.0, 0.5));

    draw_text(&format!("SPINS  {spins_since:>4}"), x + 8.0, y + 50.0, 20.0, WHITE);

    let label = if in_kakuhen { format!("KAKUHEN  ST {remaining}") } else { "BASE PLAY".into() };
    draw_text(&label, x + 8.0, y + 74.0, 16.0, if in_kakuhen { Color::from_rgba(255, 120, 60, 255) } else { Color::from_rgba(120, 180, 255, 255) });

    // Ball-tray HUD (PRD-002 R-32).
    let return_pct = if balls_fired > 0 { (balls_returned as f32 / balls_fired as f32 * 100.0) as i32 } else { 0 };
    draw_text(&format!("FIRED      {balls_fired:>5}"), x + 8.0, y + 100.0, 14.0, Color::from_rgba(180, 200, 220, 255));
    draw_text(&format!("RETURNED   {balls_returned:>5}"), x + 8.0, y + 118.0, 14.0, Color::from_rgba(180, 200, 220, 255));
    draw_text(&format!("RATE       {return_pct:>4}%"), x + 8.0, y + 136.0, 14.0, Color::from_rgba(180, 200, 220, 255));

    draw_text(&format!("JACKPOTS  {total_jackpots:>3}"), x + 8.0, y + 158.0, 14.0, Color::from_rgba(255, 220, 130, 255));
    draw_text(&format!("BALLS WON  {balls_won}"), x + 8.0, y + 176.0, 14.0, Color::from_rgba(255, 220, 130, 255));

    let bar_y = y + 192.0;
    let max_bar_h = 28.0;
    let bar_w = (w - 16.0) / 10.0;
    let max_v = history.iter().copied().max().unwrap_or(1).max(1);
    for (i, &v) in history.iter().take(10).enumerate() {
        let bh = (v as f32 / max_v as f32) * max_bar_h;
        let bx = x + 8.0 + i as f32 * bar_w;
        let by = bar_y + max_bar_h - bh;
        let c = if v > 500 { Color::from_rgba(255, 80, 80, 255) } else { Color::from_rgba(120, 200, 255, 255) };
        draw_rectangle(bx + 2.0, by, bar_w - 4.0, bh, c);
    }
    draw_text("last 10 jp-gap", x + 8.0, y + h - 4.0, 12.0, Color::new(1.0, 1.0, 1.0, 0.55));
}
