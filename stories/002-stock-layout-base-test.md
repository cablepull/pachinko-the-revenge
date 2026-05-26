# Story 002 — Stock-layout ベース test (1000-ball Monte Carlo in [25%, 40%])

**Status:** Ready  ·  **PRD-004 rules:** R-46 (acceptance), R-57  ·  **Intent:** C-2 via ADR-001  ·  **Effort:** S

## What

Add a `pachinko-game` integration test that runs 1000 simulated balls against the canonical stock layout (default `PinLayout`) and asserts the measured chucker-entry rate falls in `[0.25, 0.40]`. The test uses a seeded RNG for reproducibility. Failure means the stock layout has drifted out of the PRD-002 R-29 band.

## Why

Per ADR-001, ベース is a derived measurement and must be measured against the canonical stock layout in a test. Iter 3 measured ~15% via headless probe but had no in-tree test — the failure mode was invisible to CI. This story makes the discipline check enforceable.

## Tests

- `stock_layout_base_rate_in_band` — the test itself (asserts 25–40%).
- The test is fast (≤2s) so it runs on every PR.

## Dependencies

- Story 001 (PinLayout struct must exist before this test can reference a default).

## Open

- The test currently asserts the band; if iter-4 ships with a stock layout that produces, say, 28% (just inside), should the band tighten? Decision deferred to PRD-005.

## Not in scope

- Tests for tuned (non-stock) layouts. Those are out-of-scope for the band assertion; player-tuned layouts intentionally CAN produce rates outside the band.
