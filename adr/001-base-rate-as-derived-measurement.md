# ADR-001: ベース is a derived measurement, not a configured spec quantity

**Status:** Accepted
**Date:** 2026-05-25
**Audit driver:** audit-002-iter4-design-direction-2026-05-25 (H5 — tension)
**Supersedes:** PRD-002 R-29 (revision required as part of PRD-004)

## Context

Through iterations 1–3, PRD-002 R-29 read: *"The chucker hit rate is calibrated so ~25–40 balls returned per 100 fired."* The wording "is calibrated" presupposes ベース is a **static designer-set value** — the spec sheet declares it; the implementation matches.

Iteration 4 introduces **C-19 (釘調整 nail-adjustment)** as the player's primary depth-axis input. Under C-19, the pin layout becomes player-influenced (chapter-gated knobs adjust pin-cluster positions), and ベース becomes an **emergent measurement** of whatever layout the player has tuned. The same canonical 1/199.8 jackpot probability (C-2) holds; what varies is the chucker rate, which gates *how often* the player produces a spin.

audit-002 H5 names this as a real tension: C-3 (legible reach hierarchy) was easier to honor when ベース was predictable; under iter 4 it varies per-player. The audit's key judgment makes the C-2 ADR a precondition for PRD-004.

## Decision

**ベース is a derived measurement of the playfield physics, not a configured spec quantity.**

Concretely:

1. The `SpecSheet` struct in `pachinko-core/src/spec.rs` **does not** add a `base_game_rate` field. It never has one; this ADR makes that absence intentional.
2. PRD-002 R-29 is revised (as part of PRD-004's spec edits) to read: *"The canonical **stock pin layout** is calibrated to produce a chucker rate of 25–40 balls returned per 100 fired. Player tuning under C-19 may produce rates outside this band, subject to the tuning-constraint bounds documented in PRD-004."*
3. The "stock layout" referenced in R-29 is named explicitly in PRD-004 and lives in `pachinko-game/src/playfield.rs::canonical_pins`. It is the layout a new (un-tuned, chapter 1) player encounters.
4. The cabinet's HUD (the data lamp + the tuning workshop) display the **measured** ベース for the current layout, expressed as a confidence interval (e.g., "17.4% ± 3.2%") from a 200-ball headless Monte Carlo probe. The C-12 honest-display principle applies: no flattering numbers, no implicit upper bounds.
5. The C-2 spec sheet's other invariants (1/199.8 base jackpot probability, 1/35.9 kakuhen, 70% entry, 165-spin ST window, 16R × 90 balls, ¥4/ball) **remain fixed by the math layer** and are not influenced by tuning. C-19 only affects the chucker rate.

## Consequences

### Positive

- **C-19 becomes implementable** — without this ADR, the C-2 contract would be ambiguous and PRD-004 could not specify nail-adjustment rules.
- **Honest information display** — the cabinet displays what it actually does, not what the spec sheet wishes it did.
- **Player agency made real** — the 常連 archetype (skill §3) gets a depth axis to read.

### Negative

- **R-29's contract is now layered** — designers reading R-29 must understand that the 25–40% range applies to the stock layout, not to all layouts the player might produce.
- **Headless probe regime changes** — tests that assert ベース must now specify "for the canonical stock layout" or "for a specified layout fixture." The current implicit assumption (one rate per build) is wrong post-iter-4.
- **The data lamp ベース display** is now load-bearing for the cabinet's honesty. Misreporting ベース is a worse defect than under the prior design where ベース was implicit.

### Neutral

- The C-2 spec sheet's other numbers are unaffected. This is a narrow, surgical change to one variable in the contract.

## Alternatives considered

1. **Add `base_game_rate: f32` to SpecSheet, leave it as a configured constant, and reject nail-adjustment.** Rejected because audit-002 named C-19 as the iter-4 trunk; this alternative would invalidate the audit's key judgment without new evidence.

2. **Keep R-29 as-is and interpret it loosely.** Rejected because R-29's wording explicitly says "is calibrated" — leaving it would create the same drift audit-001 H2 named (silent drift between intent and PRD).

3. **Make `base_game_rate` a *target* field in SpecSheet (e.g., `target_base_rate: (f32, f32)`).** Considered but rejected: the spec sheet's job is to declare what the math layer guarantees; ベース is a property of the *game layer*, not the math layer. Putting a game-layer concept in `pachinko-core` would break C-1 (math-layer purity).

## How this affects existing tests

- `pachinko-core` tests: **unaffected**. The math layer never knew about ベース.
- `pachinko-game` tests (none currently green for ベース; the ~15% measurement is from a headless probe, not a test): a new test fixture should assert that the **canonical stock layout** produces a ベース in [25%, 40%] under a 1000-ball Monte Carlo. PRD-004 adds this rule.
- The existing playwright probes (`.tmp/iter*-probe.mjs`) continue to measure ベース empirically; they do not assert a specific value, so no test breakage.

## Related

- intent.md C-2 (canonical spec sheet — unchanged)
- intent.md C-19 (introduced this iteration; depends on this ADR)
- intent.md C-20 (introduced this iteration; orthogonal)
- PRD-002 R-29 (revised by PRD-004)
- audit-002 hypothesis H5 (tension)
- skill §11.5 (payout-shape decision framework)
- skill §11.6 (designer decision framework — names the choice of static-vs-derived as a load-bearing decision)
