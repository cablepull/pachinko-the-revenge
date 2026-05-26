# Story 019 — Per-theme back-panel renderer (NeonCityscape + OceanPastel)

**Status:** Ready  ·  **PRD-005 rules:** R-72 (deep-sea-song acceptance), R-73, R-76  ·  **Intent:** C-21, C-22  ·  **Effort:** M

## What

Refactor `scene::draw_back_panel(...)` into a dispatcher over `BackPanelId`:

```rust
pub enum BackPanelId { NeonCityscape, OceanPastel }

pub fn draw_back_panel(/* existing args */, theme: &ThemePack, t: f64) {
    match theme.back_panel_id {
        BackPanelId::NeonCityscape => back_panel_neon_cityscape::draw(...),
        BackPanelId::OceanPastel   => back_panel_ocean_pastel::draw(...),
    }
}
```

The existing iter-3 cityscape moves into `scene/back_panel_neon_cityscape.rs`. A new `scene/back_panel_ocean_pastel.rs` implements the deep-sea-song back panel: pastel teal gradient, drifting coral/anemone silhouettes (rect-and-curve based), ambient bubble particles, soft caustic light bands at the top.

Both back-panel renderers use the sprite cache (story 018) for any element that's actually a sprite; the rest stays procedural per the existing iter-3 pattern.

## Why

R-72 / R-73: cabinet identity reaches the player's eye primarily via the back panel. Without per-theme back-panel renderers, all cabinets look identical regardless of ThemePack.

## Tests

- `back_panel_dispatches_by_theme` — given `theme.back_panel_id = OceanPastel`, sampling the back-panel center pixel shows a teal/cyan color; given `NeonCityscape`, shows the navy + neon palette.
- `back_panel_responds_to_state_tint` — when CabinetState is Kakuhen, both back panels apply their kakuhen-tint variant.
- `back_panel_animation_phase_advances_with_t` — the rain/bubble particle phase shifts deterministically with the time parameter (visual: snapshot at t=0 differs from t=1).

## Dependencies

- Story 012 (ThemePack threading)
- Story 018 (sprite cache for any sprite usage in the back panel)

## Open

- Whether the ocean-pastel panel uses sprite bubbles or procedural ones. Decision: procedural for iter-5 (per ADR-005 Phase A); sprite bubbles can replace them in iter-6 if needed.
- Should the kakuhen-tint variant be per-theme (different colors per cabinet) or platform-uniform? Decision: per-theme — deep-sea-song's "high tide" tint is brighter teal, not red.

## Not in scope

- The other three back panels (Thunder Herald, Sync Rate 400, Neon Fever) — deferred to iter-6+ when those cabinets ship.
