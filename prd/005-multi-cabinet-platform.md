# PRD-005: F-5..F-8 — Multi-cabinet pachinko platform

**Status:** Draft
**Created:** 2026-05-25
**Intent linkage:** C-1, C-2, C-3, C-7, C-8, C-9 *(revised)*, C-10, C-12, C-13, C-14, C-15, C-16, C-17, C-18, C-19, C-20, **C-21**, **C-22**, **C-23**
**Stories:** stories/009..stories/024 (cut from this PRD; see Index)
**ADR Required:** **ADR-004** (Multi-cabinet platform architecture) is a precondition. **ADR-005** (Sprite asset pipeline + WASM size budget) is landed alongside F-8.
**Vision driver:** "Microsoft Pinball could use lots of different backgrounds and triggers for different pinball games — let's be ambitious and create a PRD that can create a rival, except with pachinko cabinets."

## Vision

A single binary that ships **multiple distinct pachinko cabinets**, each with its own visual identity, audio personality, spec-sheet variation (within regulation), reach roster, narrative, and one signature mechanic. The player picks a cabinet at session start. Switching cabinets is mutually exclusive at the session level — each cabinet is a complete experience honoring the C-9 depth contract.

This is Space Cadet's "many tables in one binary" applied to pachinko — except that pachinko's cultural canon (skill §13.1) already supplies the archetypes. We're not inventing five tables; we're materializing five archetypes that the canon has already proven work.

Initial roster (PRD-005's shippable scope is the platform + 2 cabinets — others land in subsequent PRDs):

| ID | Display name | Archetype | Mechanic plugin | Iter |
|---|---|---|---|---|
| `the-revenge` | 灰色の刃 — The Revenge | Story | `None` (baseline) | 5 (port) |
| `deep-sea-song` | 深海の歌 — Song of the Deep | Casual | `TidalRush` | 5 |
| `thunder-herald` | 雷鳴の使者 — Thunder Herald | Brutal | `BattleSequence` | 6 |
| `sync-rate-400` | シンクロ率400 — Sync Rate 400 | Premium | `SyncRateMeter` | 6 |
| `neon-fever` | ネオン・フィーバー — Neon Fever | ChromeRetro | `FeverChain` | 7 |

PRD-005 scopes F-5 (architecture), F-6 (selection), F-7 (the 2 initial cabinets + their mechanics), F-8 (asset pipeline). Iterations 6 and 7 expand the roster using the same architecture.

---

## Feature F-5: Cabinet architecture + registry

The platform layer that makes all subsequent cabinets shippable as content.

### Rule R-59: `CabinetDef` is the canonical cabinet representation

Per [C-21, C-22, ADR-004].

**Example: CabinetDef struct shape**
```
Given pachinko-game/src/cabinet/mod.rs
When `CabinetDef` is inspected
Then it has exactly these fields: id (&'static str), display_name (&'static str),
     archetype (CabinetArchetype enum), theme (ThemePack), spec_overrides (SpecOverrides),
     reach_roster (ReachRoster), chapter_labels (Vec<ChapterLabel>),
     special_mechanic (SpecialMechanic enum), default_layout (PinLayout)
  And no other fields
  And the struct derives Debug + Clone + Serialize + Deserialize
```

### Rule R-60: The cabinet registry is a `const`-array (compile-time enumerated)

Per [C-22, ADR-004].

**Example: Registry compile-time presence**
```
Given pachinko-game/src/cabinet/registry.rs
When the file is built
Then it exports a `pub const CABINETS: &[CabinetDef]` constant
  And the array has at least 2 entries in iter-5 (the-revenge, deep-sea-song)
  And each entry's `id` is unique
  And the build fails with a compile error if any id appears twice
```

### Rule R-61: `SpecOverrides` values are bounded; out-of-band rejects at load

Per [C-22, ADR-004]. Honors the regulation constraint (skill §11.1).

**Example: Valid override applies**
```
Given a cabinet with spec_overrides.base_jackpot_prob = Some(1.0 / 79.8)
When the cabinet is loaded
Then the effective SpecSheet's jackpot probability is 1/79.8
  And the cabinet's predicted long-run return rate is still ≤ 80% (regulation cap)
```

**Example: Out-of-band override rejected**
```
Given a cabinet with spec_overrides.base_jackpot_prob = Some(1.0 / 50.0)  // outside [1/399.6, 1/79.8]
When the cabinet is loaded in debug builds
Then the load panics with "spec override out of regulation band"
When the cabinet is loaded in release builds
Then the override is replaced with the canonical default (1/199.8) and a warning is logged
```

### Rule R-62: The active cabinet is mutually exclusive with all others

Per [C-23]. Switching cabinets is treated as ending the current session and starting another.

**Example: Cabinet switch semantics**
```
Given the player is in a session of cabinet `the-revenge` with state (chapter 2, layout knobs set, 47 spins)
When the player switches to cabinet `deep-sea-song`
Then the-revenge's full state is persisted to storage key `pachinko-the-revenge:v1:the-revenge`
  And a NEW session of deep-sea-song starts (or loads, if deep-sea-song has prior persisted state)
  And the deep-sea-song session has its own chapter counter, layout, summary
  And the player cannot see the-revenge's data while playing deep-sea-song (no cross-cabinet leakage)
```

### Rule R-63: Per-cabinet `PinLayout` defaults; the canonical 25–40% band per ADR-001 applies per cabinet

Per [ADR-001, ADR-004]. Each cabinet has its own canonical stock layout.

**Example: Each cabinet's stock layout is in the band**
```
Given a cabinet with default_layout = stock and no player tuning
When monte_carlo_chucker_rate runs against that layout for 1000 balls
Then the measured rate is in [0.25, 0.40] for every cabinet in the registry
  And the test asserts this for ALL cabinets (one test per cabinet, asserted with the cabinet's seed)
```

---

## Feature F-6: Cabinet selection screen

How the player chooses, switches, and discovers cabinets.

### Rule R-64: Cabinet selection is the first interactive screen of a session

Per [C-23]. Replaces the iter-4 startup flow.

**Example: First-time player sees the selection screen**
```
Given no prior session exists (fresh visit)
When the WASM finishes loading and the click-to-start overlay dismisses
Then the cabinet selection screen renders BEFORE any cabinet
  And the screen shows ≥ 1 cabinet tile per CabinetDef in the registry
```

### Rule R-65: Each cabinet tile shows a preview of the cabinet's identity

Per [C-21, skill §14.3 — "come back tomorrow" is influenced by visible variety].

**Example: Tile contents**
```
Given the cabinet selection screen is rendering
When a cabinet tile is drawn
Then the tile shows:
  - the cabinet's display_name in its theme font/color
  - a small thumbnail of the back-panel art (~200×150 sprite)
  - the archetype label ("CASUAL" / "STORY" / "BRUTAL" / etc.)
  - a one-line summary of the special mechanic ("periodic tide-mode", "battle sequence", etc.)
  - the player's prior stats for this cabinet (jackpots, highest chapter, last played) if persisted; otherwise "NEW MACHINE"
```

### Rule R-66: Tile click / Enter starts the cabinet's session

Per [C-23].

**Example: Selecting a cabinet**
```
Given the selection screen is showing 2 tiles
When the player clicks the `deep-sea-song` tile
Then the selection screen dismisses with a brief transition (~400ms)
  And the `deep-sea-song` cabinet's main loop starts
  And persistence loads under key `pachinko-the-revenge:v1:deep-sea-song`
  And the cabinet's ThemePack is applied to all subsequent rendering
```

### Rule R-67: An in-session "swap cabinets" affordance returns to the selection screen

Per [C-23, skill §14.5 "leave the machine" test].

**Example: Mid-session swap**
```
Given the player is in `the-revenge` at chapter 3 with 124 spins
When the player invokes "swap cabinets" (key: Q, or via the data lamp menu)
Then the current session's state is persisted (per R-62)
  And the selection screen re-appears
  And the player can choose any cabinet including the one they just left
```

### Rule R-68: The most-recently-played cabinet is highlighted as the default

Per [skill §14.3 — recency bias as a return-driver].

**Example: Default highlight**
```
Given the player's prior session was `deep-sea-song`
When the selection screen renders
Then the `deep-sea-song` tile has a visible "default" highlight (gold glow, prominent positioning)
  And pressing ENTER without further interaction selects `deep-sea-song`
```

---

## Feature F-7: The two initial cabinets

The platform's first concrete deliverables.

### Rule R-69: Cabinet `the-revenge` ports the iter-4 cabinet into the new platform

Per [C-21, C-22].

**Example: Functional equivalence to iter 4**
```
Given the platform's main loop is running cabinet `the-revenge`
When the player fires balls, hits the chucker, triggers reaches, hits jackpots, enters kakuhen, advances chapters, and tunes pins
Then the visual + audio + math behavior is byte-for-byte equivalent to the iter-4 standalone build
  And iter-4's PRD-004 rules (R-46..R-58) still pass when the cabinet under test is `the-revenge`
  And no new behavior is added or removed for this cabinet in iter 5 (it's a pure port)
```

### Rule R-70: Cabinet `deep-sea-song` ships with the Casual archetype + `TidalRush` mechanic

Per [skill §13.1 海物語 archetype, §11.5 flat payout shape].

**Example: deep-sea-song spec sheet**
```
Given cabinet `deep-sea-song` is loaded
When its effective SpecSheet is inspected
Then base_jackpot_prob = 1/119.8 (more frequent than canonical 1/199.8 — casual onboarding band)
  And kakuhen_entry_rate = 0.65 (lower than canonical 0.70 — gentler arc)
  And rounds_per_jackpot = 8 (flat payout shape per skill §11.5 — smaller-but-more-frequent JPs)
  And balls_per_round = 90 (canonical)
  And yen_per_ball = 4 (canonical)
  And st_window = 165 (canonical)
```

**Example: TidalRush mechanic fires periodically**
```
Given cabinet `deep-sea-song` is active with TidalRush { period_sec: 300, duration_sec: 30, base_multiplier: 2.0 }
When 5 minutes (300 sec) of base play has elapsed since session start (or since the last Tidal Rush ended)
Then a "TIDAL RUSH !!" banner slams in from the top
  And for 30 seconds: the effective ベース for the player's current layout is multiplied by 2.0
     (concretely: chucker entries are duplicated — every 1 in-flight chucker landing emits 2 ChuckerEntry events)
  And the cabinet's background shifts to a brighter palette
  And BGM crossfades to the "tidal rush" loop
When the 30-second window ends
Then ベース returns to normal
  And the next Tidal Rush is scheduled for 300 sec later
```

### Rule R-71: deep-sea-song's reach roster differs from the-revenge's

Per [C-21, skill §13.1 — different cabinets have different reach signatures].

**Example: Reach roster identity**
```
Given the deep-sea-song ReachRoster
When inspected
Then it contains different named reaches from the-revenge's roster
  And the names reference the sea-creature theme (e.g., "coral-shimmer", "whale-pass", "deep-current", "tidal-summon")
  And the tier counts match the iter-1 standard (2 calm, 3 mid, 2 premium, 1 confirmed)
  And the chapter_labels read as story beats appropriate to the theme ("CH 2 :: the reef brightens", "CH 4 :: into the deep")
```

### Rule R-72: deep-sea-song's theme uses a pastel-ocean palette + marimba+ukulele BGM

Per [skill §13.1 — 海物語 visual + audio steal table].

**Example: Theme override**
```
Given cabinet `deep-sea-song` is rendering
When the cabinet draws its back-panel
Then the back-panel art shows a stylized underwater scene (rect/line/gradient — Phase A procedural, per iter-5/6 art roadmap)
  And the dominant palette is pastel blue/teal/coral (no warm golds; no red bezel strobes)
  And the cabinet's bezel lighting in base state breathes in a gentle teal/cyan pattern
When the BGM plays
Then the synthesized base loop uses a marimba-ukulele patch (procedural synthesis per iter-3 audio.rs)
  And the fanfare is gentler (brass replaced by warm strings + chimes)
```

---

## Feature F-8: Sprite + audio asset pipeline

The substrate that makes Phase A (procedural elevation) and Phase B (embedded sprite assets) shippable.

### Rule R-73: `ThemePack` references assets via opaque IDs, not paths

Per [C-13, C-22].

**Example: ThemePack shape**
```
Given pachinko-game/src/cabinet/theme.rs
When ThemePack is inspected
Then it has fields for: back_panel_id (SpriteId), bezel_palette (Palette), bgm_set_id (AudioSetId), reach_grammar (ReachGrammarId), particle_palette (Palette), font_id (FontId)
  And no field is a runtime-loaded URL or filesystem path
  And SpriteId / AudioSetId / etc. are enum variants resolving to compile-time-embedded assets via include_bytes!()
```

### Rule R-74: Sprite assets are embedded into the WASM at compile time

Per [C-13]. No external HTTP requests at runtime.

**Example: Single-file artifact preserved**
```
Given a freshly built pachinko-game.wasm
When the served WASM is loaded in a browser
Then the network panel shows no requests for image/audio assets after the WASM bytes load
  And the cabinet renders complete art using only the embedded data
```

### Rule R-75: WASM size budget per cabinet is documented and enforced

Per [ADR-005]. Avoid uncontrolled bloat.

**Example: Per-cabinet size targets**
```
Given a new cabinet is added to the registry
When the cabinet's assets are budgeted
Then the cabinet's total asset overhead (sprites + audio compared to baseline) is documented in a comment near the CabinetDef
  And the total per-cabinet overhead is ≤ 300 KB compressed for "Story" / "Casual" archetypes (procedural-leaning)
  And ≤ 600 KB for "Premium" / "ChromeRetro" archetypes (sprite-heavy)
  And the total WASM size after iter-5 (platform + 2 cabinets) is ≤ 1.5 MB
  And the total WASM size after iter-7 (5 cabinets) is ≤ 4 MB
```

### Rule R-76: A sprite-cache renders embedded sprites via macroquad's Texture2D

Per [Phase A foundation from the prior conversation, ADR-005].

**Example: Sprite cache API**
```
Given a SpriteId
When `sprite_cache.get(sprite_id)` is called
Then it returns a `&Texture2D` ready to draw
  And the first call decodes the embedded PNG bytes and caches the texture
  And subsequent calls return the cached texture (no re-decode)
  And the cache is bounded — when memory pressure is observed, least-recently-used sprites are evicted (iter-7 concern; iter-5 has unbounded since assets are small)
```

### Rule R-77: Procedural sprites pre-render into Texture2D at startup

Per [Phase A from the prior conversation].

**Example: Procedural sprite generation**
```
Given the platform starts
When the sprite_cache initializes
Then a series of procedural sprite generators run (e.g., `gen_ball_sprite`, `gen_chucker_sprite`, `gen_pin_sprite`)
  And each renders into an off-screen Texture2D once
  And subsequent frames sample from the cached texture instead of redrawing the procedural primitives
  And the startup cost is bounded ≤ 200 ms on the macOS aarch64 reference machine
```

---

## Feature F-9: Cross-cabinet meta-progression (iter-5 scope: minimal)

A thin meta-layer that gives the player a reason to play multiple cabinets.

### Rule R-78: A "parlor card" tracks per-cabinet milestones

Per [C-21, skill §14.3 — return-driver]. Minimal iter-5 implementation.

**Example: Parlor card content**
```
Given the player has played at least one session on at least one cabinet
When the cabinet selection screen renders
Then a "PARLOR CARD" panel appears (toggleable, like the data lamp) showing:
  - per-cabinet: sessions played, total jackpots, highest chapter reached, longest dry streak, best session net ¥
  - cross-cabinet: total jackpots across all cabinets, total cabinets ever played, "favorite cabinet" (most-sessions)
  And clicking a row jumps to that cabinet's selection tile
```

### Rule R-79: Parlor-card data is persisted under a meta key

Per [C-23].

**Example: Meta persistence key**
```
Given the parlor card is updated after a session ends
When the persistence layer saves
Then the per-cabinet rows go to `pachinko-the-revenge:v1:<cabinet_id>` (existing keys)
  And the cross-cabinet aggregate goes to `pachinko-the-revenge:v1:meta`
  And neither write interferes with the other
```

---

## Sub-feature: SpecialMechanic plugin contracts

A typed catalog. Adding a new variant requires touching the exhaustive `match` sites — that's the discipline.

### Rule R-80: `SpecialMechanic` is a closed enum with these variants in iter 5

Per [C-22, ADR-004].

**Example: Iter-5 mechanic catalog**
```
Given the SpecialMechanic enum
When inspected
Then it has these variants and ONLY these variants:
  - None
  - TidalRush { period_sec: u32, duration_sec: u32, base_multiplier: f32 }
  - SyncRateMeter { fill_per_reach: f32, threshold: f32 }      // not used in iter-5 cabinets; declared for iter-6
  - BattleSequence { fire_rate_target_hz: f32, window_sec: f32 } // not used in iter-5; declared for iter-6
  - FeverChain { trigger_every_n_jp: u32, chain_bonus: f32 }      // not used in iter-5; declared for iter-7
  - ParasocialRoster { unlock_per_chapter: u32 }                  // declared for future iters
```

### Rule R-81: Mechanic plugins live in `pachinko-game/src/cabinet/mechanics/` one file per variant

Per [C-22, code organization].

**Example: Module layout**
```
Given the pachinko-game crate
When the cabinet/mechanics/ directory is inspected
Then it contains: mod.rs (re-exports), tidal_rush.rs, sync_rate.rs, battle_sequence.rs, fever_chain.rs, parasocial_roster.rs
  And each file exports one `update(&mut self, ctx: &mut MechanicCtx)` function
  And the trait `MechanicState` is the only shared abstraction (typed plugins, not dynamic dispatch)
```

### Rule R-82: Each mechanic's `update` is invoked exactly once per frame

Per [C-8, deterministic update order].

**Example: Frame update order**
```
Given the main loop's frame body
When inspected
Then the update order is: input → ball physics → session.tick → mechanic.update → render
  And the mechanic.update never modifies the SpecSheet, the SessionState, or the math layer directly — it only enqueues events that the game layer interprets (e.g., a TidalRushBegin/End event, a SyncRateThresholdHit event)
  And the math layer (pachinko-core) remains unaware that mechanics exist
```

---

## Anti-patterns to avoid

- **"Just copy-paste the cabinet and rename it."** PRD-005 exists to prevent this. Each cabinet must use the `CabinetDef` mechanism. If a developer is tempted to copy-paste, the platform abstraction is wrong; surface the gap as a story or PRD amendment.
- **Special mechanics that bypass the SpecSheet.** Per R-82, mechanics emit events; they do not directly mutate jackpot probability or kakuhen entry rate. A mechanic that wants to alter math must do so by composing with the existing event chain (e.g., TidalRush works by *doubling chucker entries*, not by changing the spin probability).
- **Per-cabinet UI components.** Cabinets parameterize the EXISTING render pipeline via ThemePack. A cabinet that wants its own bespoke render path is a design smell — the iter-3 layered render grammar should accommodate cabinet variation via theming, not branching.
- **Lazy-loading code (only data).** Per ADR-004 alt 2, code is statically linked at compile time. Lazy-loading limits to sprite + audio data via the sprite_cache.
- **More than one `SpecialMechanic` per cabinet.** Per ADR-004 alt 4, iter-5 enforces one-mechanic-per-cabinet. Compositions are deferred.
- **Cross-cabinet save bleed.** Per R-62 + R-79, the cabinet's namespace is its own. A cabinet must never read another cabinet's `PersistedState`; meta-progression goes through the dedicated `meta` key.
- **Cabinet-discovery via filesystem scan or HTTP fetch.** The registry is a `const`-array. New cabinets land via PR, not via configuration.

## Index

| ID | Title | Status | Intent |
|---|---|---|---|
| F-1 | MVP Pachinko Cabinet (math + reels) | Shipped v0.1 | C-1..C-12 |
| F-2 | Ball physics + pin field + launcher | Shipped v0.2 | C-1, C-6, C-8..C-10, C-13, C-14, C-15, C-16 |
| F-3 | Event celebrations + cabinet depth + economy | Shipped v0.3 | C-3, C-8..C-18 |
| F-4 | 釘調整 + contextual minimalism + sessions | Shipped v0.4 | C-2, C-3, C-9, C-10, C-12, C-13, C-15..C-20 |
| F-5 | Cabinet architecture + registry | Draft (v0.5) | C-1, C-9, C-21, C-22 |
| F-6 | Cabinet selection screen | Draft (v0.5) | C-3, C-21, C-23 |
| F-7 | Initial roster (the-revenge port + deep-sea-song) | Draft (v0.5) | C-1, C-9, C-21, C-22 |
| F-8 | Sprite + audio asset pipeline | Draft (v0.5) | C-8, C-13, C-22 |
| F-9 | Cross-cabinet parlor card | Draft (v0.5) | C-21, C-23 |
