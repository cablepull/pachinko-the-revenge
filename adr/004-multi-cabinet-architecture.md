# ADR-004: Multi-cabinet platform architecture

**Status:** Accepted
**Date:** 2026-05-25
**Driver:** Iteration 5 directional shift — pachinko-the-revenge becomes a multi-cabinet platform (Microsoft Pinball / Pinball Arcade analog) rather than a single deep cabinet.
**Related:** intent.md C-9 (revised), C-21..C-23 (new); PRD-005 (the implementation spec).

## Context

Iterations 1–4 shipped a single deep cabinet ("The Revenge") with math, physics, theatre, and player-tuned nail adjustment. The user's iter-5 directive: "Microsoft Pinball could use lots of different backgrounds and triggers for different pinball games — let's be ambitious and create a PRD that can create a rival, except with pachinko cabinets."

Skill §13.1 catalogs five archetypal machines in the pachinko canon (Fever / 海物語 / 北斗 / エヴァ / AKB48). Each occupies a different *design slot* — casual, brutal, premium-IP, idol, fever-pioneer. A platform that hosts multiple cabinets in these archetypes is structurally aligned with how the canon itself is shaped: there is no *one* "great pachinko cabinet" any more than there is *one* great album — there is a portfolio of distinct experiences, each scratching a different player itch.

The question this ADR settles: **how do we add multiple cabinets WITHOUT creating five separate codebases?**

## Decision

**Cabinets are declarative content + a tightly-bounded mechanic plugin.**

Concretely:

1. The core math layer (`pachinko-core`) remains unchanged. `SpecSheet` continues to be the math contract; cabinets cannot violate it.

2. A new struct `CabinetDef` (in `pachinko-game/src/cabinet/mod.rs`) carries the per-cabinet identity:
   ```rust
   pub struct CabinetDef {
       pub id: &'static str,                    // "the-revenge"
       pub display_name: &'static str,          // "灰色の刃 — The Revenge"
       pub archetype: CabinetArchetype,         // Story | Casual | Brutal | Premium | ChromeRetro
       pub theme: ThemePack,                    // visual+audio asset references
       pub spec_overrides: SpecOverrides,       // bounded variations on the canonical spec
       pub reach_roster: ReachRoster,           // cabinet-specific named reaches
       pub chapter_labels: Vec<ChapterLabel>,   // narrative titles
       pub special_mechanic: SpecialMechanic,   // ONE typed plugin from a curated set
   }
   ```

3. A `SpecialMechanic` is one of:
   - `None` — baseline platform behavior only.
   - `TidalRush { period_sec, duration_sec, base_multiplier }` — periodic ベース-doubling window. The 海物語-archetype mechanic.
   - `SyncRateMeter { fill_per_reach: f32, threshold: f32 }` — meter fills with reach activity; at threshold triggers a guaranteed confirmed reach. The エヴァ-archetype mechanic.
   - `BattleSequence { fire_rate_target: f32, target_window_sec: f32 }` — confirmed reach becomes a skill check: player must time SPACE to hit a target window. The 北斗-archetype mechanic.
   - `FeverChain { trigger_every_n_jp: u32, chain_balls_per_round_mult: f32 }` — every Nth jackpot becomes a chained auto-fire mini-mode with increased payout. The Fever-archetype mechanic.
   - `ParasocialRoster { unlock_cast_per_chapter: u32 }` — characters unlock per chapter; appear in reach cut-ins. The AKB48-archetype mechanic.

   Each plugin is a typed enum variant — no trait dispatch, no dynamic dispatch. Adding a new mechanic requires a code change + a PRD entry. Adding a new cabinet that uses an EXISTING mechanic is purely data.

4. `SpecOverrides` is bounded:
   ```rust
   pub struct SpecOverrides {
       pub base_jackpot_prob: Option<f64>,      // must be in [1/399.6, 1/79.8] per regulation band
       pub kakuhen_entry_rate: Option<f64>,     // must be in [0.55, 0.85]
       pub st_window: Option<u32>,              // must be in [120, 220] per skill §13.2
       pub rounds_per_jackpot: Option<u32>,     // must be in {4, 8, 12, 16}
       pub balls_per_round: Option<u32>,        // must be in [60, 120]
       pub yen_per_ball: Option<u32>,           // must be in {1, 4}
   }
   ```
   Overrides that fall outside the allowed bands are rejected at cabinet-load time (a panic in debug, a fallback-to-canonical in release).

5. The platform layer (`pachinko-game`) gains a `cabinet_registry` constant array: `[CabinetDef; N]`. New cabinets append to this array.

6. Per-cabinet persistence uses a namespaced storage key: `pachinko-the-revenge:v1:<cabinet_id>`. Switching cabinets stores the current cabinet's `PersistedState` and loads the target cabinet's.

7. The active cabinet is selected at session start via a **selection screen** (PRD-005 F-6). The default is the most recently played cabinet (persisted under a separate `pachinko-the-revenge:meta` key).

## Consequences

### Positive

- **Adding a cabinet is a content task, not an engineering task.** The discipline that made PinLayout work (typed config + declarative data) applies to cabinets.
- **The math layer is the contract, and it's unchanged.** Spec variation happens within bounded ranges; the canonical 1/199.8 / 1/35.9 / 165-ST / 16R math remains the platform default.
- **Plugins compose with the rest of the system.** A `TidalRush` cabinet still benefits from the iter-3 event animations + iter-4 nail-adjustment + iter-5 art improvements. Cabinets share infrastructure.
- **No `dyn Trait` / no dynamic dispatch.** Enum variants are exhaustive and the compiler enforces match completeness. New mechanics force PRD discipline.

### Negative

- **WASM size grows with each cabinet's asset pack.** Five cabinets at ~200 KB of art each = +1 MB. Mitigated by lazy-loading per cabinet (load on selection, not at startup).
- **The platform's surface area grows.** Cabinet selection screen, meta-progression, switching UX are new features that didn't exist in iter 4.
- **Plugin API churn risk.** When a new mechanic doesn't fit any existing plugin variant, adding the variant requires touching every cabinet's `match SpecialMechanic` site (mostly in render + audio). Mitigated by limiting new plugins to PRD-driven moments.

### Neutral

- C-9 is revised (now reads "per cabinet") to harmonize with C-21. The depth discipline survives; it now applies *within* a cabinet rather than to the whole project.

## Alternatives considered

1. **One cabinet with multiple "modes" toggled by the player** (like Space Cadet's bumper combos). Rejected because mode-toggling within one cabinet doesn't deliver visual identity — players see the same back panel + same bezel + same chucker regardless of mode. The visual diversity that's the user's stated goal requires multiple full cabinets.

2. **Cabinets as runtime-loaded WASM modules.** Rejected because it breaks C-13 (single WASM artifact). Lazy-loading sprite assets within one binary is fine; lazy-loading code is a different architectural commitment.

3. **A `Cabinet` trait with `dyn Cabinet` dispatch.** Rejected because it loses compiler-enforced exhaustiveness and makes the plugin contract harder to audit. The enum-variant `SpecialMechanic` is a stricter API that catches "you forgot to handle the new variant" at compile time.

4. **Cabinets that can compose multiple `SpecialMechanic`s.** Rejected for iter 5. Combinations create combinatorial QA load and dilute each mechanic's identity (skill §13.3 — information-rich, surprise-honest; combining mechanics blurs the signal). Future iterations may revisit if a specific cabinet design genuinely demands two mechanics.

## How this affects existing code

- `pachinko-core`: unchanged.
- `pachinko-game/src/playfield.rs`: `PinLayout` becomes per-cabinet (each cabinet has its own default knob set + canonical pin field). Existing iter-4 layout is moved into `the-revenge` cabinet definition.
- `pachinko-game/src/persist.rs`: storage key gains the cabinet-id namespace.
- `pachinko-game/src/main.rs`: the main loop now starts with a cabinet selection step; the existing iter-4 main loop becomes "play one cabinet" which is invoked after selection.
- `pachinko-game/src/render.rs` + `scene.rs`: gain a `ThemePack` parameter; existing draw calls become parameterized by theme.
- New module: `pachinko-game/src/cabinet/` with `mod.rs` (Cabinet types), `registry.rs` (the cabinet array), and `mechanics/` subdir (one file per `SpecialMechanic` variant).

## Related

- intent.md C-9 (revised), C-21 (multi-cabinet platform), C-22 (declarative + bounded plugin), C-23 (mutually exclusive at session level)
- PRD-005 (the implementation spec; this ADR is its precondition)
- Skill §11.5 (payout-shape variation per machine)
- Skill §13.1 (canonical machine archetypes)
- Skill §13.2 (anti-patterns to avoid in multi-cabinet design)
- Skill §14.2 (production budget per cabinet)
