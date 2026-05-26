# Iter 4 mechanics redesign — competing directions

Iter 3 closed with the cabinet *working* (math, ball-layer, depth, economy, celebrations) but with two empirically un-validated risks: (a) the chucker rate sits at ~15% versus PRD-002 R-29's 25–40% target, and (b) §14 test 4 (the 2-hour pacing) and test 5 (leave-on-your-own) are entirely speculative — headless probes have only run at 30s horizon. The skill §10 (iter 3 entry) recommends nail-adjustment 釘調整 as iter 4's lever, but that's one bet among several. This document presents four directions that each pick a *different primary defect* and answer the same ten mechanics questions accordingly. They are mutually-exclusive primary bets; the Recommendation section names the winner and what to salvage from the rest.

Spec-sheet invariants (C-2): 1/199.8 base, 1/35.9 kakuhen, 70% entry, 165-spin ST, 16R × 90 balls, ¥4/ball. Every direction below must preserve the *contract* of these numbers, though some allow chapter-scaled or player-influenced *parameters* on top.

---

## Direction A: Nail-adjustment 釘調整 — make ベース the agentic core

**Vision.** The 15→25-40% chucker-rate gap is the most concrete defect; close it by making the pin layout the player's *visible* gameplay lever rather than a tuned constant. The player sees `BASE 17%` next to their P/L, can spend a between-session "tuning" action (or in-session, between chapters) to nudge specific pin clusters, and watches the rate respond. This is the §12.4 "illusion of agency made real" — agency that *is* real because physics emergence is real. The bet: pachinko's deepest hook for regulars is the feeling of *reading* a machine, and we currently give the player nothing to read.

**Concrete mechanics changes.**
1. Pin layout becomes a typed config struct with `f32` perturbation knobs per cluster (left-funnel-tilt, right-funnel-tilt, chucker-mouth-width, guide-pin-vertical-offset). 4–6 knobs total.
2. A "tuning workshop" overlay (between chapters, or via a Tab-style toggle) lets the player drag a knob by a bounded amount per session.
3. Each tuning action runs a 500-ball headless Monte-Carlo *in-process* (already-existing physics, no UI) and surfaces the predicted ベース to the player as a confidence interval ("17.4% ± 3.2%"). This is the §12.4 RESPECTS column: the prediction is honest.
4. The data-lamp gains a "tuning history" view: prior layouts and their measured rates over the player's session.
5. Tuning is **chapter-gated** — chapter 1 = no tuning (locked, "stock layout"), chapter 2 = unlock 2 knobs, chapter 3 = 4 knobs, chapter 4 = full set. This wires §14.3 (come-back-tomorrow) to a tangible mechanic, not just narrative.
6. Visual: the pins literally redraw between chapters. The "feel" of a different machine is delivered without violating C-9 (still one cabinet).
7. Streak multiplier: **dropped** (§11 says either give it teeth or drop; this direction has no use for cosmetic-only numbers).

(Analog launch-power as a second agency lever is deliberately *out of scope* here — it would dilute the "nails are the primary lever" bet. Flagged in open questions.)

**Math implications.** No change to C-2 spec sheet. Per §11.1, EV_per_spin is `−cost_per_spin + return × ¥4`; this direction changes `cost_per_spin = balls_consumed_per_spin × ¥4` from the *denominator* side by letting the player improve return-rate-per-fired-ball. A 15→30% ベース cuts effective cost-per-spin nearly in half without touching jackpot probability — variance unchanged, runway (§11.2 `B/u`) roughly doubles. Sessions get longer without becoming flatter.

**Psychology implications.** §12.4 RESPECT column (genuine emergent agency). §12.1 RESPECT (probability remains visible). Risk: §12.3 sunk-cost amplifies if the player feels their *layout* (not just balls) is "invested." Mitigate by autosaving prior layouts so the player can revert without loss-feel.

**Skill citations.** §10 iter-3 recommendation; §11.2 (runway extension); §12.4 (agency); §13.3 (information-richness — tuning *is* information); §14.3 (chapter-tied unlocks).

**Intent constraints satisfied / strained.** Satisfies C-15 (ball-and-pin layer becomes more central), C-16 (chucker still only spin trigger; tuning changes *rate*, not *trigger*), C-12 (config-driven roster generalizes to config-driven pin layout). Strains C-3 legibility risk — if tuning UI is bad, it reads as "options menu" not pachinko; mitigate via diegetic presentation (player picks up a tool, taps a pin). Strains §14.1 test 1 (30-sec test) if tuning UI is on-screen at first impression — solve by keeping it hidden by default like the data lamp.

**Effort estimate: M.** Pin-layout config struct is small. Headless-MC tuning is a loop over the existing physics — no new physics code. Tuning overlay reuses iter-3's hidden-by-default + toggle-button + RenderState-driven-animation patterns directly. Chapter-tied unlocks reuse the existing chapter mechanic. Persistence reuses the save schema. The risk is *not* code volume; it's the UX call (see open question 1).

---

## Direction B: Session arc — make 2-hour play feel like a story, not a grind

**Vision.** Skill §14.4 says the project passes tests 1–3 but tests 4 and 5 are *untested*. Iter 4 should target the actually-unproven failure modes rather than improving the already-passing 30-sec impression. The bet: the chucker rate is sufficient (§13.2 says 25–40% is a range, not a floor; 15% is on the punishing side but not broken); what's broken is that nothing about minute 90 feels different from minute 5 except numbers. Make the *arc* the lever.

**Concrete mechanics changes.**
1. **Four session phases** mapped to §12.6: Onboarding (0–10 min, equivalent to PRD R-12 "calm only"), Calibration (10–30 min, mid tier unlocks), Flow (30 min–2hr, premium becomes available, BGM rotates between two base loops to combat audio fatigue), Late (2hr+, confirmed-reach density doubles, kakuhen entry shows an "extended session" stinger).
2. **ST window varies by chapter** (answering question 4): chapter 1 = 150, chapter 2 = 165 (canonical), chapter 3 = 180, chapter 4 = 200. All within §13.2's 150–200 band. The later in the story, the longer the rush window — feels like the protagonist gathers momentum. C-2 *contract* preserved: 165 is the canonical *default*; per-chapter overrides documented in the spec.
3. **Afterglow phase** (answering question 7): on jackpot end → BASE, a 5–10s "afterglow" where the cabinet stays in a warmer tint, BGM volume is at 80%, and any chucker entries during the window play a special trickle SFX. Matches §12.7 dopamine-spike duration. No spin processing change — the existing BETWEEN_ROUNDS already has duration; this adds a new "AFTERGLOW" state between BETWEEN_ROUNDS exit and BASE.
4. **Session summary on R / reset** (answering question 10): named moments ("the warehouse confrontation hit on spin 412", "your longest dry streak: 287 spins", "chapter 3 unlocked at minute 47"). Provides a natural "good place to stop."
5. **Calibration warm-up**: PRD R-12 stays in effect, but additionally, the first jackpot of any session is guaranteed to enter kakuhen (70% becomes "first JP = 100%, then 70% thereafter"). §12.5 RESPECT — structural warm-up, not RNG manipulation.
6. **Streak multiplier: dropped** (per §11, no teeth here).
7. **Confirmed-reach bust: stay at 5%** (§13.2 says >15% breaks trust; tightening below 5% buys little and risks overpromising).
8. **Near-miss density: not deliberately engineered** beyond what the existing tier hierarchy already gives — this direction trusts §13.3's "information-rich, surprise-honest" baseline.

**Math implications.** ST window variance by chapter shifts kakuhen-chain expected length (§11.3): at W=200 instead of 165, P(0 JP in window) drops from ~1% to ~0.3%, and mean chain length grows by ~25%. Late-game sessions get longer rushes — exactly the "rare, high-intensity bursts" §12.6 recommends. Afterglow does not change EV (jackpot is already complete by then). First-JP-always-enters-kakuhen is a small +EV bias of ~+¥3/spin amortized over the session; well below the regulation 80% return cap.

**Psychology implications.** §12.6 RESPECT (late-session pacing is more *meaningful*, not faster). §12.7 RESPECT (afterglow protects the high). §12.5 RESPECT (structural warm-up, not RNG-rigged). §14.5 ("leave the machine" test) directly targeted by session summary.

**Skill citations.** §12.5, §12.6, §12.7, §13.2 (ST band), §14.1 tests 4–5, §14.3.

**Intent constraints satisfied / strained.** Satisfies C-3 (legibility preserved), C-4 (chapter-scaled ST window deepens story-as-mechanic), C-18 (P/L untouched). Strains C-2: ST window of 165 is "canonical" — needs explicit ADR documenting that 165 is *one configuration*, with the spec sheet now declaring chapter-scaled overrides. Defensible because the 150–200 band is per §13.2, but it's a contract-edge change.

**Effort estimate: M.** Mostly state-machine + render-state additions; no new UI of significant complexity. AFTERGLOW state needs careful test coverage (new transitions). Session summary is one screen, mostly data.

---

## Direction C: Reach engineering — sharpen the near-miss / legibility gradient

**Vision.** §13.3 names the single pattern that separates success from failure: information-rich, surprise-honest. The current reach roster (8 reaches × 4 tiers) is correct *in structure* but uses generic beat sequences and undifferentiated near-miss visuals. A blind playtester probably *cannot* yet pass C-3's "rank 3 named reaches by bust rate" test. The bet: deepen the reach grammar before adding any new system. Make the cabinet's most-used animation (reaches occur on ~3.5% of spins, vs jackpots at 0.5%) genuinely diagnostic.

**Concrete mechanics changes.**
1. **Engineered near-miss visualizations** (answering question 9): on bust, reel 3 lands deterministically at `(target ± 1) mod N` for calm/mid, `(target ± 2) mod N` for premium (the reel "almost" lined up but visibly missed). This is §12.2 — the brain reads off-by-one as "almost won." Critically, the *tier-distinct distance* makes the bust itself informative: mid bust is closer than calm bust.
2. **Audio leitmotif per tier**: a distinct 2-note tag plays at the *start* of every reach (mid = perfect 4th up, premium = augmented chord, confirmed = opening-theme prefix). Implements §12.2 RESPECT — players learn the tag within session 1, after which reaches are pre-classified by audio in the first 250ms (§8 "three-beat structure" / "audio leads visual").
3. **Premium-reach character cut-ins are unique per named reach** (not shared art). `warehouse-confrontation` and `first-strike-feint` currently share `BladeSparks` + `AntagonistSilhouette` — separate them so each premium has its *own* recognizable visual signature.
4. **Calm tier de-bombing**: §13.2 anti-pattern "premium reach used too often / calm carpet-bombing." Reduce calm tier frequency 2.5% → 1.8% (and bump direct-hit slightly to preserve C-2 base rate). Each calm-tier appearance is more meaningful; reaches that bust 98% should be rare, not constant.
5. **Confirmed-reach bust: tighten 5% → 3%** (answering question 6). Still inside §13.2's "≥85% hit" trust band; the 2pp tighter pulls confirmed even further from premium, sharpening the gradient.
6. **Streak multiplier: dropped** (no teeth → drop, per §11).
7. **Chapter unlock**: stays JP-gated (answering question 3 — option among gates). This direction doesn't change progression mechanics; it improves what's *behind* the chapter gate.
8. **No tuning UI, no session-arc states, no 1パチ mode.** Pure focus.

**Math implications.** C-2 contract: tier frequencies and hit rates change. Calm 2.5%→1.8% requires direct_hit_base to absorb ~0.7pp of frequency at 2% hit = ~0.014pp of jackpot share; recalibrate direct_hit_base from 0.000530 → ~0.000670 to keep base p=1/199.8. Confirmed 95%→97% hit at 0.05% freq shifts JP share by +0.001pp, absorbed by symmetric direct_hit reduction. The Monte-Carlo tests (R-2, R-3, R-6, R-7) all need updating but stay within 3σ of canonical targets. §11.5 payout shape unchanged. §11.3 kakuhen geometry unchanged.

**Psychology implications.** §12.2 RESPECT (tier-distinct near-misses honor signal value). §13.3 RESPECT (information gradient preserved). §12.1 RESPECT (probability still visible). Risk: more theatrical busts can edge toward EXPLOIT if calm-tier becomes "constant near-misses" — mitigated by *reducing* calm frequency, not increasing it.

**Skill citations.** §8 (event grammar — audio leads visual, three-beat structure), §12.2 (near-miss neuroscience), §13.3 (information-rich principle), §13.2 (anti-pattern catalog).

**Intent constraints satisfied / strained.** Strongly satisfies C-3 (legibility is the explicit target), C-4 (each premium reach now has unique story art). Strains C-2 — touches the canonical spec numbers, needs ADR. Strains C-12 only mildly (roster stays config-driven; values change).

**Effort estimate: M.** Roster art expansion is the bulk; spec-sheet retuning and test updates are mechanical. Audio leitmotif generation is small (2-note synths, already in audio.rs grammar). Engineered near-miss requires reel-stop logic to know its target — minor refactor in render path.

---

## Direction D: Economy deepening — make ¥ the storyline

**Vision.** Iter 3 added the P/L indicator and called it the highest-leverage HUD element. But pachinko's deepest emotional layer is the *stake* — players don't remember spin counts, they remember ¥. The bet: turn the economy from a passive readout into the cabinet's mood spine. Introduce 1パチ (¥1/ball) mode as a casual onramp per §11.5, give the streak multiplier *real teeth*, and let the P/L tint drive bezel lighting so the player *feels* their session shape peripherally.

**Concrete mechanics changes.**
1. **1パチ mode** (answering question 8): selectable at session start. yen_per_ball = 1 instead of 4. Jackpot payout becomes ¥1,440 instead of ¥5,760 — *catharsis budget per JP drops*, but session runway (§11.2 `B/u`) quadruples. Onramp for new players per §3 ライトユーザー archetype. Stored as `Spec::yen_per_ball` already (anti-pattern in PRD-003 explicitly calls this out).
2. **Streak multiplier with real teeth** (answering question 5): inside kakuhen, each chained jackpot adds **+10% bonus balls to the next jackpot's payout**. JP 2 = 990 balls/round, JP 3 = 1,080, capped at +50% (JP 6+). This is a real EV bonus — §11.5 lottery-shape concept applied to chain depth. Crucially still within regulation 80% cap because chains are rare (§11.3 — 5+ JP chains are ~17%).
3. **P/L driven bezel tint**: existing bezel lighting (R-37) gets a P/L overlay — strongly negative session = cool blue undertone in base-state breathing pulse, strongly positive = warm gold accent. The player reads their session mood peripherally without staring at the P/L number. §12.3 RESPECT amplified (sunk-cost made *more* transparent, not less).
4. **Treasure-trickle gets typed**: each ball return shows `+¥4` (¥4 mode) or `+¥1` (¥1 mode); each round-clear shows `+¥360` (16R round-pay). Reinforces the *unit* of value.
5. **Session bankroll**: a starting "wallet" of ¥10,000 displayed at session start. Doesn't gate play (no game over, C-7); but when wallet reaches 0, a "you would normally stop here" advisory appears. §14.5 / §12.3 — surfacing the ruin threshold without enforcing it.
6. **Confirmed bust: stay at 5%** (this direction doesn't touch reach math).
7. **ST window: stays at 165** (this direction doesn't touch state machine).
8. **No nail tuning, no session-arc phases.** Pure economy play.

**Math implications.** 1パチ mode: linear scaling of all yen-denominated quantities; EV math (§11.1) unchanged in *ball units*, divided by 4 in ¥ units. Streak bonus: per §11.3, geometric chain mean ~1.4 JP per kakuhen entry; mean payout per kakuhen entry shifts from 1,440 × 1.4 = 2,016 balls to ~2,120 balls (+5% effective). Regulation check: 80% return rate has slack; this stays inside. Variance (§11.2): chain-bonus shape adds a small right-skew — long chains feel even more exceptional. Bankroll display does not affect math.

**Psychology implications.** §12.3 RESPECT (P/L visibility taken further; bankroll surfaces ruin threshold). §12.6 RESPECT (1パチ extends late-session viability by reducing per-spin stake). §11.5 lottery-shape applied to chains — this is the one section where this direction edges toward §12.2 EXPLOIT (chain bonus is a "near-miss" of "you almost got the big chain"). Defensible because the bonus *actually pays out*, but flag in the synthesis.

**Skill citations.** §9 (ball economy), §11.5 (payout shapes), §12.3 (sunk-cost transparency), §12.6 (session-length levers), §14.5.

**Intent constraints satisfied / strained.** Satisfies C-18 (the cabinet's "feel of money" deepens), C-7 (still no real transactions). Strains C-2 mildly (streak bonus adds a payout multiplier on top of canonical 16R × 90; ADR needed to declare the chain-bonus as a layered modifier, not a spec change). Satisfies C-9 (one machine still).

**Effort estimate: S–M.** 1パチ is a config toggle and HUD pluralization. Streak teeth is a `chain_depth` counter in BetweenRounds with a bonus calc in JackpotRound entry. Bezel-tint blend is a render shader-ish blend. Bankroll display is text + one threshold check. Total significantly less code than A or B.

---

## Comparison matrix

| Direction | §14 test pass likelihood | Math integrity (C-2) | Psychology ethics (§12) | Scope risk | Player-skill ceiling |
|---|---|---|---|---|---|
| **A: Nail-adjustment** | Improves test 4 (arc gets variety per chapter) and test 5 (tuning is a save-point); minor risk to test 1 (30-sec) if UI leaks | Preserved (no C-2 number changes; pin physics emergent) | RESPECT (§12.4 — real agency) | Medium — heavy reuse of iter-3 patterns; the risk is UX, not code volume | **Highest** — genuine optimization loop |
| **B: Session arc** | Directly targets tests 4 and 5 — the actually-untested ones | Strained (chapter-scaled ST window needs ADR; first-JP-kakuhen is small +EV) | RESPECT (§12.5/12.6/12.7 all RESPECT column) | Medium — state-machine additions, AFTERGLOW state | Low–medium |
| **C: Reach engineering** | Sharpens test 2 (5-min reach legibility) and test 4 (premium reaches feel different over time) | Strained (touches canonical tier numbers, needs Monte-Carlo retune + ADR) | RESPECT (§12.2 with care) | Medium — roster art + audio expansion | Medium (depth comes from *reading* the cabinet) |
| **D: Economy deepening** | Targets test 5 (bankroll advisory) and test 4 (1パチ extends viability) | Strained mildly (chain bonus is a layered modifier, ADR-grade) | RESPECT-mostly; §11.5 chain bonus edges toward §12.2 EXPLOIT | **Low** — mostly additive on existing iter-3 infra | Low |

---

## Recommendation

**Winner: Direction A (Nail-adjustment 釘調整).**

Three reasons:

1. **It closes the only empirically-measured PRD violation we have.** Chucker rate at 15% vs R-29's 25–40% is the one concrete failure on the board. The other defects (test 4, test 5, near-miss density) are *suspected* but untested. Iter 4 should fix what we know is broken before speculating.
2. **Skill §10 (iter-3 entry) names this explicitly.** Discarding that recommendation requires invalidating evidence; nothing has emerged that does.
3. **It satisfies the highest player-skill ceiling of the four.** §11.6's decision framework names "what does the player *do*" as a load-bearing question; A is the only direction where the answer goes beyond "watch and react." The pachinko-canon 常連 archetype (§3) explicitly cares about machine personality — nail-adjustment *is* machine personality.

**Runner-up: Direction B (Session arc).** Loses because tests 4 and 5 are speculative pending real playtest; building infrastructure for unproven failure modes is premature. But B is structurally compatible with A — chapter-tied tuning unlocks (A.5) are themselves a session-arc mechanic.

**Salvage candidates (NOT a mandate — PRD-004 should pick at most 1–2 of these, not all four):** the session summary screen (B.4, S effort) is the leading candidate because it directly addresses §14.5 test 5; engineered near-miss visualization (C.1) is second because it's one render-path change with high information density; afterglow state (B.3) and 1パチ toggle (D.1) are tertiary — both small but they expand iter 4's scope by introducing new state or new mode flags. If A.5 (chapter-tied tuning unlocks) eats the synthesis budget, *defer all four salvage candidates to iter 5*. The point of recommending A is to focus, not to assemble.

**Dropped across all four**: cosmetic-only streak multiplier (per §11 binary, no teeth → drop), per question 5. Direction D's streak-with-teeth (real chain bonus) is *also* dropped because it requires a C-2 ADR and edges toward §12.2 EXPLOIT — not worth iter 4's complexity budget.

---

## Open questions for synthesis

1. **Does player-tunable nails violate §14.1 test 1 (30-sec sniff test) by introducing setup friction?** If the cabinet shows a "tune your pins" prompt on session start, the 常連 may read this as a video-game options menu, not pachinko. Mitigation: tuning is *hidden by default* (mirroring the data-lamp pattern from iter 3) and discoverable only after the first chapter unlock. But this is a real risk that needs a UX call before PRD-004 commits.

2. **What's our position on §12 EXPLOIT levers — categorical reject, or case-by-case?** Direction D's chain-bonus is the cleanest test case: it's a real payout, but its *announcement grammar* could lean exploit (huge "STREAK ×3 +30%!" banners) or respect (a quiet `+90 BONUS` line on the round-clear). Iter 4 needs a documented stance because every animation decision recapitulates this question.

3. **Is intent.md open question #5 (data-lamp truthfulness — display true probability vs lightly-massaged tension number) blocked by Direction A?** Nail-adjustment surfaces the *predicted* ベース to the player; that prediction is itself a "data lamp" choice. If we commit to honest CI ranges ("17.4% ± 3.2%"), we've answered open #5 implicitly in the RESPECT direction. The synthesis should make that intentional.

4. **Does the C-2 ADR for Direction A need to redeclare ベース as a derived (not configured) spec quantity?** Currently the spec sheet (`pachinko-core/src/spec.rs`) does not name a base-game-rate field — ベース is implicit, emergent from the playfield physics. If iter 4 makes the pin layout player-influenced, the spec contract should explicitly say "ベース is a derived measurement, not a guaranteed constant; the canonical 25–40% band is a *layout-tuning target*, not a math-layer invariant." This must precede PRD-004 because R-29's wording presupposes a static rate.

5. **Should the analog launch-power input be a sibling lever to nails, or deferred?** Real regulars vary launch strength continuously (§12.4); SPACE is currently binary. Adding it alongside Direction A would deepen agency but dilute the "nails are the primary lever" framing. Decision for synthesis: lever-count discipline vs. authenticity ceiling. (Chapter-scaled ST window from Direction B.2 is a related but separate "add a second lever" question — same trade-off.)
