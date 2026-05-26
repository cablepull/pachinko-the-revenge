//! Canonical pin layout + playfield geometry.
//!
//! Per PRD-002 R-27: ~80 pins in a diamond-lattice pattern.
//! Per PRD-002 R-29: chucker hit rate biased to ~25–40% via gap placement.

use crate::ball::{Pin, Playfield};

/// Build the playfield geometry from a screen-coordinate bezel rect.
pub fn build_playfield(cab_x: f32, cab_y: f32, cab_w: f32, cab_h: f32) -> Playfield {
    // LCD now sits in the top quarter of the cabinet so the pin field can
    // breathe below it. The cabinet's vertical zones, top-to-bottom:
    //   0.05 .. 0.30  LCD (reel display)
    //   0.34 .. 0.70  pin-field zone
    //   ~0.74         chucker (gold cup)
    //   0.78 .. 0.92  attacker
    //   ~0.95         ball tray hint
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

/// Canonical pin field. Diamond lattice over the lower 55% of the playfield.
/// Carves a small gap directly above the chucker so balls have a path in.
pub fn canonical_pins(pf: &Playfield) -> Vec<Pin> {
    let mut out = Vec::with_capacity(96);
    let row_h: f32 = 38.0;
    let col_w: f32 = 46.0;

    let zone_top = pf.pin_zone_top;
    let zone_bottom = pf.pin_zone_bottom;

    let mut row = 0;
    let mut y = zone_top;
    while y < zone_bottom {
        // Diamond stagger
        let x_offset = if row % 2 == 0 { 0.0 } else { col_w * 0.5 };
        let mut x = pf.left + col_w * 0.5 + x_offset;
        while x < pf.right - col_w * 0.5 {
            // Carve a wide funnel directly above the chucker — the inverse-V
            // gap widens as we go up so balls fed in from anywhere in the
            // playfield drain centerward in the last few rows. Calibrated
            // empirically to hit PRD-002 R-29 (25–40% chucker rate).
            let depth_into_funnel = ((y - (zone_bottom - row_h * 5.0)) / (row_h * 5.0)).clamp(0.0, 1.0);
            let funnel_half_width = col_w * (0.6 + depth_into_funnel * 1.8);
            let near_chucker_lane = (x - pf.chucker_cx).abs() < funnel_half_width;
            let in_funnel_rows = y > zone_bottom - row_h * 5.0;
            if !(near_chucker_lane && in_funnel_rows) {
                out.push(Pin { x, y, r: 3.5 });
            }
            x += col_w;
        }
        y += row_h;
        row += 1;
    }

    // Two pairs of "guide pins" form an angled V just above the chucker,
    // narrowing the bottom of the funnel onto the cup.
    out.push(Pin { x: pf.chucker_cx - 48.0, y: pf.chucker_cy - 70.0, r: 4.5 });
    out.push(Pin { x: pf.chucker_cx + 48.0, y: pf.chucker_cy - 70.0, r: 4.5 });
    out.push(Pin { x: pf.chucker_cx - 22.0, y: pf.chucker_cy - 32.0, r: 4.5 });
    out.push(Pin { x: pf.chucker_cx + 22.0, y: pf.chucker_cy - 32.0, r: 4.5 });

    out
}

/// Launcher emit point: top of the launch chute on the right side of the playfield.
/// Balls travel up-and-left, over the top, and fall into the pin field.
pub fn launcher_emit(pf: &Playfield, fire_count: u32) -> (f32, f32, f32, f32) {
    // The launcher chute exits along the TOP edge of the playfield. Real
    // pachinko regulars tune launch power to bias the exit point; we get
    // similar variety by jittering the spawn x deterministically by fire
    // count (a poor-man's pin-stripe randomness without dragging a separate
    // RNG into the game layer).
    let jitter = ((fire_count.wrapping_mul(2654435761) >> 16) as f32) / 65535.0; // [0, 1)
    let pf_w = pf.right - pf.left;
    // Spawn somewhere along the top half of the playfield width — biased
    // toward the right (where the chute conceptually exits) but with enough
    // spread that some balls land near the center.
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
