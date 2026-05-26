# Story 006 — Session-end summary screen (R/reset)

**Status:** Ready  ·  **PRD-004 rules:** R-52  ·  **Intent:** C-20, skill §14.3, §14.5  ·  **Effort:** S–M

## What

When the player presses R, replace the cabinet rendering with a full-screen summary card showing:

- Session duration (`mm:ss`)
- Balls fired, balls won, net ¥
- Highest chapter unlocked
- Longest dry streak (spins between jackpots)
- The most-rare reach tier seen this session (one of: none / calm / mid / premium / confirmed)
- 1–2 narrative lines drawn from the session's events (e.g., *"You survived an 847-spin hama-dai before chapter 3 broke open."* — auto-generated from the session log)

Below the summary: a "PLAY AGAIN" button (starts a fresh session) and a "DONE" button (returns to a static "session complete" screen). The summary is rendered on canvas (no DOM modal) so it can be screenshotted.

The summary's data structure is the `SessionSummary` struct persisted by story 005 for the welcome-back card.

## Why

R-52 + skill §14.3 + §14.5: the player needs a story-to-tell artifact and a natural "good place to stop" moment. The session summary is both.

## Tests

- `session_summary_data_consistency` — synthetic session data round-trips through render and produces expected text.
- `session_summary_narrative_lines_chosen` — given a session with high hama-dai and a chapter unlock, the chosen narrative line mentions both.

## Dependencies

- Story 005 (persistence; the summary persists for the welcome-back card)
- iter-3 session-state tracking (already exists: spins-since-JP, last-10-JP history, chapter)

## Open

- Should the narrative line generation use a Markov-like template system or hand-authored templates? Decision: hand-authored templates, 8–12 total, deterministically selected based on session metrics. Avoids the "AI-generated text feels generic" failure mode.
- Should the summary auto-show after a long idle period (e.g., 5 min no input)? Decision: NO for v0.4 — too easy to accidentally trigger; let the player explicitly press R.

## Not in scope

- Sharing buttons (Twitter / clipboard) — adds external-system surface, defer to v0.5.
- A "highlights reel" mini-video of the session — too ambitious; just the summary card.
