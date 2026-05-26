# User feedback — 2026-05-25 — iteration 3 direction

After playing the deployed iteration 2 build, the project owner gave six
specific defects and one directive. Captured here as the input to the
iteration 3 goal proposal.

## The six defects

1. **The data lamp is always-on filler.**
   > "The stats should be shown when it makes sense, like the user clicks
   > on something to show it and they should be able to dismiss it."

   Current state: the data lamp (top-right HUD) renders continuously,
   even before the player has any data to look at. It's information
   pollution during base play and competes with the cabinet for
   attention.

2. **"Attacker closed" is filler.**
   > "I don't know why we display attacker closed."

   Current state: when the attacker is closed (i.e., not during a
   jackpot), we render `- attacker closed -` in the attacker rectangle.
   This serves no player purpose — it announces the absence of an
   event. The attacker should be invisible/recessed when closed and
   only render when actually open.

3. **The skill is static reference, not a growing artifact.**
   > "We need to consider this with game theory and have our pachinko
   > skill learn it and grow and be used to make this better."

   Current state: `.claude/skills/pachinko-expertise/SKILL.md` was
   written at project bootstrap and hasn't been updated as iterations
   produced concrete lessons. It's a domain reference rather than an
   evolving design instrument.

4. **The graphics are weak; no event animations.**
   > "The graphics are weak and there are no interesting animations
   > to hook the user that triggers on events."

   Current state: events happen but produce limited transient visuals
   — a screen flash on jackpot, a particle burst on jackpot, the
   `<<  IT  ENDS  TONIGHT  >>` text on confirmed reach. That's it.
   No character cut-ins, no full-screen wipes, no chord-stinger
   visualizations, no zoom-on-landing-7, no bezel light cascades.

5. **The cabinet is one-dimensional.**
   > "The cabinet looks very one dimensional and aside from our steel
   > balls doesn't look interesting."

   Current state: the cabinet is a flat-shaded gold rectangle with a
   dark navy inner area, an LCD strip, a pin grid, and a chucker. No
   background art, no foreground frame, no animated bezel lighting,
   no drop shadows giving depth, no layering.

6. **No sense of money or the value of winning.**
   > "There is no sense of money or the value of winning."

   Current state: we show BALLS FIRED / RETURNED / RATE and BALLS WON.
   Pachinko's real-world ball value (¥4/ball, regulation) is never
   surfaced. Jackpots show "+1440 BALLS" — true to the cabinet but
   meaningless economically without yen. No profit/loss indicator
   shows whether the session is net-positive or net-negative for the
   player. No streak indicators when chaining jackpots inside kakuhen.

## The directive

> "Propose a goal to take us to the third iteration where our game
> becomes amazing."

Iteration 3 should resolve all six defects and bring the cabinet to a
state where every event feels earned and valuable, the visual surface
has depth and theatre, and the discipline-driving skill has evolved to
encode what we've learned through iterations 1–2.

## How this connects to intent constraints

- Defect 1 (data lamp) intersects **C-3** (legible reach hierarchy) —
  competing visual elements during base play dilute the reach signal.
- Defect 2 (attacker filler) intersects **C-3** + **C-10** (常連 sniff
  test) — real cabinets do not advertise the absence of events.
- Defect 3 (skill stasis) is a process gap — no intent constraint yet
  covers "the skill must be updated by audits". Candidate C-17.
- Defect 4 (animations) intersects **C-3** + **PRD-001 §2 PXT-3**
  (jackpot produces a measurable physiological response) — without
  animations on events, the response budget is left on the table.
- Defect 5 (one-dimensional) intersects **C-10** + **C-15** — depth
  is what makes a cabinet read as authentic rather than a wireframe.
- Defect 6 (no money) is an unaddressed constraint. **C-7** explicitly
  rules out real-money handling, but doesn't address whether the
  game should *simulate* the economic dimension. Candidate C-18.

The two candidate new constraints (C-17 skill-evolution, C-18 simulated
economy) should land in `intent.md` if the iteration 3 goal is accepted.
