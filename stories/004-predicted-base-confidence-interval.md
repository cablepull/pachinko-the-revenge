# Story 004 — Predicted ベース confidence interval (200-ball MC probe in workshop)

**Status:** Ready  ·  **PRD-004 rules:** R-48  ·  **Intent:** C-12, C-19  ·  **Effort:** S–M

## What

Inside the tuning workshop (story 003), every time the player commits a knob change (mouseup on a slider OR after 200 ms of stillness), run a 200-ball headless Monte Carlo against the current `PinLayout` using the existing `ball::step()` physics, and display the predicted ベース as a 95% confidence interval. Display format: `"ベース  17.4% ± 3.2%"`.

The MC runs in-process on a background-style budget (max 8 ms per frame split across frames). The display updates the moment the MC completes. While the MC is in flight, show `"ベース  ... probing ..."` with a small animated indicator.

## Why

R-48 (honest CI display) is the C-20 cornerstone. A flattering single-number display would be the §12.1 EXPLOIT counter-move; the CI keeps the cabinet honest about what the player's tuning will actually produce.

## Tests

- `mc_probe_produces_stable_estimate` — running the probe 10 times on the same layout yields point estimates within ±1.5 pp.
- `mc_probe_ci_width_decreases_with_n` — varying probe length 100 → 500 → 1000 balls shows the CI half-width decreasing.
- `mc_probe_under_8ms_per_frame` — single-frame slice of the probe completes in ≤8 ms.

## Dependencies

- Story 001 (PinLayout)
- Story 003 (Tuning workshop UI to host the display)

## Open

- Should the MC run with the same seed as the current session's RNG, or a fresh test-only seed? Decision: a *fresh* test-only seed; the prediction is about the layout, not about the player's session-specific run.
- Should "stock" layout's ベース be displayed (with CI) even when no knobs are unlocked (chapter 1)? Decision: yes — chapter 1 sees the prediction box in read-only form, helps onboard the concept before chapter 2 unlocks tuning.

## Not in scope

- Tuning recommendation engine ("try widening the funnel for +5%") — deferred to v0.5
