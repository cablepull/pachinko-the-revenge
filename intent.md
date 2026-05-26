# Intent — pachinko-the-revenge

A single, deeply-realized digital pachinko cabinet (modern CR-style with ST-kakuhen) whose original revenge narrative is delivered *through the reach hierarchy itself*. The reach tiers map to plot beats: provocation, preparation, confrontation, catharsis. Players learn the plot by playing; the plot teaches them the reach taxonomy.

## Problem

Existing digital pachinko falls into one of two failure modes: (a) Westernized slot-with-pachinko-art that fails the 常連 (regular player) sniff test on audio identity and reach discipline, or (b) faithful Japanese ports of licensed cabinets that are inaccessible to anyone outside the existing parlor audience. Neither category builds an original cabinet that respects the canon — most notably CRエヴァンゲリオン's tiered-reach legibility and CR北斗の拳's IP-as-mechanic integration — while delivering a self-contained story. The result is that "good digital pachinko" remains a translation problem, not a design problem.

## Approach

Build one cabinet, deeply, with the reach hierarchy *as* the story engine. The math is the foundation: a real spec sheet (1/199.8 base, 70% ST-kakuhen entry, 165-spin window, 16R), implemented in a pure, headlessly-testable math layer so that authenticity is verifiable via Monte Carlo rather than asserted in prose. The aesthetic is non-negotiable Japanese-pachinko grammar (mixed-script overlays, brass-fanfare catharsis, shounen-anime cut-in animation, original-cast JP voice). The narrative is original IP — not licensed — and it gates progression: later reaches are *literally* later in the revenge arc. Constraints below are load-bearing — every PRD, ADR, RCA, and audit cites them.

## What this is not

- A parlor simulator. Single cabinet, no walking around, no multiple machines, no NPC players.
- A gambling simulator. No real money, no 三店方式 cash-out fiction, no prize exchange.
- A licensed-IP product. The revenge story is original; no attempt to license Eva, Hokuto, or any other property.
- A localized product. JP audio is canonical; subtitles only. No English dub, no machine translation.
- A Western-style slot machine in a pachinko skin. If the aesthetic doesn't pass C-10, ship date is wrong.
- A multi-machine, DLC-driven product. One arc, one machine, then ship.

## Constraints

Each row is a load-bearing invariant. Cite by ID (`C-N`) in PRDs, ADRs, audits, and RCAs. Add new rows in order; do not renumber.

| # | Constraint | Rationale |
|---|------------|-----------|
| C-1 | The math layer (Probability Engine + Cabinet Coordinator) is pure and headlessly testable; no dependencies on UI, audio, or persistence modules. | Math correctness must be reproducible without rendering. Coupled architecture makes regression testing impractical and lets visual bugs mask probability bugs. |
| C-2 | Spec-sheet numbers (1/199.8 base, 1/35.9 kakuhen, 70% ST entry, 165-spin ST window, 16R, ~35 ベース) are canonical and verifiable via Monte Carlo within 3σ tolerances stated in the PRD. | Authenticity rests on real spec discipline. 常連 players will inspect the spec sheet and reject implausible numbers immediately. |
| C-3 | Reach hierarchy is legible: 4 tiers (calm / mid / premium / confirmed), each with distinct visual *and* audio gates. A blind playtester must rank 3 named reaches by bust rate correctly after one jackpot. | Legibility of the reach hierarchy is the addictive core (the Eva lesson), not the underlying math. Illegible reaches collapse the design to a slot. |
| C-4 | The revenge narrative is delivered *through* reach beats, not over them. Each named reach has a `chapter` field; reach eligibility gates by story progression. | Differentiates from generic licensed pachinko. Story-as-mechanic is the Hokuto lesson applied to original IP. |
| C-5 | Original Japanese voice acting only. JP audio with selectable JP/EN subtitles. No dubs, no machine translation, no soundalikes. | Voice identity is half the cabinet's identity per Eva/Hokuto canon. Dubbed pachinko reads as cosplay. |
| C-6 | Determinism: given a seed + input log, the session is bit-for-bit reproducible. | Required for replay testing, regression tests, and shareable session seeds. Non-determinism makes math bugs unreproducible. |
| C-7 | No real money, no telemetry, no network calls. Fully offline. | This is a machine sim, not a gambling sim. Legal-fiction simulation (三店方式) is explicitly out. Network silence is verifiable. |
| C-8 | 60 fps sustained on a 5-year-old laptop. Frame-time p99 < 16.6ms during jackpot fanfare and confirmed-reach cinematic. | Catharsis depends on tight visual rhythm. Dropped frames during fanfare are a hard fail; the moment is unrecoverable. |
| C-9 | Single machine, single narrative arc, single save slot — that is the MVP scope. | Scope discipline. Depth-over-breadth is the chosen strategy; 海物語 has run on the same formula for 25+ years. |
| C-10 | A 常連-class consultant must identify the audio identity as pachinko from sound alone within 30 seconds. | The aesthetic north star. If this fails, no other PXT matters and the product is mis-positioned. |
| C-11 | Save format is versioned (`schemaVersion`), checksummed, and OS-portable between macOS and Windows. | Avoid silent save corruption. Enable forward migration when the schema evolves post-MVP. |
| C-12 | Reach roster (named reaches, tier assignments, beat sequences, weight within tier) is configuration-driven, not hard-coded. | The writer's pass and balancing iterate this table. Code must not block design iteration. |
| C-13 | The shipping target is browser-runnable WebAssembly. The game compiles to a single `.wasm` artifact + minimal HTML/JS harness, runnable from any modern browser with no install step. | Distribution friction is the largest barrier to non-Japanese players experiencing a real pachinko cabinet. A browser-runnable build removes the install/native-deps barrier without compromising C-1, C-6, or C-8. |
| C-14 | Implementation language is Rust; rendering/audio via macroquad. Math layer (`pachinko-core` crate) is `no_std`-compatible and depends on no I/O, no rendering, no audio — only `rand_pcg` for deterministic RNG. | Rust + macroquad is the most mature path to a single deterministic WASM artifact with audio. Isolating `pachinko-core` from the engine satisfies C-1 mechanically: it cannot import macroquad even if a developer tries. |
| C-15 | The cabinet renders a visible ball-and-pin-field theatre. Balls are launched from a knob, fall through a pin field with collisions, and either land in a pocket (chucker, tulips, attacker) or drop off the playfield. The reel-only subset of pachinko *is* a slot machine and fails C-10. | Per pachinko-expertise §1, pachinko is *defined* as a vertical pinball-meets-slot hybrid. A 常連 sniff test (C-10) is unsatisfiable without the ball layer; v0.1 confirmed this empirically — a non-expert observer (the project owner) identified the omission immediately. See `audits/audit-001-cabinet-slot-vs-pachinko-2026-05-25/` for the full ACH. |
| C-16 | Reel spins are triggered ONLY by simulated chucker-entry events emitted by the ball-physics layer. There is no direct input → spin path. The chucker hit rate (chuckers per 100 balls fired) calibrates the effective game pace and is the player's only input lever besides launch power. | This is what makes the spec sheet feel real: regulars adjust launch power to bias the ball stream toward the chucker. Bypassing the ball layer (as v0.1 did) decouples the spec sheet from observable cause-and-effect and reduces the cabinet to a slot. |
| C-17 | The `pachinko-expertise` skill is the project's accumulating institutional knowledge and is updated by every architecture audit that closes. New sections are appended; existing sections are revised when audits invalidate or refine them. The skill is not a static reference. | A discipline that documents lessons only in commit messages or chat history evaporates. A skill that grows with the project becomes the input to the next iteration's design. Drives skill-evolution sections (§7–§10 added 2026-05-25 after iter-2 feedback). |
| C-18 | The cabinet simulates the *feel* of money — yen-equivalent displays, profit/loss indicators, streak multipliers, treasure trickle visualization — without simulating any actual money transactions. ¥4/ball is the canonical rate. | C-7 rules out real-money handling, but the absence of any economic dimension makes the game feel stake-less. Real pachinko has a tangible "this ball just cost me ¥4, this jackpot just paid me ¥5,760" loop; we reproduce the *feel* without crossing the legal/regulatory line C-7 draws. |
| C-19 | The player's primary depth-axis input is **釘調整 (nail-adjustment)**: chapter-gated tuning of pin-cluster positions that drives ベース (chucker rate) as an emergent measurement, not a configured constant. The cabinet exposes the predicted ベース as an honest confidence interval, never as a flattering or massaged number. | Skill §12.4 names "illusion of agency made real" as one of the ethical ceilings of game design; pachinko's deepest hook for the 常連 archetype (§3) is reading machine personality. Iter 1–3 had no genuine input beyond launch firing; C-19 makes the cabinet honor the player's input loop. Drove audit-002 (2026-05-25) and PRD-004. |
| C-20 | The default cabinet canvas is **contextually minimal** — only the playfield, balls, chucker, knob, and the always-visible P/L survive idle base play. Every other HUD element (data lamp, tuning mode, session ledger, welcome-back card) is opt-in and surfaces only when relevant. Information surfaces appear *in place* near their source (P/L floaters off the chucker, chapter cards at the moment of unlock, reach signalling via back-panel mutations). | Skill §13.3 names "information-rich, surprise-honest" as the single pattern separating success from failure; visual noise during base play (80% of session time) dilutes the signal the rare reach is supposed to carry. C-20 protects the signal-to-noise gradient that C-3 (reach hierarchy legibility) depends on. |

## Assumptions

Beliefs that, if invalidated, would change the design. RCAs invalidate assumptions; replace `Open` with `Invalidated` and link to the RCA.

| # | Assumption | Basis | Status |
|---|-----------|-------|--------|
| A1 | Original IP can deliver the catharsis ceiling that licensed IP delivers in canonical machines. | CR北斗 and CRエヴァ succeeded *because of how they used IP*, not merely because the IP was famous. The reach grammar is what carries the emotional payload. | Open |
| A2 | One deep narrative arc beats multiple shallow arcs for player attachment and replay. | 海物語 runs on a single deep formula for 25+ years. Eva runs many entries but each is a singular arc, not a collection. | Open |
| A3 | JP audio with subtitles is acceptable to a non-Japanese-speaking playerbase. | Anime fandom precedent; subtitled JP voice is the established preference for genre-authentic experiences. | Open |
| A4 | Modern ST-kakuhen (fixed-window) feels more "canonical pachinko" to the target audience than legacy probabilistic kakuhen or 1980s Fever-only. | 2010s peak-CR convention. ST is what regulars associate with current pachinko grammar. | Open |
| A5 | 4 reach tiers is the right cardinality — neither 3 (too coarse) nor 5+ (illegible). | Eva uses ~3 functional bands with sub-variants; AKB48 uses 4 with character routing. 4 is the sweet spot for legibility-with-depth. | Open |
| A6 | A 30-minute story arc is long enough to deliver catharsis and short enough to ship at MVP scope. | Pachinko sessions can be 6+ hours, but the *story* doesn't need to be that long if reach beats reward repeat exposure. | Open |

## Open questions

1. **Story bible.** Protagonist, antagonist, arc structure, signature voice lines. Needs a writer's pass before reach design (C-4) can produce final art. Currently blocking the production tier of the TDD plan.
2. **Art direction within the canon grammar.** 2D hand-drawn or 3D-rendered cut-ins? Affects pipeline, budget, and the C-8 frame-time budget for confirmed-reach cinematics.
3. ~~**Engine / tech stack.**~~ **Resolved 2026-05-25:** Rust + macroquad → WASM (see C-13, C-14). Math layer in `pachinko-core` crate, `no_std`-compatible, deterministic via `rand_pcg`. Rendering/audio via macroquad. WASM build via `cargo build --target wasm32-unknown-unknown`.
4. **Voice casting.** Original-cast (C-5) requires identifying and contracting talent. Affects schedule.
5. **Data-lamp truthfulness.** Display true probability (authentic) or a lightly massaged tension number (better feel)? Defer to alpha playtest feedback.
6. ~~**WASM persistence.**~~ **Resolved 2026-05-25:** iter 4 will resolve via `quad-storage` (smallest WASM-size delta; the `sapp-jsutils` alternative requires custom JS glue we'd carry forever). PRD-004 stories address this on the critical path. PRD R-18..R-20 gated on this story landing.
7. ~~**BGM crossfade primitive.**~~ **Resolved 2026-05-25:** iter 4 will ship a minimal `quad-snd` fade-envelope wrapper (option b). The hard-cut grammar is acceptable for high-energy state changes (jackpot, kakuhen entry), but reach signalling via back-panel mutations (per C-20) leans on audio to lead the visual; that audio lead requires the crossfade primitive. PRD-004 stories address this on the critical path.
8. **Ball physics fidelity.** v0.2 adds the C-15/C-16 ball-and-pin layer at MVP fidelity (~50 balls, ~80 pins, simple circle-circle elastic collisions). Open: when do we add pin-pattern variation (the "nail adjustment" 釘調整 that defines machine personality in real parlors)? It's a major gameplay lever but post-MVP.

---

This file is the anchor. magnetfragnet's nudge engine detects when PRDs, stories, or ADRs drift from it. Keep this file in scope but tight — every line here is referenced by ID downstream.
