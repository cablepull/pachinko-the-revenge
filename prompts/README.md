# `prompts/` — Image-generation prompt pack for Phase B

This directory holds copy-paste-ready prompts for the assets called out in
story 020 (Phase B embedded sprite assets). The engineering side of Phase B
is ready; story 020 is gated on these assets existing on disk.

## What you need to produce

For iter 5 (the platform + 2 cabinets):

| Asset | Resolution | Format | Target compressed size | Per cabinet |
|---|---|---|---|---|
| Back-panel | 1024×768 | PNG | ~300 KB | 1 |
| Confirmed-reach character cut-in | 512×768 | PNG with alpha | ~80 KB | 1 |
| Selection-screen thumbnail | 256×192 | PNG | ~30 KB | 1 |

For iter 6 (+ thunder-herald, sync-rate-400): another 3 sets of the above.
For iter 7 (+ neon-fever): one more set.

Total at iter 7: 5 back-panels + 5 cut-ins + 5 thumbnails = 15 images.

## Workflow

1. Pick a cabinet under [`cabinets/`](cabinets/).
2. Open its prompt file (e.g. `cabinets/the-revenge.md`).
3. Read the **style anchor** (defines the cabinet's identity in one paragraph).
4. For each asset in that file, copy the prompt block into your image generator.
5. Generate. Pick the best output. If using Midjourney with `--sref`, capture
   the style reference image's URL/seed and apply it to all subsequent
   prompts in the same cabinet — this is how you keep the back-panel,
   cut-in, and thumbnail visually coherent.
6. Run the result through `pngquant --quality=80-95` and `oxipng -o 6`
   (or equivalent) before committing. The target file sizes in the table
   above are achievable if the prompt produces art with limited palette
   and smooth gradients — they'll fail under photographic noise.
7. Drop the PNG into `assets/<cabinet-id>/<role>.png` (paths defined in
   each cabinet's `CabinetDef`).
8. Add an entry to `MANIFEST.md` recording source, license, generator,
   prompt revision, and post-processing applied.

## Generator notes

The prompts are written as natural language that works across DALL-E 3,
Midjourney v6, SDXL, Flux, and Imagen. Each prompt file includes
generator-specific tuning notes at the end:

- **DALL-E 3 (ChatGPT or API)**: paste the prompt as-is. DALL-E ignores the
  `--ar` parameter; specify aspect via "1024×768" or "wide composition" in
  the natural-language prompt.
- **Midjourney v6**: append `--ar 4:3` (or `--ar 2:3` for cut-ins). Use
  `--style raw` to suppress MJ's default photographic bias. Add
  `--s 250` for stylization weight. After generating the first asset
  for a cabinet, capture its URL and use `--sref <url>` for all
  subsequent prompts in the same cabinet to lock the style.
- **Stable Diffusion XL**: use the prompt as positive; the negative prompts
  in each block go into the negative field. Recommend sampler: DPM++ 2M
  Karras, steps 30, CFG 7.
- **Flux**: minimal prompt-tuning needed; the natural-language prompts
  work directly.
- **Imagen (Vertex AI / Gemini)**: paste as-is.

## Cross-cabinet style consistency

The five cabinets should each have a **strong unique identity** AND share
some platform-level cabinet-frame quality so the parlor feels coherent. See
[`style-guide.md`](style-guide.md) for the cross-cabinet rules (palette
discipline, line weight, composition conventions). Apply the style guide
on top of each cabinet's specific direction.

## License + ToS

- **Generate from a subscription you control.** DALL-E 3 (paid OpenAI
  account), Midjourney (paid), SDXL (self-hosted or via a service you
  pay for), Flux (paid or self-hosted). Free tiers often carry
  attribution requirements or non-commercial restrictions.
- **Each generator's ToS governs the output's usage rights.** As of
  2026, OpenAI grants users full rights to outputs they generate;
  Midjourney's Standard Plan grants commercial use; SDXL outputs are
  CC0 if the model weights you used are CC0-compatible. Verify the
  current ToS when you generate.
- **Record every output's provenance in `MANIFEST.md`**. The MANIFEST
  format is enforced by CI (story 020 acceptance criteria); missing
  entries fail the build.

## What NOT to do

- Don't include real-IP character likenesses (Eva pilots, Hokuto
  Kenshiro, AKB48 members, etc.). The project's intent is original
  IP per skill §13.1 — the cabinet archetypes inform our designs but
  the assets must be original.
- Don't embed text or logos in the art. The cabinet's text overlays
  are rendered by macroquad at runtime. Embedded text would lock
  localization and make the assets stale fast.
- Don't go for photorealism. Skill §13.2 anti-pattern: photorealistic
  pachinko-machine renders compress poorly (PNG noise) and clash
  with the cabinet's deliberate hand-drawn-vector grammar.
- Don't include real Japanese trademarks or parlor-machine
  serial-number text. The cabinets are inspired by canon archetypes,
  not specific machines.
