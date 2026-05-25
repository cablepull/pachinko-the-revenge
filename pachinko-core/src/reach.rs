//! Reach roster and beat sequencing.
//!
//! Per PRD R-10: roster is config-driven.
//! Per PRD R-11: reaches gate by `chapter`.
//! Per PRD R-12: no premium+ reach in first 5 minutes.
//! Per PRD R-13: confirmed bust is pre-rolled (not mid-animation).

use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::outcome::ReachTier;
use crate::state::GameState;
use rand_core::RngCore;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BeatVisual {
    /// "Flashback to the loss." Shadow figures, slow zoom.
    FlashbackShadow,
    /// Rain-streaked window, no movement.
    RainWindow,
    /// Sharpening the blade. Close-up, sparks.
    BladeSparks,
    /// Distant antagonist silhouette, fog.
    AntagonistSilhouette,
    /// Tracked to the warehouse. Title card.
    WarehouseTitle,
    /// Close-up of protagonist's eyes. Score modulates up.
    ProtagonistEyes,
    /// "It ends tonight." Opening theme cue.
    OpeningTheme,
    /// Screen-cracks-then-clears. Hit reveal.
    ScreenCrack,
    /// Slumped, defeated. Bust reveal.
    SlumpedBust,
    /// Reels stop on matching figures.
    ReelsMatch,
    /// Reels stop, last off-by-one. Bust.
    ReelsNearMiss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BeatAudio {
    /// Calm BGM crossfade in. Marimba.
    CalmMarimba,
    /// Tension layer added.
    TensionLayer,
    /// Brass swell, key up a half step.
    BrassSwellUp,
    /// Full brass + percussion. Premium climax.
    BrassClimax,
    /// Opening theme. Confirmed only.
    OpeningTheme,
    /// Voice line: protagonist's signature.
    ProtagonistVoice,
    /// Voice line: antagonist taunt.
    AntagonistVoice,
    /// Bust SFX: deflating bass.
    BustSfx,
    /// Hit SFX: cymbal crash + brass major chord.
    HitFanfare,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beat {
    pub visual: BeatVisual,
    pub audio: BeatAudio,
    pub duration_ms: u32,
    /// If true, this beat can escalate the reach to a higher tier mid-play.
    /// (MVP: the escalation logic is owned by the engine; this is a flag.)
    pub escalation_gate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reach {
    pub id: String,
    pub tier: ReachTier,
    /// Relative weight within tier. Engine normalizes.
    pub weight: f32,
    /// Earliest story chapter this reach is eligible for. Per PRD R-11.
    pub chapter: u32,
    pub beats: Vec<Beat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReachRoster {
    pub reaches: Vec<Reach>,
}

impl ReachRoster {
    /// Canonical MVP roster: 8 named reaches across 4 tiers.
    /// Per PRD R-10 + intent C-12: roster is config; this is the default.
    pub fn canonical() -> Self {
        Self {
            reaches: vec![
                // CALM — Chapter 1 always available. The inciting incident, shadow form.
                Reach {
                    id: "flashback-loss".into(),
                    tier: ReachTier::Calm,
                    weight: 1.0,
                    chapter: 1,
                    beats: vec![
                        Beat { visual: BeatVisual::FlashbackShadow, audio: BeatAudio::CalmMarimba, duration_ms: 2200, escalation_gate: false },
                        Beat { visual: BeatVisual::ReelsNearMiss, audio: BeatAudio::BustSfx, duration_ms: 1500, escalation_gate: false },
                    ],
                },
                Reach {
                    id: "rain-window".into(),
                    tier: ReachTier::Calm,
                    weight: 0.7,
                    chapter: 1,
                    beats: vec![
                        Beat { visual: BeatVisual::RainWindow, audio: BeatAudio::CalmMarimba, duration_ms: 2500, escalation_gate: true },
                        Beat { visual: BeatVisual::ReelsNearMiss, audio: BeatAudio::BustSfx, duration_ms: 1500, escalation_gate: false },
                    ],
                },
                // MID — Chapter 2+. Preparation beats.
                Reach {
                    id: "sharpening-blade".into(),
                    tier: ReachTier::Mid,
                    weight: 1.0,
                    chapter: 2,
                    beats: vec![
                        Beat { visual: BeatVisual::BladeSparks, audio: BeatAudio::TensionLayer, duration_ms: 2500, escalation_gate: true },
                        Beat { visual: BeatVisual::ProtagonistEyes, audio: BeatAudio::BrassSwellUp, duration_ms: 2000, escalation_gate: false },
                        Beat { visual: BeatVisual::ReelsMatch, audio: BeatAudio::HitFanfare, duration_ms: 1800, escalation_gate: false },
                    ],
                },
                Reach {
                    id: "rooftop-vigil".into(),
                    tier: ReachTier::Mid,
                    weight: 0.8,
                    chapter: 2,
                    beats: vec![
                        Beat { visual: BeatVisual::AntagonistSilhouette, audio: BeatAudio::TensionLayer, duration_ms: 3000, escalation_gate: false },
                        Beat { visual: BeatVisual::ReelsMatch, audio: BeatAudio::HitFanfare, duration_ms: 1800, escalation_gate: false },
                    ],
                },
                Reach {
                    id: "hunter-and-prey".into(),
                    tier: ReachTier::Mid,
                    weight: 0.6,
                    chapter: 2,
                    beats: vec![
                        Beat { visual: BeatVisual::BladeSparks, audio: BeatAudio::TensionLayer, duration_ms: 2200, escalation_gate: true },
                        Beat { visual: BeatVisual::AntagonistSilhouette, audio: BeatAudio::BrassSwellUp, duration_ms: 2000, escalation_gate: false },
                        Beat { visual: BeatVisual::ReelsNearMiss, audio: BeatAudio::BustSfx, duration_ms: 1500, escalation_gate: false },
                    ],
                },
                // PREMIUM — Chapter 3+. Confrontation. Title card.
                Reach {
                    id: "warehouse-confrontation".into(),
                    tier: ReachTier::Premium,
                    weight: 1.0,
                    chapter: 3,
                    beats: vec![
                        Beat { visual: BeatVisual::WarehouseTitle, audio: BeatAudio::BrassClimax, duration_ms: 2500, escalation_gate: true },
                        Beat { visual: BeatVisual::AntagonistSilhouette, audio: BeatAudio::AntagonistVoice, duration_ms: 2500, escalation_gate: false },
                        Beat { visual: BeatVisual::ProtagonistEyes, audio: BeatAudio::ProtagonistVoice, duration_ms: 2000, escalation_gate: false },
                        Beat { visual: BeatVisual::ReelsMatch, audio: BeatAudio::HitFanfare, duration_ms: 1800, escalation_gate: false },
                    ],
                },
                Reach {
                    id: "first-strike-feint".into(),
                    tier: ReachTier::Premium,
                    weight: 0.7,
                    chapter: 3,
                    beats: vec![
                        Beat { visual: BeatVisual::BladeSparks, audio: BeatAudio::BrassClimax, duration_ms: 2200, escalation_gate: true },
                        Beat { visual: BeatVisual::ProtagonistEyes, audio: BeatAudio::ProtagonistVoice, duration_ms: 2000, escalation_gate: false },
                        Beat { visual: BeatVisual::ReelsNearMiss, audio: BeatAudio::BustSfx, duration_ms: 1500, escalation_gate: false },
                    ],
                },
                // CONFIRMED — Chapter 4. The catharsis. One reach only. R-13 pre-rolled bust at 5%.
                Reach {
                    id: "it-ends-tonight".into(),
                    tier: ReachTier::Confirmed,
                    weight: 1.0,
                    chapter: 4,
                    beats: vec![
                        Beat { visual: BeatVisual::WarehouseTitle, audio: BeatAudio::OpeningTheme, duration_ms: 4000, escalation_gate: false },
                        Beat { visual: BeatVisual::ProtagonistEyes, audio: BeatAudio::ProtagonistVoice, duration_ms: 3000, escalation_gate: false },
                        Beat { visual: BeatVisual::ScreenCrack, audio: BeatAudio::BrassClimax, duration_ms: 3000, escalation_gate: false },
                        Beat { visual: BeatVisual::ReelsMatch, audio: BeatAudio::HitFanfare, duration_ms: 4000, escalation_gate: false },
                    ],
                },
            ],
        }
    }

    pub fn reaches_in_tier(&self, tier: ReachTier) -> impl Iterator<Item = &Reach> {
        self.reaches.iter().filter(move |r| r.tier == tier)
    }

    /// Per PRD R-11: filter reaches by unlocked chapter.
    /// Per PRD R-12: in the first 5 minutes of a session, restrict to calm-tier.
    pub fn select(
        &self,
        tier: ReachTier,
        state: &GameState,
        rng: &mut dyn RngCore,
    ) -> Option<&Reach> {
        let effective_tier = if state.session_elapsed_ms() < 5 * 60 * 1000 && tier != ReachTier::Calm {
            // Substitute calm
            ReachTier::Calm
        } else {
            tier
        };

        let eligible: Vec<&Reach> = self
            .reaches_in_tier(effective_tier)
            .filter(|r| r.chapter <= state.unlocked_chapter)
            .collect();

        if eligible.is_empty() { return None; }
        let total: f32 = eligible.iter().map(|r| r.weight).sum();
        let u = (rng.next_u32() as f32) / (u32::MAX as f32 + 1.0);
        let mut threshold = u * total;
        for r in eligible {
            threshold -= r.weight;
            if threshold <= 0.0 { return Some(r); }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rng;
    use rand_core::SeedableRng;

    /// PRD R-10: canonical roster has 8 named reaches across 4 tiers.
    #[test]
    fn canonical_roster_has_8_reaches() {
        let r = ReachRoster::canonical();
        assert_eq!(r.reaches.len(), 8);
        assert_eq!(r.reaches_in_tier(ReachTier::Calm).count(), 2);
        assert_eq!(r.reaches_in_tier(ReachTier::Mid).count(), 3);
        assert_eq!(r.reaches_in_tier(ReachTier::Premium).count(), 2);
        assert_eq!(r.reaches_in_tier(ReachTier::Confirmed).count(), 1);
    }

    /// PRD R-11: reaches above unlocked chapter are excluded.
    #[test]
    fn chapter_gating() {
        let roster = ReachRoster::canonical();
        let mut state = GameState::new_session(0);
        state.now_ms = 10 * 60 * 1000; // past the 5-min calm gate
        state.unlocked_chapter = 2; // mid available, premium/confirmed not
        let mut rng = Rng::seed_from_u64(7);
        // Try selecting premium 1000 times; should always either return None or
        // (post-substitution) a calm reach if substitution applied — but we're past 5 min,
        // so no substitution. Premium should yield None given no premium reach is eligible.
        let mut premium_selected = 0;
        for _ in 0..1000 {
            if let Some(r) = roster.select(ReachTier::Premium, &state, &mut rng) {
                if r.tier == ReachTier::Premium { premium_selected += 1; }
            }
        }
        assert_eq!(premium_selected, 0, "premium reach selected despite chapter=2");
    }

    /// PRD R-12: first 5 minutes, premium request substitutes to calm.
    #[test]
    fn first_five_minutes_substitution() {
        let roster = ReachRoster::canonical();
        let mut state = GameState::new_session(0);
        state.now_ms = 2 * 60 * 1000; // 2 minutes in
        state.unlocked_chapter = 4;
        let mut rng = Rng::seed_from_u64(13);
        for _ in 0..100 {
            let r = roster.select(ReachTier::Premium, &state, &mut rng).expect("must have a calm reach");
            assert_eq!(r.tier, ReachTier::Calm, "expected substitution to calm, got {:?}", r.tier);
        }
    }

    /// PRD R-10 / C-12: roster is config — adding a reach changes selection probabilities.
    #[test]
    fn roster_weights_respected() {
        let mut roster = ReachRoster::canonical();
        // Inject a 9th reach with heavy weight in premium.
        roster.reaches.push(Reach {
            id: "midnight-confrontation".into(),
            tier: ReachTier::Premium,
            weight: 100.0,
            chapter: 3,
            beats: vec![Beat {
                visual: BeatVisual::WarehouseTitle,
                audio: BeatAudio::BrassClimax,
                duration_ms: 1500,
                escalation_gate: false,
            }],
        });
        let mut state = GameState::new_session(0);
        state.now_ms = 10 * 60 * 1000;
        state.unlocked_chapter = 4;
        let mut rng = Rng::seed_from_u64(99);
        let mut count = 0;
        let trials = 1000;
        for _ in 0..trials {
            if let Some(r) = roster.select(ReachTier::Premium, &state, &mut rng) {
                if r.id == "midnight-confrontation" { count += 1; }
            }
        }
        // Expected ~ 100/(100+1+0.7) ≈ 98.3%
        assert!(count as f64 / trials as f64 > 0.95,
            "heavy-weighted reach selected only {count}/{trials} times");
    }
}
