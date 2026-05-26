# Story 007 — Welcome-back continuity card on next-session start

**Status:** Ready  ·  **PRD-004 rules:** R-53  ·  **Intent:** C-20  ·  **Effort:** S

## What

When the player starts a session and `last_session_at_ms` is within the prior 7 days, show a one-time welcome-back overlay before the help/intro screen:

- "WELCOME BACK"
- "Your last session: <date>, chapter <N>, net <P/L>"
- "Your tuning: <summary of knob deltas, e.g., 'wider funnel, +2 lower row'>"
- Two buttons:
  - **"RESUME WITH PRIOR TUNING"** — loads the persisted PinLayout knobs
  - **"FRESH SESSION"** — starts with the canonical stock layout
- Below the buttons: a small "TUTORIAL" link that opens the standard help overlay (so a returning player who forgot the controls can still get help)

The card is canvas-rendered, screenshotted-friendly. If no prior session exists (first visit) or the last session is older than 7 days, the card is not shown and the player goes straight to the standard help overlay.

## Why

R-53 + audit-002 (salvage from UI Direction D, dropping the "session-responsive cabinet sympathy" anti-pattern): the cabinet remembers state across sessions, but in a continuity frame, not a sympathy frame. The "your tuning is your tuning" framing keeps the cabinet emotionally neutral.

## Tests

- `welcome_back_appears_within_7_days` — given `last_session_at_ms = now - 1day`, the card appears.
- `welcome_back_skipped_after_7_days` — given `last_session_at_ms = now - 8days`, the card does not appear.
- `welcome_back_skipped_on_first_visit` — given no persisted state, the card does not appear.
- `resume_button_restores_knobs` — clicking RESUME restores the persisted PinLayout values exactly.

## Dependencies

- Story 005 (persistence)
- Story 006 (SessionSummary struct provides the "last session" data)

## Open

- The 7-day window: arbitrary; could be configurable. Decision: hardcoded for v0.4, configurable in v0.5.
- Should the card show during a long single-tab session (e.g., the player reset 8 days ago in the same tab)? Edge case; using `last_session_at_ms` from storage handles this correctly.

## Not in scope

- Multi-player profile support — out of scope for v0.4.
