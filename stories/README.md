# `stories/` — User Stories

Stories cut PRD rules into implementable work units. Each story is one
piece of build work whose acceptance criteria map back to PRD rules.

## Discipline

- File naming: `stories/NNN-kebab-case.md`, sequential.
- Each story cites the PRD feature and rule(s) it implements (`F-N R-N`).
- Each story has acceptance criteria; each criterion cites the rule it tests.
- Stories declare whether an ADR is required (`ADR Required: yes/no`).
- Stories carry an Assumptions table — load-bearing beliefs that, if wrong,
  would invalidate the story.

## Template

```markdown
# Story NNN: <Title>

**Status:** Draft | In Progress | Complete | Invalidated
**Created:** YYYY-MM-DD
**Author:** <agent or person>

**PRD linkage:** F-N R-N
**Intent linkage:** C-N, nudge #N
**ADR Required:** yes/no

## Intent

One paragraph: what user-facing problem this story solves, and how we'll
know it's solved (the measurable outcome).

## Acceptance Criteria

- [ ] AC-1: <rule, traceable to PRD R-N> — PASS when <observable condition>
- [ ] AC-2: ...

## Assumptions

| ID | Assumption | Basis | Status |
|----|-----------|-------|--------|
| A-1 | ... | ... | Open / Verified / Invalidated |
```

## Index

| ID | Title | Status | PRD | Effort |
|---|---|---|---|---|
| 001 | PinLayout config struct with chapter-gated knobs | Ready | PRD-004 R-46/47 | S–M |
| 002 | Stock-layout ベース test (MC in [25%, 40%]) | Ready | PRD-004 R-46/57 | S |
| 003 | Tuning workshop UI (hidden-by-default) | Ready | PRD-004 R-49 | M |
| 004 | Predicted ベース confidence interval | Ready | PRD-004 R-48 | S–M |
| 005 | Persistence via quad-storage | Ready | PRD-004 R-54 | M |
| 006 | Session-end summary screen | Ready | PRD-004 R-52 | S–M |
| 007 | Welcome-back continuity card | Ready | PRD-004 R-53 | S |
| 008 | Environmental reach cues + BGM crossfade | Ready (iter-5 carry) | PRD-004 R-50/55 | M |
| 009 | CabinetDef + cabinet registry | Ready | PRD-005 R-59..61 | M |
| 010 | Cabinet selection screen | Ready | PRD-005 R-64..68 | M |
| 011 | Mid-session cabinet swap | Ready | PRD-005 R-67 | S |
| 012 | ThemePack threaded through render + audio | Ready | PRD-005 R-72/73 | L |
| 013 | Per-cabinet persistence namespace + iter-4 migration | Ready | PRD-005 R-62/79 | S–M |
| 014 | Parlor card (cross-cabinet meta-progression) | Ready | PRD-005 R-78/79 | S–M |
| 015 | `deep-sea-song` cabinet (Casual + TidalRush) | Ready | PRD-005 R-70..72/80 | M |
| 016 | TidalRush mechanic plugin | Ready | PRD-005 R-70/80..82 | S–M |
| 017 | Port `the-revenge` onto the platform | Ready | PRD-005 R-69 | M |
| 018 | Sprite cache + procedural sprite pipeline (Phase A) | Ready | PRD-005 R-74/76/77 | L |
| 019 | Per-theme back-panel renderer (Neon + Ocean) | Ready | PRD-005 R-72/73/76 | M |
| 020 | Embedded sprite assets via include_bytes!() (Phase B) | Blocked (assets) | PRD-005 R-74/75/76 | M+L |
| 021 | Stock-layout band test per cabinet | Ready | PRD-005 R-63 | S |
