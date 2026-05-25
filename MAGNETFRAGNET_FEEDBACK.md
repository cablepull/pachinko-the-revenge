# magnetfragnet — running feedback

Maintained during setup and TDD use of `pachinko-the-revenge`. Captures benefits, wishlist items, and bugs/issues as they're encountered. Each entry is dated.

---

## Benefits

### B-1 — Convention forces testable invariants up front  *(2026-05-24, setup)*
The `intent.md` shape (numbered `C-N` constraints with rationale, separated from assumptions and open questions) is a forcing function. My original hand-written `intent.draft.md` had constraints buried in prose; the magnetfragnet template surfaced them as a table and made me write rationale per row. The rationale column is the part that ages well — it answers "why is this load-bearing" six months later.

### B-2 — PRD template prevents narrative-only specs  *(2026-05-24, setup)*
The `prd/README.md` rule that every Rule (`R-N`) needs at least one `GIVEN/WHEN/THEN` example with a *falsifiable* THEN is the right discipline for TDD. It also flags the right anti-patterns ("paraphrasing intent", "examples that are not falsifiable") that an LLM would otherwise drift into.

### B-3 — `magnetfragnet stats` provides a stable zero-state  *(2026-05-24, setup)*
Running stats before any iterations returns a clean "no iterations recorded yet" message — not an error, not a crash. Good UX for a brand-new project.

### B-4 — MCP server config ships pre-wired  *(2026-05-24, setup)*
The scaffolded `.mcp.json` is correct out of the box (`command: magnetfragnet`, `args: [mcp-stdio]`). No manual wiring needed in Claude Code if the file is in the right place.

### B-5 — Architecture-audit ID convention is well-specified  *(2026-05-24, setup, doc read)*
`audit-NNN-<slug>-YYYY-MM-DD` + four required JSON files (metadata, hypotheses, evidence, sources) is a real ACH discipline, not vibes. The requirement that every `sourceIds` entry resolve in `sources.json`, and every source `path` exist on disk, makes evidence auditable.

---

## Wishlist

### W-1 — `init --here` or `init .` to scaffold in current dir without creating subdir  *(2026-05-24, setup)*
`magnetfragnet init <name>` creates a `<name>/` subdirectory even when run inside an already-named project dir. I had to manually hoist the scaffolded files up one level. A `--here` flag (or `magnetfragnet init .`) that scaffolds in cwd without creating a subdir would skip the hoist step.

### W-2 — `init --merge` to non-destructively add to a project with existing intent.md/PRD.md  *(2026-05-24, setup)*
The current `init` refuses if `intent.md` exists (and prompts `--force`). `--force` would clobber a hand-written intent. A `--merge` mode that:
- Renames existing `intent.md` → `intent.draft.md`
- Scaffolds the template version
- Prints a guidance message: "your prior intent is preserved at intent.draft.md; integrate manually"
…would automate the dance I just did manually.

### W-3 — Scaffolded `intent.md` could include a 2nd C-N row by default  *(2026-05-24, setup)*
The template has one example constraint row. The `CLAUDE.md` mentions "14 constraints" suggesting real projects target a dozen-ish. A 2nd or 3rd row in the template (with a comment like `<!-- duplicate this row for each constraint -->`) would communicate the expected density.

### W-4 — `stats` output could surface "primary nudge distribution" stub even at zero iterations  *(2026-05-24, setup)*
The zero-state message is good but it could also display the nudge dimension names (with 0 fires each) so a new user sees what's *coming* once they start iterating, rather than just "nothing yet."

### W-5 — Scaffolded `prd/README.md` could include a worked example PRD  *(2026-05-24, setup)*
The template block in `prd/README.md` is correct but abstract. A second worked example (e.g., `prd/000-example.md`) with a fictional `F-0` feature and 2–3 fully-formed rules would teach the convention faster than the template alone.

### W-6 — `magnetfragnet validate-intent` command  *(2026-05-24, setup, speculative)*
A CLI command that statically checks `intent.md` for: at least 3 constraints, every constraint has a rationale, at least 1 assumption, at least 1 open question. Catches "wrote a vague intent" before the nudge engine fires.

### W-7 — Document the 9 nudge dimensions in the scaffolded `CLAUDE.md`  *(2026-05-24, setup)*
`CLAUDE.md` references "9 nudge dimensions" but doesn't list them. A user setting up the project has to dig into source to know what the tool is even watching for. A brief table in `CLAUDE.md` would orient.

---

## Issues

### I-1 — `init <name>` is ambiguous: "name" or "subdir"?  *(2026-05-24, setup, friction)*
The help text says `init [name]` — I passed `pachinko-the-revenge` thinking it was a project-name field for template substitution. It both substituted `{{PROJECT_NAME}}` *and* created a subdir of that name. The dual role is undocumented. Either:
- Document explicitly in help text: "creates a subdir named `<name>` and substitutes `{{PROJECT_NAME}}`"
- Or separate the two: `init --name <name>` for substitution, positional arg for subdir.

(Related to W-1.)

### I-2 — `CLAUDE.md` claims 14 constraints, template has 1  *(2026-05-24, setup, minor doc bug)*
`CLAUDE.md` line: "**`intent.md`** — the foundational doc for this project. 14 constraints (numbered `C-N`) and 9 nudge dimensions." The scaffolded `intent.md` has 1 example constraint row. The "14" is presumably aspirational/representative but reads as a factual claim about the template. Soften to "typically ~10–15 constraints" or remove the number.

### I-3 — Help command parser quirk: `magnetfragnet help` returns "unknown command"  *(2026-05-24, setup, minor UX)*
`magnetfragnet --help` works. `magnetfragnet help` returns `unknown command: help`. Most CLIs accept both. Cheap fix.

### I-4 — `intent.md` template ends with a trailing instruction paragraph the writer must remember to remove  *(2026-05-24, setup)*
The bottom paragraph ("This file is the anchor. magnetfragnet's nudge engine will detect when...") is template-y meta-prose that an inattentive author will leave in. Consider moving to a `<!-- comment -->` or to a separate `intent.template-notes.md`.

### I-5 — Permission friction: harness blocks .mcp.json and CLAUDE.md install to project root  *(2026-05-24, setup, environment-specific)*
Not magnetfragnet's bug, but worth flagging: Claude Code's auto-mode safety classifier blocks an agent from moving `.mcp.json` and `CLAUDE.md` into the project root because they auto-load on subsequent invocations. This means `magnetfragnet init` from inside an agent session can't fully complete the scaffold — the user must manually move those two files (or grant a permission rule). A short note in the README ("if running inside an autonomous Claude session, you may need to move .mcp.json and CLAUDE.md manually") would set expectations.

---

### B-6 — Convention survived a real implementation session  *(2026-05-25, build)*
The intent + PRD + ADR + audits + RCA + stories layout held up cleanly under an MVP build:
adding C-13 (WASM) and C-14 (Rust+macroquad) to `intent.md` was a 2-line edit, and the PRD's
R-N → C-N citations made it obvious which rules needed platform-deferral notes when WASM
persistence and BGM crossfade primitives didn't materialize. The "every R cites a C"
discipline turned what could have been ad-hoc scope-cut into a structured platform-notes
section. Pre-existing structure paid off.

### B-7 — TDD discipline produced a green math layer first try  *(2026-05-25, build)*
Writing all 21 tests (PE + CC + reach + save) *before* shipping the math layer caught one
real spec inconsistency: the PRD's reach-frequency table didn't reconcile to 1/199.8 by
~12%. Forced me to add the "direct-hit" jackpot category — which is also more authentic to
real CR machines. Without the Monte Carlo tests (R-2 / R-3 / R-5 / R-6 / R-7), this would
have shipped wrong and looked superficially fine.

---

## Wishlist (cont.)

### W-8 — Document `magnetfragnet stats --json` schema  *(2026-05-25, build)*
`CLAUDE.md` mentions `report_card` returns the same shape as `magnetfragnet stats --json`,
but the JSON schema isn't documented. A `magnetfragnet stats --schema` (or pointer to
schema/v1/stats.schema.json) would let downstream tools type-check.

### W-9 — Pre-commit hook template for the discipline  *(2026-05-25, build, speculative)*
A `pre-commit` template that runs `magnetfragnet nudge_iteration` against the staged diff
and blocks if a `severity: "block"` nudge fires. Would close the loop between
"discipline exists" and "discipline is enforced." Could ship as `magnetfragnet hooks install`.

---

## Issues (cont.)

### I-6 — Without the MCP server wired, the discipline is documentation-only  *(2026-05-25, build)*
`magnetfragnet stats` shows 0 iterations after a full TDD session that produced 21 green
tests, two crates (`pachinko-core` + `pachinko-game`), a WASM build, and a procedural
audio bank. The nudge engine never fired because the MCP server was never started
(blocked by I-5: harness wouldn't install `.mcp.json` + `CLAUDE.md` into project root,
so a fresh Claude Code session never auto-discovered the server). The discipline is
real, the tool is healthy — but the *feedback loop* is broken until I-5 is resolved.
Recommendation: a `magnetfragnet log_iteration` CLI subcommand that lets an LLM (or a
shell hook) record an iteration **without** the MCP path, so the stats are populated
even when the MCP server can't be wired by the agent.

---

## Observations (not yet bugs or wishes)

- The scaffolded directory structure (`adr/`, `audits/`, `rca/`, `research/`, `stories/`, `prd/`) is opinionated in a useful way — it pre-creates the artifact homes so an LLM doesn't invent ad-hoc paths.
- The README convention of `audits/index.json` + `rca/index.json` suggests these are intended to be machine-indexed; haven't yet seen how the index is used by the nudge engine.
- The `magnetfragnet.config.json` thresholds (`audit_loc_threshold: 250`, `audit_counter_N: 5`, `max_function_length: 50`) are reasonable defaults but I haven't yet experienced them firing — will note here once they do.

---

## Update protocol

When a new benefit / wishlist item / issue / observation is encountered:
1. Add a new entry to the appropriate section with a `B-N`/`W-N`/`I-N` ID (continue the numbering).
2. Date-stamp with `(YYYY-MM-DD, <phase>)` — phase = setup, iteration-N, post-MVP, etc.
3. If it's an issue, describe reproducibility steps unless trivially observed.
4. Keep the doc tight — this is a feedback log, not a design doc.
