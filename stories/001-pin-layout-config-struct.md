# Story 001 — PinLayout config struct with chapter-gated knobs

**Status:** Ready  ·  **PRD-004 rules:** R-46, R-47  ·  **Intent:** C-12, C-19  ·  **Effort:** S–M

## What

Introduce a `PinLayout` struct in `pachinko-game/src/playfield.rs` that holds the canonical stock layout PLUS a small set of named knobs whose values translate pin clusters from their canonical positions. The `canonical_pins(pf: &Playfield)` function is refactored to accept a `&PinLayout` and apply the knob offsets when generating the pin list.

Six knobs total (chapter-gated availability):

| Knob | Range | Chapter unlocked | Effect |
|---|---|---|---|
| `left_funnel_tilt` | [-1.0, +1.0] | 2 | Translates the leftmost 2 columns of pins by ±8 px in x |
| `right_funnel_tilt` | [-1.0, +1.0] | 2 | Same on the right |
| `chucker_mouth_width` | [-1.0, +1.0] | 3 | Widens/narrows the funnel carve above the chucker by ±col_w |
| `guide_pin_vertical` | [-1.0, +1.0] | 3 | Slides the two guide pins above the chucker by ±12 px in y |
| `lower_row_density` | [-1.0, +1.0] | 4 | Adds or removes one row of pins in the lower 1/3 of the pin field |
| `upper_funnel_spread` | [-1.0, +1.0] | 4 | Tilts upper-row pins inward or outward by ±col_w * 0.4 |

All knobs default to 0.0 → canonical stock layout. Chapter 1 has zero knobs available (locked).

## Why

R-46 requires the playfield's pins to be derived from a typed, knob-influenced config. R-47 requires bounded, documented ranges. The 6-knob count is deliberately small (4–6 per audit-002): expansive enough to give the player a meaningful optimization space, focused enough that the UI doesn't sprawl.

## Tests

- `pin_layout_default_matches_canonical` — `PinLayout::default()` produces the same pins as the iter-3 `canonical_pins()` returned.
- `pin_layout_knob_bounds_enforced` — setting a knob to ±2.0 clamps to ±1.0.
- `pin_layout_no_pin_overlap` — for the full Cartesian product of knob extremes, no two pins land within 14 px of each other.
- `pin_layout_chapter_gated_count` — `available_knobs(chapter)` returns 0, 2, 4, 6 for chapters 1..4.

## Open

- Whether to expose `chucker_r` as a knob too — currently NO (the chucker is a designer-set constant; player tunes the funnel feeding it, not the cup itself).

## Not in scope

- The tuning UI overlay (story 003)
- Persistence of knob values (story 005)
- Display of predicted ベース (story 004)
