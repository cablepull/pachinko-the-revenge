# PRD-001: F-1 MVP Pachinko Cabinet

**Status:** Draft
**Created:** 2026-05-24
**Intent linkage:** C-1, C-2, C-3, C-4, C-5, C-6, C-7, C-8, C-9, C-10, C-11, C-12
**Stories:** _(populated as stories are cut)_
**ADR Required:** yes (engine + math-layer language; deferred from intent open question #3)

## Feature F-1: Single-cabinet pachinko simulation with story-gated reach hierarchy

The MVP cabinet implementing the math, state machine, reach engine, audio/visual coordination, persistence, and HUD necessary to deliver one complete narrative arc through play.

---

### Rule R-1: Spin outcome is a pure function of game state and seeded RNG

Per [C-1, C-6]. The Probability Engine module exposes a single `spin(state, rng) → outcome` function with no side effects.

**Example: Determinism across replays**
```
Given a Probability Engine seeded with rng_seed = 0xDEADBEEF
  And game state = BASE, spin_count = 0, kakuhen_active = false
When spin() is called 10,000 times in sequence
Then the resulting outcome sequence is byte-identical
  On every fresh process invocation with the same seed
```

**Example: No leak from outcome to state**
```
Given a Probability Engine
When spin() is called
Then the function returns SpinOutcome without mutating any module-scoped state
  And calling spin() again with the same input produces the same output
```

---

### Rule R-2: Base-state jackpot probability is 1/199.8 within 3σ over 1M spins

Per [C-2]. Verified via Monte Carlo against the canonical spec sheet.

**Example: Base-rate Monte Carlo**
```
Given a Probability Engine seeded with any rng_seed
  And game state = BASE (no kakuhen)
When spin() is called 1,000,000 times
Then the count of outcomes with isJackpot = true is in [4,905, 5,105]
  (Expected 5,005 ± 3σ where σ ≈ 70.7 for a 1/199.8 Bernoulli over 1M trials)
```

---

### Rule R-3: Kakuhen-state jackpot probability is 1/35.9 within 3σ over 100k kakuhen spins

Per [C-2].

**Example: Kakuhen-rate Monte Carlo**
```
Given a Probability Engine in KAKUHEN state
When spin() is called 100,000 times
Then the count of outcomes with isJackpot = true is in [2,736, 2,836]
  (Expected 2,786 ± 3σ where σ ≈ 16.5)
```

---

### Rule R-4: ST kakuhen window is exactly 165 spins, deterministically terminating

Per [C-2, C-6]. The ST window is fixed-length, not probabilistic continuation.

**Example: ST window auto-exit**
```
Given a Cabinet Coordinator that has just entered KAKUHEN_BASE
  And the RNG is rigged to produce zero jackpots
When 165 spins are processed
Then the state on spin 165 is still KAKUHEN_BASE
  And the state on spin 166 is BASE
  And an EXIT_KAKUHEN event was emitted between spin 165 and spin 166
```

**Example: Jackpot inside the window re-rolls the window**
```
Given a Cabinet Coordinator in KAKUHEN_BASE at spin 100 of the window
When a jackpot occurs on spin 101 with enteredKakuhen = true
Then the new KAKUHEN_BASE window starts fresh at spin 0 / 165
```

---

### Rule R-5: Kakuhen entry rate is 70% over 10,000 jackpots (within ±100)

Per [C-2].

**Example: Kakuhen entry distribution**
```
Given a Probability Engine that has produced 10,000 jackpots
When the enteredKakuhen field of each is tallied
Then the count of enteredKakuhen = true is in [6,900, 7,100]
```

---

### Rule R-6: Reach tier distribution matches the canonical table within 3σ over 1M spins

Per [C-2, C-3]. Canonical table:

| Tier | Frequency among all spins | Bust rate |
|---|---|---|
| (none) | ~96.5% | n/a |
| Calm | ~2.5% | ~98% |
| Mid | ~0.7% | ~75% |
| Premium | ~0.25% | ~30% |
| Confirmed | ~0.05% | ~5% |

**Example: Reach frequency Monte Carlo**
```
Given a Probability Engine
When 1,000,000 spins are processed
Then calm reaches are ~25,000 ± 470 occurrences (3σ)
  And mid reaches are ~7,000 ± 250 occurrences
  And premium reaches are ~2,500 ± 150 occurrences
  And confirmed reaches are ~500 ± 67 occurrences
  And total jackpot count reconciles to R-2 within 3σ
```

---

### Rule R-7: Reach tier outcome is independent of jackpot resolution before the reach starts

Per [C-3]. The bust-rate column is the tier's hit rate, not a separate roll after the tier is chosen.

**Example: Confirmed-tier hit rate**
```
Given a Probability Engine that has produced 100,000 reaches of tier = "confirmed"
When the isJackpot field of each is tallied
Then the count of isJackpot = true is approximately 95,000 (95% hit, 5% bust)
  Within 3σ tolerance
```

---

### Rule R-8: Cabinet Coordinator state machine has the documented states and only the documented transitions

Per [C-1].

**Example: Transition enumeration**
```
Given a Cabinet Coordinator
When all reachable state transitions are enumerated by exhaustive event injection
Then the set of (from_state, event, to_state) triples is exactly:
  (BASE, REACH_START, REACH)
  (REACH, REACH_BUST, BASE)
  (REACH, REACH_HIT, JACKPOT_ROUND)
  (JACKPOT_ROUND, ROUND_COMPLETE, BETWEEN_ROUNDS)
  (BETWEEN_ROUNDS, NEXT_ROUND, JACKPOT_ROUND)
  (BETWEEN_ROUNDS, JACKPOT_END_NO_KAKUHEN, BASE)
  (BETWEEN_ROUNDS, JACKPOT_END_KAKUHEN, KAKUHEN_BASE)
  (KAKUHEN_BASE, REACH_START, KAKUHEN_REACH)
  (KAKUHEN_BASE, KAKUHEN_WINDOW_EXHAUSTED, BASE)
  (KAKUHEN_REACH, REACH_BUST, KAKUHEN_BASE)
  (KAKUHEN_REACH, REACH_HIT, JACKPOT_ROUND)
  (KAKUHEN_REACH, KAKUHEN_WINDOW_EXHAUSTED, BASE)
And no other transitions exist
```

---

### Rule R-9: BETWEEN_ROUNDS holds for exactly the attacker-reset duration; player input cannot skip

Per [C-1, C-3].

**Example: BETWEEN_ROUNDS duration**
```
Given a Cabinet Coordinator in JACKPOT_ROUND with attacker_reset_ms = 1500
When a ROUND_COMPLETE event is received at t=0
  And the player sends SKIP input at t=500ms
Then the SKIP input is dropped (not queued)
  And the state remains BETWEEN_ROUNDS until t=1500ms
  And a NEXT_ROUND event is emitted automatically at t=1500ms
```

---

### Rule R-10: Reach roster is config-driven; named reaches load from configuration with no code changes

Per [C-12].

**Example: Add a reach by config**
```
Given the reach config defines 8 named reaches across 4 tiers
When a 9th named reach "midnight-confrontation" is added to the config (tier = premium, weight = 0.2, beats = [...])
  And the Reach Engine is restarted (no code recompile)
Then "midnight-confrontation" is selectable in the premium tier with the configured weight
  And Monte Carlo over 100k premium-tier outcomes yields ~20,000 ± 600 selections of it
```

---

### Rule R-11: Each named reach is tagged with a `chapter`; reaches above the player's unlocked chapter are not eligible

Per [C-4].

**Example: Story-gated reach eligibility**
```
Given a player has completed 2 jackpots in a fresh session
  And the unlock schedule defines: chapter 1 unlocked from start, chapter 2 unlocked after jackpot 1, chapter 3 after jackpot 3
When the Reach Engine selects a reach
Then it picks from reaches with chapter ≤ 2 only
  And the chapter-3 "warehouse-confrontation" reach is excluded
```

---

### Rule R-12: No reach above "calm" tier plays in the first 5 minutes of a new session

Per [C-3, C-4]. The buildup is temporal, not just statistical.

**Example: Initial calm period**
```
Given a freshly started session at t=0
When the Probability Engine would emit a non-calm reach before t=5min
Then the Reach Engine substitutes the highest-allowed (calm-tier) reach instead
  And logs the substitution for telemetry-free analytics
```

---

### Rule R-13: Confirmed-reach bust is rolled at reach-start, not mid-animation

Per [C-3]. The 5% confirmed bust must be pre-determined so the animation can commit.

**Example: Confirmed pre-rolled**
```
Given a confirmed-tier reach has been selected
When the Reach Engine emits REACH_START for it
Then the resulting isJackpot value is bound at that moment
  And no subsequent event during the reach can flip it
  And the animation sequence chosen depends on isJackpot (hit vs bust paths differ)
```

---

### Rule R-14: BGM cross-fades on state transitions within ≤200ms latency, 800ms crossfade

Per [C-8]. Audio is the player's primary state-cue channel.

**Example: BGM transition latency**
```
Given the Audio/Visual Controller subscribed to coordinator events
  And BGM "base-loop" is currently playing
When a REACH_START event arrives at t=0
Then the "reach-tension" track begins fading in by t=200ms
  And the "base-loop" begins fading out simultaneously
  And both reach equilibrium at t=1000ms (200ms latency + 800ms crossfade)
```

---

### Rule R-15: Chucker chime plays within 50ms of the ball-entered-chucker event

Per [C-8].

**Example: Chucker chime latency**
```
Given the AVC is initialized and BGM is playing
When a CHUCKER_ENTRY event is received at t=0
Then the chucker chime SFX begins playback at t ≤ 50ms
  And the latency is measured via the audio engine's playback-started callback
```

---

### Rule R-16: Jackpot fanfare is uninterruptible; input during fanfare is dropped, not queued

Per [C-3, C-8]. The fanfare is the catharsis moment and must not be skippable.

**Example: Fanfare input handling**
```
Given the AVC is playing the jackpot fanfare (duration 15s)
When the player sends a SKIP input at t=3s
  And another SKIP input at t=10s
Then both inputs are dropped (not queued, not applied)
  And the fanfare plays to completion at t=15s
```

---

### Rule R-17: All voice lines play original Japanese audio; subtitles toggle JP/EN

Per [C-5].

**Example: Voice/subtitle independence**
```
Given a voice line is triggered during a premium reach
When the subtitle setting is "EN"
Then the audio plays the original JP voice clip
  And the subtitle overlay displays the EN translation
When the subtitle setting is switched to "JP" mid-line
Then the audio continues unchanged
  And the next subtitle update displays the JP transcript
```

---

### Rule R-18: Save state autosaves on every state transition with RTO < 50ms

Per [C-11].

**Example: Autosave latency**
```
Given a session in progress with persistent state on disk
When a state transition occurs (e.g., REACH_HIT → JACKPOT_ROUND)
Then the save file is updated atomically (write-temp-then-rename)
  And the update completes within 50ms of the transition event
  And on process kill immediately after, the loaded state matches the post-transition state
```

---

### Rule R-19: Save files are versioned and checksummed; corrupt saves trigger a new session, not a crash

Per [C-11].

**Example: Corrupt save recovery**
```
Given a save file exists on disk with a deliberately corrupted byte in the payload
When the game starts
Then the checksum mismatch is detected
  And a "save corrupted; starting new session" banner is shown
  And a fresh session begins
  And the corrupted file is renamed to save.corrupted.<timestamp> for inspection
```

---

### Rule R-20: Save files are byte-portable between macOS and Windows

Per [C-11].

**Example: Cross-OS save portability**
```
Given a save file generated on macOS
When the same file is copied to a Windows build and loaded
Then the loaded GameState is identical (deep-equality)
  And spin-count, jackpot history, kakuhen window, and unlocked chapters all match
```

---

### Rule R-21: Data Lamp HUD updates within 1 frame of relevant state change

Per [C-8]. The lamp is read-only and reactive.

**Example: HUD spin-count update**
```
Given the Data Lamp shows spins-since-last-jackpot = 47
When a spin completes (not a jackpot) at frame N
Then the HUD displays 48 at frame N+1
  And the HUD never lags more than 1 frame behind the canonical state
```

---

### Rule R-22: No outbound network traffic during normal operation

Per [C-7].

**Example: Network silence**
```
Given the game is running for 1 hour of normal play (base + reaches + jackpots + kakuhen)
When network activity is monitored at the OS level
Then zero outbound packets are sent
  And no DNS lookups are issued for any non-loopback hostname
```

---

### Rule R-23: 常連 audio identification — passes the sound-alone test within 30 seconds

Per [C-10]. This is a human-judged acceptance, scheduled at alpha.

**Example: Audio identification test**
```
Given a 常連-class consultant (a real Japanese pachinko regular)
  And they are positioned out of sight of the screen
When the game runs through 30 seconds of mixed base + reach audio
Then the consultant identifies the audio as pachinko (not "slot", not "video game")
  And the consultant correctly identifies whether the snippet contained a reach-tier escalation
```

---

## Platform notes (2026-05-25)

- **R-18 / R-19 / R-20 (persistence)** are implemented on **native** target only in v0.1.
  On WASM they no-op pending a `quad-storage`-or-`sapp-jsutils`-backed `localStorage`
  shim. Tracking under intent.md open question #6.
- **R-14 (BGM crossfade ≤200ms/800ms)** is implemented as a hard-cut in v0.1; macroquad's
  audio API lacks a primitive crossfade. The state-change *is* signalled via the immediate
  BGM swap, so the *cue* lands inside R-14's latency budget; the *smooth* portion is
  deferred. Tracking under intent.md open question #7.
- **R-17 (JP voice)** is not yet wired: there are no voice clips bundled in v0.1.
  The audio bank is procedurally synthesized brass/marimba/SFX only. Voice acting
  awaits intent.md open question #4 (voice casting) resolution.
- **R-23 (常連 sound-alone test)** is a human-judged alpha acceptance and is not
  reached in v0.1.

## Anti-patterns to avoid

- **Implementing reach selection inside the Probability Engine.** Reach *tier* is PE's output (R-6). Reach *naming and beat sequencing* is the Reach Engine's job (R-10, R-11). Conflating them violates C-1 and makes the story layer unmovable.
- **Hard-coding spec-sheet numbers in tests.** The numbers in R-2/R-3/R-5/R-6 are the canonical values; reference them through a single `SpecSheet` config so a future ADR can change them in one place. Otherwise C-2 enforcement scatters.
- **Skipping the Monte Carlo tests because they're slow.** R-2 / R-3 / R-6 take seconds each — they are *the* acceptance for C-2 and must run on every PR.
- **Designing reach animations before R-11 is implementable.** Story-gated reach eligibility is what makes C-4 real. Animations built without the chapter mechanic encode story state in art and can't be reordered without re-rendering.

## Index

| ID | Title | Status | Intent linkage |
|---|---|---|---|
| F-1 | MVP Pachinko Cabinet | Draft | C-1..C-12 |
