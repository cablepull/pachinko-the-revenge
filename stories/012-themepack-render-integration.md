# Story 012 — ThemePack threaded through render + scene + audio

**Status:** Ready  ·  **PRD-005 rules:** R-72, R-73  ·  **Intent:** C-21, C-22, C-12  ·  **Effort:** L

## What

Refactor `render.rs`, `scene.rs`, and `audio.rs` to accept a `&ThemePack` parameter where they currently hard-code the-revenge's identity. Concretely:

- `scene::draw_back_panel(...)` becomes `scene::draw_back_panel(..., theme: &ThemePack)` and dispatches to a per-archetype back-panel renderer (initially: `back_panel_revenge_neon`, `back_panel_ocean_pastel`).
- `scene::draw_bezel_lighting(...)` accepts `theme.bezel_palette` instead of hard-coded warm-gold defaults.
- `audio::AudioBank` becomes `audio::AudioBank::for_theme(&ThemePack)` and builds a per-theme synth set (marimba-ukulele for deep-sea-song; brass for the-revenge).
- `render::draw_cabinet(...)` adds a `&ThemePack` parameter and forwards it to its callees.

ThemePack is data-only: enum IDs that the render layer maps to its registered renderers via exhaustive `match`. Adding a new theme is "add a variant + register a renderer + recompile."

## Why

Without this, every cabinet renders identically. R-72 + R-73 are how cabinet identity actually reaches the player's eye and ear.

## Tests

- `theme_dispatches_to_correct_back_panel` — given `theme.back_panel_id = OceanPastel`, the rendered pixels at a sampled point match the ocean-pastel implementation, not the neon-city one.
- `audio_bank_for_theme_uses_theme_patches` — bank built `for_theme(deep-sea-song)` exposes the marimba-ukulele base loop.
- `bezel_palette_per_theme` — each theme's bezel base color in CabinetState::Base matches its registered palette.

## Dependencies

- Story 009 (ThemePack type exists)
- Story 015 (deep-sea-song CabinetDef to test the dual-theme path)

## Open

- Whether to pre-load all themes at startup or lazily on cabinet activation. Decision: lazy — the active cabinet's theme is built when its session starts; previously-played themes can stay built in a small LRU cache.

## Not in scope

- Sprite asset integration (story 020 — Phase B; this story is procedural per Phase A)
- Per-theme particle palettes — deferred to story 022
