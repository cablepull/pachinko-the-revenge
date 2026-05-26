# Story 021 — Stock-layout ベース band test, per cabinet

**Status:** Ready  ·  **PRD-005 rules:** R-63  ·  **Intent:** C-2, ADR-001, ADR-004  ·  **Effort:** S

## What

Generalize story 002's stock-layout band test to run against every cabinet in the registry. Implementation: a parameterized test that iterates `CABINETS`, builds each cabinet's effective playfield + default PinLayout, runs a 1000-ball Monte Carlo, and asserts the chucker rate is in [25%, 40%].

Cabinet-specific seeds (named in `CabinetDef::test_seed` constant, deterministic per cabinet) ensure reproducibility.

## Why

R-63: per ADR-001 ベース is a derived measurement; per ADR-004 each cabinet has its own canonical stock layout. The band test must run per cabinet. Iter-4 deferred this story until the stock layout was first retuned; iter-5 is the natural moment to land it because the multi-cabinet validation needs it.

## Tests

- `cabinets_in_registry_pass_stock_layout_band` — parameterized over the registry; fails fast on the first cabinet that falls outside [25%, 40%].
- The test fixture documents the band as `R-29_BAND` constant so the rule and the test trace.

## Dependencies

- Story 009 (registry)
- Story 015 (deep-sea-song — second cabinet to validate against)
- Story 017 (the-revenge port)

## Open

- Whether to tighten the band per archetype (e.g., Casual cabinets in [30%, 45%]). Decision: NO for iter-5 — keep [25%, 40%] across all cabinets so the regulation contract is one number, not per-archetype.

## Not in scope

- Tests for player-tuned layouts (intentionally allowed outside the band per ADR-001).
- Performance benchmarks of the MC probe (separate concern).
