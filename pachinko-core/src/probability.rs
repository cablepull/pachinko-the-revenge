//! ProbabilityEngine — the pure RNG-driven outcome producer.
//!
//! Per intent C-1: depends on nothing except rand_core and our own types.
//! Per intent C-6 and PRD R-1: deterministic given (state, rng).

use rand_core::RngCore;

use crate::outcome::{ReachTier, SpinOutcome};
use crate::spec::{SpecSheet, TierProbs};
use crate::state::GameState;

pub struct ProbabilityEngine<'s> {
    spec: &'s SpecSheet,
}

impl<'s> ProbabilityEngine<'s> {
    pub fn new(spec: &'s SpecSheet) -> Self {
        Self { spec }
    }

    pub fn spec(&self) -> &SpecSheet {
        self.spec
    }

    /// Per PRD R-1: spin is a pure function of (state, rng).
    ///
    /// Picks an outcome category using one u32 from rng, optionally a second u32
    /// for the kakuhen-entry roll if a jackpot occurs (per R-5).
    pub fn spin(&self, state: &GameState, rng: &mut dyn RngCore) -> SpinOutcome {
        let in_kakuhen = state.in_kakuhen;
        let u = next_f64(rng);

        // Build cumulative thresholds in a fixed order so the distribution
        // is byte-stable across builds. Order: direct-hit, then (confirmed,
        // premium, mid, calm) — high-to-low so common branches are cheap.
        let (direct, calm, mid, premium, confirmed) = if in_kakuhen {
            (
                self.spec.direct_hit_kakuhen,
                tier_freq_kakuhen(&self.spec.calm),
                tier_freq_kakuhen(&self.spec.mid),
                tier_freq_kakuhen(&self.spec.premium),
                tier_freq_kakuhen(&self.spec.confirmed),
            )
        } else {
            (
                self.spec.direct_hit_base,
                tier_freq_base(&self.spec.calm),
                tier_freq_base(&self.spec.mid),
                tier_freq_base(&self.spec.premium),
                tier_freq_base(&self.spec.confirmed),
            )
        };

        let mut cum = 0.0_f64;

        cum += direct;
        if u < cum {
            return self.jackpot(rng, None);
        }

        cum += confirmed * self.spec.confirmed.hit_rate;
        if u < cum {
            return self.jackpot(rng, Some(ReachTier::Confirmed));
        }
        cum += confirmed * (1.0 - self.spec.confirmed.hit_rate);
        if u < cum {
            return SpinOutcome { is_jackpot: false, reach_tier: Some(ReachTier::Confirmed), entered_kakuhen: false };
        }

        cum += premium * self.spec.premium.hit_rate;
        if u < cum {
            return self.jackpot(rng, Some(ReachTier::Premium));
        }
        cum += premium * (1.0 - self.spec.premium.hit_rate);
        if u < cum {
            return SpinOutcome { is_jackpot: false, reach_tier: Some(ReachTier::Premium), entered_kakuhen: false };
        }

        cum += mid * self.spec.mid.hit_rate;
        if u < cum {
            return self.jackpot(rng, Some(ReachTier::Mid));
        }
        cum += mid * (1.0 - self.spec.mid.hit_rate);
        if u < cum {
            return SpinOutcome { is_jackpot: false, reach_tier: Some(ReachTier::Mid), entered_kakuhen: false };
        }

        cum += calm * self.spec.calm.hit_rate;
        if u < cum {
            return self.jackpot(rng, Some(ReachTier::Calm));
        }
        cum += calm * (1.0 - self.spec.calm.hit_rate);
        if u < cum {
            return SpinOutcome { is_jackpot: false, reach_tier: Some(ReachTier::Calm), entered_kakuhen: false };
        }

        SpinOutcome::quiet_miss()
    }

    fn jackpot(&self, rng: &mut dyn RngCore, reach: Option<ReachTier>) -> SpinOutcome {
        // PRD R-5: independent 70% roll for kakuhen entry on every jackpot.
        let entered = next_f64(rng) < self.spec.kakuhen_entry_rate;
        SpinOutcome { is_jackpot: true, reach_tier: reach, entered_kakuhen: entered }
    }
}

#[inline]
fn tier_freq_base(t: &TierProbs) -> f64 { t.freq_base }
#[inline]
fn tier_freq_kakuhen(t: &TierProbs) -> f64 { t.freq_kakuhen }

/// Map next_u32() into [0.0, 1.0) with 24 bits of precision.
/// Splitting into u32 keeps the RNG byte-stream consumption predictable.
#[inline]
fn next_f64(rng: &mut dyn RngCore) -> f64 {
    let u = rng.next_u32();
    (u as f64) / (u32::MAX as f64 + 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rng;
    use rand_core::SeedableRng;

    fn engine() -> (SpecSheet, GameState) {
        (SpecSheet::canonical(), GameState::new_session(0))
    }

    /// PRD R-1: same seed + same state → same output sequence.
    #[test]
    fn determinism_across_replays() {
        let (spec, state) = engine();
        let pe = ProbabilityEngine::new(&spec);

        let mut r1 = Rng::seed_from_u64(0xDEAD_BEEF);
        let mut r2 = Rng::seed_from_u64(0xDEAD_BEEF);

        for _ in 0..10_000 {
            let a = pe.spin(&state, &mut r1);
            let b = pe.spin(&state, &mut r2);
            assert_eq!(a, b);
        }
    }

    /// PRD R-1: spin does not mutate any module-scoped state.
    /// (Verified structurally: ProbabilityEngine holds only &SpecSheet,
    /// has no &mut self methods, takes &state.)
    #[test]
    fn purity_no_engine_mutation() {
        let (spec, state) = engine();
        let pe = ProbabilityEngine::new(&spec);
        let mut rng = Rng::seed_from_u64(1);
        let _ = pe.spin(&state, &mut rng);
        // If `pe` required &mut self, this line wouldn't compile after the call.
        let _ = pe.spin(&state, &mut rng);
    }

    /// PRD R-2: base jackpot rate is ~1/199.8 over 1M spins.
    /// Tolerance: ±3σ. For p=0.005005 over N=1e6, σ≈70.6 → 3σ≈212.
    #[test]
    fn base_rate_monte_carlo_1m() {
        let (spec, state) = engine();
        let pe = ProbabilityEngine::new(&spec);
        let mut rng = Rng::seed_from_u64(0x1234_5678);
        let n = 1_000_000;
        let mut jackpots = 0u32;
        for _ in 0..n {
            if pe.spin(&state, &mut rng).is_jackpot { jackpots += 1; }
        }
        let expected = (n as f64 * spec.base_jackpot_probability()) as i64; // ~5005
        let three_sigma = 212i64;
        let lo = expected - three_sigma;
        let hi = expected + three_sigma;
        assert!((lo..=hi).contains(&(jackpots as i64)),
            "jackpots={jackpots} expected={expected} ±{three_sigma}");
    }

    /// PRD R-3: kakuhen jackpot rate is ~1/35.9 over 100k spins.
    /// Tolerance: ±3σ. For p=0.02786 over N=1e5, σ≈52 → 3σ≈156.
    #[test]
    fn kakuhen_rate_monte_carlo_100k() {
        let (spec, _) = engine();
        let mut state = GameState::new_session(0);
        state.in_kakuhen = true;
        let pe = ProbabilityEngine::new(&spec);
        let mut rng = Rng::seed_from_u64(0xCAFE_F00D);
        let n = 100_000;
        let mut jackpots = 0u32;
        for _ in 0..n {
            if pe.spin(&state, &mut rng).is_jackpot { jackpots += 1; }
        }
        let expected = (n as f64 * spec.kakuhen_jackpot_probability()) as i64; // ~2786
        let three_sigma = 156i64;
        let lo = expected - three_sigma;
        let hi = expected + three_sigma;
        assert!((lo..=hi).contains(&(jackpots as i64)),
            "kakuhen jackpots={jackpots} expected={expected} ±{three_sigma}");
    }

    /// PRD R-5: kakuhen entry rate is ~70% over 10k jackpots.
    /// Tolerance: ±3σ. For p=0.7 over N=1e4, σ≈45.8 → 3σ≈138.
    #[test]
    fn kakuhen_entry_rate_70_percent() {
        let (spec, state) = engine();
        let pe = ProbabilityEngine::new(&spec);
        let mut rng = Rng::seed_from_u64(0x55AA_55AA);
        let target_jackpots = 10_000;
        let mut jackpots = 0u32;
        let mut kakuhen_entries = 0u32;
        while jackpots < target_jackpots {
            let o = pe.spin(&state, &mut rng);
            if o.is_jackpot {
                jackpots += 1;
                if o.entered_kakuhen { kakuhen_entries += 1; }
            }
        }
        let expected = (target_jackpots as f64 * spec.kakuhen_entry_rate) as i64;
        let three_sigma = 138i64;
        let diff = (kakuhen_entries as i64 - expected).abs();
        assert!(diff <= three_sigma,
            "kakuhen_entries={kakuhen_entries} expected={expected} 3σ={three_sigma}");
    }

    /// PRD R-6: reach tier distribution matches canonical frequencies over 1M spins.
    /// Tolerance: ±3σ per tier.
    #[test]
    fn reach_tier_distribution_1m() {
        let (spec, state) = engine();
        let pe = ProbabilityEngine::new(&spec);
        let mut rng = Rng::seed_from_u64(0xABCD_EF01);
        let n = 1_000_000;
        let mut count = [0u32; 4]; // calm, mid, premium, confirmed
        for _ in 0..n {
            let o = pe.spin(&state, &mut rng);
            match o.reach_tier {
                Some(ReachTier::Calm) => count[0] += 1,
                Some(ReachTier::Mid) => count[1] += 1,
                Some(ReachTier::Premium) => count[2] += 1,
                Some(ReachTier::Confirmed) => count[3] += 1,
                None => {}
            }
        }
        let tiers = [
            ("calm", spec.calm.freq_base, count[0]),
            ("mid", spec.mid.freq_base, count[1]),
            ("premium", spec.premium.freq_base, count[2]),
            ("confirmed", spec.confirmed.freq_base, count[3]),
        ];
        for (name, p, c) in tiers {
            let expected = (n as f64 * p) as i64;
            let sigma = (n as f64 * p * (1.0 - p)).sqrt();
            let three_sigma = (3.0 * sigma).ceil() as i64;
            let diff = (c as i64 - expected).abs();
            assert!(diff <= three_sigma,
                "tier={name} count={c} expected={expected} 3σ={three_sigma}");
        }
    }

    /// PRD R-7: per-tier hit rate matches `hit_rate` in spec.
    /// Uses kakuhen-state spins to gather confirmed reaches faster (5.6x rarer in base).
    /// Tolerance: ±3σ. Confirmed bust = 5%, so over ~10k confirmed reaches σ≈21.8.
    #[test]
    fn confirmed_tier_hit_rate_in_kakuhen() {
        let spec = SpecSheet::canonical();
        let mut state = GameState::new_session(0);
        state.in_kakuhen = true;
        let pe = ProbabilityEngine::new(&spec);
        let mut rng = Rng::seed_from_u64(0x99CD_1234);
        let target = 10_000u32;
        let mut confirmed_count = 0u32;
        let mut confirmed_hits = 0u32;
        let mut spins = 0u64;
        let max_spins = 10_000_000u64;
        while confirmed_count < target && spins < max_spins {
            let o = pe.spin(&state, &mut rng);
            spins += 1;
            if o.reach_tier == Some(ReachTier::Confirmed) {
                confirmed_count += 1;
                if o.is_jackpot { confirmed_hits += 1; }
            }
        }
        assert!(confirmed_count >= target,
            "did not gather {target} confirmed reaches in {max_spins} spins (got {confirmed_count})");
        let expected = (confirmed_count as f64 * spec.confirmed.hit_rate) as i64;
        let sigma = (confirmed_count as f64 * spec.confirmed.hit_rate * (1.0 - spec.confirmed.hit_rate)).sqrt();
        let three_sigma = (3.0 * sigma).ceil() as i64;
        let diff = (confirmed_hits as i64 - expected).abs();
        assert!(diff <= three_sigma,
            "confirmed_hits={confirmed_hits} expected={expected} 3σ={three_sigma}");
    }
}
