# Story 010 — Cabinet selection screen

**Status:** Ready  ·  **PRD-005 rules:** R-64, R-65, R-66, R-68  ·  **Intent:** C-21, C-23  ·  **Effort:** M

## What

A new top-level screen rendered before the cabinet's main loop. Shows the cabinet registry as a grid of tiles. Each tile shows display_name, archetype badge, mechanic one-liner, thumbnail back-panel, and per-cabinet prior-session stats (if persisted) or "NEW MACHINE" otherwise. The most-recently-played cabinet (per `meta` storage) is highlighted as default. Click or Enter selects it; the screen transitions out (~400ms) and the chosen cabinet's main loop starts.

Implementation: a new `screen::Screen` enum gates the main loop — `Selection(SelectionState)` vs `Playing(PlayState)`. Most of iter-4's main-loop code moves into a `play_cabinet(def: &CabinetDef)` async function that returns when the player presses Q (story 011) or closes the tab.

## Why

R-64 / R-65 / R-66 / R-68: this is the user-facing entry point of the platform. Without it the multi-cabinet platform is invisible.

## Tests

- `selection_renders_all_registered_cabinets` — at least N tiles for N entries.
- `selection_returns_default_on_enter` — pressing Enter without further input returns the most-recently-played cabinet's id (or CABINETS[0] if none).
- `selection_tile_shows_persisted_stats` — given persisted state for a cabinet, the tile shows the right session counts.
- `selection_transition_under_500ms` — measured wall-clock from select to first cabinet frame.

## Dependencies

- Story 009 (CabinetDef + registry)
- Story 013 (per-cabinet persistence namespace)

## Open

- Tile layout: vertical column vs horizontal grid? Decision: horizontal grid (3-wide) — Space Cadet's table picker grammar applied to pachinko.
- Should selection support keyboard navigation (arrow keys)? Decision: yes — arrow keys cycle highlight, Enter selects.

## Not in scope

- Mid-session swap (story 011)
- Parlor card panel (story 014)
- Cabinet thumbnail art generation (story 020 — for now, procedural placeholder)
