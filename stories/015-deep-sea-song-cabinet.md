# Story 015 — `deep-sea-song` cabinet (Casual archetype + TidalRush mechanic)

**Status:** Ready  ·  **PRD-005 rules:** R-70, R-71, R-72, R-80  ·  **Intent:** C-21, C-22, skill §13.1 海物語 row  ·  **Effort:** M

## What

The second cabinet in the registry, demonstrating the platform's content path.

- `CabinetDef::deep_sea_song()` constructor in `cabinet/registry.rs`.
- SpecOverrides: base_jackpot_prob = 1/119.8, kakuhen_entry_rate = 0.65, rounds_per_jackpot = 8 (flat payout shape per skill §11.5). Other fields canonical.
- ReachRoster: 8 named reaches across 4 tiers, themed (e.g., "coral-shimmer", "whale-pass", "deep-current", "tidal-summon"). Per-tier hit rates and weights match canonical proportions.
- ThemePack: `back_panel_id: OceanPastel`, `bezel_palette: TealCyan`, `bgm_set_id: MarimbaUkulele`, `reach_grammar: SeaStory`.
- ChapterLabels: 4 entries themed for the underwater progression ("the reef brightens", "into the deep", etc.).
- SpecialMechanic: `TidalRush { period_sec: 300, duration_sec: 30, base_multiplier: 2.0 }`.

The procedural ocean-pastel back-panel renderer and the marimba-ukulele audio synth patches are scoped to this story.

## Why

R-70 / R-71 / R-72: this is the first concrete *new* cabinet. It validates the architecture by exercising every per-cabinet variation point: spec override, reach roster, theme, mechanic. If the cabinet renders and plays correctly, the platform abstraction works.

## Tests

- `deep_sea_song_spec_in_regulation` — the effective SpecSheet's long-run return rate is ≤ 80% (skill §11.1 regulation cap).
- `deep_sea_song_base_rate_in_band` — Monte Carlo against the cabinet's default PinLayout produces a chucker rate in [25%, 40%] (per ADR-001, applies per cabinet).
- `deep_sea_song_reach_roster_complete` — 8 reaches, 4 tiers in the canonical proportion (2/3/2/1).
- `tidal_rush_fires_at_period` — given a session with no Tidal Rush yet, advancing time by 300 sec triggers the TidalRushBegin event.
- `tidal_rush_doubles_chucker_entries` — during a Tidal Rush window, each ball entering the chucker emits 2 ChuckerEntry events; outside the window emits 1.

## Dependencies

- Story 009 (CabinetDef)
- Story 012 (ThemePack rendering)
- Story 016 (TidalRush mechanic plugin)

## Open

- Whether the Tidal Rush window cancels in-flight reaches. Decision: NO — Tidal Rush only affects ChuckerEntry duplication; reaches still progress on their own timeline.
- Whether the cabinet's default PinLayout differs from the-revenge's. Decision: yes — deep-sea-song has a gentler funnel (wider mouth, more guide pins) consistent with the casual archetype.

## Not in scope

- Sprite art for the underwater scene (Phase B — iter 6+)
- Voice acting (none for casual cabinet — per skill §13.1 海物語 has no voice-acted reach lines)
