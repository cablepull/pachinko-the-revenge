# Story 016 — `TidalRush` SpecialMechanic plugin

**Status:** Ready  ·  **PRD-005 rules:** R-70 (acceptance test), R-80, R-81, R-82  ·  **Intent:** C-22, ADR-004  ·  **Effort:** S–M

## What

Implement the `TidalRush` variant of the `SpecialMechanic` enum in `pachinko-game/src/cabinet/mechanics/tidal_rush.rs`.

API contract:
```rust
pub struct TidalRushState { phase: Phase, elapsed_sec: f32, next_fire_at_sec: f32 }
pub enum Phase { Idle, Active }

pub fn update(state: &mut TidalRushState, ctx: &mut MechanicCtx, dt: f32) {
    // Tick elapsed. On phase Idle → check next_fire_at; transition to Active on threshold.
    // On Active → tick elapsed_sec; transition to Idle at end of duration.
    // Emit events on transitions: TidalRushBegin, TidalRushEnd.
}
```

Plus a per-frame multiplier hook: `pub fn chucker_entry_multiplier(state: &TidalRushState) -> u32` returning 2 during Active, 1 during Idle. The main loop reads this during the per-chucker-entry handler and emits N pull_chucker calls instead of 1.

## Why

R-80 (the mechanic is declared in the enum). R-81 (one file per variant). R-82 (mechanics emit events; don't mutate math directly). This story implements the plumbing for the deep-sea-song cabinet (story 015).

## Tests

- `tidal_rush_idle_to_active_at_period` — TidalRushState with config period=300 fires Begin event at elapsed=300.
- `tidal_rush_active_for_full_duration` — Begin→End event spacing matches `duration_sec` exactly.
- `tidal_rush_multiplier_during_active` — `chucker_entry_multiplier` returns 2 during Active and 1 elsewhere.
- `tidal_rush_resets_after_end` — after End, next_fire_at_sec is set to (now + period).

## Dependencies

- Story 009 (`SpecialMechanic` enum exists)
- The `MechanicCtx` event-bus shape — small typed event struct in `cabinet/mod.rs` (declared as part of story 009).

## Open

- Whether the visual "TIDAL RUSH !!" banner is rendered by the mechanic or by the render layer. Decision: render layer (the mechanic just emits events; render reads them). Keeps mechanic free of macroquad imports.
- Whether the player can stack multiple Tidal Rushes if a JP is in progress. Decision: NO — Tidal Rush only ticks during BASE / KAKUHEN_BASE states; jackpot rounds pause the timer.

## Not in scope

- The "TIDAL RUSH !!" banner animation grammar (deferred to F-7 visual polish)
- Other mechanic variants (SyncRate / Battle / Fever / Parasocial — separate stories)
