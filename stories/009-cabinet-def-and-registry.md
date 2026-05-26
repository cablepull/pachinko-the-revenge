# Story 009 — `CabinetDef` struct + cabinet registry

**Status:** Ready  ·  **PRD-005 rules:** R-59, R-60, R-61  ·  **Intent:** C-21, C-22  ·  **Effort:** M

## What

Create `pachinko-game/src/cabinet/mod.rs` with the `CabinetDef`, `CabinetArchetype`, `ThemePack`, `SpecOverrides`, and `ChapterLabel` types. Create `pachinko-game/src/cabinet/registry.rs` exporting `pub const CABINETS: &[CabinetDef]` initially populated with exactly one entry (`the-revenge` — a faithful port of the iter-4 cabinet, used as the validation case for the abstraction). `deep-sea-song` is added in story 015.

Includes the bounded-overrides validation logic: a `SpecOverrides::validated()` that returns `Result<SpecSheet, OverrideError>` enforcing the per-field bands documented in R-61. Debug builds panic on out-of-band; release builds fall back to canonical.

## Why

R-59 / R-60 / R-61 are the foundation. Every other F-5..F-9 story depends on this abstraction existing.

## Tests

- `cabinet_def_required_fields` — compile-time test (#[allow(dead_code)] constructor that exercises every field).
- `registry_ids_unique` — runtime test asserting all `CABINETS[i].id` are unique.
- `spec_overrides_in_band_passes` — `SpecOverrides { base_jackpot_prob: Some(1.0/199.8), .. }` produces a valid SpecSheet.
- `spec_overrides_out_of_band_panics_debug` — `Some(1.0/50.0)` panics under debug-assertions.
- `spec_overrides_out_of_band_falls_back_release` — same input under release returns canonical SpecSheet.

## Dependencies

None. This is the foundation for the rest of F-5..F-9.

## Open

- Whether `display_name` should be `String` (for future TOML-loadable cabinets) or `&'static str` (for compile-time const). Decision: `&'static str` for iter-5 — TOML loading is post-MVP per ADR-004 alt 2.

## Not in scope

- The actual cabinet selection screen (story 010)
- ThemePack rendering integration (story 012)
- Any `SpecialMechanic` plugin implementations (stories 016, 017)
