# PRD-002: F-2 Ball physics, pin field, and chucker-driven spins

**Status:** Draft
**Created:** 2026-05-25
**Intent linkage:** C-1, C-6, C-8, C-9, C-10, C-13, C-14, **C-15**, **C-16**
**Stories:** _(populated as stories are cut)_
**ADR Required:** yes (physics integration scheme; collision broadphase)

## Feature F-2: The pachinko playfield — balls, pins, launcher

PRD-001's F-1 implemented the reel layer + state machine + reach engine. That layer is correct (R-1..R-13 green) but missing the ball-and-pin theatre that the pachinko-expertise skill identifies as core to pachinko's identity. The audit at `audits/audit-001-cabinet-slot-vs-pachinko-2026-05-25/` documents why. F-2 adds the missing layer and re-wires F-1's chucker input so reel spins are driven by simulated ball-chucker entries instead of direct keypresses.

---

### Rule R-24: Ball entity has position, velocity, radius, and lifecycle state

Per [C-15]. The game layer (NOT pachinko-core, which stays pure per C-1) gains a `Ball` struct.

**Example: Ball construction**
```
Given a Launcher fires a ball
When the ball is spawned
Then the ball has (x, y) position within the launch chute
  And a velocity vector pointing upward-and-leftward (into the playfield)
  And a fixed radius of 6 px at native canvas resolution
  And lifecycle state = InFlight
```

**Example: Ball lifecycle states**
```
Given a Ball
When the ball is updated
Then its state is one of: InFlight | InChucker | InTulip | Lost
  And transitions are: InFlight → {InChucker, InTulip, Lost}
  And no other transitions exist (terminal states do not re-spawn the ball)
```

---

### Rule R-25: The launcher fires balls at a rate proportional to launch-power input, between idle and max-rate

Per [C-16]. Hold SPACE / left mouse = launch.

**Example: Launch rate**
```
Given the launcher is idle (no input)
When the player holds SPACE for 1 second
Then 5 ± 1 balls are spawned in that second
  (target rate 5 balls/sec, matching real-machine knob max ~100/min in JP regulation)
```

**Example: No input → no balls**
```
Given no input is held
When 1 second elapses
Then 0 balls are spawned
```

---

### Rule R-26: Balls collide with pins via 2D circle-circle elastic resolution

Per [C-8] (60 fps budget) and [C-15].

**Example: Collision deflects velocity**
```
Given a Ball at (100, 50) with velocity (0, +200) px/sec moving downward
  And a Pin at (100, 100) with radius 3 px
When the ball center reaches a distance equal to (ball.r + pin.r) from the pin
Then the ball's velocity is reflected about the contact normal
  And |v_new| ≥ 0.85 * |v_old| (some energy loss is acceptable, 100% conservation is not required)
  And the ball does not become embedded in the pin (no overlap > 0 after resolution)
```

**Example: Two balls don't collide with each other in v0.2**
```
Given two balls in flight
When they are checked for collisions
Then only ball-pin and ball-pocket collisions are checked, not ball-ball
  (Ball-ball collisions are a v0.3 candidate; deferred to keep collision cost O(n_balls * n_pins))
```

---

### Rule R-27: Pin field is a configurable grid of ~80 pins arranged in a diamond pattern in the playfield zone

Per [C-12] (config-driven) and [C-15].

**Example: Pin count and arrangement**
```
Given the canonical pin field config
When the playfield renders
Then 80 ± 5 pins are drawn
  And they are arranged in a diamond-lattice pattern in the LCD-overlap zone
  And the chucker (HESO) is centered below the lowest row of pins
  And there are gaps in the pattern that bias ball trajectories toward / away from the chucker
```

**Example: Reconfiguring the pin field**
```
Given the pin config is edited to add 10 pins or remove 10 pins
When the game restarts (no code change required)
Then the new pin layout is used
  And the change does not require a rebuild
  (Aligns with C-12; supports future "nail adjustment" 釘調整 gameplay.)
```

---

### Rule R-28: A ball entering the chucker emits a ChuckerEntry event that triggers exactly one `session.pull_chucker()`

Per [C-16]. The direct SPACE-to-spin path is REMOVED.

**Example: Chucker entry triggers a single spin**
```
Given a ball is InFlight with the chucker rim at y=Y
When the ball's center crosses Y with |x - chucker_cx| < chucker_radius
Then the ball's state becomes InChucker
  And exactly one ChuckerEntry event is emitted (idempotent — re-entering the chucker by a single ball does not re-fire)
  And session.pull_chucker() is called once with that event
  And the ball's lifecycle ends (InChucker is terminal)
```

**Example: SPACE no longer directly spins reels**
```
Given the session is in CabinetState::Base
When the player presses SPACE without holding it long enough to fire a ball
  OR before any ball has reached the chucker
Then session.pull_chucker() is NOT called
  And the reels do not spin
```

---

### Rule R-29: The chucker hit rate is calibrated so ~25–40 balls returned per 100 fired (PRD-001 §4 base game rate ベース)

Per [C-2] / PRD-001 §4. Tune via pin layout.

**Example: Base game rate hits target window**
```
Given the canonical pin field config
When 5000 balls are fired with no jackpot (RNG forced)
Then between 1250 and 2000 of those balls enter the chucker
  (25–40% per PRD-001 §4 "BASE = 35", with ±15% tolerance for layout iteration)
```

---

### Rule R-30: Lost balls fall off the bottom of the playfield without triggering anything

Per [C-15].

**Example: A ball that doesn't hit chucker disappears**
```
Given a ball is InFlight and its y position exceeds playfield_bottom_y
When the next physics tick runs
Then the ball's state becomes Lost
  And no ChuckerEntry event is emitted
  And the ball is removed from the simulation
```

---

### Rule R-31: Frame time stays within NFR-1's 16.6ms p99 budget at 50 simultaneous balls + 80 pins

Per [C-8] (60 fps sustained).

**Example: Performance under load**
```
Given 50 balls are simultaneously InFlight
When the physics + collision step runs once
Then the elapsed wall time is ≤ 4ms on the macOS aarch64 reference machine
  And the resulting frame time is ≤ 16.6ms (p99) including render
```

---

### Rule R-32: Ball-tray HUD shows balls-fired, balls-returned (chucker), and effective rate

Per [C-15]. Pachinko has visible ball economy; the player should see it.

**Example: HUD values update each frame**
```
Given the player has fired 100 balls and 33 entered the chucker
When the HUD is rendered
Then it shows "FIRED 100 / RETURNED 33 / RATE 33%"
  And the values update on every ChuckerEntry / Lost transition
```

---

### Rule R-33: The launcher knob has a visible draw + a hold-state indicator (proxy for real knob deflection)

Per [C-15].

**Example: Knob state**
```
Given the player is holding SPACE
When the launcher renders
Then the knob graphic shows a deflected / engaged state (color or rotation)
  And on release it returns to its rest state within 0.2s
```

---

## Anti-patterns to avoid

- **Adding ball physics to `pachinko-core`.** F-2 is a game-layer feature. `pachinko-core` stays C-1-pure: no balls, no pins, no rendering, no I/O. The math layer's interface to F-2 is one new function call (`session.pull_chucker()` already exists; F-2 just changes who calls it).
- **Re-rolling chucker-entry as a probability.** The chucker entry is now an *emergent* probability of the physics — pin layout determines the rate (R-29). Hardcoding a "% chance per ball" defeats the purpose; that's just a slot with extra steps.
- **Skipping the broadphase.** At 50 balls × 80 pins = 4000 checks per frame, a naïve O(n*m) loop is fine for v0.2. Add a spatial hash only if R-31 fails.
- **Coupling ball state to reel state.** A ball entering the chucker during a jackpot or reach should be queued or dropped (per the game's design rules), but the ball physics layer must not directly inspect `CabinetState`. The integration point is `session.pull_chucker()` which already returns `NoChange` if the state machine isn't in Base/KakuhenBase.

## Index

| ID | Title | Status | Intent linkage |
|---|---|---|---|
| F-1 | MVP Pachinko Cabinet (reel + math) | Draft / shipped v0.1 | C-1..C-12 |
| F-2 | Ball physics, pin field, launcher | Draft (v0.2 target) | C-1, C-6, C-8, C-9, C-10, C-13, C-14, **C-15, C-16** |
