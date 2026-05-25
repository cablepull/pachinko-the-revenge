#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod spec;
pub mod outcome;
pub mod probability;
pub mod state;
pub mod coordinator;
pub mod reach;
pub mod session;

#[cfg(feature = "std")]
pub mod save;

pub use spec::SpecSheet;
pub use outcome::{ReachTier, SpinOutcome};
pub use probability::ProbabilityEngine;
pub use state::GameState;
pub use coordinator::{CabinetCoordinator, CabinetEvent, CabinetState};
pub use reach::{Reach, ReachRoster, Beat, BeatVisual, BeatAudio};
pub use session::Session;

pub use rand_core::{RngCore, SeedableRng};
pub use rand_pcg::Pcg64Mcg;

/// Re-export the deterministic RNG type used throughout the math layer.
/// Pcg64Mcg is chosen for: small state, fast, deterministic, period 2^126,
/// passes Crush statistical tests. Per [C-6, C-14].
pub type Rng = Pcg64Mcg;
