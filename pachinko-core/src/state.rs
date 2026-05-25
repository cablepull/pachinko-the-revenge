//! GameState — the canonical state passed into `ProbabilityEngine::spin`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    /// True iff in kakuhen mode (high jackpot probability window).
    pub in_kakuhen: bool,
    /// Spins elapsed in current kakuhen window. Reset on entry and exit.
    pub kakuhen_window_spins: u32,
    /// Total spins this session.
    pub total_spins: u64,
    /// Total jackpots this session.
    pub total_jackpots: u32,
    /// Spins since last jackpot (the "data lamp" ハマり counter). Per PRD R-21.
    pub spins_since_last_jackpot: u32,
    /// Story chapter unlocked (0..=N). Gates reach eligibility per PRD R-11.
    pub unlocked_chapter: u32,
    /// Session start time in milliseconds since UNIX_EPOCH (or arbitrary monotonic
    /// origin for non-web platforms). Used by PRD R-12 (no premium reach for 5 min).
    pub session_start_ms: u64,
    /// Current time in ms (caller updates each frame). Same origin as session_start_ms.
    pub now_ms: u64,
}

impl GameState {
    pub fn new_session(now_ms: u64) -> Self {
        Self {
            in_kakuhen: false,
            kakuhen_window_spins: 0,
            total_spins: 0,
            total_jackpots: 0,
            spins_since_last_jackpot: 0,
            unlocked_chapter: 1,
            session_start_ms: now_ms,
            now_ms,
        }
    }

    /// Per PRD R-12: minutes elapsed since session start.
    pub fn session_elapsed_ms(&self) -> u64 {
        self.now_ms.saturating_sub(self.session_start_ms)
    }
}
