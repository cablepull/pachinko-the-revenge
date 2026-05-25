# User feedback — 2026-05-25 — "this is a slot machine"

Direct quote from the user on inspecting the deployed Iteration 1 cabinet:

> Is this a version of pachinko that doesn't use balls? It doesn't seem right? It seems like a slot machine.

## Context

The user is the project owner (`cablepull`). They had just played the live build at
`https://cablepull.github.io/pachinko-the-revenge/iteration1/`, which features:

- A 3-reel LCD display
- An "attacker" door region below
- A chucker labelled HESO (purely cosmetic — bypassable by SPACE)
- A data-lamp HUD
- No visible balls, no pin field, no launcher knob, no ball cascade

## Why the feedback is load-bearing

The pachinko-expertise skill (`.claude/skills/pachinko-expertise/SKILL.md`) §1 is
explicit:

> Pachinko is a vertical pinball-meets-slot hybrid. Balls (11mm steel) fall through
> a pin field; landing in the start chucker (中央ヘソ, *heso*) triggers a digital
> reel spin (the "digi-pachi" / CR machine layer). The pin field is mostly
> aesthetic theater — the real game is the reel layer the chucker gates access to.

The v0.1 build implements the reel layer (correctly, with green math) but *omits
the pin field theatre*. A user without prior pachinko exposure has nothing
distinguishing the cabinet from a slot machine.

intent.md C-10 requires:

> A 常連-class consultant must identify the audio identity as pachinko from sound
> alone within 30 seconds.

We have not validated C-10. The user's feedback is the first informal data point —
and it suggests *visual* identity, not just audio, is also failing for non-regulars.

## Implication

Iteration 2 must add the ball-physics + pin-field layer to bridge the
"slot machine"-shaped output back to authentic pachinko grammar.
