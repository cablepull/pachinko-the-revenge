# Story 018 — Sprite cache + procedural sprite generators (Phase A)

**Status:** Ready  ·  **PRD-005 rules:** R-74, R-76, R-77  ·  **Intent:** C-8, C-13, C-22  ·  **ADR:** ADR-005 (drafted by this story)  ·  **Effort:** L

## What

Introduce a sprite-cache abstraction backed by macroquad's `Texture2D`:

```rust
pub struct SpriteCache { cache: HashMap<SpriteId, Texture2D> }
impl SpriteCache {
    pub async fn warm_procedural(&mut self) -> Result<(), SpriteError>;
    pub fn get(&self, id: SpriteId) -> Option<&Texture2D>;
}

pub enum SpriteId {
    Ball, Pin, Chucker, Knob, AttackerDoor,
    // ... add as needed; embedded sprite variants added in story 020 (Phase B)
}
```

For Phase A: a series of procedural generators render into off-screen `RenderTarget`s and capture the result into `Texture2D`. Each generator stacks multiple shading passes (base + highlight + rim light + drop shadow + specular) to produce sprite-quality art from primitives.

Initial procedural sprites (iter-5 scope): Ball, Pin, Chucker, Knob. The cabinet's existing direct `draw_circle(...)` calls for these elements are replaced with `draw_texture(sprite_cache.get(SpriteId::Ball)?, ...)`.

ADR-005 documents the size budget per R-75 and the choice to do Phase A procedural before any Phase B embedded sprites.

## Why

R-74 (embedded only, no runtime HTTP fetches), R-76 (sprite cache API), R-77 (procedural generators run at startup). This is the foundation of "graphics that rival Microsoft Pinball" — without sprite-quality rendering, the elements look like wireframes.

## Tests

- `sprite_cache_warm_completes_under_200ms` — measured wall time of `warm_procedural()` ≤ 200 ms on macOS aarch64.
- `sprite_cache_lookup_returns_same_texture_handle` — calling `get(Ball)` twice returns the same `&Texture2D` (no re-decode).
- `procedural_ball_has_lit_highlight` — sampling the ball sprite at the highlight position (upper-left) shows a near-white pixel; sampling at the opposite (lower-right) shows the base color darker. Verifies the shading pass actually ran.
- `sprite_cache_handles_missing_id` — looking up a not-yet-implemented sprite id returns None without panicking.

## Dependencies

None on the cabinet platform; this is an independent infrastructure story. Story 020 (Phase B embedded sprites) builds on it.

## Open

- Sprite atlas vs individual textures. Decision: individual `Texture2D`s for iter-5 (simpler API; macroquad allocates one GPU texture per call). Atlas packing is post-MVP.
- Sprite resolution: 64×64? 128×128? Decision: 128×128 for ball/pin (small but anti-aliased), 256×256 for chucker/knob/attacker. Total Phase A texture footprint: ~64 KB GPU.

## Not in scope

- Embedded PNG sprites via `include_bytes!()` — that's story 020 (Phase B)
- Lighting passes that respond to per-frame light position — iter-5 uses a fixed virtual light source (upper-left)
- Particle sprites — deferred to story 022
