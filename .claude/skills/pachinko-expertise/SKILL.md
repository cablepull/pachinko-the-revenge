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

