# FRETish 3.1 output profile

Selected identity `quire.output.fretish31/v1`. Target
reference: NASA FRET release `v3.1.0` and its Classic FRETish
scope/condition/component/shall/timing/response grammar and documentation.

## Admitted fragment

The profile generates Classic, non-probabilistic FRETish with an exact component
catalog binding, optional explicit scope and condition, mandatory `shall`, a
selected timing phrase and one total Boolean response. Response/condition
expressions contain mapped Boolean signals, `!`/`&`/`|`/implication, equal-type
comparison and bounded checked scalar arithmetic supported by the selected FRET
variable catalog.

The initial temporal fragment admits:

- a total state predicate under an explicit universal or selected mode scope
  using `always`;
- same-sample response using `immediately`;
- fixed-sample bounded eventual response over native interval `[0,b]` using
  `within b` only when sample period/unit, trigger instant and complete decision
  window have exact correspondences; and
- one-step response using `at the next timepoint` only for exact native `[1,1]`
  fixed-sample semantics.

## Conditional and unsupported meaning

Every temporal mapping is conditional on the retained native clock, unit,
sample period, trigger/capture, history and completeness facts that FRETish text
does not itself carry. Event-position counts are not elapsed time;
timestamped-event ordering is not a fixed-sample clock.

Nonzero lower bounds other than `[1,1]`, bounded always/until/since, past
operators, timestamped-event windows, open or incomplete decision windows,
protocol/choreography structure, probabilistic FRETish and any source fact with
no exact variable/component binding are unrepresented or refused. Timing-free
requirements receive no invented temporal phrase.

## Output and qualification boundary

FRETish output and any FRET-generated formula are derived artifacts, not native
source or the FR-095 TL bridge. FRET parsing/formalization under `v3.1.0` may be
recorded only as a separately qualified structural observation. It cannot erase
conditions, turn finite-trace behavior into native MLTL correspondence or
establish native truth.
