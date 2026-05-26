# Story 013 — Per-cabinet persistence namespace + meta storage

**Status:** Ready  ·  **PRD-005 rules:** R-62, R-79  ·  **Intent:** C-23  ·  **Effort:** S–M

## What

Refactor `persist.rs` so each cabinet stores its `PersistedState` under a namespaced key: `pachinko-the-revenge:v1:<cabinet_id>`. Add a separate meta key `pachinko-the-revenge:v1:meta` containing the cross-cabinet aggregate: most-recently-played cabinet id, total cabinets ever opened, cross-cabinet aggregate session count + jackpot count + balls fired.

A migration path for iter-4 saves: if `pachinko-the-revenge:v1` (the iter-4 un-namespaced key) exists at load time, migrate its contents to `pachinko-the-revenge:v1:the-revenge` and delete the old key. One-shot, idempotent.

## Why

R-62 forbids cross-cabinet state bleed. R-79 specifies the meta key for cross-cabinet aggregation. The iter-4 migration is required so existing players (who have iter-4 persisted state) keep their tuning + chapter progress.

## Tests

- `cabinet_save_uses_namespaced_key` — saving state for cabinet `foo` writes to `pachinko-the-revenge:v1:foo` and NOT to `pachinko-the-revenge:v1`.
- `meta_key_independent_of_cabinet_keys` — writes to a cabinet key do not affect the meta key contents.
- `iter4_migration_preserves_state` — given a pre-existing `pachinko-the-revenge:v1` key with iter-4 shape, loading migrates it to `pachinko-the-revenge:v1:the-revenge` and the old key is gone.
- `iter4_migration_idempotent` — running migration twice does nothing on the second run.

## Dependencies

- Story 009 (cabinet ids known at compile time)

## Open

- Whether the meta key includes per-cabinet aggregates as well, or only cross-cabinet ones. Decision: cross-cabinet ONLY; per-cabinet stats live in each cabinet's own state. Avoids duplication and staleness.

## Not in scope

- Parlor card UI rendering (story 014)
- Cloud sync. Local-only per C-7.
