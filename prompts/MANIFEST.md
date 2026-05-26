# `assets/` MANIFEST — provenance, license, and post-processing per file

Story 020 acceptance criteria require this manifest. CI parses it; missing
entries cause the build to fail. **One row per asset file.**

## Schema

| Path | Cabinet | Role | Resolution | Bytes | Source / generator | Prompt revision | License | Post-processing | Date added |
|---|---|---|---|---|---|---|---|---|---|

## Asset paths convention

`assets/<cabinet-id>/<role>.png`

- `<cabinet-id>` matches the CabinetDef's `id` (e.g., `the-revenge`)
- `<role>` is one of: `back-panel`, `cut-in-confirmed`, `thumbnail`
  (more roles added as future stories require)

## Entries

_(populate as assets are added)_

| Path | Cabinet | Role | Resolution | Bytes | Source / generator | Prompt revision | License | Post-processing | Date added |
|---|---|---|---|---|---|---|---|---|---|
| _(none yet — prompts/README.md describes the workflow)_ | | | | | | | | | |

## License values

The `License` field accepts:

- `Owned-DALL-E-3`  → generated via the maintainer's OpenAI account; OpenAI ToS grants full rights
- `Owned-Midjourney-Std`  → MJ Standard Plan or Pro Plan; commercial use granted
- `Owned-SDXL-Local`  → self-hosted SDXL; output is CC0 if base weights are CC0-compatible
- `Owned-Flux-Local`  → self-hosted Flux; depends on Flux variant — record which one
- `Commissioned`  → paid artist; record contract or commission terms separately
- `PublicDomain`  → record provenance URL
- `CC-BY-4.0` / `CC-BY-SA-4.0` / `CC0-1.0`  → record source URL + author attribution
- `Original-Authored`  → maintainer-drawn; "you" as author

Any other license requires an ADR-style writeup before the asset can land.

## Prompt revision tracking

When you generate an asset, record:
- The prompt file: `prompts/cabinets/the-revenge.md`
- The prompt block ID: e.g., `the-revenge#back-panel-v3`
- The generator's seed (if exposed): e.g., MJ job id, DALL-E variant index, SD seed

If you re-generate the same asset after refining the prompt, **bump the
prompt revision** in this manifest. This is how we trace "the iter-6 art
came from a different prompt than the iter-5 art" — important for
visual-consistency audits.

## Post-processing field

Record what was applied. Examples:
- `pngquant --quality=80-95; oxipng -o 6 --strip safe` (the default)
- `pngquant --quality=70-85; oxipng -o 6 --strip safe` (tighter, for size-critical assets)
- `Manual-touch-up; oxipng -o 6` (the maintainer edited the AI output in Photoshop/Krita/Aseprite)
- `None` (raw generator output — should be rare; document why)

## Validation script

A future story can land `scripts/validate-asset-manifest.{sh,rs}` that:

1. Lists every PNG under `assets/`.
2. Checks each appears in this MANIFEST with all required fields.
3. Verifies file size matches the recorded `Bytes`.
4. Fails the build on any mismatch.

For iter 5, the manifest is honor-system; CI enforcement lands with story
020 acceptance.
