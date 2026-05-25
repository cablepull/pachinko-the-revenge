//! pachinko-the-revenge — game entry.
//!
//! Drives the headless `pachinko-core` session with macroquad's run loop,
//! input, rendering, and procedural audio. Targets native + WASM.

use macroquad::prelude::*;
use pachinko_core::coordinator::CabinetState;
use pachinko_core::outcome::ReachTier;
use pachinko_core::session::{Session, SessionEvent};

mod audio;
mod render;
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
    let bank = AudioBank::build().await;

    let seed = (get_time() * 1_000.0) as u64 ^ 0xCAFE_F00D;
    let mut session = Session::new(seed, (get_time() * 1000.0) as u64);
    if let Some(saved) = persist::load() {
        session.state = saved.state;
        session.coord = saved.coord;
    }
    let mut rs = RenderState::new();

    // BGM state
    let mut current_bgm = BgmTrack::Base;
    audio::play_loop(&bank.base_bgm, 0.7);

    // Reach / round timing state
    let mut reach_timer: f32 = 0.0;
    let mut round_timer: f32 = 0.0;
    let mut between_timer: f32 = 0.0;
    let mut fanfare_timer: f32 = 0.0;

    let mut prev_state = session.coord.state;
    let mut last_jp_history: Vec<u32> = Vec::new();
    let mut spins_at_last_jp: u32 = 0;

    let mut launch_cooldown: f32 = 0.0;
    let mut session_start_time = get_time();

    loop {
        let dt = get_frame_time();
        session.update_time(((get_time() - session_start_time + (session.state.session_start_ms as f64 / 1000.0)) * 1000.0) as u64);
        rs.tick(dt);
        launch_cooldown = (launch_cooldown - dt).max(0.0);
        reach_timer = (reach_timer - dt).max(0.0);
        round_timer = (round_timer - dt).max(0.0);
        between_timer = (between_timer - dt).max(0.0);
        fanfare_timer = (fanfare_timer - dt).max(0.0);

        // ---- INPUT (PRD R-16: fanfare uninterruptible) ----
        let can_input = fanfare_timer <= 0.0 && reach_timer <= 0.0;
        let pull_requested = (is_key_pressed(KeyCode::Space)
            || is_mouse_button_pressed(MouseButton::Left))
            && can_input;

        if is_key_pressed(KeyCode::R) {
            session = Session::new(seed.wrapping_add(1), (get_time() * 1000.0) as u64);
            session_start_time = get_time();
            rs = RenderState::new();
            audio::stop(&bank.reach_bgm);
            audio::stop(&bank.kakuhen_bgm);
            audio::play_loop(&bank.base_bgm, 0.7);
            current_bgm = BgmTrack::Base;
        }

        // Auto-fire mode: hold space to keep pulling (with a cooldown)
        let auto_fire = is_key_down(KeyCode::Space) && launch_cooldown <= 0.0 && can_input;
        let do_pull = pull_requested || auto_fire;

        if do_pull && matches!(session.coord.state, CabinetState::Base | CabinetState::KakuhenBase) {
            launch_cooldown = 0.18; // ~5.5 spins/sec
            // Chucker chime — PRD R-15
            audio::play_one(&bank.chucker_chime, 0.8);

            let ev = session.pull_chucker();
            match ev {
                SessionEvent::SpinResolved { outcome, reach_id: _ } => {
                    rs.reel_offsets = [0.0, 0.0, 0.0];
                    rs.reel_targets = [None; 3];
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
                        // Show banner text
                        match tier {
                            ReachTier::Calm => rs.show_overlay("REACH . . .", dur),
                            ReachTier::Mid => rs.show_overlay("REACH !!", dur),
                            ReachTier::Premium => rs.show_overlay("PREMIUM REACH !!!", dur),
                            ReachTier::Confirmed => {
                                rs.show_overlay("<<  IT  ENDS  TONIGHT  >>", dur);
                                audio::play_one(&bank.confirmed_cue, 1.0);
                                rs.shake(dur);
                            }
                        }
                    }
                }
                SessionEvent::JackpotStart => {
                    rs.reel_targets = [Some(7); 3];
                    rs.flash(0.5);
                    rs.show_overlay("F E V E R !!", 2.5);
                    audio::play_one(&bank.hit_fanfare, 1.0);
                    audio::play_one(&bank.jackpot_fanfare, 1.0);
                    fanfare_timer = 6.0;
                    rs.spawn_jackpot_particles(screen_width() * 0.5, screen_height() * 0.5);
                    // Record JP gap
                    let gap = session.state.total_spins as u32 - spins_at_last_jp;
                    spins_at_last_jp = session.state.total_spins as u32;
                    last_jp_history.insert(0, gap);
                    last_jp_history.truncate(10);
                    round_timer = 1.4;
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

        // ---- DRAW ----
        let kak_remaining = if matches!(session.coord.state, CabinetState::KakuhenBase | CabinetState::KakuhenReach) {
            session.spec.st_window.saturating_sub(session.coord.kakuhen_window_spins)
        } else { 0 };

        render::draw_cabinet(
            &rs,
            session.coord.state,
            kak_remaining,
            session.spins_since_last_jackpot(),
            &last_jp_history,
            session.state.unlocked_chapter,
        );

        // Session stats overlay (small text top-left below state)
        draw_text(
            &format!("spins {}  /  JP {}  /  chapter {}",
                session.state.total_spins,
                session.state.total_jackpots,
                session.state.unlocked_chapter),
            14.0, 44.0, 16.0,
            Color::new(0.6, 0.85, 1.0, 0.8),
        );

        next_frame().await;
    }
}

#[derive(Clone, Copy, PartialEq)]
enum BgmTrack { Base, Reach, Kakuhen }

fn reach_duration_for(t: ReachTier) -> f32 {
    match t {
        ReachTier::Calm => 2.2,
        ReachTier::Mid => 3.8,
        ReachTier::Premium => 5.5,
        ReachTier::Confirmed => 8.0,
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
            rs.show_overlay("KAKUHEN  START!!", 2.5);
            rs.flash(0.5);
        }
        (CabinetState::Reach, CabinetState::Base) | (CabinetState::KakuhenReach, CabinetState::KakuhenBase) => {
            audio::play_one(&bank.bust_sfx, 0.7);
        }
        _ => {}
    }
}
