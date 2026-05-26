# Story 008 — Environmental reach cues + BGM crossfade primitive

**Status:** Ready  ·  **PRD-004 rules:** R-50, R-55  ·  **ADR:** ADR-003 (forthcoming, this story drafts it)  ·  **Intent:** C-3, C-20, resolves open question 7  ·  **Effort:** M

## What

Two coupled changes that together convert reach signalling from banner overlays to in-place environmental cues:

### Part 1: Audio crossfade primitive

Wrap `quad-snd` with a small fade-envelope helper in `pachinko-game/src/audio.rs`:

```rust
pub fn crossfade(out: &Sound, into: &Sound, duration_ms: u32);
```

Internally schedules a per-frame volume ramp on the outgoing sound (vol → 0) and the incoming sound (0 → nominal_vol) over `duration_ms`. Uses macroquad's per-sound volume control.

### Part 2: Reach signalling refactor

Replace the current overlay-text reach banners (per tier) with environmental cues:

| Tier | Iter-3 grammar | Iter-4 grammar |
|---|---|---|
| Calm | Overlay text "CALM REACH :: flashback" | Back-panel rain intensifies (line count + speed × 1.5); LCD bg desaturates 15%; BGM crossfade to a tense calm-reach loop |
| Mid | Overlay text + character cut-in | Back-panel city silhouettes zoom in 8%; cut-in PARTIAL (no banner); BGM crossfade with key modulation up a half-step |
| Premium | Overlay text + full character cut-in | Back-panel character silhouette emerges FROM the back panel (not from a corner); cut-in still present; BGM crossfade with stinger chord |
| Confirmed | Overlay text + full-screen wipe | Back panel cracks open; sky-changes visual; full-screen wipe RETAINED (per R-50, confirmed keeps the banner — catharsis moment) |

The audio cue leads the visual by ≥1 frame (per skill §8 audio-leads-visual). The crossfade primitive is what makes this implementable cleanly.

## Why

R-50 + R-55 + audit-002 H1 (synthesis salvage). Banner overlays during reach are the most-visible iter-3 UI element; under C-20 they should appear only when irrelevant alternatives don't exist. For calm/mid/premium tiers, environmental cues honor C-20 better. Confirmed is the catharsis moment where redundancy is appropriate.

## Tests

- `crossfade_volume_ramps_complete` — at duration_ms / 2, outgoing.vol ≈ 0.5 nominal, incoming.vol ≈ 0.5 nominal (within ±0.1).
- `reach_calm_no_banner_overlay` — under CALM reach, the cabinet renders no overlay text. (Test-mode flag inspects render queue.)
- `reach_calm_back_panel_rain_intensifies` — line count for rain animation increases by ≥40% during CALM reach.
- `reach_confirmed_retains_banner` — under CONFIRMED reach, the full-screen wipe banner IS rendered.

## Dependencies

- ADR-003 drafted as part of this story (audio crossfade choice).
- The existing `audio.rs` synth bank (iter-3) — new "calm-reach-bgm" loop must be synthesized as part of this story.

## Open

- Crossfade default duration: 200 ms (per R-55) vs longer (800 ms). Decision: 200 ms for state transitions (audio-leads-visual depends on it); a separate `slow_crossfade` (800 ms) might be needed for music-bed transitions in v0.5.
- Whether the rain-intensify and city-zoom effects should be implemented as new RenderState fields (per iter-3 §10 lesson) or as direct scene.rs queries. Decision: new RenderState fields (preserves the data-driven pattern).

## Not in scope

- Voice-acted reach lines (deferred — needs casting, open question 4).
- New character cut-in art per named reach (story 009 candidate in v0.5).
