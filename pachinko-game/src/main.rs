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
    let mut prev_chapter = session.state.unlocked_chapter;
    let mut last_jp_history: Vec<u32> = Vec::new();
    let mut spins_at_last_jp: u32 = 0;
    let mut balls_won_total: u64 = 0;

    let mut session_start_time = get_time();

    // ---- Ball physics state (PRD-002 F-2) ----
    // Compute the playfield from current canvas dimensions.
    let (mut pf, mut pins) = build_playfield_from_screen();
    let mut balls: Vec<ball::Ball> = Vec::with_capacity(80);
    let mut last_launch_t: f64 = 0.0;
    let mut balls_fired_total: u32 = 0;
    let mut balls_returned_total: u32 = 0;
    let mut last_canvas_size = (screen_width(), screen_height());

    loop {
        let dt = get_frame_time();
        session.update_time(((get_time() - session_start_time + (session.state.session_start_ms as f64 / 1000.0)) * 1000.0) as u64);
        rs.tick(dt);
        reach_timer = (reach_timer - dt).max(0.0);
        round_timer = (round_timer - dt).max(0.0);
        between_timer = (between_timer - dt).max(0.0);
        fanfare_timer = (fanfare_timer - dt).max(0.0);

        // Rebuild playfield if canvas resized
        let cur_size = (screen_width(), screen_height());
        if (cur_size.0 - last_canvas_size.0).abs() > 0.5 || (cur_size.1 - last_canvas_size.1).abs() > 0.5 {
            let (new_pf, new_pins) = build_playfield_from_screen();
            pf = new_pf;
            pins = new_pins;
            last_canvas_size = cur_size;
        }

        if is_key_pressed(KeyCode::R) {
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
        }

        // ---- LAUNCH (PRD-002 R-25) — hold SPACE / mouse to spawn balls ----
        let space_held = is_key_down(KeyCode::Space) || is_mouse_button_down(MouseButton::Left);
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

            let ev = session.pull_chucker();
            match ev {
                SessionEvent::SpinResolved { outcome, reach_id: _ } => {
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
                        // (Players learn the hierarchy by watching which one fires.)
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
                    }
                }
                SessionEvent::JackpotStart => {
                    rs.snap_reels([7, 7, 7]);
                    rs.flash(0.5);
                    let payout = session.spec.rounds_per_jackpot as u64 * session.spec.balls_per_round as u64;
                    balls_won_total += payout;
                    rs.show_overlay(format!("F E V E R !!   +{payout} BALLS"), 2.8);
                    audio::play_one(&bank.hit_fanfare, 1.0);
                    audio::play_one(&bank.jackpot_fanfare, 1.0);
                    fanfare_timer = 6.0;
                    rs.spawn_jackpot_particles(screen_width() * 0.5, screen_height() * 0.5);
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

        // Chapter advancement — announce the new story beat.
        if session.state.unlocked_chapter > prev_chapter {
            prev_chapter = session.state.unlocked_chapter;
            let label = match prev_chapter {
                2 => "CHAPTER 2  ::  sharpening the blade",
                3 => "CHAPTER 3  ::  tracked to the warehouse",
                4 => "CHAPTER 4  ::  it ends tonight",
                _ => "NEW CHAPTER UNLOCKED",
            };
            rs.show_overlay(label.to_string(), 3.0);
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
            balls_won_total,
            session.state.total_jackpots,
            &pf,
            &pins,
            &balls,
            balls_fired_total,
            balls_returned_total,
            space_held,
        );

        // Session stats overlay (small text top-left below state)
        draw_text(
            &format!("spins {}  /  JP {}  /  chapter {}  /  balls won {}",
                session.state.total_spins,
                session.state.total_jackpots,
                session.state.unlocked_chapter,
                balls_won_total),
            14.0, 44.0, 16.0,
            Color::new(0.6, 0.85, 1.0, 0.8),
        );

        next_frame().await;
    }
}

#[derive(Clone, Copy, PartialEq)]
enum BgmTrack { Base, Reach, Kakuhen }

fn build_playfield_from_screen() -> (ball::Playfield, Vec<ball::Pin>) {
    let sw = screen_width();
    let sh = screen_height();
    // Mirror render.rs's cabinet rect computation.
    let cab_w = sw * 0.72;
    let cab_h = sh * 0.82;
    let cab_x = sw * 0.5 - cab_w * 0.5;
    let cab_y = sh * 0.5 - cab_h * 0.5;
    let pf = playfield::build_playfield(cab_x, cab_y, cab_w, cab_h);
    let pins = playfield::canonical_pins(&pf);
    (pf, pins)
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
            rs.show_overlay("KAKUHEN  START!!", 2.5);
            rs.flash(0.5);
        }
        (CabinetState::Reach, CabinetState::Base) | (CabinetState::KakuhenReach, CabinetState::KakuhenBase) => {
            audio::play_one(&bank.bust_sfx, 0.7);
        }
        _ => {}
    }
}
