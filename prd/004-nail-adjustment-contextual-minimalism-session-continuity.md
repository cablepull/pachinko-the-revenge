# PRD-004: F-4 Nail-adjustment, contextual minimalism, session continuity

**Status:** Draft
**Created:** 2026-05-25
**Intent linkage:** C-2, C-3, C-9, C-10, C-12, C-13, C-15, C-16, C-17, **C-19**, **C-20**
**Stories:** stories/001..stories/008 (cut from this PRD; see Index)
**ADR Required:** yes — **ADR-001** (ベース is a derived measurement) is a precondition. **ADR-002** (persistence shim choice: `quad-storage`) and **ADR-003** (audio crossfade: `quad-snd` envelope wrapper) are landed alongside the relevant stories.
**Audit driver:** audits/audit-002-iter4-design-direction-2026-05-25/

## Feature F-4: The player's machine — agency, room to breathe, continuity

PRDs 001–003 built the cabinet (math, ball physics, theatre). It works; iter 3 passes skill §14 acceptance gates 1–3. What it does NOT yet do: reward the 常連 archetype (skill §3) with a depth axis to read, survive 2-hour pacing (test 4), or give returning players a story to tell (test 5). F-4 closes these gaps by giving the player **one real input** (釘調整) and **getting out of their way** the rest of the time.

Two subagents (UI + mechanics) converged independently on this direction; audit-002 validated it against four competing hypotheses. The skill's iter-3 closing recommendation predicted this; PRD-004 cashes that prediction.

---

### Rule R-46: Pin layout is a typed config struct with chapter-gated knobs

Per [C-19, C-12]. The playfield's pins are derived from a `PinLayout` containing a small number of named knobs; canonical stock layout is the chapter-1 baseline.

**Example: Stock layout produces ベース in 25–40%**
```
Given the canonical stock PinLayout (chapter 1, no tuning applied)
When a 1000-ball headless Monte Carlo probe runs over the physics
Then the measured chucker-entry rate is in [25%, 40%]
  And the test asserts on a seeded RNG to keep the result reproducible
```

**Example: Knob count is chapter-gated**
```
Given a Session with unlocked_chapter = N
When the player queries available tuning knobs
Then N=1 yields zero knobs (locked, "stock layout only")
  And N=2 yields exactly two knobs
  And N=3 yields exactly four knobs
  And N=4 yields the full set (six knobs)
```

---

### Rule R-47: Each knob has a bounded, documented range

Per [C-19, C-12]. Out-of-bounds tuning is not possible.

**Example: Funnel-tilt knob bounds**
```
Given the "left-funnel-tilt" knob
When the player sets it
Then the value is clamped to [-1.0, +1.0]
  And the corresponding pin cluster translates by at most ±8 px from canonical
  And the resulting layout cannot move any pin closer than 14 px to another pin (no overlap)
```

---

### Rule R-48: The cabinet displays predicted ベース as an honest confidence interval

Per [C-19, C-20, C-12]. No flattering or massaged numbers.

**Example: ベース display format**
```
Given the player is in the tuning workshop
  And the current layout has been probed with 200 simulated balls
When the predicted ベース is displayed
Then it shows the point estimate and the 95% CI: e.g., "17.4% ± 3.2%"
  And the CI is recomputed every time the player changes a knob
  And the display never shows the point estimate without the CI
```

---

### Rule R-49: 釘調整 mode is hidden by default and accessed via the data-lamp pattern

Per [C-20, C-3]. The 30-second sniff test (skill §14.1 test 1) must not break.

**Example: First-impression cabinet is unchanged**
```
Given a fresh session at chapter 1
When the cabinet renders on first frame
Then no 釘調整 UI element is visible
  And the only HUD elements are: P/L indicator, data-lamp toggle, marquee, the cabinet itself
```

**Example: Discoverability via chapter-2 unlock**
```
Given the player has just unlocked chapter 2 (just hit first jackpot)
When the chapter title card animation completes
Then a small persistent "tuning available" indicator appears near the data-lamp toggle
  And opening the data lamp now shows a "TUNING" tab in addition to the existing data
  And the indicator glow-pulses for the first 5 minutes after unlock to draw attention
```

---

### Rule R-50: Reach signalling moves toward environmental cues; banner overlays retained for confirmed tier only

Per [C-20, C-3]. Information surfaces in place, not as overlays.

**Example: Calm reach environmental cue**
```
Given a CALM tier reach fires
When the next frame renders
Then the back-panel rain animation intensifies (line count + speed × 1.5)
  And the LCD bg desaturates by ~15%
  And NO banner text overlay appears
  And the chord modulation in BGM provides the audio cue (per §8 audio-leads-visual)
```

**Example: Confirmed reach retains the banner**
```
Given a CONFIRMED tier reach fires
When the next frame renders
Then the back panel cracks open (existing screen-crack visual) AND the full-screen title card wipes ("IT ENDS TONIGHT")
  And the banner is retained because confirmed is the catharsis moment where text + visual + audio all triple-confirm
```

---

### Rule R-51: P/L floaters rise off the chucker on each ball return; the persistent P/L strip is removed

Per [C-20]. Contextual information surfaces near the event.

**Example: P/L floater on chucker entry**
```
Given a ball enters the chucker
When the chucker-entry event fires
Then a "+¥4" floater appears just above the chucker
  And it rises ~40 px over 700 ms while fading to transparent
  And the persistent top-center P/L strip is no longer drawn
  And the cumulative P/L is queryable via the data lamp (toggleable)
```

---

### Rule R-52: Session-end summary screen on R/reset

Per [C-20, skill §14.3]. The "story to tell" artifact.

**Example: Summary contents**
```
Given a session has at least one jackpot and at least 100 balls fired
When the player presses R (reset)
Then a summary card appears (not an overlay — replaces the cabinet) with:
  - session duration in minutes
  - balls fired, balls won, net ¥
  - highest chapter unlocked
  - the longest dry streak (spins between jackpots)
  - the most-rare reach tier seen this session
  - 1-2 narrative lines drawn from the session's events
And the summary can be screenshotted (no DOM-based modal; rendered on canvas)
And pressing R again starts a fresh session
```

---

### Rule R-53: Welcome-back card on next-session start when prior session was recent

Per [C-20]. Continuity, framed as state-persistence not cabinet-sympathy.

**Example: Recent return**
```
Given the player's last session ended within the prior 7 days (per persisted timestamp)
  And the player starts a new session
When the cabinet renders the first frame
Then a "welcome back" card appears as a one-time overlay with:
  - the prior session's highest chapter
  - the prior session's tuning knob values (if any)
  - a "RESUME WITH PRIOR TUNING" prompt and "FRESH SESSION" alternative
And the player's choice persists for the duration of the session
```

---

### Rule R-54: Pin layout, chapter progress, and last-session summary are persisted via `quad-storage`

Per [C-13, ADR-002]. Resolves intent open question 6.

**Example: Persistence roundtrip**
```
Given a session with tuning knob values [0.2, -0.4, 0.0, 0.1, 0.0, -0.2] and unlocked_chapter = 3
When the player closes the browser tab
  And reopens the same URL within 7 days
Then the welcome-back card is presented (per R-53)
  And accepting "RESUME WITH PRIOR TUNING" restores those exact knob values
  And the unlocked_chapter is 3
```

---

### Rule R-55: BGM crossfade primitive (`quad-snd` envelope wrapper)

Per [ADR-003]. Resolves intent open question 7.

**Example: Reach-state crossfade latency**
```
Given the cabinet transitions from BASE to REACH state
When the BGM crossfade is initiated
Then the previous track's volume fades to 0 over 200 ms
  AND the new track's volume fades from 0 to nominal over 200 ms (parallel fade)
  AND the audio cue leads the visual change by ≥ 1 frame (per skill §8 audio-leads-visual rule)
```

---

### Rule R-56: Streak multiplier rendering is removed

Per [skill §11 "no teeth → drop", audit-002 H1]. The cosmetic counter that adds nothing is gone.

**Example: No streak rendering**
```
Given a player chains 3 jackpots inside a kakuhen window
When the cabinet renders
Then NO "STREAK ×N" badge appears
  And the chained jackpot count is visible only via the data lamp's session-history view
  And the cabinet does not visually distinguish a chained jackpot from a standalone one (the chain IS the kakuhen state already; doubling the visual signal is redundant)
```

---

### Rule R-57: Stock-layout ベース test fixture lives in `pachinko-game` tests, asserts the band

Per [ADR-001, R-46]. The empirical test the iter-3 build did not have.

**Example: Test runs against canonical layout**
```
Given the test `stock_layout_base_rate_in_band`
When invoked with `cargo test --package pachinko-game --release`
Then the test produces 1000 simulated balls against the canonical_pins() layout
  And asserts the measured chucker-entry rate is in [0.25, 0.40]
  And fails loudly if the rate falls outside (this is the discipline check for layout drift)
```

---

### Rule R-58: PRD-002 R-29 is revised to acknowledge ADR-001

Per [ADR-001]. Spec consistency.

**Example: R-29 reads correctly**
```
Given a reviewer reads PRD-002 R-29 (chucker rate calibration)
When they reach the rule text
Then the rule now references the "canonical stock pin layout" and points to ADR-001 for the derived-measurement framing
  And R-29's 25–40% band is preserved but scoped to the stock layout
```

---

## Anti-patterns to avoid

- **Putting `base_game_rate` in `SpecSheet`.** Per ADR-001 this would break C-1 (math-layer purity). ベース is a game-layer measurement; it belongs in `pachinko-game`, not `pachinko-core`.
- **Showing the predicted ベース as a single number.** R-48 explicitly requires the confidence interval. The cabinet's honesty depends on this; flattening to a point estimate is a §12.1 EXPLOIT.
- **Making 釘調整 visible on first impression.** R-49 requires hidden-by-default; visible-on-startup would fail skill §14.1 test 1 (the 30-sec sniff test).
- **Adding banner overlays back for non-confirmed reaches.** R-50 deliberately removes the calm/mid/premium banners in favor of environmental cues. Adding them back negates the C-20 minimalism contract.
- **Persistence stories that gate gameplay.** R-53/R-54 must preserve C-7: no game-over, no "you can't play because you didn't save." Persistence is additive; missing-save defaults to fresh-session, never to error.
- **The welcome-back card acting like cabinet sympathy.** Skill §13.2 anti-pattern: a sympathetic cabinet edges into §12.3 EXPLOIT. R-53's framing must be "your tuning persists" not "we missed you."
- **R-N rules that aren't enforced by tests.** Audit-002 E11 flagged R-44 (text-size palette) as unenforced; iter 4 must NOT add new unenforced rules. Every R in PRD-004 either has a unit test, an integration test, or an explicit "human-judged at alpha" tag.

## Index

| ID | Title | Status | Intent linkage |
|---|---|---|---|
| F-1 | MVP Pachinko Cabinet (reel + math) | Shipped v0.1 | C-1..C-12 |
| F-2 | Ball physics, pin field, launcher | Shipped v0.2 | C-1, C-6, C-8, C-9, C-10, C-13, C-14, C-15, C-16 |
| F-3 | Event celebrations, cabinet depth, economy | Shipped v0.3 | C-3, C-8..C-18 |
| F-4 | Nail-adjustment + contextual minimalism + session continuity | Draft (v0.4 target) | C-2, C-3, C-9, C-10, C-12, C-13, C-15..C-17, **C-19, C-20** |
