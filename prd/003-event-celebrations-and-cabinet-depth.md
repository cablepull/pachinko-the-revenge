# PRD-003: F-3 Event celebrations, cabinet depth, and economy

**Status:** Draft
**Created:** 2026-05-25
**Intent linkage:** C-3, C-8, C-9, C-10, C-12, C-13, C-14, C-15, C-16, **C-17**, **C-18**
**Stories:** _(populated as stories are cut)_
**ADR Required:** no (no new tech-stack decisions; all changes are within the established Rust + macroquad + WASM stack)

## Feature F-3: Every event earns its celebration

PRD-001 built the math + reels. PRD-002 added the ball-and-pin theatre. The cabinet now *works*, but it doesn't yet *feel*. Iteration 2 user feedback identified six defects: data-lamp filler, attacker-closed filler, skill stasis, weak event animations, one-dimensional cabinet, no sense of money. F-3 resolves all six, with the design grounded in the new §7–§9 sections of the `pachinko-expertise` skill.

---

### Rule R-34: Data lamp is hidden by default and toggled by user input

Per [C-3]. The default canvas should not advertise stats the player hasn't asked for.

**Example: Default state**
```
Given a fresh session
When the cabinet first renders
Then the data lamp HUD is not drawn
  And the data-lamp toggle button is drawn (small ⓘ or 📊 icon on the bezel)
```

**Example: Toggle on**
```
Given the data lamp is hidden
When the player presses H (or clicks the toggle button)
Then the data lamp HUD becomes visible in its top-right slot
  And the toggle button reflects the "on" state
```

**Example: Toggle off via Esc**
```
Given the data lamp is visible
When the player presses Esc (or H, or clicks the toggle button)
Then the data lamp is hidden again
  And the toggle button reflects the "off" state
```

**Example: Lamp glow on new info**
```
Given the data lamp is hidden
  And a jackpot occurs (new info: jackpot count incremented, balls won updated)
When the next frame renders
Then the toggle button glow-pulses briefly (1–2 seconds) to hint there's new info
  And the pulse ends when the lamp is opened
```

---

### Rule R-35: Attacker is invisible/recessed unless actually open

Per [C-3, C-10]. Real cabinets do not announce closed doors.

**Example: Closed attacker is part of the bezel art**
```
Given CabinetState is one of Base | Reach | KakuhenBase | KakuhenReach
When the cabinet renders
Then no "- attacker closed -" text appears
  And the attacker rectangle reads as a recessed area of the bezel (a dark inset, not an advertised UI element)
```

**Example: Open attacker is theatrical**
```
Given CabinetState is JackpotRound or BetweenRounds
When the cabinet renders
Then the attacker is rendered with prominent gold doors
  And "OPEN  !!  ATTACKER  !!  OPEN" text appears (or per the §8 grammar)
  And bezel lighting strobes per §7 jackpot grammar
```

---

### Rule R-36: Cabinet renders with six perceptual depth planes per skill §7

Per [C-10, C-15]. The flat-shaded v0.2 cabinet reads as a working diagram; v0.3 has visible depth.

**Example: All six planes are visible**
```
Given a base-state cabinet
When the cabinet renders
Then the following layers are distinguishable visually:
  1. back-panel art (a subtle geometric/cityscape pattern, not plain navy)
  2. mid-layer pin theatre (the pins with steel highlights)
  3. LCD layer (the reels, with their own inset bezel)
  4. ball plane (steel balls with shadow + rim highlight)
  5. foreground UI (animated bezel lighting, corner accents, title marquee)
  6. overlay animations (when active — banners, particle bursts)
And drop shadows separate the LCD, chucker, attacker, and knob from the playfield behind them.
```

---

### Rule R-37: Bezel lighting animates in state-driven patterns per skill §7

Per [C-3, C-10]. A static bezel reads as dead.

**Example: Per-state lighting pattern**
```
Given the cabinet bezel renders each frame
When CabinetState is Base
Then the bezel lights breathe in a slow gold pulse (~0.6 Hz)
When CabinetState is Reach
Then the bezel lights flow in an orange wave (~1.4 Hz, left-to-right)
When CabinetState is JackpotRound or BetweenRounds
Then the bezel strobes gold/red at ~4 Hz
When CabinetState is KakuhenBase or KakuhenReach
Then the bezel washes red with cooler-color accents pulsing slowly
```

---

### Rule R-38: Each named event triggers an animation distinct from the base render

Per [C-3] + skill §8. Reproduced here as the test surface; implementation follows the §8 table.

**Example: Chucker hit produces a flash + rim glow**
```
Given a ball enters the chucker
When the chucker-entry event is emitted
Then within 1 frame: a flash of ~150ms appears at the chucker
  And the chucker rim glows for ~400ms
  And the chucker chime SFX fires (per PRD-001 R-15)
  And the visible elements are distinguishable from the idle chucker render
```

**Example: Reach start cut-in (mid+ tier)**
```
Given a mid-tier reach begins
When the SpinResolved event fires with reach_tier = Mid
Then within 100ms: a character-silhouette cut-in slides in from a corner
  And the cut-in remains visible for the duration of the reach
  And on bust, the cut-in shatters or fades; on hit, it remains until the jackpot animation begins
```

**Example: Jackpot radial burst**
```
Given a jackpot starts
When the JackpotStart event fires
Then within 1 frame: a radial particle burst is emitted from the cabinet center
  And ≥ 8 directional gold rays sweep outward from center
  And the bezel strobes gold/red
  And the LCD shows "F E V E R !!" with letter-by-letter reveal over 0.7 seconds
  And the jackpot fanfare audio plays (per PRD-001 R-16, uninterruptible)
```

**Example: Kakuhen entry color flip**
```
Given a jackpot ended with entered_kakuhen = true
When the EnterKakuhen event fires
Then within 1 frame: the cabinet palette flips to crimson
  And a "CHANCE TIME !!" banner slams in from the top of the screen
  And the bezel transitions to the kakuhen lighting pattern (per R-37)
```

**Example: Chapter unlock**
```
Given session.state.unlocked_chapter increments from N to N+1
When the next frame renders
Then a chapter title card wipes across the screen (left-to-right) over ~1.2 seconds
  And the chapter title text matches the canonical roster (e.g., "CHAPTER 2 :: sharpening the blade")
  And the card fades after a 1.5-second hold
```

---

### Rule R-39: Every ball count is paired with its yen equivalent

Per [C-7, C-18]. ¥4/ball is the canonical rate; configurable.

**Example: HUD yen pairing**
```
Given a session with N balls fired and M balls won
When the HUD renders
Then "FIRED" line shows both balls and yen: e.g., "FIRED 247 / ¥988"
  And "BALLS WON" line shows: e.g., "BALLS WON 1440 / ¥5,760"
```

---

### Rule R-40: Profit/loss indicator is always visible (even when data lamp is hidden)

Per [C-18]. This is the *one* HUD element that survives the toggle.

**Example: P/L tint and value**
```
Given balls_fired = 100, balls_won = 0
When the cabinet renders
Then a small P/L indicator is visible in a fixed screen position (e.g., top-center under the title)
  And it reads "−¥400" (or equivalent format)
  And the text color is red-tinted (loss)

Given balls_fired = 100, balls_won = 1440
When the cabinet renders
Then the P/L indicator reads "+¥5,360" with a green tint
```

---

### Rule R-41: Jackpot reveal shows yen alongside balls

Per [C-18].

**Example: Combined reveal**
```
Given a JackpotStart event
When the celebration overlay renders
Then the overlay includes both "+1440 BALLS" and "+¥5,760" lines
  And both are visible for the full duration of the celebration banner
```

---

### Rule R-42: Streak multiplier appears on chained jackpots inside a kakuhen window

Per [C-18]. The math is unchanged; the visualization rewards the chain.

**Example: Second JP in kakuhen → ×2**
```
Given a kakuhen window is active
  And the player just hit their 2nd jackpot inside the window (without exit)
When the JackpotStart event fires
Then the celebration banner includes "STREAK  ×2" with a glow
  And the streak number increments by 1 on each subsequent in-window jackpot
  And on window exit, the streak is cleared
```

---

### Rule R-43: Treasure trickle floater rises off the chucker on each ball return

Per [C-18]. Subtle but constant; affirms that the game IS paying back in micro-doses.

**Example: Per-ball-return floater**
```
Given a ball enters the chucker
When the chucker-entry event fires
Then a small "+1" or "+¥4" floater appears just above the chucker
  And it rises ~40 px over 700ms while fading to transparent
  And multiple floaters can overlap (queue independently)
```

---

### Rule R-44: Text rendering uses ≤ 6 distinct sizes across the entire cabinet

Per [C-8]. macroquad's font-atlas churn warning is real and degrades visual quality over long sessions.

**Example: Constrained size palette**
```
Given the cabinet renders any frame
When every draw_text and measure_text call is inspected
Then each call uses a font size drawn from a fixed palette: {14, 18, 22, 32, 48, 96}
  And no other sizes are passed
```

---

### Rule R-45: Iteration log section of the skill is appended at iteration close

Per [C-17].

**Example: Skill grows after iter 3**
```
Given iteration 3 is closing (just before final commit)
When the iteration produces concrete transferable lessons (what worked, what didn't, surprises)
Then a new "### From iteration 3" subsection is appended to skill §10
  And it summarizes ~3–6 specific lessons with enough detail to act on them in iteration 4
```

---

## Anti-patterns to avoid

- **Loading images for back-panel art.** Per the proposal, the back-panel is drawn from rectangles, lines, and gradients only. Importing image files breaks the single-file WASM artifact (would need separate assets, more network round-trips, CORS surface) and bloats the build. Stylized geometric is a constraint, not a fallback.
- **Hardcoding ¥4/ball.** It's the canonical rate, but expose it as `Spec::yen_per_ball` so a future low-stakes mode (1パチ at ¥1/ball) is a config change.
- **Driving event animations from frame timers in render.rs.** Animations should be data-driven from the game state machine (event → animation queue → render). Render reads the queue. This is how we keep the animation grammar testable without rendering.
- **Forgetting the lamp glow-pulse when toggle is off.** The "new info available" pulse is what makes the toggle discoverable; without it, players who closed the lamp early in their session never know there's anything to see.
- **Skipping the skill update at iter 3 close.** C-17 makes this mandatory. The lesson log in §10 is what makes the discipline accumulating rather than amnesic.

## Index

| ID | Title | Status | Intent linkage |
|---|---|---|---|
| F-1 | MVP Pachinko Cabinet (reel + math) | Draft / shipped v0.1 | C-1..C-12 |
| F-2 | Ball physics, pin field, launcher | Draft / shipped v0.2 | C-1, C-6, C-8, C-9, C-10, C-13, C-14, **C-15, C-16** |
| F-3 | Event celebrations, cabinet depth, economy | Draft (v0.3 target) | C-3, C-8, C-9, C-10, C-12, C-13, C-14, C-15, C-16, **C-17, C-18** |
