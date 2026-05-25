---
name: intelligence-analyst
description: "Rigorous intelligence analysis"
---

# Intelligence Analyst skills

#ACH Evidence Classification Skill

Use this skill when analyzing competing explanations for a disputed event, media narrative, intelligence question, organizational incident, or suspected influence operation.

The purpose is to avoid weak suspicion tables. A proper ACH must separate:

1. What happened.
2. What evidence exists.
3. How reliable each evidence item is.
4. Which hypotheses each evidence item supports or contradicts.
5. Which evidence is actually diagnostic.

## When to Use

Use this skill when the user asks to:

- Build an ACH matrix.
- Analyze competing hypotheses.
- Determine who benefits from a narrative or event.
- Investigate possible misframing, manipulation, coordinated messaging, or influence.
- Compare “ordinary incompetence” against “deliberate action.”
- Evaluate whether a media story, policy push, corporate action, or public narrative was accidental, incentivized, or coordinated.

## Core Principle

Do not begin with a matrix of suspicions.

Begin with classified evidence.

A weak ACH asks:

> Who benefits?

A better ACH asks:

> Which evidence, properly classified, best distinguishes between competing hypotheses?

## Required Output Structure

### 1. Define the Analytic Question

State the question precisely.

Example:

> Why did Outlet X revive and distort a three-month-old quote from Person Y?

Avoid vague questions like:

> Who is behind this?

Instead use:

> Which hypothesis best explains the timing, framing, amplification, and beneficiary pattern of the article?

### 2. Define the Distortion or Event

Describe the observable event without overclaiming intent.

Example:

> The source quote referred to automation of “tasks,” while later coverage converted this into automation of “jobs” or “all white-collar work.”

Separate:

- What was actually said.
- How it was reframed.
- Who reframed it.
- When the reframing occurred.
- How it propagated.

### 3. Generate Competing Hypotheses

Include both suspicious and mundane explanations.

Minimum hypothesis set:

| ID | Hypothesis |
|---|---|
| H1 | Ordinary editorial/traffic opportunism |
| H2 | Deliberate narrative reframing by the outlet |
| H3 | Commercial ecosystem influence |
| H4 | Political/labor/regulatory narrative pressure |
| H5 | Rival actor or competitive targeting |
| H6 | Sloppy headline compression or aggregation error |
| H7 | Narrative spillover from a related but stronger claim by another actor |

Do not omit innocent or mundane hypotheses. ACH is strongest when it can eliminate them.

### 4. Classify Evidence Before Scoring It

Every evidence item must be classified before it is placed into the ACH matrix.

Use this evidence metadata table:

| Field | Description |
|---|---|
| Evidence ID | Unique identifier, e.g. E1, E2 |
| Observation / claim | What the evidence says or shows |
| Source | Where the evidence came from |
| Source type | Primary, secondary, derivative, opinion, transcript, PR, corporate essay, headline, syndicated copy, AI summary |
| Source reliability | Admiralty-style A–F |
| Information credibility | Admiralty-style 1–6 |
| Provenance | Direct quote, paraphrase, headline, social card, transcript, archived page, AI summary |
| Timeliness | Fresh, stale, revived, retrospective, contemporaneous |
| Corroboration | Independent, circular, same-source echo, uncorroborated |
| Manipulation risk | Low, medium, high |
| Diagnosticity | Low, medium, high |
| Confidence impact | Low, medium, high |

### 5. Use Admiralty-Style Source and Information Ratings

Rate source reliability and information credibility independently.

#### Source Reliability

| Rating | Meaning |
|---|---|
| A | Completely reliable |
| B | Usually reliable |
| C | Fairly reliable |
| D | Not usually reliable |
| E | Unreliable |
| F | Cannot be judged |

#### Information Credibility

| Rating | Meaning |
|---|---|
| 1 | Confirmed by other sources |
| 2 | Probably true |
| 3 | Possibly true |
| 4 | Doubtfully true |
| 5 | Improbable |
| 6 | Cannot be judged |

Important rule:

A reliable source can produce weak information.  
An unreliable source can occasionally provide true information.

Do not collapse source trust into claim trust.

### 6. Evidence Families

Group evidence into families before scoring.

Use these categories when applicable:

| Evidence family | Core question |
|---|---|
| Primary utterance evidence | What was actually said? |
| Authorial-position evidence | What has the person written or argued elsewhere? |
| Transformation evidence | How did wording mutate from source to coverage? |
| Timing evidence | Why did this happen now? |
| Narrative-contamination evidence | Did a related stronger claim contaminate interpretation? |
| Editorial-packaging evidence | What did the headline, lede, SEO title, social card, or summary do? |
| Amplification evidence | Who repeated the frame, and how fast? |
| Relationship / sway evidence | Who has access to the outlet, reporter, editor, sponsor ecosystem, or syndication chain? |
| Motivation / incentive evidence | Who benefits from this framing? |
| Means / capability evidence | Who could cause, pitch, amplify, or exploit this? |
| Beneficiary-specific evidence | Who used the narrative after publication? |
| Counterevidence | What weakens the suspicious explanation? |
| Alternative-cause evidence | What supports mundane explanations? |
| Prior-pattern evidence | Has the actor or outlet done similar things before? |
| Cost / risk evidence | Who could afford reputational, legal, or relationship risk? |

### 7. Build the ACH Matrix

Only after classification, score evidence against hypotheses.

Use this scale:

| Symbol | Meaning |
|---|---|
| ++ | Strongly consistent |
| + | Consistent |
| 0 | Neutral or not relevant |
| - | Inconsistent |
| -- | Strongly inconsistent |

The matrix should look like this:

| Evidence | H1 | H2 | H3 | H4 | H5 | H6 |
|---|---:|---:|---:|---:|---:|---:|
| E1. Original quote said “tasks,” not “jobs” | + | + | + | 0 | 0 | -- |
| E2. Headline escalated “tasks” into “all work” | ++ | ++ | + | + | + | -- |
| E3. Article was revived three months later | ++ | ++ | + | + | 0 | -- |
| E4. Downstream outlets copied the distorted frame | ++ | + | + | + | 0 | - |

### 8. Weight Diagnostic Evidence More Heavily

Not all evidence is equal.

Evidence is diagnostic when it helps distinguish between hypotheses.

Low diagnostic evidence:

> Many actors benefit from bad AI PR.

High diagnostic evidence:

> The article was updated three months later, explicitly tied to a new narrative peg, and changed “tasks” into “all white-collar work.”

Use this rule:

- Evidence that supports every hypothesis is weak.
- Evidence that contradicts one or more hypotheses is valuable.
- Evidence that distinguishes deliberate reframing from ordinary sloppiness is especially valuable.

### 9. Separate Motive, Means, Access, and Trace

Never treat motive as proof.

Use four separate tests:

| Category | Question |
|---|---|
| Motive | Who benefits? |
| Means | Who could shape, place, edit, syndicate, or amplify the story? |
| Access / relationship | Who has channels into the outlet, author, editor, sponsor network, or executive ecosystem? |
| Trace | What observable evidence shows that influence actually occurred? |

A hypothesis with motive but no means is weak.

A hypothesis with motive and means but no trace is plausible but unproven.

A hypothesis with motive, means, access, timing alignment, and amplification trace becomes materially stronger.

### 10. Require Counterevidence

For each leading hypothesis, state what would weaken it.

Example:

| Hypothesis | Evidence that would weaken it |
|---|---|
| Deliberate reframing | Archived headline showed the same framing in February |
| Sponsor influence | No sponsor or enterprise actor amplified or benefited from the story |
| Rival targeting | Similar framing was applied equally to all AI leaders, not specifically the target |
| Sloppy headline | Internal article body, headline, and downstream summaries all preserved the task/job distinction |

### 11. Produce Findings With Confidence Levels

Use confidence language carefully.

Suggested scale:

| Confidence | Meaning |
|---|---|
| Low | Evidence is thin, circular, or mostly inferential |
| Moderate | Multiple independent indicators align, but direct evidence is missing |
| High | Strong, corroborated, diagnostic evidence supports the judgment |
| Very high | Direct evidence exists and alternatives are strongly contradicted |

Never say “proved” unless direct evidence exists.

Use phrases like:

- “Best-supported hypothesis”
- “Most evidence-aligned explanation”
- “Structurally supported but operationally unproven”
- “Plausible but currently under-evidenced”
- “Strong motive, weak trace evidence”
- “Consistent with but not diagnostic of”

## Final Answer Template

Use this structure:

### Analytic Question

[State the question.]

### Key Judgment

[State the best-supported explanation and confidence level.]

### Hypotheses

| ID | Hypothesis |
|---|---|

### Evidence Classification Table

| ID | Evidence | Source type | Reliability | Credibility | Diagnosticity | Notes |
|---|---|---|---|---|---|---|

### ACH Matrix

| Evidence | H1 | H2 | H3 | H4 | H5 | H6 |
|---|---:|---:|---:|---:|---:|---:|

### Most Diagnostic Evidence

Explain which evidence items mattered most and why.

### Hypothesis Assessment

| Hypothesis | Assessment |
|---|---|

### Counterevidence and Falsifiers

List what would weaken or falsify the leading hypothesis.

### Bottom Line

State the conclusion plainly, separating evidence from inference.

## Style Rules

- Do not overclaim intent.
- Do not confuse “benefits from” with “caused.”
- Do not treat media sloppiness as proof of conspiracy.
- Do not treat lack of direct evidence as proof of innocence.
- Preserve distinction between:
  - stated words,
  - paraphrase,
  - headline,
  - interpretation,
  - downstream amplification,
  - AI-generated summary.
- When possible, use primary sources first.
- Highlight circular reporting and same-source echoes.
- Identify who profits, who can influence, and who left traces separately.

## Example Bottom-Line Language

Strong:

> The best-supported explanation is deliberate narrative reframing by the outlet under conditions of high AI-labor anxiety. This is supported by the stale timing, the task-to-job wording shift, the explicit new narrative peg, and downstream amplification. However, direct evidence of a named external actor pushing the article is not currently available.

Weaker:

> Several actors benefited from the framing, but beneficiary evidence alone is not diagnostic.

Useful distinction:

> This is not yet evidence of a coordinated campaign. It is evidence of a narrative supply chain in which a commercially useful distortion propagated through outlets and summaries.

## Common Failure Modes

Avoid these mistakes:

1. Building a suspicion table instead of an evidence matrix.
2. Ranking hypotheses before classifying evidence.
3. Treating motive as proof.
4. Treating source reliability and claim credibility as the same thing.
5. Ignoring stale timing.
6. Ignoring headline/body mismatch.
7. Ignoring downstream amplification.
8. Ignoring counterevidence.
9. Treating every citation as independent when outlets are copying each other.
10. Failing to distinguish task automation from job elimination.