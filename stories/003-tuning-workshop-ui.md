# Story 003 — Tuning workshop UI (hidden-by-default, data-lamp pattern)

**Status:** Ready  ·  **PRD-004 rules:** R-49  ·  **Intent:** C-19, C-20  ·  **Effort:** M

## What

Add a "tuning workshop" overlay accessible via the data lamp (a new "TUNING" tab). The workshop shows the available knobs (per chapter), each rendered as a small slider with a labeled current value. Dragging a slider updates the `PinLayout` immediately; the rendered playfield reflects the new pin positions in real time (the canvas behind the overlay shows the updated layout).

A "RESET TO STOCK" button reverts all knobs to 0.0. A "DONE" button closes the overlay.

The overlay is opt-in only — chapter 1 sessions have no TUNING tab (since 0 knobs are available); chapter 2+ sessions have it. A small glow-pulse indicator near the data-lamp toggle appears for the first 5 minutes after chapter 2 unlocks, drawing attention to the new capability per R-49.

## Why

Per audit-002 H1 and the subagent UX consensus: the tuning UI must NOT compete with the first-impression playfield. Hiding it behind the data-lamp toggle (which is already hidden-by-default per iter-3 R-34) puts it on the same opt-in surface as the data lamp itself. Chapter-gating prevents new players from seeing it before they can use it.

## Tests

- `tuning_tab_hidden_at_chapter_1` — at chapter 1, the data lamp's TUNING tab is not rendered (assertion via test-mode flag).
- `tuning_slider_clamps_input` — programmatic drag-event injection beyond ±1.0 clamps the knob value.
- `tuning_workshop_persists_within_session` — closing and reopening the workshop preserves knob values.

## Dependencies

- Story 001 (PinLayout)
- iter-3 data-lamp toggle infrastructure (already exists)

## Open

- Should the workshop pause the ball flow while open? Probably yes for chapter 2 first-time experience; auto-pause for ≥10s of inactivity in the workshop. Decision: yes, pause-while-open as a UX safety.
- The chapter-2 onboarding glow-pulse: should it persist past 5 min if the player ignores it? Decision: yes, persistent at lower intensity until first use.

## Not in scope

- Predicted ベース display (story 004)
- Persistence (story 005)
- Diegetic "pickup a tool" presentation (deferred — slider UI is good enough for v0.4)
