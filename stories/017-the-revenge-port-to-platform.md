# Story 017 — Port `the-revenge` cabinet onto the new platform

**Status:** Ready  ·  **PRD-005 rules:** R-69  ·  **Intent:** C-21, C-22  ·  **Effort:** M

## What

Move the iter-4 cabinet (current single-machine implementation) onto the new platform infrastructure. Specifically:

- Create `CabinetDef::the_revenge()` in `cabinet/registry.rs`.
  - SpecOverrides: all None (uses canonical SpecSheet — matches iter-4 behavior).
  - ReachRoster: the existing canonical roster (8 named reaches across 4 tiers).
  - ThemePack: `back_panel_id: NeonCityscape`, `bezel_palette: WarmGold`, `bgm_set_id: BrassChiptune`, `reach_grammar: TheRevenge`.
  - ChapterLabels: the 4 existing chapter labels ("sharpening the blade", "tracked to the warehouse", "it ends tonight", + chapter 1 placeholder).
  - SpecialMechanic: `None`.
  - default_layout: the iter-4 canonical PinLayout::stock().

- Refactor `main.rs::main()` so that:
  1. It loads the meta key + persisted state.
  2. It enters the selection screen (story 010).
  3. On selection of a cabinet, it calls `play_cabinet(def: &CabinetDef) -> async`.
  4. `play_cabinet` contains what is currently the iter-4 main loop, parameterized by `def`.
  5. On Q (story 011), `play_cabinet` returns to the selection screen.

The acceptance test for this story is functional equivalence: under cabinet `the-revenge`, the iter-4 PRD-004 rules R-46..R-58 must still pass byte-for-byte.

## Why

R-69. Validates that the platform abstraction doesn't break the existing cabinet. The port is the "hello world" of the new architecture — if iter-4 doesn't work under the platform, the platform is wrong.

## Tests

- `the_revenge_spec_matches_iter4` — `CabinetDef::the_revenge().spec_overrides.validated().unwrap()` produces the exact canonical SpecSheet.
- `iter4_persistence_migration_lands_in_the_revenge` — given pre-existing iter-4 persisted state, after migration the player's tuning + chapter progress appears under cabinet `the-revenge`.
- `the_revenge_e2e_smoke` — a headless probe runs 30s of auto-fire under cabinet `the-revenge`; the resulting render is visually equivalent to iter-4 (compared screenshot-wise within a tolerance threshold).
- `play_cabinet_returns_on_q` — `play_cabinet(the_revenge)` returns Ok when the test injects a Q keypress; persisted state is written.

## Dependencies

- Story 009 (CabinetDef)
- Story 010 (Selection screen)
- Story 011 (Mid-session swap)
- Story 012 (ThemePack rendering)
- Story 013 (Per-cabinet persistence)

## Open

- Whether the chapter labels move into the CabinetDef or stay hardcoded in the chapter-unlock overlay rendering. Decision: into CabinetDef — per-cabinet narrative is per-cabinet content.

## Not in scope

- Adding new mechanics to the-revenge (it ships with `SpecialMechanic::None`)
- Visual polish beyond iter-4's level (the-revenge is a port, not an art pass)
