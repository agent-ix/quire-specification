# Native simulation sampler definition

Definition identity: `quire.simulation.sampler/v1`.
This is a leaf definition selected by a simulation request
([FR-181](../../../spec/functional/runtime/FR-181-simulate-finite-executions.md))
and by statistical checking
([FR-408](../../../spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md));
it has no reverse dependency on the value or model definition roots.

## Draw values

The generator is counter-mode. `seed` is the run's `u64` seed, written as a
decimal string. `trace` is the trace's 0-based index within the run, `step`
the transition's 0-based index within that trace, and `choice` the 0-based
index of a choice within that step, each a decimal string of a `u64` value.
At a given `trace`/`step`/`choice`, `draw` is a decimal string of a `u64`
value that starts at 0 and increments after every draw value computed for that
choice, accepted or rejected.

Draw `draw` is `SHA-256(JCS{choice, draw, seed, step, trace})`, and `v` is the
draw value bytes read as a big-endian `u256`.

## Choices within a step

A step makes the choices of exactly one of the first two lists below, by
whether the run has a workload, and then, over a timed subject, the choices of
the third. The choices are made in the listed order.

1. Without a workload: one uniform choice over the canonically ordered enabled
   successor set of FR-181's exploration contract.
2. Under a workload: one weighted choice of the operation among the enabled
   operations, in canonical operation order, by the workload's weights; then
   one uniform choice of the scheduled identity among the operation's enabled
   scheduled identities, in canonical transition order; then one weighted
   choice per random parameter, in parameter declaration order, over its
   support in canonical value order;
3. Over a timed subject: one choice per racing identity with a given delay,
   in canonical transition order, and one uniform choice per tie-break
   position ([FR-420](../../../spec/functional/analysis/FR-420-sample-stochastic-timed-models.md)).

A choice that draws takes the next choice index, starting from 0 at each
step. A choice over one item (`n = 1`), such as one enabled operation, one
enabled scheduled identity or a one-value support, selects that item, draws
no draw value and takes no choice index: the next choice that draws takes the
index the one-item choice would have taken.

## Uniform and weighted selection

A weighted choice over items with positive exact rational weights scales the
weights by the least common multiple of their denominators to positive
integers `w_0, …, w_{k−1}` with sum `n`; a uniform choice over `k` items has
`w_i = 1` and `n = k`. `v` is accepted when `v < n · ⌊2^256 / n⌋`, and the
choice selects the least index `i` with `v mod n < w_0 + … + w_i`; a rejected
`v` increments `draw` and recomputes the draw value. A total `n > 2^256` leaves no
accepted `v` (the bound `n · ⌊2^256 / n⌋` is 0), so the run stops at the first such
choice, before any draw and with no draw value or choice index taken, and settles
`unsupported` with cause `total-above-draw-range{total}`, `total` being `n` as a
decimal string. The stop applies to the whole sampling run, not only to the
current trace, and retains `total` only; the cause names no trace, step or choice
position. A total of exactly `2^256` is in range: its bound is `2^256`, so the
draw-0 draw value is accepted. The sampler never widens the draw. `n = 1` selects index 0 with
no draw value and no choice index. An empty item set ends the trace without a draw.

## Grid draws for delays

A `discrete` delay is a weighted choice over its delays conditioned on the
window, by the weights of the delays inside the window. A `uniform`,
`uniform[a, b]` or `exponential` delay with grid exponent `q >= 1` reads the
grid value `u = (v mod 2^q) / 2^q` from one accepted draw value when `q <= 256`
(every `v` is accepted), and from the big-endian concatenation of the draw values
at `draw = 0, 1, …` reduced modulo `2^q` when `q > 256`. It returns the
distribution's inverse distribution function, conditioned on the window, at
`u`: for a uniform family, the point at length `u · |W|` along the window `W`
taken in increasing order; for `exponential(λ)`, the point `x` of `W` with
`Pr(X <= x | X ∈ W) = u`, computed with FR-408's rational bounds and rounded
up to a multiple of `2^-q` of the model's unit. Every sampled delay is an
exact rational.

## Identity

The recorded generator is this definition's identity. Two runs recording
equal seeds and the same sampler identity draw the identical sequence and
therefore reproduce identical traces (FR-181-AC-2). Changing the seed changes
the run identity.
