# Story 014 — Parlor card (cross-cabinet meta-progression panel)

**Status:** Ready  ·  **PRD-005 rules:** R-78, R-79  ·  **Intent:** C-21, C-23, skill §14.3  ·  **Effort:** S–M

## What

A "PARLOR CARD" panel toggleable from the selection screen (key: P, or via a button) that shows:

- **Per cabinet**: sessions played, total jackpots, highest chapter reached, longest dry streak, best session net ¥
- **Cross cabinet**: total jackpots across all cabinets, total cabinets ever opened, favorite cabinet (most sessions)
- A "JUMP" affordance per row that highlights that cabinet's tile in the selection grid

Rendered on canvas (per iter-3 grammar — no DOM modal). Data comes from each cabinet's PersistedState (per R-79 namespacing) + the meta key aggregate.

## Why

R-78 / R-79 + skill §14.3: the come-back-tomorrow test is sharpened by visible progression across cabinets. Without the parlor card, the multi-cabinet platform feels like five disconnected games; with it, the player has a cross-cabinet narrative.

## Tests

- `parlor_card_aggregates_across_cabinets` — given persisted state for 2 cabinets with 3 + 5 jackpots respectively, the cross-cabinet total reads 8.
- `parlor_card_per_cabinet_row_present` — every CabinetDef in the registry has a row in the card (even cabinets the player hasn't played yet).
- `parlor_card_jump_highlights_tile` — clicking the JUMP affordance on row X moves the selection-screen highlight to tile X.

## Dependencies

- Story 010 (selection screen)
- Story 013 (per-cabinet persistence namespace)

## Open

- Whether to show comparative stats ("you're at chapter 2 in the-revenge vs 3 in deep-sea-song"). Decision: NO for iter-5 — comparing cabinets implicitly grades them, and that's not the intended narrative.

## Not in scope

- Achievements / unlockables. Defer to iter-6+ once 3+ cabinets exist.
