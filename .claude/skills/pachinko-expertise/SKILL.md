---
name: pachinko-expertise
description: Domain expertise for designing pachinko games — game theory and probability mechanics (payout curves, kakuhen/jitan loops, ball economy), Japanese pachinko culture and parlor atmosphere, and the design language of historically successful machines (CR Hokuto no Ken, Evangelion, Umi Monogatari, Fever series). Use whenever the project touches game balance, reel/feature design, sound/visual feel, theming, or parlor authenticity.
---

# Pachinko Expertise

Reference knowledge for designing `pachinko-the-revenge`. Use this skill when work touches mechanics, math, theming, audio/visual feel, or culture. Cite specific machines when relevant — generic "casino game" thinking will produce something that feels nothing like pachinko.

## 1. Mechanics — what pachinko actually is

Pachinko is a vertical pinball-meets-slot hybrid. Balls (11mm steel) fall through a pin field; landing in the **start chucker** (中央ヘソ, *heso*) triggers a digital reel spin (the "digi-pachi" / CR machine layer). The pin field is mostly aesthetic theater — the real game is the reel layer the chucker gates access to.

Core loop:
1. Player fires balls continuously (knob controls launch strength, not aim per shot).
2. Ball enters chucker → triggers a *figure* spin (3 reels, typically).
3. Matching figures = **大当たり (oo-atari, "jackpot")** → attacker (アタッカー) opens, balls pour into the payout pocket for a fixed round count (e.g., 16 rounds × 9 balls).
4. After jackpot, machine may enter **確変 (kakuhen, "probability change")** — jackpot odds dramatically improved (e.g., 1/319 → 1/35.9) until the next non-kakuhen jackpot or a fixed spin count.
5. **時短 (jitan, "time reduction")** — electric tulips open more readily, slowing ball depletion. Often follows kakuhen.

The emotional architecture: long stretches of slow ball loss punctuated by sudden, escalating "this might be the one" moments. **Pachinko is not about winning. It is about *the buildup to possibly winning*.** Reach / リーチ (reach) animations — when 2 of 3 reels match and the 3rd is spinning — are where 80% of the design effort goes. The reach is the story; the result is the punctuation.

## 2. Game theory & math

### Spec sheet anatomy (a real machine has all of these)

- **Jackpot probability** (大当たり確率): low base, e.g., 1/319.69, 1/199.8, 1/99.9. Higher number = harder, bigger payout per hit.
- **Kakuhen probability** (確変突入率): % of jackpots that enter kakuhen. 50–80% typical.
- **Kakuhen continuation rate** (継続率): chance of chaining jackpots in kakuhen. 65–80% is the modern sweet spot.
- **Round count per jackpot**: 4R / 8R / 16R variants; "lucky number" 7 is heavily themed.
- **Payout per round**: ~9 balls per pocket entry × ~10 entries per round ≈ 90 balls; 16R ≈ 1,440 balls ≈ ¥5,760 at ¥4/ball.
- **Base game rate (ベース)**: balls returned per 100 fired in normal play. ~30–40 means slow bleed, the desired pacing.

### Why these numbers matter for design

The expected return is set by regulation (Japan: cannot exceed ~80% over a session-length window after fees). All "feel" knobs sit *inside* that constraint:

- **Variance** is the design lever. Same EV, wildly different feel: a 1/99 machine with 50% kakuhen feels like a casual chat; a 1/399 machine with 80% continuation is a multi-hour death-or-glory ride.
- **Reach hierarchy** drives perceived agency. Low-tier reaches (slow spin, dim lighting) bust ~95% of the time. Mid-tier (character cut-in, voice line) ~60%. **Premium reaches** (full anime sequence, screen-takeover, opening theme) ~5–15% bust — they're effectively winning announcements with theatrical delay. Players read the reach taxonomy *by heart* within an hour of play.
- **Hot/cold streak illusion**: spin counts since last jackpot are visible on the data display (データランプ) above each machine. Players hunt "ハマり台" (hama-dai, "stuck machines") past their theoretical mean. Memoryless probability, but the *belief* is load-bearing for the parlor economy.
- **STチャート (ST chart)** machines: kakuhen is a fixed-length window (e.g., 165 spins) rather than probabilistic. Pure "can you hit inside the window" tension. Modern preference.

### Don't shortcut these

If `pachinko-the-revenge` is a digital recreation, replicate the *spec sheet* explicitly. Players who know pachinko will check: "what's the 確変突入率?" If the answer is "we didn't model that," it's not pachinko, it's a slot machine wearing a costume.

## 3. Culture & parlor atmosphere

The sensory environment is half the product. A pachinko parlor is:

- **Loud** — 85–90 dB sustained. Machine BGM, ball cascade, jackpot fanfares, in-store announcements over PA, the *kacha-kacha-kacha* of balls in trays. Quiet is wrong.
- **Smoky** (historically; less so post-2020 separation laws) — yellowed walls, ventilation hum.
- **Densely lit** — every machine is a self-contained light show, plus parlor signage, plus the data lamps strobing above each station. No ambient dimness; uniformly bright but chaotic.
- **Anonymized social** — rows of players, no eye contact, no conversation. The community is parasocial: you're playing *alongside* strangers, all watching different machines tell different stories.
- **Ritualized** — players arrive at 開店 (opening), queue for their preferred台 (dai, machine), inspect data lamps, settle in. Smoking room breaks. Lunch from the parlor counter. Many sessions are 6–10 hours.
- **Tied to local economy** — winnings are exchanged for *prizes* (景品, keihin), then traded for cash at a separate, deliberately off-premise 三店方式 (sanmagaten-houshiki, three-shop system) booth. This legal fiction shapes everything: pachinko is officially gambling-adjacent entertainment, never gambling. Authenticity demands acknowledging this even if the digital recreation doesn't simulate cash-out.

### Player archetypes worth designing for

- **常連 (jouren)** — regulars. Know every machine's spec, track ハマり, optimize seating. Want depth and accurate math.
- **新台狙い (shindai-nerai)** — chase new releases. Want spectacle, novelty, fresh IP tie-ins.
- **ライトユーザー** — casual. Pulled in by theme (favorite anime). Want flashy reaches, friendly variance.
- **Foreign / nostalgia players** — drawn by aesthetic and Japaneseness. Want the atmosphere recreated faithfully — the smoke, the chime, the data lamp.

## 4. The canon — historically dominant machines

Reference these by name. They each solved a different design problem.

### CR Fever (1980, SANKYO)
The first digital reel pachinko. Established the 3-reel jackpot trigger and the "fever" verbal language — "フィーバー!" is still shouted on jackpot in legacy machines. Look: chrome, neon, primary colors, disco. Sound: pure 1980s synth fanfare. **Lesson:** the jackpot announcement is a *catharsis ritual*, not a notification.

### CR 海物語 (Umi Monogatari, "Sea Story") — Sanyo, 1999, still in production
The reference "casual" pachinko. Cute sea creatures, ukulele BGM, low variance (~1/199), high kakuhen rate. The machine grandma plays. **Lesson:** warmth and predictability are valid design goals. Not every pachinko has to be a punishment. Soft pastel palette, gentle steel-drum/marimba sound design, simple reaches.

### CR 北斗の拳 (Hokuto no Ken / Fist of the North Star) — Sammy, 2003 onward
The IP-tie-in template. Used the anime's bombast — "おまえはもう死んでいる" voice clips, lightning effects, screen-shaking reaches. High variance, brutal. **Lesson:** if you license IP, *use it ruthlessly*. The 北斗 line is famous because every reach feels like a Kenshiro fight scene, not a slot dressed in Kenshiro art. Voice acting must be original cast, not soundalikes.

### CR エヴァンゲリオン (Evangelion) — Bisty/Sammy, 2005 onward, 17+ entries
The benchmark for tiered reaches and "premium" announcement design. Has dozens of reach types, each with a known bust rate the regulars memorize. Uses the actual show's score (Decisive Battle, A Cruel Angel's Thesis) for premium triggers — hearing the opening theme cue mid-spin is a confirmed-win signal that produces a physiological response in lapsed players. **Lesson:** build a *legible* reach hierarchy. Players should be able to learn "if I see X visual cue, I'm probably winning." That literacy is the addictive part, not the math.

### CRぱちんこ AKB48 — Kyoraku, 2014
The idol-tie-in model. Live-action video, character routes, "encore" mechanics. **Lesson:** parasocial attachment to characters extends session length more reliably than jackpot frequency.

### Pachi-slot adjacency
Note `pachinko-the-revenge` should not blur into パチスロ (pachi-slot / 4-reel slot machines). Pachislot has different mechanics (manual reel stop, AT/ART modes, different regulation). The genres feel related but design conventions diverge sharply. Decide which you're making.

## 5. Look, feel, atmosphere — concrete design directives

When designing screens, sound, or feel:

### Visual
- **Layered density.** A pachinko machine front has: pin field, reel display (LCD), side LED strips, attacker doors, payout tray, knob, data lamp above. Empty space is wrong. Even the bezel is decorated.
- **Color saturation is high.** Pastel-cute or chrome-aggressive, rarely muted.
- **Typography is mixed-script chaos.** 漢字 + ひらがな + katakana + romaji + numerals, often in the same line. "確変中!! KAKUHEN MODE ★ FEVER ☆"
- **Animation is shounen-anime grammar.** Speed lines, camera pushes, freeze-frame zooms, screen cracks, character cut-ins from the corner. Easing curves are punchy, not smooth — snap to keyframe, hold, snap to next.
- **Reels are 3 or 1 (single-window).** 7s are special. Matching numbers > matching characters > matching specials.

### Audio
- **BGM is loopy and short** (30–90s), with explicit "calm phase" and "tension phase" tracks the machine swaps between by game state. Music change *is* a tell — regulars hear a key change and know a reach upgraded.
- **SFX layers thickly.** Ball physics clicks, pin hits, chucker chime ("チャリン"), reel spin whir, reel stop *clack*, jackpot fanfare. Never have just one sound playing.
- **Voice lines.** Character call-outs on reach start, mid-reach, bust, hit. Original cast if licensed. Japanese with optional subtitles, not localized dubs — the original VO is part of the feel.
- **Jackpot fanfare is sacred.** 10–20 seconds, brass-heavy, key of victory. Distinct from any other audio in the machine.

### Feel / pacing
- **Slow base, explosive peaks.** Average spin should feel uneventful. The 1-in-30 spin that escalates should feel like a heart event.
- **Reach has phases.** Start (something changed — color shift, BGM swap), build (animation, voice line), climax (one of: bust, escalate to higher reach, hit). Each phase needs a clear visual/audio gate.
- **Failures are theatrical.** A bust isn't a quiet "no" — it's a SFX, a slumped character, a screen wipe. The pain is the point.
- **Continuation feels different from initial hit.** Kakuhen mode has its own BGM, its own visual overlay (background tint, particle effects), its own pacing. Players should feel they're in a different game state, not "more of the same."

## 6. For `pachinko-the-revenge` specifically

This is a fresh project — directionful questions to settle early:

1. **Authentic-Japanese or stylized-revenge-themed?** The title suggests a narrative twist. Lean into a *theme machine* (like Hokuto/Eva did) with a revenge plot, original IP. Don't generically "do pachinko" — pick a story.
2. **CR-style (low base, kakuhen loops) or modern ST-style (fixed-window battles)?** Affects core math.
3. **Is the meta-game cash-out, prize-out, or just for fun?** Determines whether to model 換金率 (cash-back rate) and三店方式 fiction.
4. **Single machine or parlor sim?** A parlor sim (multiple machines, data lamp hunting, seat-claiming) is a fundamentally different game than one beautifully simulated machine.
5. **Era target.** 1980s Fever-style, 2000s peak-CR, 2010s premium-IP, or modern smart-pachinko (スマパチ, RFID-balls, 2023+)? Each has a distinct aesthetic.

When asked to design or evaluate any element, reach for: *which canon machine does this echo, what player archetype does it serve, what spec-sheet number does it imply, and what's the reach hierarchy it sits inside?* If you can't answer those four, the design isn't load-bearing yet.

---

## 7. Cabinet visual layering — the depth grammar

Added 2026-05-25 after iteration 2 user feedback: "the cabinet looks one-dimensional." This section codifies what gives a real pachinko cabinet its perceived depth so the next iteration can build that.

A pachinko cabinet has **six conceptual visual planes**, stacked from back to front:

1. **Back-panel art (一番奥)** — the deepest layer, behind the playfield glass. Static or slowly-animated background art that establishes the machine's theme. In Eva: a starfield with EVA Unit silhouettes; in 海物語: a coral reef with fish drifting; in Hokuto: a wasteland with cracked sky. Almost never plain black.
2. **Mid-layer pin theatre** — the pins and any non-LCD playfield features (tulip catches, side pockets, character cut-outs that are *part of the playfield*, not animations). Pins are physically in front of back-panel art.
3. **LCD layer** — the reel/animation screen, embedded *within* the playfield. The reel window has its own bezel (a chrome or gold frame) that distinguishes it from the surrounding playfield. The LCD is a "screen within a screen".
4. **Ball plane** — steel balls in front of pins and LCD. Catch the room light → highlight; cast soft shadows on the pin field.
5. **Foreground UI** — cabinet bezel art, LED strips, the gold corner accents, the title strip at the top, the data lamp dome above the cabinet. These are PHYSICALLY in front of the playfield, not in the playfield.
6. **Overlay animations** — character cut-ins, full-screen wipes, fanfare banners. These break the 4th wall (drawn over everything including the bezel).

A flat 2D rendering reads as "wireframe" or "slot machine" because it collapses planes 1, 2, 3, 4, 5 into one. Even WITHOUT 3D rendering, you can imply depth via:

- **Drop shadows** between planes (the LCD has a shadow ON the back-panel; the chucker has a shadow on the playfield)
- **Parallax** (back-panel art shifts slightly on cabinet shake)
- **Soft gradients** on the bezel (chrome lighting), playfield (vignetting at edges)
- **Different palette per plane** (back-panel desaturated, mid-layer steel/dark, LCD vibrant, bezel gold)
- **Layered glow** (the LCD bezel rim glows; pins have tiny highlights; balls have rim lighting)

**Bezel theatre.** The cabinet bezel is not static. Real machines have LED strips on every edge that:
- Pulse slowly during base play (slow gold breathing)
- Flow / chase during reach (orange wave from left to right)
- Strobe during jackpot (gold/red alternating at 4Hz)
- Color-flip during kakuhen (deep red wash, with cooler accents)
- Cascade during kakuhen+reach (red strobe sweeping toward the chucker)

If the bezel does nothing, the cabinet looks dead even when the LCD is alive.

## 8. Event animation grammar

Added 2026-05-25. Every state transition should produce a distinct visual+audio event. Patterns drawn from the canon:

| Event | Visual grammar | Audio grammar | Lessons from |
|---|---|---|---|
| Ball enters chucker | Small flash at chucker; ball "absorbs" into the cup (not a hard pop-out-of-existence); rim of the cup briefly glows | Single bright chime, treble-rich, 200ms | All canonical machines |
| Reel spin start | Reels accelerate with motion-blur streaks; LCD bg desaturates slightly to focus attention | Whir tone fading in | Universal |
| Reel lands (no reach) | Brief shake on the reel that just stopped, micro-zoom; if it's a 7, half-second hold with chime | Click (low-tier), or chime (7) | 海物語 (gentle), Eva (severe) |
| Reach start — calm | Reel 3 keeps spinning with overlay banner "CALM REACH"; background dims to ~70% | BGM crossfades to tension layer; muted percussion | 海物語 (low-key) |
| Reach start — mid | Character SILHOUETTE cut-in slides from corner; freeze-frame zoom on reel 3 | Brass swell + key modulation up 1/2 step | Eva (cut-ins) |
| Reach start — premium | Full character ART cut-in with name banner; background freezes to anime stencil; reel 3 visibly slow-motion | Stinger chord (major or augmented) + signature voice line | Hokuto (voice + visual) |
| Reach start — confirmed | Full-screen title card wipe ("IT ENDS TONIGHT"); cabinet color flips to deep crimson; opening-theme cue plays | Opening theme stinger (recognized by regulars immediately) | Eva (premium reach grammar applied to confirmed) |
| Reach bust | Reel 3 SNAPS to a near-miss digit; reel field shake; sad piano sting; background washes back to base color | Bust SFX (deflating bass, descending pitch) | Universal |
| Reach hit (any tier) | Reel 3 SLAMS to matching digit; particles burst from the reel; whole-cabinet flash | Hit cymbal + brass major chord | Universal |
| Jackpot fanfare | Full-screen radial particle burst from cabinet center; directional gold rays sweep outward; bezel strobes; LCD shows "F E V E R !!" with letter-by-letter reveal | 6-second brass fanfare in major key, uninterruptible | Fever-series (where the name comes from) |
| Round complete | Attacker doors visibly open further; ball cascade pours from the top into the payout pocket (animated); ball count ticks up in real-time | "Cha-CHING" round-clear chime; running total counter | Modern CR |
| Jackpot end → BASE | Attacker doors slam shut; BGM returns to base loop; small "thank you" stinger | Bell-tone "session continues" cue | Universal |
| Jackpot end → KAKUHEN | Full-screen color flip to crimson; "CHANCE TIME!!" banner SLAMS in from top; cabinet bezel strobes red; kakuhen BGM (major key, upbeat) crossfades in | Power-up stinger; kakuhen BGM at higher volume than base | Eva ST (the canonical reference) |
| Kakuhen exit (window exhausted) | Crimson fades to base palette; "ST END" banner; sad piano cue | Reverse stinger (key drop) | Modern ST |
| Chapter unlock | Full-screen wipe with chapter title card; brief 2–3 second animated sigil; background returns | Story-specific cue (could be a leitmotif for the protagonist) | Eva (where chapter ↔ episode beats are explicit) |

**Two design rules that ALL these share:**

- **Three-beat structure: setup → escalation → climax.** Every animation has a beginning that *signals* something is about to happen (the player should be able to recognize the tier within 250ms of start), a middle that builds (1–4 seconds of escalation appropriate to tier), and a climax (the moment of resolution). Skipping the setup beat is the most common cabinet-design mistake; players don't feel the escalation if they don't notice the start.
- **Audio leads visual by 1–2 frames.** The chime, stinger, or chord HITS slightly before the visual peak. This is how real machines feel "crisp" — your ear registers first, your eye confirms a moment later. In a 60 fps game, that means audio fires at frame N, visual peak at frame N+1 or N+2.

## 9. The ball economy

Added 2026-05-25. v0.2 added ball physics; v0.3 should add the economy that makes ball-counts mean something.

### Real-world economy

- **Ball cost.** Japanese regulation caps at ¥4/ball (1パチ = ¥1/ball is a lower-stakes variant; standard is ¥4). Players "rent" balls — they pay yen for a tray of balls, fire them, win more balls (or lose them), and at session end exchange returned balls for prizes (景品) which they trade for cash at a separately-located 三店方式 booth.
- **Jackpot value.** 16R × ~90 balls = ~1,440 balls = **¥5,760** per jackpot at ¥4/ball. A typical multi-jackpot session yields ¥20,000–¥50,000 in winnings, against ~¥10,000–¥30,000 spent.
- **Base game rate (ベース).** 25–40% of fired balls return via the chucker. This is the calibration the player feels: if you fire 100 balls, you get back 30. Net loss per spin is small but constant in base play.
- **Kakuhen profitability.** Inside the ST window, the effective return rate flips positive — return rate stays the same but jackpot probability is ~5.6x higher, so balls won >>> balls fired.

### What to simulate

Per intent C-7, we model **no real money**. But the *feel* of value is achievable:

- Display ball counts WITH a yen-equivalent (¥4/ball is the canonical rate; expose as configurable).
- **Profit/loss indicator.** A running counter of `balls_won × 4 − balls_fired × 4`, displayed with a tint that flips between red (loss) and green (profit). Updates in real-time as balls fire and chucker entries fire.
- **Jackpot reveal includes yen.** "+1440 BALLS / +¥5,760" — both, side by side. The yen number is what the player viscerally cares about.
- **Streak bonus visualization.** When chaining jackpots inside kakuhen, show a "STREAK ×2", "STREAK ×3" etc. with a multiplier glow. Mathematically not adding anything (the math is just sequential jackpots), but psychologically the multiplier ratchets the perceived value.
- **Idle treasure trickle.** A small visual ledger at the bottom of the cabinet shows a "+1" or "+90" ticker every time a ball is returned or a jackpot round completes. The ledger trickle is small but constant — confirms the game IS paying off in micro-doses.

### What NOT to simulate (per intent C-7)

- No actual money transactions
- No 換金率 (cash-back rate) tuning
- No三店方式 booth simulation
- No "buy in" or "cash out" buttons that imply real currency exchange

The economy is a *feel* layer, not a transaction layer.

---

## 10. Lessons captured (iteration log)

This section is appended to as iterations produce concrete, transferable lessons that should outlive the project. Each entry includes the iteration, what was learned, and where to apply it.

### From iteration 1 (MVP cabinet, math + reach)

- **The math layer is necessary but not sufficient.** A complete and tested probability/state-machine implementation is required for authenticity but produces a slot-machine-shaped artifact if shipped without the ball-and-pin layer. See `audits/audit-001-cabinet-slot-vs-pachinko-2026-05-25/`.
- **Tests-green is a lagging quality signal.** 21/21 green tests on the math layer did not predict the user's "this is a slot machine" feedback. Visual/auditory identity needs human-judged acceptance gates (PRD-001 §2 PXT-1..PXT-5), not just unit tests.
- **Default font CJK gap is real.** macroquad's default font has no CJK glyphs. Authentic JP text overlays (ヘソ, データランプ, 確変中) require either a subset font (~50–100 KB) or romanized substitutes (lossy).
- **WASM frame budget: a 2-frame "loading…" pre-render is mandatory before any audio await.** Macroquad's runtime needs a `next_frame().await` to start drawing; awaiting audio decode before that yields a black canvas.

### From iteration 2 (ball physics + pin field)

- **Empirical pin tuning is hard from cold start.** First pin layout produced a 1–2% chucker rate (target 25–40%). Multiple iterations of funnel-widening, guide-pin placement, chucker-r increase, and launcher-vx tuning got it to 9% in headless probes. Tuning to PRD-002 R-29's target band needs either: (a) more guide pins shaping a funnel, (b) variable launch power as an input, (c) explicit pin-layout iteration with measured rate per layout.
- **Launch trajectory should bypass the right-side chute simulation.** Spawning balls at the top of the playfield with deterministic horizontal jitter (instead of arcing up from a bottom-right launcher) gave much better field coverage with much less physics overhead. The chute can be cosmetic.
- **macroquad's font-atlas churn is real.** Using many distinct text sizes (14, 16, 18, 20, 22, 24, 28, 32, 36, 40, 44, 48, 52, 64, 96) triggers a stream of `glBindTexture called with an already deleted texture ID` warnings every frame. Cluster to ~6 sizes max (e.g. 14, 18, 22, 32, 48, 96) to keep the atlas stable.
- **The discipline ratchet works.** magnetfragnet's nudge-9 fired 6 iterations in a row before audit-001 reset it. Producing the audit forced the H2 (drift) and H5 (tension) hypotheses that drove iteration 2's scope. Without the nudge, the slot-machine drift could have persisted indefinitely.

### From iteration 3 (event celebrations, depth, economy)

Iteration 3 went hard at the iter-2 critique "weak graphics, no animations, one-
dimensional cabinet, no sense of money." Six lessons worth carrying forward:

- **The six-plane grammar (§7) translates directly to a draw-call order.** Once
  the cabinet was structured as `clear → state tint → outer bezel → back-panel
  → LCD shadow → LCD bezel+screen → pin field → balls → chucker → attacker (if
  open) → launcher → animated bezel → cut-ins → fever reveal → kakuhen slam →
  chapter card → particles → P/L → toggle button`, every depth question
  ("should this be in front of that?") had a non-arbitrary answer. Without that
  explicit ordering, layered rendering becomes a brittle puzzle.

- **Event animations should be data-driven from RenderState, not from frame
  timers in render.** Adding fields like `chucker_flashes: Vec<(x,y,life)>`,
  `cutin_active: bool`, `fever_reveal_t: f32`, `kakuhen_slam_t: f32`,
  `chapter_card_elapsed: f32` plus `tick(dt)` that decays them — then `render`
  reads them — makes the animations testable and trivially composable. Multiple
  chucker flashes can coexist (Vec); one cut-in at a time (single Option). The
  shape of the state determines the multiplicity story.

- **"Always-visible P/L" is the single highest-leverage HUD element for
  conveying value.** A small, color-tinted "+¥248" / "−¥648" indicator at the
  top of the screen carried more emotional weight per pixel than the entire
  data lamp panel. The data lamp is for the regular who wants to dig; the P/L
  is for everyone else. The default UI should be one P/L number + one toggle
  button + the cabinet itself — nothing else.

- **Hidden-by-default + glow-pulse on new info is the right pattern for HUDs.**
  The data lamp's `data_lamp_glow_t` field that decays after a jackpot or
  chucker entry makes the toggle button itself communicate "look at me." Real
  pachinko's data lamps are dome-shaped LEDs that pulse when state changes;
  the digital equivalent is the toggle-button rim glow.

- **Stylized "art" can come from rects + lines + gradients alone.** The back-
  panel rain-soaked neon cityscape uses zero image assets — just 9 rectangle
  silhouettes, per-building neon strips, window dots, diagonal rain lines, a
  moon disc, and scanlines. Total cost: ~120 lines of Rust, ~1ms per frame.
  This unblocks single-file-WASM distribution per intent C-13. Embed nothing;
  paint everything.

- **HTML/WASM key-binding conflicts will silently swallow inputs.** Iter 3
  first deployed with `H` bound to BOTH the HTML help overlay and the WASM
  data-lamp toggle — the HTML's `preventDefault()` won and the WASM never saw
  the key. Visible only when running headless probes asked "did the toggle
  work?" The fix is to partition the keyboard cleanly between layers; in this
  project, HTML now owns `?` only and `H` / `Tab` go to WASM.

Operational improvements: magnetfragnet 0.1.0 gained `check` (CLI path mirroring
the MCP `nudge_iteration` tool) and `hooks install` (auto-wires PostToolUse +
Stop hooks in `.claude/settings.json`). Iter-3 retired the `.tmp/mfn.mjs` stdio
driver — the CLI is the cleaner integration surface.

Recommendation for iteration 4: surface the still-flat chucker rate (~15%,
target 25–40% per PRD-002 R-29) as the *gameplay lever* (nail-adjustment 釘
調整) rather than a static config — let the player retune the funnel and watch
the rate respond. That makes the §9 economy meaningful: better nail layouts
yield better ベース which yields more chucker hits per yen spent.

### From iteration 4 (釘調整 + contextual minimalism + session continuity)

Iteration 4 implemented the audit-002 synthesis. Five lessons worth carrying forward:

- **The "hidden by default + chapter-gated unlock" pattern composes.** iter 3 hid the data lamp behind a toggle; iter 4 hides the tuning workshop behind a chapter-2 unlock that THEN appears as a tab inside the data lamp. The compounding "earn access by playing" pattern means the cabinet doesn't reveal complexity until the player has demonstrated readiness to see it. The same pattern is available for any future depth: voice-acted reach cinematics could unlock at chapter 3, advanced tuning at chapter 4, a new BGM at chapter 5.

- **Honest CI display is structurally cheaper than a flattering point estimate.** R-48 mandates "17.4% ± 3.2%" not "17.4%". Implementing it required ONE additional MC probe and ONE more text format-string. The cost of *not* doing this would have been the §12.1 EXPLOIT trap — a flattering number on the workshop screen would have undermined every other honest claim the cabinet makes. The cheapest part of an ethical design is often the literal change.

- **`quad-storage` resolves the WASM-persistence question that's been open since iter 1.** Net cost: +25 KB WASM, one new dependency, three test cases. The shim wraps `localStorage` on web and falls back to in-memory on native. Story 005's roundtrip test passes; the welcome-back card works. Lesson: deferred infrastructure decisions get cheaper to make, not more expensive — five iterations of deferral didn't make the choice harder, it made the alternatives more obviously identical.

- **Modal-aware input gating is the right pattern for game-state-driven controls.** iter 4 has three modals (workshop, session summary, welcome-back) that suppress launch input when active. Implementing this as a single `let modals_active = ...; let space_held = ... && !modals_active;` line at the input layer is cleaner than per-modal early-returns. Lesson: keep the input layer thin and gate-aware; let the modals own their dismiss logic.

- **Removing a feature is sometimes the deliverable.** R-56 removed the cosmetic streak multiplier badge. Net delta: −1 render call, −1 visual element. The session summary still tracks the chain count (data preserved). This satisfies skill §11's "no teeth → drop" binary without sacrificing future flexibility. Lesson: a release that REMOVES weak UI is as valuable as one that adds strong UI.

Operational note: magnetfragnet's `check --if-changed` auto-hook fires on every Edit/Write/MultiEdit and on Stop. iter 4's iteration counter accumulation has been entirely automatic since iter 3 close; the `.tmp/mfn.mjs` driver is now retired.

### From iteration 3.1 (skill expansion — game theory, psychology, canon, success rubric)

User-directed skill update outside the audit cycle. Added §11–§14 to push the
skill from "encyclopedic reference" toward "decision instrument." The bet: a
designer (or LLM) presented with a question — "should this reach tier exist?",
"should this jackpot be 8R or 16R?", "is this animation worth the budget?" —
should be able to find the framework in the skill, not have to derive it.

Specific additions:
- §11 (game theory) now contains EV/variance formulas, the gambler's-ruin
  walk, why 1/199.8 is canonical, kakuhen as a geometric distribution,
  the runway calculation that determines whether a session can survive
  the next dry spell.
- §12 (psychology) names six cognitive biases with their roles in the
  pachinko experience and the design counter-moves that EXPLOIT vs RESPECT
  them. The skill takes a position on which is ethical and which isn't.
- §13 (canon decoded) gives a per-machine "steal this / avoid that" pair
  for the five canonical lines (Fever, 海物語, 北斗, エヴァ, AKB48) plus a
  named anti-pattern list distilled from machines that failed.
- §14 (success rubric) is a 5-test acceptance gate (30sec / 5min / 30min /
  2hr / "leave the machine") and a production budget allocation that names
  which moments deserve the largest spend.

---

## 11. Game theory — the proper math

Section added 2026-05-25 (iter 3.1 skill expansion).

§2 covered the spec sheet at the level of "what numbers to type into the
canonical config." This section is the math BEHIND those numbers — the levers
a designer adjusts to bias variance, runway, and perceived fairness.

### 11.1 Expected value and the house edge

Per Japanese regulation, the long-run payout rate (`払戻率`) is capped at
~80% over a session-length window. Concretely: for every ¥100 inserted,
~¥80 returns in ball form long-run. The 20% gap is split between:

- The operator's margin (parlor takes ~10–15%)
- Variance reserve (the parlor needs to absorb hot-machine periods)
- Regulatory cushion

The PLAYER'S effective EV per spin is calculated thus:

```
EV_per_spin = (P_jackpot × jackpot_payout)  +  (P_chucker_return × chucker_return_value)
              − cost_per_spin

# For our spec:
P_jackpot          = 1/199.8 ≈ 0.005005
jackpot_payout     = 16 rounds × 90 balls × ¥4 = ¥5,760
P_chucker_return   = ~0.35 (the ベース)
chucker_return_val = 1 ball × ¥4 = ¥4
cost_per_spin      = balls_consumed_per_spin × ¥4 ≈ 3 × ¥4 = ¥12

EV_per_spin ≈ 0.005005 × 5760 + 0.35 × 4 − 12
            = 28.83 + 1.4 − 12
            = +¥18.23
```

That POSITIVE EV is misleading — it assumes every spin is at base rate AND
no balls are lost waiting for the chucker. In practice ベース is observed
including all "spin events" where the chucker pays out, but the cost side
includes balls fired BETWEEN chucker hits. The actual effective EV after
correcting for the ball funnel is approximately **−¥2 to −¥5 per spin** in
base, and **+¥40 to +¥80 per spin in kakuhen**. Sessions are designed to
oscillate between these regimes.

### 11.2 Variance and the gambler's ruin

The variance of a single spin is dominated by the rare-jackpot term:

```
Var_per_spin ≈ P_jackpot × (payout − EV)²
             ≈ 0.005005 × (5760 − 18)²
             ≈ 165,000

σ_per_spin   ≈ √165,000 ≈ ¥406
```

A player burning ¥30,000 of balls (~750 spins) sees a Gaussian-ish session
P/L with σ ≈ 406√750 ≈ ¥11,100. So the typical session ranges roughly
±¥11k around the EV midpoint. Half of all sessions end NET POSITIVE despite
the −EV — that's what keeps players coming back. The other half include the
brutal multi-thousand-yen losses, but those are LESS COMMON than the
"reasonable loss / occasional win" middle.

**Gambler's-ruin runway**: how many spins can a player afford before a
budget B is exhausted, given a unit-loss-per-spin `−u` and σ?

```
Expected runway = B / u   (mean ruin)
P(ruin before N) ≈ Φ(−B / (σ√N))   (drift-adjusted)
```

For B=¥10,000, u=¥3/spin, σ=¥406, the player can typically last ~3,300
spins, but variance can shorten this to ~1,500 OR extend to "the jackpot
saves the runway." This shape of "can be saved by one jackpot" is the
EMOTIONAL HOOK pachinko exploits — it makes "one more spin" feel
mathematically reasonable even when it isn't.

### 11.3 Kakuhen as a geometric distribution

Inside an ST kakuhen window of length `W` (165 for our spec), the chance
of NO jackpot is `(1 − p_kakuhen)^W = (1 − 1/35.9)^165 ≈ 0.0096`. That's
just under 1%. So **99% of kakuhen windows produce at least one jackpot**.

The number of jackpots in a kakuhen window is approximately geometric with
mean `W × p_kakuhen / (1 − p_W^kakuhen_chain)` once you fold in re-entry.
For our spec (70% kakuhen entry, ~99% in-window hit chance), chain lengths
follow:

| chain length | probability |
|---|---|
| 0 jackpots | ~30% (no kakuhen entry at all from the triggering JP) |
| 1 jackpot  | ~21% (entered, hit once, no re-entry) |
| 2 jackpots | ~15% |
| 3 jackpots | ~10% |
| 4 jackpots | ~7%  |
| 5+ jackpots| ~17% (long tail — the "RUSH" sessions regulars chase) |

The long tail is where pachinko makes its mark on the player's memory.
Every regular has a "best session" story with 7-8 chained jackpots.
Statistically those are once-in-50-sessions events.

### 11.4 Why 1/199.8 specifically

The base jackpot probability is regulated within a narrow band. 1/199.8
is on the LOW-volatility side of legal — high enough that jackpots feel
attainable in a session (~3-5 per hour at modern spin rates), low enough
that the variance hasn't blown past what casual players tolerate. Machines
have shipped at 1/79.8 (very casual, low payout), 1/319.6 (intense,
high-stakes), 1/399.6 (legacy high-volatility). The 1/199.8 mid-band is
where 海物語 lives and where most modern CR-style ships.

**Lever for the designer**: lower jackpot rate → bigger session swings
(both directions). Higher rate → flatter sessions, less catharsis per
jackpot. The sweet spot for "story-driven" pachinko (this project) is
1/150 to 1/250 — frequent enough to advance the plot, rare enough that
each jackpot is a story beat.

### 11.5 Payout curve shapes

Beyond the spec sheet, the SHAPE of the payout matters. Three common
shapes, each with a different EV-equivalent feel:

- **Flat (e.g., 8R × ~90 balls = ~720 balls / jackpot)**: feels casual.
  海物語 territory. Lower per-jackpot peak excitement, more frequent
  jackpots.
- **Steep (16R × ~90 = ~1,440 / jackpot, OUR SPEC)**: classic CR feel.
  Each jackpot is a "session-saver." The catharsis budget per JP is high.
- **Lottery (variable — sometimes 1R, sometimes 16R)**: the "ratch"
  pattern. Each JP has a sub-probability of being a "big" jackpot. Adds a
  layer of "did I get the BIG one?" anxiety to each JP. Modern ST machines
  often layer this on.

For OUR project's story-as-mechanic intent, the steep shape is correct —
each jackpot is a chapter beat, and chapter beats should be rare and large.

### 11.6 Decision framework

When asked "what's the right value for X probability or Y payout?", reach
for:

1. What's the target spin rate and session length? → sets total spins
2. How many jackpots PER SESSION do we want the median player to see?
   (For story-pachinko: 2–4. For casual: 5–10. For high-stakes: 1–2.)
3. Solve for p_jackpot ≈ target_JPs / total_spins
4. Choose payout shape (flat / steep / lottery) → determines per-JP
   catharsis
5. Pick variance target (σ_per_session) → adjust payout magnitude
6. Verify the regulation constraint (≤80% return) is satisfied long-run

If any constraint conflicts (e.g., target JPs implies a probability that
makes σ too low for emotional impact), the design is fighting itself.
Surface the tension explicitly — don't paper over with cosmetics.

---

## 12. Psychology of fun — what makes pachinko compulsive

Section added 2026-05-25 (iter 3.1). The math (§11) explains how the game
*works*; this section explains how it *feels* — and why those feelings
persist past where rational EV calculation would stop a player.

The skill takes a position: some of these mechanisms are ETHICAL to deploy
in a game-as-entertainment context; others cross into exploitation. The
distinction is drawn at the end of each subsection.

### 12.1 Variable ratio reinforcement (Skinner)

Pachinko is the most pure variable-ratio (VR) schedule in mainstream
entertainment. Reinforcement (jackpot) is delivered at unpredictable
intervals, averaging ~1/200 spins, with no temporal pattern. VR schedules
are the most resistant to extinction — a player who has just gone 800
spins without a jackpot is MORE motivated to keep playing, not less,
because "the next spin could be the one."

**Design counter-move that EXPLOITS**: hiding the actual probability
from the player. Real pachinko works around regulatory transparency by
flooding the player with NEAR-misses and reach animations that imply the
jackpot is "close" when statistically it isn't.

**Design counter-move that RESPECTS**: show the canonical 1/199.8 prominently
(on the cabinet, in a help overlay) AND show running spins-since-JP via the
data lamp. The player can read both numbers and decide rationally. Pachinko-
the-revenge does this — the math IS the README. The compulsive force comes
from the *grammar*, not from withheld information.

### 12.2 The near-miss effect (Reid 1986, Habib & Dixon 2010)

When two of three reels match and the third is *one position off*, the
brain's reward system activates ALMOST as much as for an actual win.
Neuroimaging shows ventral striatum activation in near-miss conditions at
60–80% of jackpot intensity. The brain pattern-matches "I almost won" to
"I won," even though the rational interpretation is "I lost, full stop."

Pachinko machines are designed around this. The reach hierarchy IS the
near-miss generator. Every premium reach that busts is a controlled
near-miss event. Players FEEL like they got close 100 times per session.
Statistically they got close to nothing — the next spin's outcome is
independent.

**EXPLOITS**: tuning reach bust rates to maximize near-miss density without
delivering wins. Some machines have 99%-bust calm reaches every 20 spins —
that's near-miss carpet-bombing.

**RESPECTS**: tier the near-misses so they ARE diagnostic. Mid reach
busts 75% — that's a real 25% chance the player got. Confirmed reach busts
5% — that IS an almost-certain win. Honor the signal value of the
animation. Don't pollute the calm tier with so much theatre that players
stop trusting the hierarchy. The skill's §8 grammar (each tier has a
distinct chord modulation, animation tier, character cut-in) is
information-revealing not information-hiding.

### 12.3 Sunk cost and loss-chasing

After ¥10,000 burned without a jackpot, the player feels they've INVESTED
in the machine — they can't quit because that investment becomes a
"realized loss." Walking away while at +¥0 is psychologically harder than
walking away while at +¥3,000.

Pachinko machines AMPLIFY this via the data lamp (which shows hama-dai
counts that "investments" in time). Players hunt high-hama-dai machines
because they believe a jackpot is "due" — a clear gambler's fallacy.

**EXPLOITS**: design the data lamp to emphasize hama-dai but suppress
session-level P/L. The player sees "850 spins since last JP — this machine
is hot!" but doesn't see "you've spent ¥17,000."

**RESPECTS**: surface the session P/L prominently (as iter 3 did) and
make it the always-visible HUD element. If the player can see they're
−¥6,000 deep, the sunk-cost trap is at least transparent. Pachinko-
the-revenge's always-visible P/L indicator with the red/green tint
satisfies this.

### 12.4 The illusion of agency

Players believe they can influence outcomes via launch power, machine
choice, timing, etc. Statistically: launch power affects ベース (chucker
rate), nothing else. Machine choice affects nothing for a given spec.
Timing affects nothing. But these illusions are CRITICAL to the
experience — pure-RNG slot machines feel emptier than pachinko precisely
because the player can't kid themselves about agency.

**EXPLOITS**: claim there's "skill" when there isn't. Some machines have
"timing-based bonus chances" that are pure RNG dressed up as skill.

**RESPECTS**: make the agency REAL where possible. ベース is genuinely
affected by launch power and nail layout in real pachinko. Our project
makes ベース EMERGENT from the pin layout via physics — that's real
agency. iter 4's nail-adjustment 釘調整 makes it explicit.

### 12.5 Anchoring

The first spin's outcome anchors the player's expectations. If the first
30 spins produce 2 reaches, the player calibrates to "this machine gives
2 reaches per 30." Subsequent dry periods feel WORSE than they would
without the anchor.

Pachinko machines are designed with a "warm-up" calibration — the first
~100 spins tend to feel more eventful (more reaches, more chucker hits)
than statistical fair share. This is partly RNG variance and partly
selection bias (players abandon machines whose first 100 spins felt
dead). The result is that the first 10 minutes feel disproportionately
exciting.

**EXPLOITS**: literally rigging the early-session RNG to be more
favorable (this is illegal but allegedly happens in unregulated markets).

**RESPECTS**: provide the warm-up via *structure* not *math*. The
project's "no premium reach in first 5 minutes" rule (PRD R-12) is the
inverse — it BUILDS UP rather than artificially front-loading. Combined
with the chapter-unlock progression (chapter 1 only at start), the
warm-up is a story beat not an RNG manipulation.

### 12.6 Flow and session arc

Pachinko sessions follow a Csikszentmihalyi flow arc:

- **Onboarding (0–10 min)**: high challenge / low skill. Player learns the
  cabinet, makes mistakes (firing too softly, missing the data lamp).
- **Calibration (10–30 min)**: matches challenge to skill. Reaches start
  to register; ベース is understood.
- **Flow zone (30 min–2 hours)**: optimal challenge. Variance produces
  ups and downs that engage but don't overwhelm.
- **Late session (2+ hours)**: fatigue. Skill is high, challenge is fixed,
  flow degrades into either grind (boredom) or tilt (anxiety).

A well-designed machine extends the flow zone. Mechanisms that help:
- Chapter progression (new content prevents staleness)
- Streak / kakuhen mechanics (rare, high-intensity bursts)
- Subtle BGM rotation (combats audio fatigue)
- Variable reach tier density over time (more premium reaches available
  late, gating ramps up)

**EXPLOITS**: pacing late-session toward more rapid losses (the brain is
tired; loss-chase becomes easier). Some machines deliberately speed up
in late session.

**RESPECTS**: pacing late-session toward more *meaningful* moments. Higher
chapter reaches require longer to set up but have larger payoffs. The
project's confirmed-reach tier (5% bust, gates to chapter 4) is this — it
only becomes available after many jackpots, but each one feels seismic.

### 12.7 The "winner's high" duration

A real jackpot produces a 10–90 second dopamine spike. This is the
ENTIRE EMOTIONAL JUSTIFICATION for the prior hour of base play. Design
must protect that high:

- The fanfare must be ≥6 seconds (anything shorter feels rushed)
- The attacker open + ball cascade must be visible (the player needs to
  SEE the payout, not just be told)
- The data lamp tick-up of "balls won" must be observable in real-time
- A post-jackpot 5-10 second "afterglow" with reduced game pace allows
  the high to peak before the next spin starts

The skill's §8 grammar honors all of these. PRD R-16 (fanfare
uninterruptible) protects the 6+ second window.

---

## 13. The canon decoded — patterns and anti-patterns

Section added 2026-05-25. §4 introduced the canonical machines. This
section extracts the DESIGN LESSON from each one — concrete patterns to
copy and traps to avoid.

### 13.1 What to steal, what to skip

| Machine | Steal this | Skip this |
|---|---|---|
| **CR Fever (1980)** | The catharsis ritual — the WORD "FEVER" as a sacred announcement; the chrome+neon visual signature; the 6+ second fanfare as a non-negotiable. | The lack of reach variety — Fever has essentially one tier of escalation, which feels thin by modern standards. |
| **海物語 (Sea Story, 1999+)** | Low-variance casual feel for onboarding; warm-color palette; gentle marimba/ukulele BGM as a counter-example to "pachinko must be aggressive"; the persistence of character familiarity (the same fish, year after year — players bond). | Premium reaches are too rare and too similar — the tier hierarchy is mostly flat. Don't copy that. |
| **CR 北斗の拳 (Hokuto, 2003+)** | Voice acting INTENSITY — voice lines hit at the same volume as music, not below. Screen-shake budget — confirmed reaches shake the entire cabinet visualization. Signature line treatment — "おまえはもう死んでいる" is the catharsis trigger; players ARE waiting to hear it. | The grindy base game — Hokuto often punishes for the sake of catharsis ratio. Modern players don't have 8-hour sessions to spare. Tighten the base-to-reach ratio. |
| **CR エヴァンゲリオン (Eva, 2005+)** | Reach hierarchy LEGIBILITY — every Eva regular can rank ~15 reaches by bust rate from memory. The cut-in grammar (character art slides in from corner, slows the moment). Opening-theme weaponization — the OP plays only on confirmed-tier reaches; physiological response is conditioned. | 15+ reach variants is too many for a one-off project — players can't form a hierarchy if every reach is novel. Limit to ~8 named reaches across 4 tiers (the iter 1 spec). |
| **AKB48** | Parasocial mechanic — characters address the player by inferred relationship; encore mechanic where the same character returns across sessions. | Idol-specific branding — translates poorly to non-Japanese audiences. The parasocial PATTERN translates; the specific idols do not. |

### 13.2 Anti-patterns distilled from failed machines

Machines that failed empirically (low parlor placement, short production run,
or unfavorable regular review):

- **"Slot in a pachinko shell"**: the cabinet has reels but no visible ball
  physics. iter 1 of this project was this. Players feel the cabinet is
  hollow. *Counter*: ensure the chucker is the visible/audible trigger for
  EVERY spin.

- **IP licensed but not integrated**: the cabinet has anime branding but the
  reach animations are stock — the character art is wallpaper. Players read
  this as a cynical cash-in within 10 minutes. *Counter*: every premium-tier
  reach should be a SCENE FROM THE IP, not a generic "anime girl waves."

- **Soundalike voice acting**: cheaper than original cast but recognizable
  to fans within 30 seconds. *Counter*: budget for original cast on the 3-5
  most-frequent voice triggers (chucker chime is fine generic; signature
  reach lines must be cast).

- **Too many reach tiers**: 6+ tiers means players can't form a
  hierarchy. The brain caps at ~7±2 distinct categories for fast
  recognition. *Counter*: 4 tiers maximum, distinct visual + audio
  grammar per tier (per §8 table).

- **Premium reach used too often**: if premium fires every 200 spins
  with 50% bust, players numb to it. *Counter*: premium should be every
  ~500 spins with ~30% bust (the iter 1 spec is good here).

- **Confirmed reach with high bust rate**: if "confirmed" busts >15%, the
  player trust contract is broken. *Counter*: confirmed must be ≥90%
  hit. The iter 1 spec of 95% hit is correct; lower would betray.

- **Kakuhen window too short** (<100 spins): chains don't have time to
  develop, no "rush" feeling. *Counter*: 150-200 spins is the sweet spot;
  iter 1's 165 is on the conservative side.

- **Kakuhen window too long** (>250 spins): chains become inevitable,
  catharsis dilutes. *Counter*: same band as above.

- **Base game too punishing** (ハマり>800 with no events): players abandon
  the machine. *Counter*: ensure SOME mid-tier reach within every 300
  spins on average. The iter 1 spec gives mid every ~140 spins.

- **Jackpot reveal that doesn't pause the game**: if the cabinet
  immediately accepts the next pull during the fanfare, the high is
  truncated. *Counter*: PRD R-16 (fanfare uninterruptible).

- **Data lamp without a sense of accumulated progress**: hama-dai counts
  feel meaningless. *Counter*: pair hama-dai with last-10-jackpot gap
  visualization so the player sees their session's variance shape.

### 13.3 The single pattern that separates success from failure

**Information-rich, surprise-honest.** A machine that floods the player
with reaches and animations but where 95% don't matter (calm tier
carpet-bombing) trains the player to ignore them. A machine where every
reach is rare but truthful trains the player to LEAN IN when one fires.

The information content of a reach signal is proportional to its rarity.
Calm reaches at 2.5% frequency carrying ~2% hit rate are nearly information-
free — they say "something might happen, but probably not." Mid reaches
at 0.7% frequency carrying 25% hit rate are HIGHLY informative — "this
is a real chance." Premium at 0.25% / 70% is "you should be paying
attention." Confirmed at 0.05% / 95% is "stop everything."

Successful machines preserve this gradient. Failed machines collapse it
(e.g., by making premium reaches too common, which converts them to a
modest variant of mid).

---

## 14. The success rubric — how to know if a design is working

Section added 2026-05-25. A checklist for evaluating a pachinko design at
each stage of development. Pulled from playtest patterns across the canon
and the project's own iteration audits.

### 14.1 The five acceptance gates

1. **The 30-second test.** A non-pachinko player who sees the cabinet
   running (without input) should identify it as pachinko (not a slot
   machine, not a video game) within 30 seconds, from audio + visuals
   alone. Iter 1 failed this. Iter 2 passed it. Iter 3 strengthened
   the failure mode by adding visible event animations and depth.

2. **The 5-minute test.** A new player firing balls for 5 minutes should
   experience their first reach AND understand from the animation that
   it's NOT a jackpot but IS "close." If they can't articulate "I almost
   won" after 5 minutes, the reach hierarchy is broken.

3. **The 30-minute test.** Within 30 minutes of play, the player should
   experience their first jackpot AND understand from the audio+visual
   reveal that they have won AND see the payout reflected in their ball
   count / yen indicator within 5 seconds. If any of these breaks, the
   catharsis ritual is broken.

4. **The 2-hour test.** A 2-hour session should NOT feel like a 2-hour
   session — it should feel like 30 minutes elapsed (time dilation under
   variable ratio is healthy) or like a *story* arc with distinct phases
   (warm-up, hot streak, drought, catharsis). If it feels grindy and
   linear, the pacing is broken.

5. **The "leave the machine" test.** The player should decide to stop
   playing ON THEIR OWN, not because they ran out of balls/money. They
   should feel "I had a good session" or "I want to come back," NOT "I
   couldn't escape." This is the ethical ceiling — a game that traps
   players is not a successful game, it's an exploitive product.

### 14.2 Production budget allocation

If forced to rank where to spend art / animation / voice / music budget:

| Tier | What | Why | Budget share |
|---|---|---|---|
| 1 | Jackpot fanfare (the catharsis moment) | The entire session leads here. | 25% |
| 2 | Confirmed reach cinematic | The plot's "it ends tonight" moment. Per Eva canon, this is the most-anticipated single animation. | 20% |
| 3 | Kakuhen entry slam | The state-change that doubles the player's session. Must feel earned. | 15% |
| 4 | Premium reach character cut-ins | Story moments. The hierarchy hinges on these reading as "different from mid." | 12% |
| 5 | Mid reach grammar (BGM modulation, partial cut-ins) | Frequency-weighted: mid reaches are seen 10x more than premium. Per-second-on-screen, they may be #2 in importance. | 10% |
| 6 | Attacker-open ball cascade | The visual proof of payout. Brief but essential. | 5% |
| 7 | Chapter unlock title cards | Per-session, 1-3 occurrences. Big visual payoff per occurrence. | 4% |
| 8 | Base BGM + ambient SFX | Plays for 80%+ of session time. Per-second cost is low; total cost via duration is significant. | 5% |
| 9 | Idle animations (bezel breathing, marquee, back-panel parallax) | Per-second cost minimal but cumulative effect on "alive" feel is substantial. | 4% |

Total: 100%. If a category exceeds its share, demand a justification — why
is this particular thing more important than the catharsis?

### 14.3 The "come back tomorrow" test

The final, hardest test: does the player WANT to play again the next day?
The answer depends on:

- Did they end on a high note? (Last-session-event recency dominates memory.)
- Did they unlock something new? (Chapter progression, new reach
  available next time.)
- Did they feel respected? (No dark patterns, no dishonest reaches.)
- Did they have a story to tell? (One specific moment they want to
  recount.)

The project's chapter progression + story-gated reaches is designed for
this. A player who saw chapter 2 unlock but not chapter 3 has a
specific reason to come back — they want to see chapter 3. A player
who fired 1,000 balls with no narrative progress just spent ¥4,000 on
nothing memorable.

### 14.4 What this means for pachinko-the-revenge

The project currently passes tests 1–3 (iter 3 evidence in the visual
probes). Test 4 (2-hour pacing) is untested — the session length we've
probed is 30 seconds, not 2 hours; the long-horizon dynamics are
speculative. Test 5 (leave-on-your-own) requires a real human playtest;
headless probes can't validate it.

Iteration 4 should target test 4 and test 5 explicitly. Concrete
proposals: instrument session-length telemetry (without phoning home
per C-7 — store locally and surface to the player), add a "session
summary" screen on R / reset that names the high notes, and make
kakuhen entry the moment that "extends" a session that was about to
end.

