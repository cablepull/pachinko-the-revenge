# Style guide — cross-cabinet visual consistency

Every cabinet has its own theme. But every cabinet is *also* a cabinet in
the same parlor — they should read as members of one product line, not
five disconnected games. This guide names the rules every cabinet's art
honors regardless of theme.

## Composition rules (apply to ALL cabinets)

1. **Back-panel composition**: 1024×768 landscape, full-bleed. The CENTER
   AXIS must be slightly less busy than the edges — the cabinet's LCD
   reels overlay the center top quarter, and the chucker + attacker
   occupy the center bottom quarter. Detail and visual interest belong
   in the LEFT, RIGHT, and TOP/BOTTOM ZONES where the cabinet's
   foreground UI does NOT cover them.

2. **Character cut-in composition**: 512×768 portrait, with alpha
   channel. The character occupies the LEFT THIRD of the canvas; the
   right two-thirds is partially transparent so the cabinet's reels
   and HUD remain visible when the cut-in slides in. Background is
   stylized (lines, geometric shapes, atmosphere) — NOT a full scene.

3. **Thumbnail composition**: 256×192 (4:3, matches back-panel aspect).
   A reduced/simplified version of the back-panel that's still
   recognizable at small size. Critical: the cabinet's archetype must
   be readable at 256-pixel width.

## Palette discipline (per cabinet)

Each cabinet has a **3-color core palette + 1 accent**. Every asset for
that cabinet uses only those colors (and shades of them). Specifically:

| Cabinet | Color 1 | Color 2 | Color 3 | Accent |
|---|---|---|---|---|
| `the-revenge` | Deep navy (#0a0f24) | Crimson red (#c8364a) | Electric blue (#3a8cff) | Warm gold (#f3b54a) |
| `deep-sea-song` | Pastel teal (#5fb3b3) | Coral (#ff9a76) | Pale cyan (#bfe6e6) | Sandy gold (#e8c87a) |
| `thunder-herald` | Charcoal (#1a1815) | Dust orange (#c66c2a) | Cracked red (#8a1818) | Lightning yellow (#fff066) |
| `sync-rate-400` | Deep space blue (#0b1438) | Biomech orange (#f06030) | Sterile white (#e8eef8) | Sync purple (#8c5cdc) |
| `neon-fever` | Magenta (#ff2da6) | Cyan (#00d4ff) | Chrome silver (#d8d8e0) | Sunset yellow (#ffcb50) |

Asset-level palette compliance is the single biggest factor in whether
the cabinet feels coherent. A back-panel that uses 12 colors is broken
even if the dominant 3 are right.

## Line/edge grammar

- **Hard edges**, NOT painterly. The cabinet's procedural fallback art
  is hard-edged vectors; sprite assets must match.
- **Limited line-weight palette**: thin (1 px), medium (3 px), thick (6 px).
  No artistic stroke variation within a line.
- **Geometric simplification**: forms read as combinations of rectangles,
  circles, triangles, and gentle curves. Avoid organic complexity that
  reads as "AI-art slop."

## Lighting

- **One implied light source per cabinet**, consistent across all assets
  for that cabinet. Default: upper-left. This means highlights are on
  the upper-left of forms, shadows on the lower-right.
- **No bloom, no chromatic aberration, no lens flares.** These compress
  badly and read as "post-processed photo," not "pachinko art."

## File format + post-processing

After generation, every asset is run through:
1. `pngquant --quality=80-95 --speed 1` — quantizes to indexed-palette
   where possible. Often cuts file size by 60% with no visible
   degradation.
2. `oxipng -o 6 --strip safe` — losslessly re-encodes the PNG with
   optimal filter selection. Strips metadata.

The target sizes in the README are reachable for all 5 cabinets if these
steps run.

## Things to STRONGLY avoid

- **Embedded text**: any Japanese kanji, romaji, numbers, brand marks.
  All text is rendered at runtime.
- **Real-IP likenesses**: Eva, Hokuto, Sea Story characters, AKB idols.
  The archetypes inform our design; the assets are original.
- **Watermarks or signatures** from the generator.
- **Photographic textures**: skin pores, fabric weaves, lens grain.
  These bloat PNG files dramatically and clash with the cabinet
  grammar.
- **Heavy gradients with subtle banding**: SDXL outputs sometimes
  show 16-band gradients that don't compress well. Prefer hard
  color transitions or 3–4 step gradients.

## When in doubt

Reach for skill §13.1 — each canonical machine has an archetypal visual
identity. If your prompt is producing something that doesn't slot into
*one* of the five archetypal slots, the prompt is fighting the project's
direction.
