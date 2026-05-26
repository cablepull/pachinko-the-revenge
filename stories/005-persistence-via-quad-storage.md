# Story 005 — Persistence via `quad-storage` (PinLayout, chapter, last-session summary)

**Status:** Ready  ·  **PRD-004 rules:** R-54  ·  **ADR:** ADR-002 (forthcoming, this story includes drafting it)  ·  **Intent:** C-13, resolves open question 6  ·  **Effort:** M

## What

Add `quad-storage = "0.1"` (smallest crate that wraps `localStorage` on WASM and a JSON file on native) and use it to persist:

- The current `PinLayout` knob values (one struct)
- `unlocked_chapter: u32`
- `last_session_summary` (story 006 produces this) — a `SessionSummary` struct
- A `last_session_at_ms` timestamp (UNIX millis) used to gate the welcome-back card (story 007)

Storage key: `pachinko-the-revenge:v1` (versioned so future schema changes don't crash older clients).

On startup, attempt to load. On failure (corrupt JSON, version mismatch), silently fall back to a fresh session (per anti-pattern: persistence must not gate gameplay).

## Why

R-54 requires persistence for the layout + chapter + summary. iter 3's `persist.rs` was no-op on WASM; this story fills that gap. `quad-storage` is the smallest viable shim (~5 KB extra WASM) and aligns with intent C-13 (single-file WASM artifact spirit).

## Tests

- `persist_roundtrip_native` — write → read on the native target produces byte-equivalent struct.
- `persist_fallback_on_corrupt` — supplying invalid JSON returns `None` and does not panic.
- `persist_version_mismatch_drops_safely` — supplying `pachinko-the-revenge:v0` returns `None` (forward-only).

## Dependencies

- Story 006 (SessionSummary struct — for the data this story persists)

## Open

- Whether to compress the persisted blob: probably not, the data is small (<1 KB).
- Whether `last_session_at_ms` should be IPS-local or UTC: UTC (deterministic across players' time-zone changes).
- ADR-002 should be drafted as part of this story; it documents the choice of `quad-storage` vs `sapp-jsutils` (which the iter-1 audit flagged) — the audit's H5-equivalent reasoning.

## Not in scope

- Migration tooling for v1 → v2. Defer to first schema change.
