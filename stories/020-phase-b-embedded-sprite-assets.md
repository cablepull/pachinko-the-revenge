# Story 020 — Phase B: embedded sprite assets via `include_bytes!()`

**Status:** Blocked (needs asset sourcing)  ·  **PRD-005 rules:** R-74, R-75, R-76  ·  **Intent:** C-8, C-13, **ADR-005**  ·  **Effort:** M (engineering) + L (asset sourcing)

## What

Add PNG assets for high-impact sprite elements, embedded into the WASM via `include_bytes!()`. Decoded at startup via `image` crate (`image = { version = "0.25", default-features = false, features = ["png"] }`) and registered in the sprite cache (story 018).

iter-5 scope (minimum viable Phase B):
- 1 back-panel for `the-revenge` (1024×768 PNG, ~300 KB compressed): the rain-soaked neon cityscape but with proper depth, atmosphere, and detail beyond what code-only achieves.
- 1 back-panel for `deep-sea-song` (1024×768 PNG, ~300 KB compressed): the coral-reef scene with caustic light beams.
- 1 character-cut-in sprite per cabinet for the confirmed-tier reach (~256×512 PNGs, ~80 KB each).
- 1 cabinet-thumbnail per cabinet for the selection screen (~200×150 PNGs, ~30 KB each).

Total per-cabinet asset overhead: ~420 KB (within R-75's 600-KB ceiling for "Premium" archetype, comfortably above the 300-KB target for "Story"/"Casual" — the back-panel art is the dominant cost).

The procedural pipeline (story 019) remains as the fallback / sprite-source for elements not Phase-B-ified.

## Why

R-74 + R-75: embedded sprite assets are how the cabinet's visual quality crosses from "best procedural pachinko" into "rivals Space Cadet." This is the user's stated goal from the prior conversation turn.

## Tests

- `embedded_back_panels_decode` — both back-panel PNGs decode to non-empty `Texture2D`s during sprite-cache warmup.
- `embedded_assets_compressed_under_budget` — file size of each included PNG ≤ documented budget in CabinetDef comments.
- `phase_b_overrides_phase_a_when_available` — when a SpriteId has both a procedural generator and an embedded asset, the embedded one is used.

## Dependencies

- Story 018 (sprite cache)
- Story 019 (per-theme back-panel renderer — provides the API the embedded backdrop replaces)
- **External**: asset sourcing (you, an artist, AI image generator, or public-domain curation). Engineering is blocked on this. The PRD names this dependency explicitly so the asset cost isn't surprised.

## Open

- Asset license: only assets we own or have CC-Attribution / public-domain rights to. Decision: enforce via a manifest file `assets/MANIFEST.md` listing each asset's source and license; CI step parses it.
- Whether to use AVIF or WebP for better compression. Decision: PNG for iter-5 — adding decoders adds WASM size that may not pay back. Revisit at iter-7 if asset count grows.
- Image-gen prompt pack: do we ship a `prompts/` directory documenting the prompts that would produce each asset, so future iterations can re-source consistently? Decision: yes, lightweight markdown notes.

## Not in scope

- Audio assets (story 023 — separate effort)
- Voice acting (deferred — needs casting decision per intent open question 4)
- Procedural-to-sprite migration for ball/pin/chucker/knob/attacker — staying procedural for iter-5; revisit in iter-7 if Space Cadet-level art quality requires it
