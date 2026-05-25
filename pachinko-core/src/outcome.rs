//! Outcomes returned by `ProbabilityEngine::spin`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReachTier {
    Calm,
    Mid,
    Premium,
    Confirmed,
}

impl ReachTier {
    pub const fn name(self) -> &'static str {
        match self {
            ReachTier::Calm => "calm",
            ReachTier::Mid => "mid",
            ReachTier::Premium => "premium",
            ReachTier::Confirmed => "confirmed",
        }
    }

    pub const fn all() -> [ReachTier; 4] {
        [Self::Calm, Self::Mid, Self::Premium, Self::Confirmed]
    }
}

/// Per PRD R-1: spin produces a deterministic outcome from state + RNG.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SpinOutcome {
    pub is_jackpot: bool,
    /// None = no reach animation (either silent base spin or a direct hit).
    pub reach_tier: Option<ReachTier>,
    /// Only meaningful when is_jackpot=true. Per PRD R-5.
    pub entered_kakuhen: bool,
}

impl SpinOutcome {
    pub const fn quiet_miss() -> Self {
        Self { is_jackpot: false, reach_tier: None, entered_kakuhen: false }
    }
}
