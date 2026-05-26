# Prompts — `the-revenge` cabinet (灰色の刃)

**Archetype:** Story (original-IP noir revenge)
**Mechanic:** None (baseline)
**Palette:** Deep navy #0a0f24, Crimson red #c8364a, Electric blue #3a8cff, Warm gold #f3b54a

## Style anchor

A noir revenge tale rendered as a 1990s-vintage pachinko cabinet. Rain-soaked
neon-lit cityscape — Tokyo / Hong Kong / Blade Runner without the franchise
references. Dark navy + crimson dominant; warm gold accent reserved for
catharsis moments only. Hard-edged vector grammar. Hand-drawn-feel but
NOT painterly — flat shading, deliberate geometry. A single implied light
source from upper-left.

The cabinet's emotional tone is "the protagonist has been waiting for this
night." Brooding, contained, deliberate. NOT chaotic, NOT busy.

---

## the-revenge#back-panel-v1 — 1024×768 back-panel

```
A stylized rain-soaked neon cityscape at night, viewed as a flat 2D
backdrop. Geometric building silhouettes in dark indigo and navy, arranged
with the tallest mid-frame, descending toward the edges. Vertical neon
strips in crimson, electric blue, and warm gold light a few of the
buildings — sparse, deliberate, NOT every building lit. Diagonal rain
streaks in pale blue-white. A faint pale moon disc in the upper-right
quadrant, partially obscured by a building.

The composition leaves the CENTER OPEN — the upper-center and lower-center
have less detail than the LEFT, RIGHT, and TOP edges, because the cabinet's
reel display and chucker will overlay the center axis.

Subtle horizontal scanlines suggest CRT glow over the whole scene at very
low opacity. The atmosphere is contained, brooding, like the protagonist
has just stepped out of the rain to wait.

Style: hard-edged vector art, flat shading, limited palette of 4 colors
(deep navy #0a0f24, crimson #c8364a, electric blue #3a8cff, warm gold
#f3b54a), with shades but no gradients beyond 3-step. Composition is
1024×768, full-bleed, landscape orientation.

NEGATIVE: characters, faces, people, text, signage, logos, watermarks,
brand names, kanji, photorealistic, painterly, soft edges, bloom, lens
flare, JPEG artifacts, gradient banding, more than 6 distinct colors,
center-of-frame detail.
```

**Generator notes:**
- Midjourney v6: append `--ar 4:3 --style raw --s 250`. Use this as your
  `--sref` anchor for the cut-in and thumbnail.
- DALL-E 3: paste as-is. May add "ensure the center is less busy than
  the edges" if first attempt is symmetrical.
- SDXL: positive=above, negative=after-NEGATIVE-tag. Sampler: DPM++ 2M
  Karras, CFG 7, steps 30, 1024×768.

---

## the-revenge#cut-in-confirmed-v1 — 512×768 character cut-in (confirmed-tier reach)

The cut-in fires during a confirmed-tier reach — the catharsis moment.
The character is the protagonist; the framing is "the moment they raise
the blade."

```
A stylized figure standing in profile, occupying the LEFT THIRD of a
512×768 portrait canvas. The right two-thirds is mostly transparent /
empty — only atmospheric lines and a faint geometric backdrop, NOT a
full scene.

The character is a silhouette in deep navy with crimson rim-lighting
along the left edge (light source from upper-left). They wear a hooded
coat with hard angular lines; one arm is raised holding a slim blade
that extends diagonally up and to the right, with a thin warm-gold
glint along the blade edge. The face is mostly in shadow except for
one eye visible under the hood, glowing pale electric blue.

The composition reads as "the protagonist has chosen this moment."
Brooding stillness, NOT motion. The blade is the visual emphasis.

Background (right two-thirds): a few diagonal warm-gold speed lines
emanating from the figure, fading to nearly transparent at the right
edge. NO city scene, NO crowd, NO secondary characters. The character
must read as alone.

Style: hard-edged vector art, flat shading, 4-color palette (deep navy
#0a0f24, crimson #c8364a, electric blue #3a8cff, warm gold #f3b54a) plus
the alpha channel. 512×768 portrait, with alpha channel.

NEGATIVE: face details, photorealistic, multiple characters, crowds,
text, signage, watermarks, signature, real-IP likeness (no Eva, no
Hokuto Kenshiro, no specific anime characters), painterly, soft edges,
bloom, full background scenes.
```

**Generator notes:**
- The alpha channel is the critical output detail. For DALL-E 3 (which
  can't natively produce alpha), generate on a uniform background (e.g.,
  "on a deep navy uniform background, no atmosphere") and key out the
  background in post.
- Midjourney v6: `--ar 2:3 --style raw --sref <back-panel URL>` so the
  style matches the back-panel.
- SDXL with alpha: use a model variant that supports it, or
  background-remove via `rembg` post-process.

---

## the-revenge#thumbnail-v1 — 256×192 selection-screen thumbnail

The cabinet's identity at small size. Reads as the same place as the
back-panel but simplified.

```
A small-scale reduction of the rain-soaked neon cityscape from the
the-revenge back-panel. 256×192 landscape. The composition shows:
3-4 geometric building silhouettes (simplified from the back-panel
count of 9), 2-3 visible neon vertical strips (crimson, electric blue,
warm gold), faint diagonal rain streaks, the moon disc in upper-right.

Detail is reduced for legibility at thumbnail size — but the cabinet's
archetype must remain readable: a noir city, a rainy night, a brooding
atmosphere. The CENTER is open; detail is in the upper-left and
upper-right quadrants.

Style: matches the back-panel exactly (hard-edged vector, 4-color
palette: deep navy #0a0f24, crimson #c8364a, electric blue #3a8cff,
warm gold #f3b54a). NO characters, NO text. 256×192 landscape PNG.

NEGATIVE: characters, faces, text, signage, watermarks, photorealistic,
soft edges, more than 6 colors, center-of-frame detail.
```

**Generator notes:**
- Generate at 1024×768 with the back-panel prompt; downscale to 256×192
  via `convert input.png -filter Lanczos -resize 256x192 -strip output.png`
  as an alternative to generating directly. Often gives a more
  consistent thumbnail than re-prompting.
- Confirm legibility at 256-px width before committing.
