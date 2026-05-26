# Story 011 — Mid-session cabinet swap (Q key + data-lamp menu)

**Status:** Ready  ·  **PRD-005 rules:** R-67  ·  **Intent:** C-23, skill §14.5  ·  **Effort:** S

## What

A key binding (`Q`) and a data-lamp menu entry ("Swap Cabinets") that ends the current session — persisting all state — and returns the player to the selection screen. The persisted state is the same shape as the R/reset path (story 006).

## Why

R-67: the platform needs a graceful path between cabinets without forcing a tab-refresh. Also serves skill §14.5 "leave the machine" — Q is an explicit "I'm done with this one" affordance.

## Tests

- `swap_persists_current_session` — pressing Q from a non-empty session writes the cabinet's PersistedState before returning to selection.
- `swap_returns_to_selection_screen` — after Q, the Screen state is `Selection(_)`.
- `swap_during_modal_dismisses_modal_first` — pressing Q with the workshop / summary / welcome-back card open closes that modal first; second Q presses then swap.

## Dependencies

- Story 010 (Selection screen — Q returns to it)
- Story 013 (per-cabinet persistence)

## Open

- Whether to show a "Are you sure?" confirmation if the player has an active jackpot in progress. Decision: NO — Q is rare and explicit; adding a confirmation step adds friction without preventing real mistakes.

## Not in scope

- Auto-resume of in-progress jackpot when returning to a cabinet. Decision deferred — for iter-5 a JP that was mid-round at swap time is recorded as completed when the cabinet is re-entered.
