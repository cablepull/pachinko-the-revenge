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
