//! Canonical spec-sheet for the pachinko cabinet.
//!
//! Per intent C-2: numbers here are the canonical machine spec and are
//! verifiable via Monte Carlo. Per C-12: reach roster is config-driven —
//! this struct holds the tier-level probabilities; the named-reach roster
//! lives in `reach.rs`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TierProbs {
    /// Probability of this tier occurring on a base-state spin (0.0..=1.0).
    pub freq_base: f64,
    /// Probability of this tier occurring on a kakuhen-state spin (0.0..=1.0).
    pub freq_kakuhen: f64,
    /// Probability of hit *given* this tier was rolled (0.0..=1.0).
    /// Bust rate is implicitly (1.0 - hit_rate).
    pub hit_rate: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SpecSheet {
    /// "Direct hit" jackpot probability per spin in base state.
    /// A direct hit produces a jackpot with no reach animation —
    /// authentic to real CR machines.
    pub direct_hit_base: f64,
    /// "Direct hit" jackpot probability per spin in kakuhen state.
    pub direct_hit_kakuhen: f64,

    pub calm: TierProbs,
    pub mid: TierProbs,
    pub premium: TierProbs,
    pub confirmed: TierProbs,

    /// Probability a jackpot enters kakuhen. Per PRD R-5: 70%.
    pub kakuhen_entry_rate: f64,

    /// ST kakuhen window length in spins. Per PRD R-4: 165.
    pub st_window: u32,

    /// Round count per jackpot. Per PRD §4: 16R MVP.
    pub rounds_per_jackpot: u32,

    /// Balls per round. Per PRD §4: ~90.
    pub balls_per_round: u32,

    /// Attacker reset duration between rounds (ms). Per PRD R-9: 1500.
    pub between_rounds_ms: u32,
}

impl SpecSheet {
    /// Returns the canonical spec sheet derived in intent.md and PRD §4.
    ///
    /// Frequencies are tuned so that:
    /// - Total base jackpot probability = 1/199.8 (PRD R-2)
    /// - Total kakuhen jackpot probability = 1/35.9 (PRD R-3)
    /// - Tier reach frequencies match PRD §4 (PRD R-6)
    /// - Tier hit rates match PRD §4 bust column (PRD R-7)
    ///
    /// Reconciliation: reach hits sum to 0.4475%; direct-hit fills to 0.5005% = 1/199.8.
    pub const fn canonical() -> Self {
        Self {
            direct_hit_base: 0.000530,
            direct_hit_kakuhen: 0.002951,

            // Base reach frequencies match PRD §4 table.
            // Kakuhen frequencies scaled by 5.566 to hit 1/35.9 total.
            calm: TierProbs {
                freq_base: 0.0250,
                freq_kakuhen: 0.1392,
                hit_rate: 0.02,
            },
            mid: TierProbs {
                freq_base: 0.0070,
                freq_kakuhen: 0.0390,
                hit_rate: 0.25,
            },
            premium: TierProbs {
                freq_base: 0.0025,
                freq_kakuhen: 0.01392,
                hit_rate: 0.70,
            },
            confirmed: TierProbs {
                freq_base: 0.0005,
                freq_kakuhen: 0.00278,
                hit_rate: 0.95,
            },

            kakuhen_entry_rate: 0.70,
            st_window: 165,
            rounds_per_jackpot: 16,
            balls_per_round: 90,
            between_rounds_ms: 1500,
        }
    }

    /// Sum of base-state reach + direct-hit probabilities, for verification.
    pub fn base_jackpot_probability(&self) -> f64 {
        self.direct_hit_base
            + self.calm.freq_base * self.calm.hit_rate
            + self.mid.freq_base * self.mid.hit_rate
            + self.premium.freq_base * self.premium.hit_rate
            + self.confirmed.freq_base * self.confirmed.hit_rate
    }

    pub fn kakuhen_jackpot_probability(&self) -> f64 {
        self.direct_hit_kakuhen
            + self.calm.freq_kakuhen * self.calm.hit_rate
            + self.mid.freq_kakuhen * self.mid.hit_rate
            + self.premium.freq_kakuhen * self.premium.hit_rate
            + self.confirmed.freq_kakuhen * self.confirmed.hit_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Per PRD R-2: base jackpot probability ≈ 1/199.8 = 0.005005.
    #[test]
    fn canonical_base_rate_is_one_in_199_8() {
        let s = SpecSheet::canonical();
        let p = s.base_jackpot_probability();
        let target = 1.0 / 199.8;
        assert!((p - target).abs() < 1e-4, "base p={p} target={target}");
    }

    /// Per PRD R-3: kakuhen jackpot probability ≈ 1/35.9 = 0.027855.
    #[test]
    fn canonical_kakuhen_rate_is_one_in_35_9() {
        let s = SpecSheet::canonical();
        let p = s.kakuhen_jackpot_probability();
        let target = 1.0 / 35.9;
        assert!((p - target).abs() < 1e-3, "kakuhen p={p} target={target}");
    }
}
