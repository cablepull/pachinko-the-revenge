//! High-level Session: ties Probability + Coordinator + Reach roster together.
//!
//! The game layer drives this. Headless tests of the math layer call directly
//! into the lower-level modules.

use alloc::string::String;
use rand_core::{RngCore, SeedableRng};

use crate::coordinator::{CabinetCoordinator, CabinetState};
use crate::outcome::{ReachTier, SpinOutcome};
use crate::probability::ProbabilityEngine;
use crate::reach::{Reach, ReachRoster};
use crate::spec::SpecSheet;
use crate::state::GameState;
use crate::Rng;

/// Aggregated event emitted to the game/UI layer after each spin or tick.
#[derive(Debug, Clone, PartialEq)]
pub enum SessionEvent {
    NoChange,
    SpinResolved {
        outcome: SpinOutcome,
        reach_id: Option<String>,
    },
    EnterKakuhen,
    ExitKakuhen,
    JackpotStart,
    RoundAdvanced { round: u32, of: u32 },
    JackpotEnd,
}

pub struct Session {
    pub spec: SpecSheet,
    pub roster: ReachRoster,
    pub state: GameState,
    pub coord: CabinetCoordinator,
    pub rng: Rng,
    /// The id of the currently-playing reach (drives AVC animation lookup).
    pub current_reach_id: Option<String>,
    /// The pre-rolled jackpot bool of the current reach (R-13).
    pub current_reach_will_hit: Option<bool>,
}

impl Session {
    pub fn new(seed: u64, now_ms: u64) -> Self {
        Self {
            spec: SpecSheet::canonical(),
            roster: ReachRoster::canonical(),
            state: GameState::new_session(now_ms),
            coord: CabinetCoordinator::new(),
            rng: Rng::seed_from_u64(seed),
            current_reach_id: None,
            current_reach_will_hit: None,
        }
    }

    pub fn update_time(&mut self, now_ms: u64) {
        self.state.now_ms = now_ms;
    }

    /// Per PRD R-11: bump unlocked_chapter as jackpots accumulate.
    fn maybe_unlock_chapter(&mut self) {
        // Simple staircase: 1 from start, 2 after first jackpot, 3 after third,
        // 4 (confirmed eligible) after fifth.
        let next = match self.state.total_jackpots {
            0 => 1,
            1..=2 => 2,
            3..=4 => 3,
            _ => 4,
        };
        if next > self.state.unlocked_chapter {
            self.state.unlocked_chapter = next;
        }
    }

    /// Drive the cabinet from the chucker: produce a spin, run the math,
    /// pick a named reach if applicable, advance the state machine.
    pub fn pull_chucker(&mut self) -> SessionEvent {
        // Only valid in Base or KakuhenBase
        if !matches!(self.coord.state, CabinetState::Base | CabinetState::KakuhenBase) {
            return SessionEvent::NoChange;
        }

        // Sync coord -> state.in_kakuhen
        self.state.in_kakuhen = matches!(self.coord.state, CabinetState::KakuhenBase);

        let pe = ProbabilityEngine::new(&self.spec);
        let outcome = pe.spin(&self.state, &mut self.rng);

        self.state.total_spins += 1;
        if !outcome.is_jackpot {
            self.state.spins_since_last_jackpot += 1;
        }

        // Pick named reach + pre-roll hit (R-13)
        let reach_id = if let Some(tier) = outcome.reach_tier {
            self.current_reach_will_hit = Some(outcome.is_jackpot);
            self.roster.select(tier, &self.state, &mut self.rng)
                .map(|r: &Reach| {
                    self.current_reach_id = Some(r.id.clone());
                    r.id.clone()
                })
        } else if outcome.is_jackpot {
            // Direct hit: no reach animation.
            self.current_reach_will_hit = None;
            self.current_reach_id = None;
            None
        } else {
            self.current_reach_will_hit = None;
            self.current_reach_id = None;
            None
        };

        let prev_state = self.coord.state;
        self.coord.on_spin(outcome);
        let new_state = self.coord.state;

        let mut emit = SessionEvent::SpinResolved { outcome, reach_id: reach_id.clone() };

        // Detect kakuhen exit (window-exhausted path)
        if matches!(prev_state, CabinetState::KakuhenBase | CabinetState::KakuhenReach)
            && matches!(new_state, CabinetState::Base)
        {
            emit = SessionEvent::ExitKakuhen;
        }

        if outcome.is_jackpot {
            self.state.total_jackpots += 1;
            self.state.spins_since_last_jackpot = 0;
            self.maybe_unlock_chapter();
            emit = SessionEvent::JackpotStart;
        }

        emit
    }

    /// Called by the game layer when a round of payout animation completes.
    pub fn round_complete(&mut self) -> SessionEvent {
        let prev = self.coord.state;
        self.coord.round_complete(&self.spec);
        if prev == CabinetState::JackpotRound && self.coord.state == CabinetState::BetweenRounds {
            SessionEvent::RoundAdvanced { round: self.coord.current_round, of: self.spec.rounds_per_jackpot }
        } else {
            SessionEvent::NoChange
        }
    }

    pub fn tick(&mut self, dt_ms: u32) -> SessionEvent {
        let prev = self.coord.state;
        self.coord.tick_between_rounds(dt_ms, &self.spec);
        let now = self.coord.state;

        if prev == CabinetState::BetweenRounds && now == CabinetState::Base {
            SessionEvent::JackpotEnd
        } else if prev == CabinetState::BetweenRounds && now == CabinetState::KakuhenBase {
            SessionEvent::EnterKakuhen
        } else {
            SessionEvent::NoChange
        }
    }

    /// PRD R-21: HUD reads spins_since_last_jackpot from here.
    pub fn spins_since_last_jackpot(&self) -> u32 {
        self.state.spins_since_last_jackpot
    }
}
