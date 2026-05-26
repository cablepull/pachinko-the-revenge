# Iter 4 UI redesign — competing directions

Iteration 3 closed the v0.2 critique (data-lamp filler, flat cabinet, no
sense of money) with a six-plane render, animated bezel, cut-ins, FEVER
reveal, kakuhen slam, chapter cards, and an always-visible P/L. The
cabinet now passes the 30-sec / 5-min / 30-min acceptance gates per skill
§14.1. **What it has not yet been asked to pass is test 4 (the 2-hour
session feels like 30 minutes or like a story arc) and test 5 (the player
leaves on their own with a story to tell).** Skill §14.4 names those as
iter 4's explicit targets.

Iter 4 is therefore a directional choice about *what kind of cabinet to
be*, not a polish pass. Three candidate directions below are mutually
exclusive at their core (each says *no* to something specific) and each
spends its largest §14.2 budget line on a different moment. A fourth is
offered as a hybrid that may be tempting but is shown to be a trap.

Constraints that constrain every direction below (don't slip):
- **C-9** single machine, single arc — parlor-floor immersion can be
  *evoked* via ambient layers, but adjacent machines / NPC players are
  out of scope.
- **C-13** single WASM artifact — image/video assets break distribution;
  CJK subset font (~50–100 KB per §10 iter-1) is the only path to
  authentic JP text.
- **C-5** JP VO canonical — kills any English-narrated cinematic.
- **A5 + §13.2** four reach tiers max — collapse 6+ tier proposals.
- **§13.3** information-rich, surprise-honest — theatre stays rare.

---

## Direction A: Parlor Authenticity — the 常連 sniff test, doubled down

**Vision.** Iter 3 looks like *a* pachinko machine; iter 4 looks like
*the* pachinko machine from a specific imagined parlor in 2008-vintage
peak-CR Japan. The CJK subset font lands. The data lamp moves up onto a
**physical dome above the cabinet** instead of being a panel inside the
canvas — a real parlor's データランプ. A faint ambient layer (off-screen
parlor murmur, distant fanfares, the smoke-coloured light wash on the
cabinet edges) sells the 三店方式 atmosphere without simulating it. P/L
stays visible but is reframed as a 持ち玉 (mochi-dama, "held balls") count
in a small tray graphic at the bottom of the cabinet. The marquee gains
mixed-script (確変中 ★ FEVER ★ チャンスタイム). The bet: authenticity
*is* the moat — the project's intent says "translation problem, not
design problem," and this direction takes that literally.

**5–7 concrete UI changes**
1. Ship a CJK subset font (~75 KB) covering ~150 glyphs (確変, 時短,
   ヘソ, チャンス, 確変中, 大当たり, ベース, 持ち玉, ST, ラッシュ,
   numerals, hiragana digits, the chapter titles). Retire all
   romanized substitutes from the bezel/marquee/HUD.
2. Move the data lamp HUD *out* of the canvas and onto a rendered dome
   above the cabinet, drawn within the same canvas but visually
   "above the machine." Closed dome shows two segment-display LCDs
   (spins-since-JP, last 10 JP gaps as a tiny waveform). Toggle still
   exists but expands the dome with full stats.
3. Replace the bottom-left P/L with a 持ち玉 tray: a small graphic of
   a ball tray under the cabinet with a count + ¥-equivalent. Positive
   P/L = tray fills; negative = tray empties below the starting line.
4. Add a thin ambient parlor sublayer: 30-sec BGM-bed of distant balls,
   muffled neighbouring-machine fanfares (audio only — no visible NPCs),
   and a warm yellow vignette at the canvas edges suggesting overhead
   parlor lighting.
5. Marquee gains a 7-glyph mixed-script announcement strip that
   crossfades by state: ベース中 ▸ リーチ ▸ 大当たり!! ▸ 確変突入 ▸ ST残.
6. Bezel decoration upgraded with the canonical "machine number plate"
   (e.g., 台番 No. 247) and a fake 機種名 (machine name) — implicit world-
   building, no narrative content.
7. Chapter cards adopt katakana subtitles + the canonical CR-machine
   "話" (episode) marker. "第二話 :: 牙を研ぐ夜" instead of
   "CHAPTER 2 :: sharpening the blade."

**Skill citations.** §3 (parlor sensory environment), §5 (mixed-script
chaos, layered density), §7 (foreground UI plane 5 includes the data
dome *above* the cabinet), §13.1-Eva (mixed-script + 漢字 announcement
treatment), §14.2 line 8 (base BGM + ambient SFX as a per-second budget
item the project has under-invested in).

**Intent constraints satisfied / strained.**
- Satisfies: C-3 (legibility unchanged; mixed script *adds* signal),
  C-5 (JP VO unaffected), C-9 (still one machine), C-10 (this is the
  direction most directly serving the 常連 sniff test), C-15/16
  (unchanged).
- Strains: **C-13** — adding the CJK subset is the biggest WASM-size
  hit since the build was minted; needs the subset budget verified
  before committing. **C-8** — additional render layers (dome, tray,
  vignette, ambient BGM stream) add per-frame cost; budget for a
  frame-time probe.

**Effort: M.** No new game systems. Net new code is a font-loading
path, a dome renderer, a tray renderer, an ambient audio stream, and
~150 glyphs of JP text scattered across the existing UI. Risk centers
on the CJK subset and the audio crossfade primitive (open question
7 in intent.md).

**Says NO to:** narrative-as-surface. Iter 3's chapter cards become
incidental again — the cabinet's job is to *be* a pachinko machine
first, the revenge plot is a beat *inside* that.

**§14.2 budget winner:** line 8 (base BGM + ambient SFX, currently
~5%) absorbs an outsized share — this direction bets that the parlor
*feel* during the 80% of session time spent in base play is what wins
test 4 (2-hour pacing).

---

## Direction B: Cinematic Revenge — the narrative is the surface

**Vision.** The title says "the revenge" and iter 3 buries it. Iter 4
elevates the original IP to the foreground: the cabinet has a **persistent
narrative HUD strip** (chapter title, current beat, protagonist's name)
on the left side, replacing the data-lamp-toggle pattern as the primary
information surface. Reach tiers are renamed to story beats
(provocation / preparation / confrontation / catharsis — per intent §1).
A **character roster** lives at the bottom of the cabinet: 3-4 silhouettes
that fill in as chapters unlock. The confirmed reach gets a full
**story-card cinematic** — a 4-second narrative panel ("the night you
took back the blade") with VO over the existing reel animation.
Session-end (R / reset) shows a **session ledger** naming the player's
high notes: "you saw chapter 3, you hit 4 jackpots, your longest reach
was confirmed-tier." This direction bets the §14.3 "story to tell" test
is what wins return visits.

**5–7 concrete UI changes**
1. Persistent left-side narrative HUD (200 px wide): current chapter
   title (top), current beat ("YOU ARE PREPARING"), protagonist
   silhouette, faint progress sigil. Stays visible in all states.
2. Rename reach tier banners to story beats: REACH . . . →
   "PROVOCATION", REACH !! → "PREPARATION", PREMIUM → "CONFRONTATION",
   CONFIRMED → "CATHARSIS." Tier hierarchy unchanged; labels carry
   plot weight.
3. Add a 4-character roster strip across the bottom 60 px: silhouettes
   that brighten as chapters unlock. Each character has a "last seen"
   timestamp so returning players see who they met last time.
4. Confirmed-reach cinematic gets a 3-4 sec story-card panel
   (background dimmed, character art if pipeline permits / silhouette
   if not, single line of VO + subtitle). This *replaces* the current
   "IT ENDS TONIGHT" banner with a beat from the actual revenge arc.
5. Session-end / reset screen: a "what happened tonight" ledger with
   3-5 narrative lines pulled from the session's events ("chapter 3
   unlocked at spin 247", "you survived an 850-spin hama"). Player
   can screenshot it — this is the §14.3 story-to-tell artifact.
6. Title marquee shows the protagonist's name + arc title instead of
   "PACHINKO :: THE REVENGE": "灰色の刃 — 復讐譚" (subtitle:
   "GRAY BLADE — A REVENGE TALE"). Implicit world-building.
7. Data lamp is demoted: still toggleable, but defaults to a single
   line of "session age + last-event ago" framing. Numerical depth is
   for the 常連; this direction prioritizes the narrative reader.

**Skill citations.** §4-Hokuto (IP-as-mechanic), §4-Eva (chapter ↔
episode mapping), §6.1 (original-IP theme machine), §8 row 14 (chapter
unlock + story-specific cue), §12.6 (chapter progression as flow-zone
extender), §13.1-AKB48 (parasocial character roster), §14.3 (story to
tell as return driver).

**Intent constraints satisfied / strained.**
- Satisfies: C-4 (narrative through reach beats — this direction is
  the most literal C-4 implementation), C-9 (still one arc), C-5
  (JP VO carries the cinematic).
- Strains: **A1** (original IP can carry catharsis) — this direction
  is the highest-stakes bet *on* A1, so failure invalidates it more
  cleanly than other directions. **Open question 1** (story bible) —
  this direction is BLOCKED until the writer's pass produces the
  chapter beats and the protagonist's name. **§13.2 anti-pattern
  "IP licensed but not integrated"** is the failure mode if the
  characters end up as wallpaper; mitigation requires the confirmed-
  reach cinematic to actually depict a story moment.

**Effort: L–XL.** The cinematic asset pipeline + the story bible
unblock are the two big costs. Without character art (per intent
question 2), this direction degrades to "silhouettes plus subtitles,"
which works for prototyping but does not deliver A1.

**Says NO to:** parlor authenticity for its own sake. The cabinet
stops trying to feel like a generic machine in a parlor and starts
trying to feel like *this specific revenge story rendered as pachinko*.
JP text is welcome but not mandatory; mixed-script substitutes are
acceptable if they serve the narrative.

**§14.2 budget winner:** line 2 (confirmed reach cinematic, 20%)
expands toward 30%, absorbing line 7 (chapter unlock cards) and part
of line 4 (premium cut-ins). The bet: the confirmed-reach moment is
where narrative + math collide hardest.

---

## Direction C: Contextual Minimalism + Diegetic Agency — the player tunes their own machine

**Vision.** Iter 3 painted everything; iter 4 *hides* everything that
isn't relevant *right now* and adds the one missing player input that
real pachinko has and the project doesn't: **釘調整 (nail-adjustment)**.
The cabinet defaults to a near-bare playfield — no data lamp, no P/L
strip, no chapter HUD. Information surfaces *appear in place* when
relevant: P/L pops up briefly at the chucker on each return; chapter
title floats at the top after a chapter unlock then fades; reach tier
is signalled by **environmental cues** (back-panel art shifts, ambient
audio changes, bezel reshapes) rather than overlay text. Between
sessions (R / reset), the player enters **nail-adjustment mode**: drag
pins ±2 px to retune the funnel, watch a 100-ball Monte Carlo probe
update the projected ベース in real-time. This direction bets that
agency + room-to-breathe is what makes a 2-hour session sustainable.

**5–7 concrete UI changes**
1. Default canvas removes the P/L strip, the data-lamp toggle, the
   marquee strapline, and the bottom-bar control hint. What remains:
   cabinet, playfield, balls, chucker, knob. Period.
2. Contextual P/L: instead of always-visible, a "+¥248" / "−¥4"
   floater rises off the chucker on each return-or-fire event,
   color-tinted. The current P/L is summarised as a single tray-fill
   level (continuous proprioceptive cue, no number).
3. Reach signalling moves from banner text to environmental cues:
   calm = back-panel rain intensifies; mid = the city silhouette
   *zooms* slightly; premium = a character silhouette emerges from
   the back panel itself (not a corner cut-in); confirmed = the
   back panel cracks open / sky changes. Banner text is retained
   only on confirmed (as the catharsis moment).
4. Data lamp becomes a **press-and-hold** reveal: holding TAB
   shows the full panel; release hides it. No persistent toggle
   state. The "new info" glow-pulse stays as the affordance.
5. **釘調整 mode (the headline change):** on R, the cabinet enters
   a layout-edit overlay. Pins are draggable ±2 px from their
   layout-defined position. A 200-ball Monte Carlo probe runs
   continuously in the background, displaying the projected ベース
   and chucker rate. Player commits a layout; it persists for the
   session.
6. Session-end (after R commits) shows a 3-line summary: "ベース
   34% (your tuning improved it from 28%) // you survived 1,200
   spins // chapter 3 unlocked." Story is implicit, not authored.
7. Sound design takes on more of the signalling load: BGM crossfade
   layer (resolving intent open question 7) flips at state changes
   and the player hears the upgrade before they see it (§8 rule:
   audio leads visual by 1-2 frames).

**Skill citations.** §7 (back-panel art as a reach signalling surface,
not just decoration), §8 row "reach start — premium" (character
emergence as a back-panel event, not a corner cut-in), §10 iter-3
closing rec (surface ベース as a gameplay lever via 釘調整), §12.4
(illusion of agency — make it real where possible), §13.3
(information-rich, surprise-honest — minimalism *protects* the signal),
§14.4 (the project hasn't tested 2-hour pacing; reducing visual noise
is the most direct lever).

**Intent constraints satisfied / strained.**
- Satisfies: C-3 (signalling *moves* into the back-panel, doesn't
  lose legibility — though this needs playtesting), C-15/16 (釘調整
  makes ベース an *emergent* property of player input, which is
  the deepest possible reading of C-16), C-10 (real machines *do*
  hide information; the dataランプ is opt-in), C-7 (no money, just
  ball flow).
- Strains: **C-3** legibility risk — if the back-panel reach cues
  don't read as crisp as banner text, the hierarchy collapses;
  needs an A/B playtest. **PRD-002 R-29 chucker rate target** is
  now player-set, not designer-set; the headless probe regime
  changes shape. **C-12** (reach roster config-driven) is fine but
  the new pin-layout config needs the same treatment.

**Effort: L.** 釘調整 mode is the biggest single addition since
ball physics (iter 2). Needs: pin-position serialization, the live
Monte Carlo probe widget, the drag-and-snap UX, and the back-panel
reach-cue art. Audio crossfade primitive (open question 7) is on
the critical path.

**Says NO to:** information density as a value. The data lamp is
no longer the design's centerpiece — it's an opt-in inspection mode
for the 常連. The default cabinet trusts the player to read the
environment.

**§14.2 budget winner:** line 9 (idle animations — bezel breathing,
marquee, back-panel parallax) absorbs line 5 (mid-reach grammar) and
part of line 4. The bet: the back-panel *is* the reach signalling
plane, so investment there pays per-spin, not per-jackpot.

---

## Direction D (hybrid): Session-Responsive Cabinet — the machine remembers

**Vision.** The cabinet mutates by session state. A losing player sees
the bezel tint sadder (cool blues replacing warm golds), the marquee
text shifts ("hold the line"), and the protagonist silhouette in the
HUD looks weary. A winning player sees the marquee escalate ("the
blade is sharp tonight"), the bezel gains animated lightning, and the
data-lamp dome lights up with a victory rim. A returning player (R
within an hour, persisting across the WASM session window) sees a
"welcome back" card naming their last chapter and what reach they were
chasing.

**Why this is offered as a TRAP rather than a recommended direction.**
This direction *sounds* compelling but conflicts with skill §13.3
(information-rich, surprise-honest): a cabinet that telegraphs the
player's loss is *information-rich* but also *demoralizing in a way
real pachinko is not*. Real cabinets are indifferent to outcomes —
that's part of why losing feels survivable. A cabinet that empathizes
with losses risks tipping into the same failure mode as §13.2's
"premium reach used too often" — emotional cues lose meaning when
they're constant. Also conflicts with **C-7** spiritually: making
the cabinet a P/L-sensitive companion edges toward the dark-pattern
side of §12.3 (sunk-cost amplification — even sympathetic mutation
emphasizes that the player has "invested"). Recommend salvaging *one*
element (the "welcome back" card per §14.3) into whichever direction
wins, and dropping the rest.

**§14.2 budget winner:** N/A — this direction would have to dilute
across all idle lines, which is the anti-pattern of trying to
prioritize everywhere.

---

## Comparison matrix

| Direction | §14 5-test pass likelihood | §14.2 budget fit | Intent fit (C-N) | Scope risk | Novelty |
|---|---|---|---|---|---|
| A. Parlor Authenticity | Strong on tests 1–3 (already passing); **modest** on test 4 (ambient layer helps); weak on test 5 (no story-to-tell artifact) | Honest: invests in line 8 (base BGM/ambient), an under-funded line | C-10 strongest; C-13 strained (font); C-4 weakest | M (CJK subset is the unknown) | M — the cabinet looks *more* like its inspirations but is recognizably the same kind of object |
| B. Cinematic Revenge | Strong on test 5 (story to tell); strong on test 4 if chapters pace correctly; risk on test 3 if cinematic interrupts catharsis | Concentrates on line 2 (confirmed cinematic) — defensible | C-4 strongest; A1 most-staked; story-bible-blocked | L–XL (asset pipeline + writer's pass) | H — the cabinet becomes a *narrative artifact*, distinct from any canon |
| C. Contextual Minimalism + 釘調整 | Strong on test 4 (room to breathe); strong on test 5 (agency is the story); risk on test 2 if reach signalling reads as quieter than current overlays | Concentrates on line 9 (idle/back-panel/bezel) — currently under-funded and is *per-second* the highest-impact spend | C-15/16 deepest; C-3 strained (legibility); C-10 honored (real machines hide info) | L (釘調整 mode is the new system) | H — moves the player's *input surface* outside the firing-balls loop |
| D. Session-Responsive (hybrid) | Risky on test 5 (sympathy ≠ story); §13.2 anti-pattern | Diluted | C-7 spiritually strained | M | M (but trap-shaped) |

---

## Recommendation

**Winner: Direction C — Contextual Minimalism + Diegetic Agency.**

Three reasons.

1. **It's the direction that targets the *untested* acceptance gates.**
   Per skill §14.4, iter 4's job is test 4 (2-hour pacing) and test 5
   (leave-on-your-own with a story to tell). Direction C's minimalism
   directly addresses 2-hour pacing — visual noise is the most reliable
   fatigue source over long sessions — and 釘調整 produces the "I
   tuned this machine to a 34% return" story the player walks away
   with. Iter 3 already won tests 1–3; spending iter 4 polishing those
   gates further is the diminishing-returns trap.

2. **It honors skill §10 iter-3's explicit closing recommendation:**
   "surface the still-flat chucker rate as the gameplay lever (nail-
   adjustment 釘調整) rather than a static config." That's a
   pre-existing pull from the project's own institutional learning.
   The other directions ignore it.

3. **It's the only direction that adds a genuinely new player input.**
   A is more polish on the existing input surface. B is more polish
   on the existing output surface. C adds a between-session input
   loop that the project has never had — and §12.4 (illusion of
   agency, REAL where possible) is one of the skill's ethical north
   stars. Real agency is the rarest design currency.

**Second-best: Direction B — Cinematic Revenge.** It loses because it
is **blocked by the writer's pass** (intent open question 1) and its
asset pipeline (intent open question 2). Iter 4 cannot start B before
those resolve, and the brief is asking for a directional bet *for iter
4*. B's full impact lands when the story bible exists; until then it
degrades to silhouettes-and-subtitles, which is iter 3.5, not iter 4.

**Salvage from the losing directions into the winner.**
- **From A.** Ship the CJK subset font *anyway*. Direction C's
  minimalism does not preclude authentic JP text in the few places it
  remains (the marquee, the chucker label ヘソ, the chapter cards).
  Also adopt the parlor ambient audio sublayer — under §14.2 it's
  under-funded across all directions.
- **From B.** The session-end "what happened tonight" ledger is the
  §14.3 story-to-tell surface and survives perfectly inside C. The
  narrative HUD strip does NOT survive — it contradicts C's
  minimalism — but the session-end summary is purely additive.
- **From D.** The "welcome back" card on session restart survives,
  framed not as cabinet sympathy but as continuity (your nail
  layout persists, your chapter persists). Drop the bezel-mood
  mutation.

The winning direction is therefore: **C as the trunk, with the CJK
font + ambient parlor audio (from A), the session-end ledger (from
B), and the welcome-back continuity card (from D) grafted in.**

---

## Open questions for the synthesis phase

1. **釘調整 fidelity scope.** Are pins draggable individually (full
   nail-adjustment), in groups (preset funnel widths), or only a few
   "regulator pins" (cheapest)? Affects whether iter 4 needs new
   physics tests or just a UI on the existing layout system. The
   answer determines whether C is L or XL.

2. **CJK subset font budget.** Skill §10 iter-1 estimates ~50–100 KB
   for a subset; current WASM is ~480 KB. Is +15–20% size acceptable
   per C-13's "minimal HTML/JS harness" spirit, or do we need a
   per-glyph audit? Blocks any direction that adopts JP text.

3. **Back-panel-as-reach-cue legibility.** Direction C moves reach
   signalling from banner overlays to back-panel mutations. Per
   §13.3 the signal must remain crisp at the gradient (calm
   2.5% / mid 0.7% / premium 0.25% / confirmed 0.05%). What's the
   minimum-viable A/B playtest that validates this *before* PRD-004
   commits? Headless probes cannot answer it.

4. **Audio crossfade primitive (intent open question 7).** C leans
   on the BGM crossfade for reach signalling more heavily than A or
   B do. Do we accept the hard-cut grammar (option a) or invest the
   `quad-snd` envelope wrapper (option b) before iter 4 starts? B
   and C both want this; A can ship without.

5. **Persistence story for 釘調整 layouts and the welcome-back
   card.** Intent open question 6 is still open (`quad-storage` vs.
   `sapp-jsutils` vs. dropping miniquad). Iter 4 needs a real
   persistence shim for the layout + chapter + last-session-summary.
   Pick the shim *before* PRD-004 is cut.
