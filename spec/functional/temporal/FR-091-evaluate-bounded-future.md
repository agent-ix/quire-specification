---
id: FR-091
title: "Evaluate bounded future temporal operators"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-008
    type: implements
  - target: ix://agent-ix/quire-specification/FR-090
    type: references
  - target: ix://agent-ix/quire-specification/FR-250
    type: references
  - target: ix://agent-ix/quire-specification/FR-253
    type: references
  - target: ix://agent-ix/quire-specification/FR-254
    type: references
  - target: ix://agent-ix/quire-specification/FR-255
    type: references
  - target: ix://agent-ix/quire-specification/FR-258
    type: references
  - target: ix://agent-ix/quire-specification/FR-161
    type: references
  - target: ix://agent-ix/quire-specification/FR-367
    type: references
  - target: ix://agent-ix/quire-specification/FR-160
    type: references
  - target: ix://agent-ix/quire-specification/FR-094
    type: references
---
## Description

The temporal evaluator shall apply the inclusive interval and boundary rules of
the selected profile to every admitted bounded future operator without
shortening the interval to the supplied file.

## Inputs

A temporal subject from [FR-090](FR-090-select-temporal-profile-and-clock.md),
an admitted evaluation anchor, total Boolean predicate valuations, and validated
progress/closure information for the subject's required observations.

## Outputs

A `satisfied`, `violated` or `pending` temporal truth disposition with the exact
decision scope, settlement basis and decision-support identities needed by the
shared result contract, or a separate `indeterminate`, incomplete or refused
evaluator outcome with the affected clause, interval and missing premise. A non-completed
evaluator outcome carries no truth fact at this boundary; the canonical result
adapter represents that absence as truth `unavailable` with settlement basis
`unavailable`.

## Behavior

For the two false-extension profiles, let the admitted positions be
`D = {0, ..., n-1}`. At every integer position outside `D`, an atomic predicate
is false while Boolean constants retain their values. At anchor `i`, for
inclusive integers `0 <= a <= b`, the evaluator shall use:

| Operator | Exact false-extension meaning |
| --- | --- |
| `eventually[a,b] p` | Some offset `d` in `[a,b]` makes `p` true at `i+d` |
| `always[a,b] p` | Every offset `d` in `[a,b]` makes `p` true at `i+d` |
| `p until[a,b] q` | Some `d` in `[a,b]` makes `q` true at `i+d`, and every `e` in `[a,d)` makes `p` true at `i+e` |
| `p release[a,b] q` | The Boolean dual of `not p until[a,b] not q` under the same profile |

The lower-bound convention for `until` is deliberate: positions before `a` do
not satisfy its left operand. It matches `mltl.closed-trace/v1` and shall not be
replaced by a different unbounded-until convention under the same identity.

Temporal `not`, `and`, `or` and `implies` shall evaluate pointwise at every
offset traversed by their enclosing temporal operator. They shall not first
reduce an operand over the entire interval and then apply one untimed Boolean
operation to that aggregate.

For the timestamped finite-window profile, an eligible future instant has an
admitted clock distance in `[a,b]`. Existential and universal operators quantify
only eligible admitted instants; `until` uses the admitted order and requires
its left operand at every eligible earlier instant from the lower bound to the
witness. For an activation at admitted tick `t0` of a clause whose nested interval
operators reach `H` ticks into the future (the greatest sum of upper bounds of
the future interval operators along a nesting path; past operators add nothing,
since a look-back needs only the retained history of
[FR-092](FR-092-evaluate-bounded-past.md)), the activation is *established*
exactly when the observation authority asserts completeness through `t0 + H` and
the progress watermark `W` satisfies `W >= t0 + H`. An established activation is evaluated over its eligible admitted
instants only, with no synthetic atom after closure, and its result is definite
true or false, including the empty existential (false) and empty universal
(true) window. Before establishment, or when the input is incomplete, the
evaluator applies the possibility rules of
[FR-160](FR-160-evaluate-uncertain-revisioned-time.md) over every admissible
continuation: the result is definite when every possibility agrees (a valid
early witness or counterexample, or a missing fact outside the decision
support); otherwise it is `pending` while `W < t0 + H` (declared progress could
resolve it) and `indeterminate` once `W >= t0 + H` (closed evidence still
unresolved: a missing or conflicting valuation, a completeness assertion short
of `t0 + H`, or missing history, which a look-back-only clause reports at once because its `H` is 0). `incomplete` is reserved for a missing
required current valuation at the anchor and for resource exhaustion. Two
instants with equal clock distance are ordered only by an admitted order key; the
eligible set, `until`/`since` and their duals use that order and refuse without
it. `release` remains the Boolean dual.

The evaluator shall return `satisfied` or `violated` for an open decision scope only when the
selected profile proves that every admitted continuation preserves it. Such a
result shall identify `decisive-witness` for `satisfied` or
`decisive-counterexample` for `violated` and shall retain the exact valuation,
capture, clock, progress and profile facts that form its decision-support set.
Closure of that decision scope authorizes `closed-scope`; it does not imply
closure of the surrounding execution. An undecided open scope shall remain
pending with settlement basis `unsettled`.

For the false-extension profiles, the evaluator shall represent missing required
current valuations or an incomplete closed input as incomplete rather than
pending future time or `violated`; for the timestamped finite-window profile an
incomplete closed input is `indeterminate` as stated above and `incomplete` keeps
only the two cases reserved there.
The evaluator shall make truth unavailable when a fact is missing inside the
exact decision-support set. The evaluator shall retain a fact missing outside a
completed decision-support set in completeness and any dependent
adequacy/package result without changing an already settled temporal truth.

Where a bounded temporal profile is selected, the evaluator shall refuse an unbounded interval. Where the infinite-trace profile is selected, the evaluator shall admit the unbounded always, eventually, until and release operators with the meaning [FR-161](FR-161-analyze-infinite-trace-properties.md) states. Where the infinite-trace profile is selected, the evaluator shall admit the interval operators above, nested in any order with unbounded ones, with the offset meaning of the false-extension table read over the infinite trace, where every future position exists and no closed-boundary rule applies ([FR-367](FR-367-evaluate-interval-operators-under-infinite-trace.md)).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-091-AC-1 | Independent truth tables cover all four operators, every inclusive endpoint, zero-width intervals, nonzero lower bounds, nesting and short traces. | Test (TC-111) |
| FR-091-AC-2 | On a one-position closed trace where atomic `p` is true, `always[0,1] p` is false under false-extension and true under a complete finite-window interpretation; the profiles cannot share one result identity. | Test (TC-112) |
| FR-091-AC-3 | `p until[1,2] q` may be true with `p` false at the anchor when `q` is true at offset one, pinning the selected lower-bound convention. | Test (TC-111) |
| FR-091-AC-4 | A decisive open decision scope may settle early only with the matching decisive basis, complete exact decision support and proof over every admitted continuation; every other unclosed future remains pending, while, under the false-extension profiles, a closed incomplete input reports incomplete rather than applying false extension. | Test (TC-116) |
| FR-091-AC-5 | Timestamped empty existential/universal window truth is emitted only after complete progress through the inclusive upper endpoint; an eligible event exactly at the deadline participates before settlement; an incomplete input never admits that truth (FR-091-AC-14). | Test (TC-115) |
| FR-091-AC-6 | Where a bounded temporal profile is selected, unbounded intervals return a non-Boolean disposition; unavailable total valuations and exhausted semantic work return non-Boolean dispositions under every profile. | Test (TC-118) |
| FR-091-AC-7 | On a one-position closed trace with `p` true, `always[0,1] not holds(p)` is false and `eventually[0,1] not holds(p)` is true under false extension, distinguishing pointwise temporal negation from an untimed reading. | Test (TC-111) |
| FR-091-AC-8 | Where the infinite-trace profile is selected, the unbounded future operators evaluate under their FR-161 infinite-trace meaning and return that requirement's conclusive or non-conclusive disposition. | Test (TC-200) |
| FR-091-AC-9 | Where the infinite-trace profile is selected, `eventually[a,b]`, `always[a,b]`, `until[a,b]` and `release[a,b]` evaluate with their offset meaning over the infinite trace, including nested under an unbounded operator, and no position reads a closed-boundary value. | Test (TC-298) |
| FR-091-AC-10 | Over stamps 0, 2, 5 ms with `W` 5 and completeness through 5, `always[0,5] holds(p)` is false at position 0 when `p` is true at 0 and 5 and false at 2 and true when `p` is true at all three; `eventually[0,1] holds(p)` with `p` true only at stamp 2 is false and `eventually[0,2] holds(p)` is true. A position stamped 6 is outside `[0,5]` and one stamped 5 is inside (`FWV-01` to `FWV-05`). | Test (TC-980) |
| FR-091-AC-11 | One position stamped 0 with `p` true, `W` 1 and completeness through 1: `always[0,1] holds(p)` is true, with no synthetic false atom after closure, where false extension gives false. With `W` 3 and completeness through 3, `eventually[2,3] holds(p)` is false and `always[2,3] holds(p)` is true, the empty windows of an established activation (`FWV-06` to `FWV-08`). | Test (TC-980) |
| FR-091-AC-12 | Stamps 0 and 1 with `p` false at both, `W` 1 and completeness through 1, `eventually[0,5] holds(p)` is `pending` with deadline 5; the same positions with `W` 5 and completeness through 5 give false with no further observation. With an open scope, `p` true at stamp 1 and `W` 1 gives true (decisive witness), and `always[0,5] holds(p)` with `p` false at stamp 1 gives false (decisive counterexample) (`FWV-09` to `FWV-12`). | Test (TC-980) |
| FR-091-AC-13 | Stamps 0, 2, 5, `eventually[0,5] holds(p)`, `W` 5 and completeness through 5: `p` false at 0 and 5 and absent at 2 is `indeterminate` (missing valuation at position 1); `p` true at 5 and absent at 2 is true; `p` both true and false at stamp 2 is `indeterminate` (conflicting valuation at position 1); the absent case with `W` 3 is `pending` with deadline 5. None is false (`FWV-13` to `FWV-16`). | Test (TC-980) |
| FR-091-AC-14 | With `p` true at stamp 0, a closed scope, `W` 5 and completeness not asserted, `always[0,5] holds(p)` is `indeterminate` (incomplete input), and with completeness through 5 it is true; with completeness not asserted `eventually[2,3] holds(p)` is `indeterminate` and not false, and with completeness through only 2 and `W` 3 it is also `indeterminate` (`FWV-17` to `FWV-20`). | Test (TC-980) |
| FR-091-AC-15 | An eligible instant at the inclusive deadline participates (`eventually[0,3] holds(q)` with `q` true at stamp 3 and `W` 3 is true) and one after it does not (`q` true only at stamp 4 gives false). A single position with `p` false and `W` 3 gives false for `eventually[0,3] holds(p)` with no later observation. A half-open watermark `[.., 3)` without an FR-289 mapping is the inclusive watermark 2 and leaves that window `pending` (`FWV-21` to `FWV-24`). | Test (TC-980) |
| FR-091-AC-16 | Stamps 0, 1, 1 without order keys refuse `until`, `release`, `since` and `triggered` at the third position while `eventually[0,1]` evaluates; keys from one order authority admit `until`, keys from two authorities refuse it, and reversing the insertion order with keys unchanged leaves the admitted order unchanged (`FWV-31` to `FWV-38`). | Test (TC-980) |
| FR-091-AC-17 | Stamps 3 and 4, `once[0,4] holds(p)` at the second position with `p` false at both: a history-cutoff lower boundary gives `indeterminate` (missing history) and an authoritative origin gives false; `p` true at stamp 3 gives true for both (`FWV-25` to `FWV-28`). | Test (TC-980) |
| FR-091-AC-18 | With counter cover and uncertainty 0, a request at tick 0 and `holds(req) implies eventually[0,3000] holds(ack)`, an acknowledgement at tick 2998 is true, at 3000 `indeterminate` (the cover straddles the bound) and at 3001 false; under exact cover the three are true, true and false. The neighbouring ticks and the same-tick pair are `TCV-01` to `TCV-15` of FR-160 (`FWV-39` to `FWV-44`). | Test (TC-980) |
| FR-091-AC-19 | `eventually[0,2] holds(p)` activated at each position, over stamps 0 and 10 with `p` false at both, `W` 10 and completeness through 10: the activation at 0 is definitely false and the one at 10 is `pending`, so the outcome is false at position 0; with `p` true at stamp 0 the outcome is `pending` at position 1 with deadline 12, not true (`FWV-29`, `FWV-30`). | Test (TC-980) |

## Dependencies

- [FR-090](FR-090-select-temporal-profile-and-clock.md).
- [FR-160](FR-160-evaluate-uncertain-revisioned-time.md) for the possibility
  sets, the counter-tick cover and the `pending`/`indeterminate` split.
- [FR-094](FR-094-interpret-progress-history-and-closure.md) for the inclusive
  watermark. The vectors are `proposals/finite-window/finite-window-vectors.json`.
- Total Boolean predicate projection from
  [contract-IR issue #63](https://github.com/agent-ix/quire-contract-ir/issues/63).

## Status

Accepted by FS04 for bounded future meaning and non-Boolean failure outcomes.
Evaluator and bridge conformance remain separately tracked.
