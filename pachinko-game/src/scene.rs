//! Scene helpers — back-panel art, drop shadows, bezel lighting.
//!
//! Per PRD-003 R-36..R-37 and skill §7 (visual layering grammar).
//! Everything drawn from rectangles, lines, and circles only — no external
//! image assets (preserves the single-file WASM artifact). The revenge-theme
//! backdrop is a stylized rain-soaked neon cityscape.

use macroquad::prelude::*;
use pachinko_core::coordinator::CabinetState;

/// Draw the back-panel art behind the pin field — the deepest plane.
/// Drawn before pins so pins overlay it.
pub fn draw_back_panel(
    x: f32, y: f32, w: f32, h: f32,
    cab_state: CabinetState,
    t: f64,
) {
    // Base gradient (top dark to mid darker — sky behind a city)
    for i in 0..28 {
        let frac = i as f32 / 28.0;
        let alpha = 0.04 + frac * 0.06;
        let row_y = y + h * frac;
        let row_h = h / 28.0 + 1.0;
        // State-tinted gradient
        let base = match cab_state {
            CabinetState::KakuhenBase | CabinetState::KakuhenReach => {
                Color::new(0.36 + frac * 0.08, 0.06, 0.10, alpha + 0.05)
            }
            CabinetState::Reach => {
                Color::new(0.18 + frac * 0.10, 0.06, 0.12, alpha)
            }
            _ => {
                Color::new(0.06, 0.06 + frac * 0.03, 0.16 + frac * 0.04, alpha)
            }
        };
        draw_rectangle(x, row_y, w, row_h, base);
    }

    // Moon disc (faint, high)
    let moon_cx = x + w * 0.75;
    let moon_cy = y + h * 0.18;
    let moon_r = h * 0.05;
    draw_circle(moon_cx, moon_cy, moon_r, Color::new(0.95, 0.90, 0.78, 0.18));
    draw_circle(moon_cx + moon_r * 0.3, moon_cy - moon_r * 0.3, moon_r * 0.65, Color::new(0.05, 0.05, 0.10, 0.25));

    // Rain — thin diagonal lines, slow-scrolling phase from t
    let rain_phase = ((t * 22.0) as f32).fract() * 14.0;
    for col in 0..50 {
        let line_x = x + ((col as f32 * 31.0 + rain_phase) % w);
        let len = 14.0 + (col as f32 * 7.13).sin().abs() * 6.0;
        draw_line(
            line_x, y + (col as f32 * 19.0 % h) * 0.6,
            line_x - 5.0, y + (col as f32 * 19.0 % h) * 0.6 + len,
            1.0,
            Color::new(0.7, 0.8, 1.0, 0.10),
        );
    }

    // Distant building silhouettes (bottom half)
    let building_base_y = y + h * 0.55;
    let silhouettes: [(f32, f32, f32); 9] = [
        (0.05, 0.30, 0.22),
        (0.13, 0.45, 0.16),
        (0.20, 0.32, 0.24),
        (0.30, 0.55, 0.14),
        (0.42, 0.36, 0.20),
        (0.55, 0.50, 0.18),
        (0.66, 0.30, 0.26),
        (0.78, 0.46, 0.16),
        (0.88, 0.36, 0.22),
    ];
    for (cx, ch, cw) in silhouettes {
        let bx = x + w * cx - w * cw * 0.5;
        let by = building_base_y - h * ch;
        let bw = w * cw;
        let bh = (y + h) - by;
        // Dark building body
        draw_rectangle(bx, by, bw, bh, Color::new(0.04, 0.04, 0.10, 0.85));
        draw_rectangle_lines(bx, by, bw, bh, 1.0, Color::new(0.10, 0.10, 0.20, 0.85));
        // Neon strip — random vertical color line per building
        let neon_phase = (cx * 100.0 + t as f32 * 0.7).sin() * 0.5 + 0.5;
        let neon_color = match (cx * 10.0) as i32 % 3 {
            0 => Color::new(1.0, 0.2, 0.3, 0.35 + 0.20 * neon_phase),
            1 => Color::new(0.3, 0.7, 1.0, 0.35 + 0.20 * neon_phase),
            _ => Color::new(1.0, 0.85, 0.3, 0.35 + 0.20 * neon_phase),
        };
        let nx = bx + bw * 0.25 + (cx * 47.0).sin().abs() * bw * 0.4;
        draw_rectangle(nx, by + bh * 0.10, 2.0, bh * 0.7, neon_color);
        // A few window dots
        for w_i in 0..5 {
            for w_j in 0..3 {
                let wx = bx + bw * (0.15 + w_j as f32 * 0.30);
                let wy = by + bh * (0.20 + w_i as f32 * 0.15);
                let lit = ((cx * 100.0 + w_i as f32 * 1.7 + w_j as f32 * 2.3) * 7.0).sin() > 0.3;
                if lit {
                    draw_rectangle(wx, wy, 2.0, 2.0, Color::new(0.95, 0.85, 0.4, 0.30));
                }
            }
        }
    }

    // Subtle scanlines across the whole panel (CRT/parallax cue)
    for i in 0..(h as i32 / 4) {
        let sy = y + (i * 4) as f32;
        draw_line(x, sy, x + w, sy, 1.0, Color::new(0.0, 0.0, 0.0, 0.06));
    }
}

/// Draw a drop-shadow under a rectangle.
pub fn drop_shadow_rect(x: f32, y: f32, w: f32, h: f32, offset: f32, alpha: f32) {
    draw_rectangle(x + offset, y + offset, w, h, Color::new(0.0, 0.0, 0.0, alpha));
}

/// Draw a drop-shadow under a circle.
pub fn drop_shadow_circle(cx: f32, cy: f32, r: f32, offset: f32, alpha: f32) {
    draw_circle(cx + offset, cy + offset, r, Color::new(0.0, 0.0, 0.0, alpha));
}

/// Animated bezel lighting around the cabinet rect. Pattern depends on state.
pub fn draw_bezel_lighting(
    cab_x: f32, cab_y: f32, cab_w: f32, cab_h: f32,
    cab_state: CabinetState,
    t: f64,
) {
    let strip_thickness = 6.0;
    let inner_offset = 4.0;
    let bx = cab_x - inner_offset;
    let by = cab_y - inner_offset;
    let bw = cab_w + inner_offset * 2.0;
    let bh = cab_h + inner_offset * 2.0;

    // Compute color/pattern per state
    let frame_t = t as f32;
    match cab_state {
        CabinetState::Base => {
            // Slow gold breathing (~0.6 Hz)
            let pulse = 0.55 + 0.45 * (frame_t * 1.2 * std::f32::consts::PI).sin().abs();
            let c = Color::new(1.0, 0.78, 0.18, 0.45 + 0.45 * pulse);
            draw_bezel_strip(bx, by, bw, bh, strip_thickness, c);
        }
        CabinetState::Reach => {
            // Orange wave flowing left-to-right (~1.4 Hz)
            draw_bezel_wave(bx, by, bw, bh, strip_thickness,
                            Color::new(1.0, 0.55, 0.15, 0.85),
                            Color::new(0.5, 0.20, 0.05, 0.30),
                            frame_t * 1.4);
        }
        CabinetState::JackpotRound | CabinetState::BetweenRounds => {
            // Gold/red strobe at ~4 Hz
            let strobe = (frame_t * 4.0 * std::f32::consts::PI).sin() > 0.0;
            let c = if strobe {
                Color::new(1.0, 0.85, 0.20, 0.95)
            } else {
                Color::new(1.0, 0.30, 0.20, 0.85)
            };
            draw_bezel_strip(bx, by, bw, bh, strip_thickness + 2.0, c);
            // Outer glow ring on strobe peaks
            if strobe {
                draw_bezel_strip(bx - 4.0, by - 4.0, bw + 8.0, bh + 8.0, 3.0,
                    Color::new(1.0, 0.95, 0.55, 0.35));
            }
        }
        CabinetState::KakuhenBase => {
            // Red wash with cool accents pulsing slowly
            let pulse = 0.5 + 0.5 * (frame_t * 1.5).sin();
            draw_bezel_strip(bx, by, bw, bh, strip_thickness + 1.0,
                Color::new(0.95, 0.18, 0.30, 0.85));
            draw_bezel_strip(bx, by, bw, bh, 2.0,
                Color::new(0.3, 0.5, 1.0, 0.45 * pulse));
        }
        CabinetState::KakuhenReach => {
            // Red strobe sweeping toward chucker
            draw_bezel_wave(bx, by, bw, bh, strip_thickness + 1.0,
                            Color::new(1.0, 0.10, 0.25, 0.95),
                            Color::new(0.4, 0.05, 0.10, 0.40),
                            frame_t * 2.4);
        }
    }
}

fn draw_bezel_strip(x: f32, y: f32, w: f32, h: f32, thickness: f32, c: Color) {
    // Four strips: top, bottom, left, right
    draw_rectangle(x, y, w, thickness, c);
    draw_rectangle(x, y + h - thickness, w, thickness, c);
    draw_rectangle(x, y, thickness, h, c);
    draw_rectangle(x + w - thickness, y, thickness, h, c);
}

/// Draw a "wave" along the bezel — colors fade between hot and cold along
/// the perimeter, with the hot center moving over time.
fn draw_bezel_wave(x: f32, y: f32, w: f32, h: f32, thickness: f32, hot: Color, cold: Color, phase: f32) {
    let segments = 40;
    let perim = 2.0 * (w + h);
    let hot_pos = ((phase * std::f32::consts::PI).sin() * 0.5 + 0.5) * perim;
    for i in 0..segments {
        let s_start = i as f32 / segments as f32 * perim;
        let s_end = (i + 1) as f32 / segments as f32 * perim;
        let s_mid = (s_start + s_end) * 0.5;
        let dist = ((s_mid - hot_pos).abs()).min(perim - (s_mid - hot_pos).abs());
        let intensity = (1.0 - (dist / (perim * 0.15)).min(1.0)).max(0.0);
        let c = Color::new(
            cold.r + (hot.r - cold.r) * intensity,
            cold.g + (hot.g - cold.g) * intensity,
            cold.b + (hot.b - cold.b) * intensity,
            cold.a + (hot.a - cold.a) * intensity,
        );
        // Map perimeter position to (x, y) on the bezel
        let (sx, sy, sw, sh) = perim_segment_rect(x, y, w, h, thickness, s_start, s_end);
        draw_rectangle(sx, sy, sw, sh, c);
    }
}

/// Convert a [s_start, s_end] perimeter range to a screen-space rectangle on the bezel strip.
fn perim_segment_rect(x: f32, y: f32, w: f32, h: f32, t: f32, s0: f32, s1: f32) -> (f32, f32, f32, f32) {
    // Perimeter laid out as: top edge (0..w), right (w..w+h), bottom (w+h..2w+h), left (2w+h..2w+2h)
    let pos = (s0 + s1) * 0.5;
    let seg_len = s1 - s0;
    if pos < w {
        (x + pos - seg_len * 0.5, y, seg_len, t)
    } else if pos < w + h {
        let dy = pos - w;
        (x + w - t, y + dy - seg_len * 0.5, t, seg_len)
    } else if pos < 2.0 * w + h {
        let dx = pos - (w + h);
        (x + w - dx - seg_len * 0.5, y + h - t, seg_len, t)
    } else {
        let dy = pos - (2.0 * w + h);
        (x, y + h - dy - seg_len * 0.5, t, seg_len)
    }
}

/// Chucker hit flash — a quick bright bloom on the cup.
pub fn draw_chucker_flash(cx: f32, cy: f32, life: f32, max_life: f32) {
    let p = (life / max_life).clamp(0.0, 1.0);
    let r = 30.0 + (1.0 - p) * 40.0;
    let alpha = p * 0.55;
    draw_circle(cx, cy, r, Color::new(1.0, 1.0, 0.7, alpha));
    draw_circle(cx, cy, r * 0.55, Color::new(1.0, 0.95, 0.4, alpha * 1.4));
    // Rim glow ring
    for i in 0..12 {
        let a = (i as f32 / 12.0) * std::f32::consts::TAU;
        let rx = cx + a.cos() * (r * 0.9);
        let ry = cy + a.sin() * (r * 0.9);
        draw_circle(rx, ry, 2.0 + (1.0 - p) * 3.0, Color::new(1.0, 0.8, 0.2, alpha));
    }
}

/// Character silhouette cut-in (mid/premium/confirmed reaches).
/// Slides in from a corner, holds, slides out. tier determines color + corner.
pub fn draw_cutin(sw: f32, sh: f32, tier_ord: i32, elapsed: f32, reach_t: f32) {
    let total_life = (elapsed + reach_t).max(0.0001);
    // Slide-in: first 0.3s. Slide-out: last 0.4s of reach_t < 0.4 (after reach_t expires).
    let slide_in_dur = 0.3;
    let _slide_out_dur = 0.4;
    let appearing = elapsed < slide_in_dur;
    let leaving = reach_t <= 0.0 && elapsed > 0.0; // reach ended

    let progress: f32 = if appearing {
        (elapsed / slide_in_dur).clamp(0.0, 1.0)
    } else if leaving {
        0.0 // for the snap-out we just instantly hide
    } else {
        1.0
    };

    // From which corner — alternate by tier to vary the framing.
    let from_left = tier_ord % 2 == 0;
    let target_x = if from_left { sw * 0.04 } else { sw * 0.74 };
    let off_x = if from_left { -sw * 0.28 } else { sw * 1.04 };
    let x = off_x + (target_x - off_x) * progress;
    let y = sh * 0.36;
    let w = sw * 0.22;
    let h = sh * 0.36;

    // Colors per tier
    let (tint, accent, name) = match tier_ord {
        1 => (Color::from_rgba(180, 180, 220, 200), Color::from_rgba(220, 220, 240, 255), "FLASHBACK"),
        2 => (Color::from_rgba(255, 200, 80, 220), Color::from_rgba(255, 230, 120, 255), "PREPARATION"),
        3 => (Color::from_rgba(255, 120, 40, 230), Color::from_rgba(255, 180, 80, 255), "THE  CONFRONTATION"),
        4 => (Color::from_rgba(255, 60, 60, 250), Color::from_rgba(255, 220, 200, 255), "IT  ENDS  TONIGHT"),
        _ => (Color::from_rgba(180, 180, 220, 200), Color::from_rgba(220, 220, 240, 255), ""),
    };

    let alpha = progress;
    // Shadow + body — a stylized silhouette frame
    draw_rectangle(x + 6.0, y + 6.0, w, h, Color::new(0.0, 0.0, 0.0, 0.45 * alpha));
    draw_rectangle(x, y, w, h, Color::new(0.05, 0.05, 0.10, 0.85 * alpha));
    draw_rectangle_lines(x, y, w, h, 3.0, accent);

    // Diagonal accent stripes
    for i in 0..6 {
        let off = i as f32 * 22.0;
        draw_line(x + off, y, x + off - 30.0, y + h, 1.0, Color::new(tint.r, tint.g, tint.b, 0.30 * alpha));
    }

    // Silhouette: an abstract figure assembled from rects/circles.
    // Head + body + sword (for the protagonist)
    let cx = x + w * 0.5;
    let head_r = h * 0.10;
    draw_circle(cx, y + h * 0.20, head_r, Color::new(tint.r, tint.g, tint.b, 0.90 * alpha));
    // Body trapezoid
    draw_rectangle(cx - w * 0.18, y + h * 0.30, w * 0.36, h * 0.40,
                   Color::new(tint.r, tint.g, tint.b, 0.85 * alpha));
    // Arms
    draw_rectangle(cx - w * 0.36, y + h * 0.33, w * 0.18, h * 0.06,
                   Color::new(tint.r, tint.g, tint.b, 0.85 * alpha));
    draw_rectangle(cx + w * 0.18, y + h * 0.33, w * 0.18, h * 0.06,
                   Color::new(tint.r, tint.g, tint.b, 0.85 * alpha));
    // A blade extending up
    draw_rectangle(cx + w * 0.28, y + h * 0.05, 3.0, h * 0.50,
                   Color::new(accent.r, accent.g, accent.b, alpha));
    // Glint on blade
    draw_rectangle(cx + w * 0.28, y + h * 0.06, 2.0, h * 0.10,
                   Color::new(1.0, 1.0, 1.0, 0.7 * alpha));

    // Name banner at the bottom of the panel
    let name_m = measure_text(name, None, 18, 1.0);
    draw_rectangle(x, y + h - 26.0, w, 26.0, Color::new(0.10, 0.05, 0.05, 0.9 * alpha));
    draw_text(name, x + (w - name_m.width) * 0.5, y + h - 8.0, 18.0, accent);

    // Tier badge in the corner
    let badge = match tier_ord {
        1 => "I", 2 => "II", 3 => "III", 4 => "IV", _ => "?",
    };
    draw_text(badge, x + 8.0, y + 24.0, 22.0, accent);
}

/// Letter-by-letter "F E V E R !!" reveal during the jackpot fanfare.
pub fn draw_fever_reveal(sw: f32, sh: f32, t: f32) {
    let text = "F E V E R !!";
    let chars: Vec<char> = text.chars().collect();
    let total_letters = chars.len();
    // Each char appears over 0.07s; full reveal at ~total_letters * 0.07 = 0.84s
    let per_char = 0.07;
    let visible_letters = ((t / per_char) as usize).min(total_letters);
    let visible_text: String = chars.iter().take(visible_letters).collect();

    // Hold + slight pulse for the rest of the time
    let size = 110.0_f32;
    let m = measure_text(&visible_text, None, size as u16, 1.0);
    let cx = (sw - m.width) * 0.5;
    let cy = sh * 0.42;
    // Background panel
    draw_rectangle(cx - 24.0, cy - 90.0, m.width + 48.0, 120.0, Color::new(0.0, 0.0, 0.0, 0.7));
    // Glow halo
    for i in 0..6 {
        let alpha = 0.10 * (1.0 - i as f32 / 6.0);
        draw_text(&visible_text, cx + i as f32 * 1.5, cy + i as f32 * 0.5, size, Color::new(1.0, 0.9, 0.3, alpha));
    }
    draw_text(&visible_text, cx, cy, size, GOLD);
}

/// Directional gold rays sweeping outward from cabinet center.
pub fn draw_gold_rays(cx: f32, cy: f32, life: f32, max_life: f32) {
    let p = (life / max_life).clamp(0.0, 1.0);
    let sweep_angle = (1.0 - p) * std::f32::consts::TAU * 0.5; // rotates a quarter turn over fade
    let alpha = p * 0.55;
    let length = 600.0 + (1.0 - p) * 300.0;
    for i in 0..12 {
        let a = i as f32 * std::f32::consts::TAU / 12.0 + sweep_angle;
        let x2 = cx + a.cos() * length;
        let y2 = cy + a.sin() * length;
        draw_line(cx, cy, x2, y2, 4.0, Color::new(1.0, 0.85, 0.3, alpha));
        draw_line(cx, cy, x2, y2, 1.0, Color::new(1.0, 1.0, 0.9, alpha * 1.5));
    }
}

/// "CHANCE TIME!!" banner that slams in from the top of the screen.
pub fn draw_kakuhen_slam(sw: f32, sh: f32, life: f32, max_life: f32) {
    let p = (life / max_life).clamp(0.0, 1.0);
    let total = max_life;
    let elapsed = total - life;
    let slam_in = 0.3;
    // y goes from -100 (off-screen) to its target at ~sh*0.30 over the slam_in duration
    let target_y = sh * 0.30;
    let y = if elapsed < slam_in {
        let q = (elapsed / slam_in).clamp(0.0, 1.0);
        -120.0 + (target_y + 120.0) * (1.0 - (1.0 - q).powi(3))
    } else {
        target_y
    };

    // Alpha fade in the last 0.5s
    let alpha = if life < 0.5 { (life / 0.5).clamp(0.0, 1.0) } else { 1.0 };
    let _ = p;

    // Banner
    let banner_h = 80.0;
    draw_rectangle(0.0, y, sw, banner_h, Color::new(0.85, 0.05, 0.10, 0.92 * alpha));
    draw_rectangle(0.0, y + banner_h - 4.0, sw, 4.0, Color::new(1.0, 0.95, 0.5, alpha));
    draw_rectangle(0.0, y, sw, 4.0, Color::new(1.0, 0.95, 0.5, alpha));
    let text = "C H A N C E   T I M E  !!";
    let m = measure_text(text, None, 48, 1.0);
    let x = (sw - m.width) * 0.5;
    draw_text(text, x + 3.0, y + 56.0, 48.0, Color::new(0.2, 0.0, 0.0, alpha));
    draw_text(text, x, y + 54.0, 48.0, Color::new(1.0, 0.95, 0.55, alpha));
}

/// Chapter title card — wipes across the screen, holds, wipes off.
pub fn draw_chapter_card(sw: f32, sh: f32, elapsed: f32, label: &str) {
    // 1.0s wipe-in, 1.5s hold, 1.0s wipe-out
    let wipe_in = 1.0;
    let hold = 1.5;
    let _wipe_out = 1.0;

    let card_h = 120.0;
    let card_y = sh * 0.42;

    let progress: f32 = if elapsed < wipe_in {
        (elapsed / wipe_in).clamp(0.0, 1.0)
    } else if elapsed < wipe_in + hold {
        1.0
    } else {
        let q = (elapsed - wipe_in - hold) / 1.0;
        (1.0 - q).clamp(0.0, 1.0)
    };

    // Reveal width = full sw * progress
    let reveal_w = sw * progress;
    // Reveal from left or from center? Wipe from left-to-right.
    draw_rectangle(0.0, card_y, reveal_w, card_h, Color::new(0.06, 0.04, 0.10, 0.95));
    draw_rectangle(0.0, card_y, reveal_w, 3.0, Color::new(0.95, 0.75, 0.30, 0.95));
    draw_rectangle(0.0, card_y + card_h - 3.0, reveal_w, 3.0, Color::new(0.95, 0.75, 0.30, 0.95));

    // Text shown only when the wipe has cleared past the text position
    let text_m = measure_text(label, None, 32, 1.0);
    let text_x = (sw - text_m.width) * 0.5;
    let text_visible_alpha = if elapsed < wipe_in {
        (elapsed / wipe_in - 0.4).max(0.0)
    } else if elapsed > wipe_in + hold {
        progress
    } else {
        1.0
    };
    draw_text(label, text_x, card_y + 70.0, 32.0, Color::new(0.95, 0.75, 0.30, text_visible_alpha));
    // Subtitle
    let sub = "* * *";
    let sub_m = measure_text(sub, None, 18, 1.0);
    draw_text(sub, (sw - sub_m.width) * 0.5, card_y + card_h - 22.0, 18.0,
              Color::new(0.95, 0.75, 0.30, text_visible_alpha * 0.6));
}

/// Treasure trickle floaters — small "+¥4" text rising and fading.
pub fn draw_trickle(items: &[(f32, f32, f32, String)]) {
    for (x, y, life, label) in items {
        let alpha = (life / 1.2).clamp(0.0, 1.0);
        let m = measure_text(label, None, 14, 1.0);
        draw_text(label, x - m.width * 0.5, *y, 14.0, Color::new(1.0, 0.85, 0.35, alpha));
    }
}

/// Marquee title strip — a thin row at the very top of the cabinet with the
/// project name scrolling slowly across.
pub fn draw_marquee(cab_x: f32, cab_y: f32, cab_w: f32, t: f64) {
    let m_y = cab_y - 28.0;
    let m_h = 22.0;
    draw_rectangle(cab_x, m_y, cab_w, m_h, Color::from_rgba(20, 10, 16, 230));
    draw_rectangle_lines(cab_x, m_y, cab_w, m_h, 1.0, Color::from_rgba(120, 80, 30, 200));
    let text = "  *  PACHINKO  ::  THE REVENGE  *  ORIGINAL CABINET  *  CR-STYLE  ::  ST KAKUHEN  *  ";
    let m_w = measure_text(text, None, 14, 1.0).width;
    let scroll = ((t * 30.0) as f32) % m_w;
    // Draw the text twice, side by side, so it wraps seamlessly
    draw_text(text, cab_x - scroll, m_y + 16.0, 14.0, Color::from_rgba(243, 181, 74, 220));
    draw_text(text, cab_x - scroll + m_w, m_y + 16.0, 14.0, Color::from_rgba(243, 181, 74, 220));
}
