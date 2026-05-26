//! Canonical pin layout + playfield geometry.
//!
//! Per PRD-002 R-27: ~80 pins in a diamond-lattice pattern.
//! Per PRD-002 R-29: chucker hit rate biased to ~25–40% via gap placement
//! (for the canonical *stock* layout — see ADR-001).
//! Per PRD-004 R-46/47: PinLayout is a typed config with 6 chapter-gated knobs.

use crate::ball::{Pin, Playfield};
use serde::{Deserialize, Serialize};

/// Player-tunable layout. Each knob is `[-1.0, +1.0]`; defaults to 0.0 (stock).
/// Chapter-gating happens in `available_knob_count` and `apply_knob_clamped`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PinLayout {
    pub knobs: [f32; 6],
}

impl Default for PinLayout {
    fn default() -> Self {
        Self { knobs: [0.0; 6] }
    }
}

/// Indices into `PinLayout::knobs`. Order is stable for persistence.
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum Knob {
    LeftFunnelTilt = 0,
    RightFunnelTilt = 1,
    ChuckerMouthWidth = 2,
    GuidePinVertical = 3,
    LowerRowDensity = 4,
    UpperFunnelSpread = 5,
}

/// Metadata: label + chapter required to unlock + brief description.
pub struct KnobMeta {
    pub label: &'static str,
    pub unlock_chapter: u32,
    pub description: &'static str,
}

pub const KNOBS: [KnobMeta; 6] = [
    KnobMeta { label: "LEFT  FUNNEL  TILT",  unlock_chapter: 2, description: "Tilt the leftmost columns of pins toward / away from center." },
    KnobMeta { label: "RIGHT FUNNEL  TILT",  unlock_chapter: 2, description: "Tilt the rightmost columns of pins toward / away from center." },
    KnobMeta { label: "CHUCKER MOUTH WIDTH", unlock_chapter: 3, description: "Widen or narrow the funnel carve above the chucker." },
    KnobMeta { label: "GUIDE PIN VERTICAL",  unlock_chapter: 3, description: "Slide the guide pins above the chucker up or down." },
    KnobMeta { label: "LOWER ROW DENSITY",   unlock_chapter: 4, description: "Add an extra pin row near the bottom of the field." },
    KnobMeta { label: "UPPER FUNNEL SPREAD", unlock_chapter: 4, description: "Tilt upper-row pins inward or outward." },
];

/// How many knobs are available at the given chapter (0, 2, 4, 6).
pub fn available_knob_count(chapter: u32) -> usize {
    match chapter {
        0 | 1 => 0,
        2 => 2,
        3 => 4,
        _ => 6,
    }
}

impl PinLayout {
    pub fn stock() -> Self { Self::default() }

    /// Set a knob's value with clamping to [-1, +1]. Returns the clamped value.
    pub fn set(&mut self, knob: Knob, value: f32) -> f32 {
        let v = value.clamp(-1.0, 1.0);
        self.knobs[knob as usize] = v;
        v
    }

    pub fn get(&self, knob: Knob) -> f32 { self.knobs[knob as usize] }
}

/// Build the playfield geometry from a screen-coordinate bezel rect.
pub fn build_playfield(cab_x: f32, cab_y: f32, cab_w: f32, cab_h: f32) -> Playfield {
    Playfield {
        left: cab_x + cab_w * 0.04,
        right: cab_x + cab_w * 0.96,
        top: cab_y + cab_h * 0.04,
        bottom: cab_y + cab_h - 8.0,
        chucker_cx: cab_x + cab_w * 0.5,
        chucker_cy: cab_y + cab_h * 0.745,
        chucker_r: 32.0,
        pin_zone_top: cab_y + cab_h * 0.34,
        pin_zone_bottom: cab_y + cab_h * 0.70,
        lcd_x: cab_x + cab_w * 0.08,
        lcd_y: cab_y + cab_h * 0.05,
        lcd_w: cab_w * 0.84,
        lcd_h: cab_h * 0.25,
    }
}

/// Canonical pin field (stock layout). Convenience wrapper for callers that
/// don't yet thread a PinLayout.
pub fn canonical_pins(pf: &Playfield) -> Vec<Pin> {
    pins_for_layout(pf, &PinLayout::stock())
}

/// Build the pin field for the given layout. Honors knob offsets per PRD-004 R-46.
pub fn pins_for_layout(pf: &Playfield, layout: &PinLayout) -> Vec<Pin> {
    let mut out = Vec::with_capacity(96);
    let row_h: f32 = 38.0;
    let col_w: f32 = 46.0;

    let zone_top = pf.pin_zone_top;
    let zone_bottom = pf.pin_zone_bottom;

    // Knob values
    let k_left = layout.get(Knob::LeftFunnelTilt);
    let k_right = layout.get(Knob::RightFunnelTilt);
    let k_mouth = layout.get(Knob::ChuckerMouthWidth);
    let k_guide_y = layout.get(Knob::GuidePinVertical);
    let k_density = layout.get(Knob::LowerRowDensity);
    let k_spread = layout.get(Knob::UpperFunnelSpread);

    // Determine column index range; balls per row span the playfield width
    let columns_per_row = ((pf.right - pf.left - col_w) / col_w) as i32;

    let mut row: i32 = 0;
    let mut y = zone_top;
    while y < zone_bottom {
        // Diamond stagger
        let x_offset = if row % 2 == 0 { 0.0 } else { col_w * 0.5 };
        let mut col = 0;
        let mut x = pf.left + col_w * 0.5 + x_offset;
        while x < pf.right - col_w * 0.5 {
            // Funnel carve width is influenced by k_mouth (±50% scaling)
            let depth_into_funnel = ((y - (zone_bottom - row_h * 5.0)) / (row_h * 5.0)).clamp(0.0, 1.0);
            let mouth_scale = 1.0 + k_mouth * 0.5;
            let funnel_half_width = col_w * (0.6 + depth_into_funnel * 1.8) * mouth_scale;
            let near_chucker_lane = (x - pf.chucker_cx).abs() < funnel_half_width;
            let in_funnel_rows = y > zone_bottom - row_h * 5.0;
            if !(near_chucker_lane && in_funnel_rows) {
                // Apply per-pin knob offsets
                let mut x_off = 0.0_f32;
                // Left funnel tilt: leftmost 2 columns shift x by k_left * 8
                if col <= 1 { x_off += k_left * 8.0; }
                // Right funnel tilt: rightmost 2 columns shift x by -k_right * 8 (negative tilts outward)
                if col >= columns_per_row - 1 { x_off -= k_right * 8.0; }
                // Upper funnel spread: top 2 rows shift x outward by k_spread * col_w * 0.4
                if row < 2 {
                    let outward_sign = if x < pf.chucker_cx { -1.0 } else { 1.0 };
                    x_off += outward_sign * k_spread * col_w * 0.4;
                }
                out.push(Pin { x: x + x_off, y, r: 3.5 });
            }
            x += col_w;
            col += 1;
        }
        y += row_h;
        row += 1;
    }

    // Two pairs of "guide pins" form an angled V just above the chucker,
    // narrowing the bottom of the funnel onto the cup.
    let guide_y_off = k_guide_y * 12.0;
    out.push(Pin { x: pf.chucker_cx - 48.0, y: pf.chucker_cy - 70.0 + guide_y_off, r: 4.5 });
    out.push(Pin { x: pf.chucker_cx + 48.0, y: pf.chucker_cy - 70.0 + guide_y_off, r: 4.5 });
    out.push(Pin { x: pf.chucker_cx - 22.0, y: pf.chucker_cy - 32.0 + guide_y_off, r: 4.5 });
    out.push(Pin { x: pf.chucker_cx + 22.0, y: pf.chucker_cy - 32.0 + guide_y_off, r: 4.5 });

    // Lower row density knob: when > 0.5, ADD an extra row at the bottom of the field
    if k_density > 0.5 {
        let extra_y = zone_bottom - row_h * 0.5;
        let stagger = if (((zone_bottom - zone_top) / row_h) as i32) % 2 == 0 { col_w * 0.5 } else { 0.0 };
        let mut x = pf.left + col_w * 0.5 + stagger;
        while x < pf.right - col_w * 0.5 {
            let funnel_half_width = col_w * 2.5;
            if (x - pf.chucker_cx).abs() > funnel_half_width {
                out.push(Pin { x, y: extra_y, r: 3.5 });
            }
            x += col_w;
        }
    }

    out
}

/// Launcher emit point: top of the launch chute on the right side of the playfield.
/// Balls travel up-and-left, over the top, and fall into the pin field.
pub fn launcher_emit(pf: &Playfield, fire_count: u32) -> (f32, f32, f32, f32) {
    let jitter = ((fire_count.wrapping_mul(2654435761) >> 16) as f32) / 65535.0;
    let pf_w = pf.right - pf.left;
    let x = pf.left + pf_w * (0.30 + 0.55 * jitter);
    let y = pf.top + 8.0;
    let vx = (jitter - 0.5) * 80.0;
    let vy = 80.0;
    (x, y, vx, vy)
}

/// The right-side launch chute rect — the visible "barrel" the balls climb.
pub fn launcher_chute_rect(pf: &Playfield) -> (f32, f32, f32, f32) {
    let x = pf.right - 22.0;
    let y = pf.top + (pf.bottom - pf.top) * 0.20;
    let w = 20.0;
    let h = pf.bottom - y - 30.0;
    (x, y, w, h)
}

// ---- Headless Monte Carlo probe (PRD-004 R-48) ----

/// Run a headless ball-physics simulation to measure the chucker rate for the
/// given layout. Returns (chucker_entries, balls_fired) so the caller can
/// compute rate + CI. Per ADR-001 ベース is a *measurement*, not a config.
///
/// Each ball is simulated to either chucker-entry, "lost off the bottom," or
/// a hard step-count timeout. No rendering, no audio, no game state — just
/// physics. Deterministic given the seed.
pub fn monte_carlo_chucker_rate(
    pf: &Playfield,
    layout: &PinLayout,
    n_balls: u32,
    seed: u64,
) -> (u32, u32) {
    use crate::ball::{step, Ball};
    let pins = pins_for_layout(pf, layout);
    let mut chucker_hits = 0u32;
    let mut total = 0u32;
    let max_steps_per_ball = 1200u32; // safety: ~20s at 60fps
    let dt = 1.0 / 60.0_f32;
    // Use the seed via a simple LCG to vary the fire_count jitter
    let mut state = seed;
    for _ in 0..n_balls {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let fire_count = ((state >> 32) & 0xFFFF_FFFF) as u32;
        let (lx, ly, vx, vy) = launcher_emit(pf, fire_count);
        let mut balls = vec![Ball::new(lx, ly, vx, vy)];
        let mut steps = 0;
        while steps < max_steps_per_ball
            && balls[0].state == crate::ball::BallState::InFlight
        {
            let r = step(&mut balls, &pins, pf, dt);
            chucker_hits += r.chucker_entries;
            steps += 1;
        }
        total += 1;
    }
    (chucker_hits, total)
}
