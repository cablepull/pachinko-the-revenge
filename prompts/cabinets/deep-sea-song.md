# Prompts — `deep-sea-song` cabinet (深海の歌)

**Archetype:** Casual (海物語-inspired)
**Mechanic:** TidalRush (periodic ベース-doubling window)
**Palette:** Pastel teal #5fb3b3, Coral #ff9a76, Pale cyan #bfe6e6, Sandy gold #e8c87a

## Style anchor

A gentle, casual underwater scene rendered as a pachinko cabinet. Warm,
welcoming — NOT a noir thriller. The 海物語 (Sea Story) archetype is the
reference: this is the machine grandma plays. Pastel ocean palette with
coral and sandy-gold accents. Marimba-and-ukulele music plays in the
mental ear. Hard-edged vector grammar but the FORMS are organic
(curves of sea creatures, drifting kelp) — the LINE WORK is hard-edged
but the SHAPES are soft-rounded.

The cabinet's emotional tone is "drift, watch, breathe." Languid, not
slow. Inviting.

---

## deep-sea-song#back-panel-v1 — 1024×768 back-panel

```
A stylized underwater coral reef scene rendered as a 2D backdrop.
A pastel teal water column dominates; corals in the lower portion in
sandy gold and coral-pink silhouettes; a few stylized fish drifting
across the upper-middle, each rendered as 2-3 hard-edged geometric
shapes (NOT detailed). Soft caustic light beams angle from the
upper-left, drawn as 3-step pale-cyan gradients rather than smooth
gradients.

A few bubble particles drift upward in scattered diagonals — each
bubble is a 2-3-pixel circle with a tiny highlight. Total bubble
count ~30, distributed unevenly (denser in lower-left, sparser
upper-right).

Composition: 1024×768 landscape, full-bleed. The CENTER AXIS is
deliberately less busy — corals and fish populate the left, right,
and lower regions; the center is open water (where the cabinet's
LCD and chucker will overlay).

Atmosphere: warm, inviting, languid. NOT dark, NOT mysterious.
NO threatening sea creatures — the largest fish is a gentle
silhouette in pastel teal, not a shark. Mood reference: 海物語
(Sea Story) pachinko machine backdrop, but with original creature
designs (no recognizable franchise characters).

Style: hard-edged vector art, flat shading, 4-color palette (pastel
teal #5fb3b3, coral #ff9a76, pale cyan #bfe6e6, sandy gold #e8c87a),
shades within those colors but no smooth gradients. 1024×768 landscape.

NEGATIVE: characters, faces, people, text, signage, logos, brand
names, kanji, photorealistic, painterly, soft edges, bloom, lens
flare, JPEG artifacts, gradient banding, more than 6 distinct
colors, dark/threatening atmosphere, sharks, predators, blood,
nighttime, deep ocean trench (this is a SHALLOW REEF, not the
abyss).
```

**Generator notes:**
- Midjourney v6: `--ar 4:3 --style raw --s 200`. The lower `--s` keeps
  the output gentler.
- DALL-E 3: paste as-is. If output is too "realistic ocean
  documentary," add "pachinko-machine art style, flat vector
  illustration" to the opening.

---

## deep-sea-song#cut-in-confirmed-v1 — 512×768 character cut-in

The cut-in fires during a confirmed-tier reach. For deep-sea-song,
the "character" is a gentle sea-spirit — there is no human
protagonist; the cabinet's narrative is mood, not plot.

```
A stylized whale-like sea creature occupying the LEFT THIRD of a
512×768 portrait canvas. The whale's body curves gently — head
upper-left, tail flowing toward the lower-right. The creature
is a silhouette in pastel teal with sandy-gold rim-lighting along
its upper edge (light from upper-left, suggesting surface light
filtering down).

The whale has one visible eye — gentle, calm, pale cyan. Around
the whale, soft caustic light beams (3-step pale-cyan gradients,
NOT smooth) suggest the surface is overhead and the creature
is rising toward it.

The right two-thirds of the canvas is mostly alpha-transparent
with a few atmospheric bubble particles drifting upward and
2-3 thin caustic-light streaks. NO secondary characters, NO
full background scene.

The whale must read as gentle, welcoming, NOT majestic-and-distant.
This is a creature you would WANT to encounter, not flee from.

Style: hard-edged vector, flat shading, 4-color palette (pastel
teal #5fb3b3, coral #ff9a76, pale cyan #bfe6e6, sandy gold #e8c87a)
plus alpha. 512×768 portrait with alpha channel.

NEGATIVE: face details (the eye is a simple shape), photorealistic
whale anatomy, multiple creatures, full ocean scene background,
text, signage, watermarks, dark/threatening framing, sharks,
killer whales, blood, predatory expressions.
```

**Generator notes:**
- Alpha channel is critical. DALL-E 3: generate on a uniform pale-cyan
  background and key out.
- The whale's expression is the make-or-break detail — if it reads
  threatening, the cabinet's "casual" identity breaks. Iterate until
  the eye reads gentle.

---

## deep-sea-song#thumbnail-v1 — 256×192 selection-screen thumbnail

```
A small-scale reduction of the underwater coral reef from the
deep-sea-song back-panel. 256×192 landscape. Composition shows:
2-3 simplified coral silhouettes in the lower portion (sandy gold +
coral-pink), one drifting fish in the upper-middle (pastel teal
silhouette), a few caustic light beams from upper-left.

Detail reduced for legibility at thumbnail size; the cabinet's
archetype (casual, welcoming, underwater) must remain readable.
CENTER is open water.

Style: matches the back-panel exactly (hard-edged vector, 4-color
palette: pastel teal #5fb3b3, coral #ff9a76, pale cyan #bfe6e6,
sandy gold #e8c87a). NO characters, NO text. 256×192 landscape PNG.

NEGATIVE: characters, faces, text, signage, photorealistic, soft
edges, more than 6 colors, dark atmosphere, predators.
```

**Generator notes:**
- Downscaling from the back-panel at 1024×768 may be sufficient;
  alternatively generate directly at 256×192 with the simplified
  prompt above.
