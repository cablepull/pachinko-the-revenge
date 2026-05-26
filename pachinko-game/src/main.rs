//! pachinko-the-revenge — game entry.
//!
//! Drives the headless `pachinko-core` session with macroquad's run loop,
//! input, rendering, and procedural audio. Targets native + WASM.

use macroquad::prelude::*;
use pachinko_core::coordinator::CabinetState;
use pachinko_core::outcome::ReachTier;
use pachinko_core::session::{Session, SessionEvent};

mod audio;
mod ball;
mod playfield;
mod render;
mod scene;
mod persist;

use audio::AudioBank;
use render::RenderState;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pachinko: The Revenge".to_owned(),
        window_width: 1100,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Critical: render at least one frame BEFORE awaiting audio load.
    // On WASM, `AudioBank::build` awaits Web Audio decode of every clip and
    // can take ~hundreds of ms (or stall entirely until the user gesture
    // unlocks the AudioContext). Without a `next_frame().await` first, the
    // macroquad runtime never gets to draw — the canvas stays black with no
    // visible error. The "loading…" frame lets users see something is alive.
    for _ in 0..2 {
        clear_background(Color::from_rgba(8, 6, 16, 255));
        let sw = screen_width();
        let sh = screen_height();
        let m = measure_text("loading…", None, 40, 1.0);
        draw_text("loading…", (sw - m.width) * 0.5, sh * 0.5, 40.0, Color::from_rgba(243, 181, 74, 255));
        let sub = "synthesizing audio bank";
        let ms = measure_text(sub, None, 16, 1.0);
        draw_text(sub, (sw - ms.width) * 0.5, sh * 0.5 + 28.0, 16.0, Color::from_rgba(160, 130, 100, 255));
        next_frame().await;
    }

    let bank = AudioBank::build().await;

    let seed = (get_time() * 1_000.0) as u64 ^ 0xCAFE_F00D;
    let mut session = Session::new(seed, (get_time() * 1000.0) as u64);

    // Load persisted state (PinLayout + chapter + last-session summary)
    let persisted = persist::load().unwrap_or_default();
    session.state.unlocked_chapter = persisted.unlocked_chapter.max(1);
    let mut current_layout = persisted.layout;
    let last_summary = persisted.last_summary.clone();
    let prior_session_at_ms = persisted.last_session_at_ms;

    let mut rs = RenderState::new();
    // Show welcome-back card if the last session was within the prior 7 days
    if prior_session_at_ms > 0 && last_summary.is_some() {
        let now_ms = (get_time() * 1000.0) as u64;
        let week = 7u64 * 24 * 60 * 60 * 1000;
        if now_ms.saturating_sub(prior_session_at_ms) < week {
            rs.welcome_back_active = true;
            rs.welcome_back_summary = last_summary.clone();
        }
    }

    // BGM state
    let mut current_bgm = BgmTrack::Base;
    audio::play_loop(&bank.base_bgm, 0.7);

    // Reach / round timing state
    let mut reach_timer: f32 = 0.0;
    let mut round_timer: f32 = 0.0;
    let mut between_timer: f32 = 0.0;
    let mut fanfare_timer: f32 = 0.0;

    let mut prev_state = session.coord.state;
    let mut prev_chapter = session.state.unlocked_chapter;
    let mut last_jp_history: Vec<u32> = Vec::new();
    let mut spins_at_last_jp: u32 = 0;
    let mut balls_won_total: u64 = 0;
    let mut kakuhen_streak: u32 = 0;
    let mut prev_total_jp: u32 = 0;
    let mut in_kakuhen_prev: bool = false;

    let mut session_start_time = get_time();

    // ---- Ball physics state (PRD-002 F-2 + PRD-004 R-46) ----
    let mut pf = build_playfield_from_screen_geom();
    let mut pins = playfield::pins_for_layout(&pf, &current_layout);
    let mut balls: Vec<ball::Ball> = Vec::with_capacity(80);
    let mut last_launch_t: f64 = 0.0;
    let mut balls_fired_total: u32 = 0;
    let mut balls_returned_total: u32 = 0;
    let mut last_canvas_size = (screen_width(), screen_height());

    // Track session start (UNIX-ish ms via get_time() seconds-since-init + epoch placeholder)
    let session_real_start_ms = (get_time() * 1000.0) as u64;
    // Dirty flag for layout: when true, regenerate pins next frame
    let mut layout_dirty = false;
    let mut longest_dry_streak: u32 = 0;
    let mut rarest_reach_seen: Option<ReachTier> = None;

    loop {
        let dt = get_frame_time();
        session.update_time(((get_time() - session_start_time + (session.state.session_start_ms as f64 / 1000.0)) * 1000.0) as u64);
        rs.tick(dt);
        reach_timer = (reach_timer - dt).max(0.0);
        round_timer = (round_timer - dt).max(0.0);
        between_timer = (between_timer - dt).max(0.0);
        fanfare_timer = (fanfare_timer - dt).max(0.0);

        // Rebuild playfield if canvas resized OR layout was tuned
        let cur_size = (screen_width(), screen_height());
        if (cur_size.0 - last_canvas_size.0).abs() > 0.5
           || (cur_size.1 - last_canvas_size.1).abs() > 0.5
           || layout_dirty
        {
            pf = build_playfield_from_screen_geom();
            pins = playfield::pins_for_layout(&pf, &current_layout);
            last_canvas_size = cur_size;
            layout_dirty = false;
        }

        // Dismiss welcome-back card on any input (PRD-004 R-53)
        if rs.welcome_back_active && (is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::Enter)
            || is_mouse_button_pressed(MouseButton::Left)) {
            rs.welcome_back_active = false;
        }

        // R = session reset. Showing summary first (PRD-004 R-52); pressing R again starts fresh.
        if is_key_pressed(KeyCode::R) {
            if !rs.session_summary_active && session.state.total_spins > 0 {
                // First R: produce summary
                let now_ms = (get_time() * 1000.0) as u64;
                let duration = now_ms.saturating_sub(session_real_start_ms);
                let net_yen = (balls_won_total as i64 * session.spec.yen_per_ball as i64)
                            - (balls_fired_total as i64 * session.spec.yen_per_ball as i64);
                let mut narrative = Vec::new();
                if longest_dry_streak > 500 {
                    narrative.push(format!("You survived a {longest_dry_streak}-spin hama-dai."));
                }
                if session.state.total_jackpots > 0 {
                    narrative.push(format!("Highest chapter unlocked: {}.", session.state.unlocked_chapter));
                }
                if let Some(t) = rarest_reach_seen {
                    narrative.push(format!("Rarest reach tier seen: {:?}.", t));
                }
                let summary = persist::SessionSummary {
                    duration_ms: duration,
                    balls_fired: balls_fired_total,
                    balls_won: balls_won_total,
                    net_yen,
                    highest_chapter: session.state.unlocked_chapter,
                    longest_dry_streak,
                    rarest_reach_tier: rarest_reach_seen.map(|t| format!("{:?}", t).to_lowercase()),
                    narrative_lines: narrative,
                };
                rs.session_summary_active = true;
                rs.session_summary = Some(summary.clone());
                // Persist
                let to_persist = persist::PersistedState {
                    schema_version: 1,
                    layout: current_layout,
                    unlocked_chapter: session.state.unlocked_chapter,
                    last_summary: Some(summary),
                    last_session_at_ms: now_ms,
                };
                persist::save(&to_persist);
            } else {
                // Second R: fresh session
                session = Session::new(seed.wrapping_add(1), (get_time() * 1000.0) as u64);
                session_start_time = get_time();
                rs = RenderState::new();
                audio::stop(&bank.reach_bgm);
                audio::stop(&bank.kakuhen_bgm);
                audio::play_loop(&bank.base_bgm, 0.7);
                current_bgm = BgmTrack::Base;
                balls.clear();
                balls_fired_total = 0;
                balls_returned_total = 0;
                kakuhen_streak = 0;
                prev_total_jp = 0;
                longest_dry_streak = 0;
                rarest_reach_seen = None;
            }
        }

        // Data-lamp toggle (PRD-003 R-34) — H / Tab / Esc cycles
        if is_key_pressed(KeyCode::H) || is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Tab) {
            rs.data_lamp_visible = !rs.data_lamp_visible;
            if rs.data_lamp_visible { rs.data_lamp_glow_t = 0.0; }
            // Close workshop when lamp closes
            if !rs.data_lamp_visible { rs.workshop_active = false; }
        }

        // T = toggle tuning workshop (PRD-004 R-49). Available only at chapter >= 2.
        if is_key_pressed(KeyCode::T)
           && playfield::available_knob_count(session.state.unlocked_chapter) > 0 {
            rs.workshop_active = !rs.workshop_active;
            // Run an MC probe on open to populate the predicted ベース
            if rs.workshop_active {
                let (hits, total) = playfield::monte_carlo_chucker_rate(&pf, &current_layout, 100, 0xCAFE);
                let p = hits as f32 / total.max(1) as f32;
                let se = (p * (1.0 - p) / total.max(1) as f32).sqrt();
                rs.workshop_predicted_base = Some((p * 100.0, 1.96 * se * 100.0));
            }
        }
        // Right-click anywhere toggles data lamp (keyboard-free path)
        if is_mouse_button_pressed(MouseButton::Right) {
            rs.data_lamp_visible = !rs.data_lamp_visible;
        }

        // Workshop drag handling (PRD-004 R-46/R-47/R-49)
        if rs.workshop_active {
            let n_avail = playfield::available_knob_count(session.state.unlocked_chapter);
            let (mx, my) = mouse_position();
            // Workshop modal is rendered by scene::draw_workshop; we mirror its
            // slider geometry here for hit testing.
            let modal_x = screen_width() * 0.30;
            let modal_y = screen_height() * 0.22;
            let modal_w = screen_width() * 0.40;
            let slider_row_h = 56.0_f32;
            if is_mouse_button_pressed(MouseButton::Left) {
                for i in 0..n_avail {
                    let row_y = modal_y + 80.0 + i as f32 * slider_row_h;
                    let bar_x = modal_x + 12.0;
                    let bar_w = modal_w - 24.0;
                    if mx >= bar_x && mx <= bar_x + bar_w
                       && my >= row_y + 22.0 && my <= row_y + 38.0 {
                        rs.workshop_drag_knob = Some(i);
                    }
                }
            }
            if !is_mouse_button_down(MouseButton::Left) {
                // Release: run an MC probe to update predicted ベース
                if rs.workshop_drag_knob.is_some() {
                    layout_dirty = true;
                    let (hits, total) = playfield::monte_carlo_chucker_rate(&pf, &current_layout, 100, 0xBEEF);
                    let p = hits as f32 / total.max(1) as f32;
                    let se = (p * (1.0 - p) / total.max(1) as f32).sqrt();
                    rs.workshop_predicted_base = Some((p * 100.0, 1.96 * se * 100.0));
                }
                rs.workshop_drag_knob = None;
            }
            if let Some(idx) = rs.workshop_drag_knob {
                let bar_x = modal_x + 12.0;
                let bar_w = modal_w - 24.0;
                let frac = ((mx - bar_x) / bar_w).clamp(0.0, 1.0);
                let new_val = frac * 2.0 - 1.0; // [-1, +1]
                let knob = [
                    playfield::Knob::LeftFunnelTilt,
                    playfield::Knob::RightFunnelTilt,
                    playfield::Knob::ChuckerMouthWidth,
                    playfield::Knob::GuidePinVertical,
                    playfield::Knob::LowerRowDensity,
                    playfield::Knob::UpperFunnelSpread,
                ][idx];
                current_layout.set(knob, new_val);
                layout_dirty = true;
            }
        }

        // ---- LAUNCH (PRD-002 R-25) — hold SPACE / mouse to spawn balls ----
        // Pause launch when modal overlays are active (R-49, R-52, R-53).
        let modals_active = rs.workshop_active || rs.session_summary_active || rs.welcome_back_active;
        let space_held = (is_key_down(KeyCode::Space) || is_mouse_button_down(MouseButton::Left)) && !modals_active;
        let now_t = get_time();
        const LAUNCH_INTERVAL: f64 = 0.2; // 5 balls/sec
        if space_held && now_t - last_launch_t > LAUNCH_INTERVAL && balls.len() < 80 {
            let (lx, ly, vx, vy) = playfield::launcher_emit(&pf, balls_fired_total);
            balls.push(ball::Ball::new(lx, ly, vx, vy));
            balls_fired_total += 1;
            last_launch_t = now_t;
        }

        // ---- PHYSICS step (PRD-002 R-26) ----
        let phys = ball::step(&mut balls, &pins, &pf, dt);
        balls_returned_total += phys.chucker_entries;
        ball::prune(&mut balls);

        // ---- For each chucker entry, run the same handler as before (PRD-002 R-28) ----
        for _ in 0..phys.chucker_entries {
            // Chucker chime — PRD R-15
            audio::play_one(&bank.chucker_chime, 0.8);
            // Visual chucker-hit flash + treasure trickle (PRD-003 R-38, R-43)
            rs.trigger_chucker_flash(pf.chucker_cx, pf.chucker_cy);
            // New info — pulse the data-lamp glow even if hidden
            rs.trigger_data_lamp_glow();

            let ev = session.pull_chucker();
            match ev {
                SessionEvent::SpinResolved { outcome, reach_id: _ } => {
                    // Track rarest reach tier seen (for session summary)
                    if let Some(tier) = outcome.reach_tier {
                        let new_rank = match tier {
                            ReachTier::Calm => 1, ReachTier::Mid => 2,
                            ReachTier::Premium => 3, ReachTier::Confirmed => 4,
                        };
                        let cur_rank = rarest_reach_seen.map(|t| match t {
                            ReachTier::Calm => 1, ReachTier::Mid => 2,
                            ReachTier::Premium => 3, ReachTier::Confirmed => 4,
                        }).unwrap_or(0);
                        if new_rank > cur_rank { rarest_reach_seen = Some(tier); }
                    }
                    // Start reel spin animation. Reels 1 + 2 stagger; reel 3
                    // (the "reach" reel) holds longer if there's a reach.
                    let stops = match outcome.reach_tier {
                        None => [0.35, 0.55, 0.80],
                        Some(tier) => {
                            let dur = reach_duration_for(tier);
                            [0.35, 0.65, (dur - 0.35).max(1.0)]
                        }
                    };
                    rs.start_spin(stops);
                    if let Some(tier) = outcome.reach_tier {
                        rs.last_reach_tier = Some(tier);
                        let dur = reach_duration_for(tier);
                        reach_timer = dur;
                        rs.reach_t = dur;
                        // Crossfade BGM to reach
                        if current_bgm != BgmTrack::Reach {
                            audio::stop(&bank.base_bgm);
                            audio::stop(&bank.kakuhen_bgm);
                            audio::play_loop(&bank.reach_bgm, 0.75);
                            current_bgm = BgmTrack::Reach;
                        }
                        // Show banner text — tier name + a tiny story hint per tier.
                        match tier {
                            ReachTier::Calm => rs.show_overlay("CALM REACH  ::  flashback", dur),
                            ReachTier::Mid => rs.show_overlay("MID REACH  ::  preparing", dur),
                            ReachTier::Premium => rs.show_overlay("PREMIUM REACH  ::  the confrontation", dur),
                            ReachTier::Confirmed => {
                                rs.show_overlay("<<  IT  ENDS  TONIGHT  >>", dur);
                                audio::play_one(&bank.confirmed_cue, 1.0);
                                rs.shake(dur);
                            }
                        }
                        // Character cut-in for mid+ tiers (PRD-003 R-38)
                        rs.trigger_cutin(tier);
                    }
                }
                SessionEvent::JackpotStart => {
                    // Update longest dry streak (now broken)
                    if session.state.spins_since_last_jackpot > longest_dry_streak {
                        longest_dry_streak = session.state.spins_since_last_jackpot;
                    }
                    rs.snap_reels([7, 7, 7]);
                    rs.flash(0.5);
                    let payout = session.spec.rounds_per_jackpot as u64 * session.spec.balls_per_round as u64;
                    balls_won_total += payout;
                    let yen = payout * session.spec.yen_per_ball as u64;
                    // Streak counter — only meaningful inside kakuhen
                    if in_kakuhen_prev { kakuhen_streak += 1; } else { kakuhen_streak = 1; }
                    // Trigger FEVER letter-by-letter reveal (PRD-003 R-38)
                    rs.trigger_fever_reveal();
                    rs.show_overlay(format!("+{payout} BALLS  /  +¥{yen}"), 3.5);
                    audio::play_one(&bank.hit_fanfare, 1.0);
                    audio::play_one(&bank.jackpot_fanfare, 1.0);
                    fanfare_timer = 6.0;
                    rs.spawn_jackpot_particles(screen_width() * 0.5, screen_height() * 0.5);
                    rs.spawn_payout_trickle(pf.chucker_cx, pf.chucker_cy - 20.0, payout as u32);
                    rs.end_cutin();
                    let gap = session.state.total_spins as u32 - spins_at_last_jp;
                    spins_at_last_jp = session.state.total_spins as u32;
                    last_jp_history.insert(0, gap);
                    last_jp_history.truncate(10);
                    round_timer = 1.4;
                    rs.trigger_data_lamp_glow();
                }
                SessionEvent::EnterKakuhen | SessionEvent::ExitKakuhen | SessionEvent::JackpotEnd | SessionEvent::RoundAdvanced { .. } | SessionEvent::NoChange => {}
            }
        }

        // ---- Auto-resolve reach (after reach_timer ends, bust SFX if no jackpot followed) ----
        // The Session already advanced state machine inside pull_chucker; we just play SFX
        // when reach_timer ticks down to 0 if no jackpot took us elsewhere.
        if reach_timer == 0.0 && prev_state_was_reach(prev_state) && !matches!(session.coord.state, CabinetState::JackpotRound | CabinetState::BetweenRounds) {
            // bust played already? Only play once on transition.
            // Track with rs.reach_t hitting zero exactly.
        }

        // Round timing during jackpot
        if matches!(session.coord.state, CabinetState::JackpotRound) {
            if round_timer == 0.0 {
                session.round_complete();
                round_timer = 1.0;
                rs.round_t = 1.0;
            }
        }
        if matches!(session.coord.state, CabinetState::BetweenRounds) {
            // Tick the coordinator's between-rounds timer
            session.tick((dt * 1000.0) as u32);
        }

        // BGM crossfade on state change
        if session.coord.state != prev_state {
            handle_state_transition(prev_state, session.coord.state, &bank, &mut current_bgm, &mut rs);
            prev_state = session.coord.state;
        }

        // Chapter advancement — announce the new story beat.
        if session.state.unlocked_chapter > prev_chapter {
            prev_chapter = session.state.unlocked_chapter;
            let label = match prev_chapter {
                2 => "CHAPTER 2  ::  sharpening the blade",
                3 => "CHAPTER 3  ::  tracked to the warehouse",
                4 => "CHAPTER 4  ::  it ends tonight",
                _ => "NEW CHAPTER UNLOCKED",
            };
            // Full title-card wipe instead of plain text overlay (PRD-003 R-38)
            rs.trigger_chapter_card(label);
        }

        // ---- DRAW ----
        let kak_remaining = if matches!(session.coord.state, CabinetState::KakuhenBase | CabinetState::KakuhenReach) {
            session.spec.st_window.saturating_sub(session.coord.kakuhen_window_spins)
        } else { 0 };

        // Update kakuhen tracking for streak detection
        let in_kakuhen_now = matches!(session.coord.state, CabinetState::KakuhenBase | CabinetState::KakuhenReach);
        if !in_kakuhen_now && in_kakuhen_prev {
            kakuhen_streak = 0; // kakuhen ended
        }
        in_kakuhen_prev = in_kakuhen_now;
        let _ = prev_total_jp;

        let balls_fired_yen = balls_fired_total as i64 * session.spec.yen_per_ball as i64;
        let balls_won_yen = balls_won_total as i64 * session.spec.yen_per_ball as i64;
        let pl_yen = balls_won_yen - balls_fired_yen;

        render::draw_cabinet(
            &rs,
            session.coord.state,
            kak_remaining,
            session.spins_since_last_jackpot(),
            &last_jp_history,
            session.state.unlocked_chapter,
            balls_won_total,
            session.state.total_jackpots,
            &pf,
            &pins,
            &balls,
            balls_fired_total,
            balls_returned_total,
            space_held,
            pl_yen,
            kakuhen_streak,
            session.spec.yen_per_ball,
            session.state.total_spins,
            &current_layout,
        );

        next_frame().await;
    }
}

#[derive(Clone, Copy, PartialEq)]
enum BgmTrack { Base, Reach, Kakuhen }

fn build_playfield_from_screen_geom() -> ball::Playfield {
    let sw = screen_width();
    let sh = screen_height();
    let cab_w = sw * 0.72;
    let cab_h = sh * 0.82;
    let cab_x = sw * 0.5 - cab_w * 0.5;
    let cab_y = sh * 0.5 - cab_h * 0.5;
    playfield::build_playfield(cab_x, cab_y, cab_w, cab_h)
}

fn reach_duration_for(t: ReachTier) -> f32 {
    // Pacing tuned for auto-fire feel: short calms keep the base loop crisp;
    // premium/confirmed hold for catharsis. Real CR machines are slower but
    // also have a continuous ball stream filling the wait — we don't.
    match t {
        ReachTier::Calm => 1.4,
        ReachTier::Mid => 2.8,
        ReachTier::Premium => 4.5,
        ReachTier::Confirmed => 7.5,
    }
}

fn prev_state_was_reach(s: CabinetState) -> bool {
    matches!(s, CabinetState::Reach | CabinetState::KakuhenReach)
}

fn handle_state_transition(
    prev: CabinetState,
    curr: CabinetState,
    bank: &AudioBank,
    current_bgm: &mut BgmTrack,
    rs: &mut RenderState,
) {
    match (prev, curr) {
        (_, CabinetState::Base) if *current_bgm != BgmTrack::Base => {
            audio::stop(&bank.reach_bgm);
            audio::stop(&bank.kakuhen_bgm);
            audio::play_loop(&bank.base_bgm, 0.7);
            *current_bgm = BgmTrack::Base;
        }
        (_, CabinetState::KakuhenBase) if *current_bgm != BgmTrack::Kakuhen => {
            audio::stop(&bank.base_bgm);
            audio::stop(&bank.reach_bgm);
            audio::play_loop(&bank.kakuhen_bgm, 0.7);
            *current_bgm = BgmTrack::Kakuhen;
            // Per PRD-003 R-38: kakuhen entry → slam banner instead of plain overlay
            rs.trigger_kakuhen_slam();
            rs.flash(0.5);
        }
        (CabinetState::Reach, CabinetState::Base) | (CabinetState::KakuhenReach, CabinetState::KakuhenBase) => {
            audio::play_one(&bank.bust_sfx, 0.7);
        }
        _ => {}
    }
}
