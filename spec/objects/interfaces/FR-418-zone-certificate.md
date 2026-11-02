---
id: FR-418
title: "Zone certificate and its checker"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/TC-371
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-372
    type: verifies
  - target: ix://agent-ix/quire-specification/US-008
    type: implements
  - target: ix://agent-ix/quire-specification/FR-416
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-417
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-419
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-482
    type: references
---

# FR-418: Zone certificate and its checker

## Description

When a zone-based provider proves a timed claim, the deadlock-freedom item or
the time-lock-freedom item over a timed subject, it SHALL emit a zone
certificate with exactly the members below, and the checker SHALL accept it,
recomputing every successor with its own difference-bound-matrix code in
exact arithmetic, before the item settles `proved` with basis `zone-certified`.

## Properties

The value is a reachability certificate, extended to a Büchi emptiness
certificate for liveness forms and the time-lock-freedom item, over zones.

### Zones

A zone is a conjunction of constraints `x_i − x_j ≺ c` over the subject's
clocks and a reference clock fixed at 0, `≺` one of `<` and `<=`, held as a
canonical difference-bound matrix (DBM). Every constant of the model and the
claim is scaled once to an integer by the least common multiple of the
denominators, so each DBM bound is an arbitrary-precision integer with its
strictness, or `∞`.

### Reachability certificate

For a safety form (TT-1, TT-2, TT-3, the deadlock-freedom item):

| Member | Content |
| --- | --- |
| `identity` | The item's obligation identity, the structural `{node_id, model, ordinal}` (FR-479) binding the proof to the exact subject and claim. For a task-automaton proof, this binding selects the checked schedulability claim through FR-322's `schedulability_claims` row and FR-482 body, and therefore its policy; policy is not supplied as an independent certificate or `task_product` value. |
| `scale` | The common denominator the constants were scaled by. |
| `bounds` | The per-clock lower and upper bounds `L` and `U`, per discrete state, used by the simulation test. |
| `nodes` | A finite list of nodes `(s, q, Z)`: discrete state key, claim-automaton state, canonical DBM. For a task-product certificate, `s` is the complete source-plus-scheduler product key modulo the finite live-job renaming below. |
| `edges` | For each node and each recomputed source transition or finite task-transition template enabled from it, the index of a target node. |
| `initials` | Nonempty. The position in `nodes` of the initial node of each initial state of the subject, in the subject's initial-state order (FR-361): the node that covers that state's initial symbolic state. A form with one initial state lists one position. QSL FR-244 reads its single `initial` as `initials[0]`; the wire carries no `initial` member. |
| `admitted` | The ADR-026 TD-6 witness: a start node (in a refinement form, any positioned `initials[i]`, so a time-locked first initial state does not decide it) and a path of certificate edges to a quiescent node or to a node of a component with a cycle of positive total delay (QSL FR-244 `admitted`). |
| `refinement` | Present only for the safety half of a timed refinement (QSL FR-430): the refinement part below. |
| `task_product` | Present for a fixed-work task-automaton proof or a verified no-reachable-release proof: the `bounded-subtraction-diagonal/v1` abstraction name, a structured `time_progress` closure method, each task's enforced arrival kind, minimum interarrival, nominal relative deadline and maximum candidate slot bound, each demand interval, auxiliary clock ceilings, diagonal constraints and typed `TaskTransitionTemplate` subtractions. Fixed-work closure requires equal `work_lower`/`work_upper`; no-reachable-release closure permits intervals only when the strict reader proves no checked release binding is reachable. |

QSL FR-244 names `identity`, `bounds` and a node's `automaton` as `obligation`,
`lu_bounds` and `automaton_state`; the wire spellings here are the ones the
FR-331 schema carries and QSL FR-245 writes in its loci. A node is marked
`locked` (QSL FR-244 "Locked nodes") by a `locked` member present, as `true`,
on exactly that node.

For a task-automaton certificate, the reader resolves `identity` to the exact
checked task claim and derives fixed-priority or EDF policy from that claim.
The checked claim is the FR-322 `schedulability_claims` identity row joined to
the admitted FR-482 body. The IR reader owns that typed carrier read only.
This FR-418 reader is the QSL checker: it joins the authenticated claim to
the selected timed model's source release declarations and recomputes the
source-plus-scheduler successors. It does not ask IR to interpret
`TaskTransitionTemplate` or run a second scheduler evaluator.
It independently recompiles the task product and recomputes every release,
ready queue order, dispatch, preemption, service and completion successor
under that policy before it accepts coverage or derives positive-delay SCCs.
The certificate cannot select or override policy through node state, an edge,
or `task_product`; no duplicate unauthenticated policy value is required in
the task-product carrier. For EDF, absolute deadline controls order and ties
use the FR-419 canonical-task-name/source-activation/occurrence rule; source
declaration order and declared priority do not affect ties. A checked declared
priority is retained as job provenance but ignored for EDF order.

### Finite task-transition identity

`TaskTransitionIdentity` (FR-419) remains the exact identity in replayed task
steps and timed witnesses: it carries the zero-based activation occurrence,
the slot generation, selected jitter and absolute deadline. Those values are
concrete prefix data and are never renumbered in a witness.

An FR-418 finite task-product certificate uses a `TaskTransitionTemplate` for
each task edge and bounded-subtraction row. It carries `action`, `taskset`,
`task`, `job_rank`, `slot`, the source `release` transition, the optional
`source_transition`, and `priority` exactly when the checked task declares
one. `job_rank` is the zero-based position of that job among the task's live
jobs, ordered by source activation transition identity and then the exact
occurrence in the path represented by the source node. `slot` binds that rank
to one of the task's finite candidate slots. On an activation edge, rank and
slot name the newly live job in the target node; on every other edge they name
the affected live job in the source node. The template carries no global job
occurrence, slot generation, demand, jitter or absolute deadline. The reader
derives those from the checked task, source path and clock region. It refuses
an out-of-range rank or slot, a slot bound to another live job, or a template
whose action is not enabled.

The task state key and every edge template identify active jobs by a finite,
canonical bijection between each task's live jobs and its ranks and slots.
When a job completes, its exact identity leaves the product state; a later
activation may reuse its finite rank and slot. This is capture-avoiding
renaming: no live job changes identity, and the equivalence preserves each
live job's task, phase, running/ready order, clock association and source
activation order. Completed-job occurrence and generation counters do not
enter the finite closure because no future transition reads them after that
job has left the state. The reader recomputes successors modulo this
renaming, and derives positive-delay SCCs only from that closed quotient.
When a concrete counterexample is replayed, FR-364 instantiates every template
along its actual path, counts source activations from the prefix, and checks
the exact occurrence, generation, jitter, deadline and service values. A
finite certificate therefore never asserts an unbounded counter value or
stands in for replay evidence.

For a task-automaton certificate, each node's `state` is the complete
source-plus-scheduler product key. Its DBM uses the canonical clock order:
source clock places, task arrival clocks, pending-release jitter clocks, then
each task slot's service and nominal-deadline clocks. The checker recompiles
the task product and requires `task_product` to match its task-slot bounds,
task-set arrival and demand intervals, ceilings, diagonal constraints,
subtraction table and finite-product gate. Fixed-work `time_progress` is
`{method: "closed-finite-product/v1", live_job_cap, closure:
"recompute-all-reachable-successors/v1"}`. Neither the method nor cap is a
producer assertion of closure. The reader recompiles the source/task product
and exhaustively explores every reachable source control, refined clock
region, scheduler control, release/jitter phase and queue state within the
declared finite cap. It recomputes periodic/sporadic activation, jitter,
service, completion and feedback successors, and accepts exact mode only if
every reachable successor is represented and no attempted allocation
exceeds the cap. It requires `live_job_cap` to equal the sum of per-task
`max_jobs`, then derives positive-delay SCCs from this closed graph; a
source-only no-time-stops claim or an overflow edge cannot satisfy the gate.

When the subject's source graph has no reachable checked task-release binding,
the certificate may instead use
`{method: "closed-no-release-product/v1", live_job_cap: 0, closure:
"recompute-all-source-successors-no-task-release/v1"}`. The reader
recompiles the task model and closes every source successor from every initial
state, and accepts this method only when no release or task-activation edge is
reachable. Demand intervals are immaterial in this case, so queue bounds may
retain unequal `work_lower` and `work_upper`, and scheduler-clock ceilings,
diagonal constraints and subtractions are empty. The graph still uses the
subject's source clocks and must satisfy the ordinary timed closure and
time-divergence checks. If either closure fails, the task remains admitted on
FR-419's bounded-search path. A `queue_growth` continuation may replay one
concrete divergent refutation but does not establish completeness or permit
`proved`.

For fixed-work closure, each task `work_lower = work_upper` and the reader
verifies these equal the normalized `BCET` and applicable WCET. For
no-reachable-release closure, the reader verifies both interval endpoints
against the checked task declaration and proves that no task release is
reachable, so neither endpoint is selected or used as a job's demand.
Each task's slot bound is `ceil(D_i/T_i)+1` from the compiled enforced nominal
activation spacing; maximum jitter does not extend the deadline window
because deadlines start at nominal activation. The checker refuses a bound
not derived from that task's resolved periodic/sporadic arrival, `T_i`, `D_i`
and `J_i`.
It checks each ceiling and exact scaled constant, and proves that every
reachable subtraction of `c` from `x` is guarded by `c <= x <= M_x`, yields a
non-negative value, and uses the named clock and transition. The task product
uses finite refined regions: for each clock up to its ceiling, equivalence
preserves integer part, zero fractional parts and fractional-part order; every
declared diagonal predicate keeps its truth value; values above a ceiling are
represented by their saturated region. The bounded-subtraction side condition
makes subtraction by a fixed integer preserve this equivalence. The checker
uses this refined-region relation instead of aLU for task-product successor
coverage.

For a fixed-work task proof, the finite scheduler control, FR-419's finite job
slots, finite clock ceilings and finite refined-region partition make the
product graph finite. A task-product certificate covers its entire reachable
graph. A bounded search frontier is not a `ZoneCertificate` and cannot prove
absence of deadline misses. A finite-slot overflow is not a product state or
certificate node; it stops certification and routes to replayable incomplete
search evidence. A reachable deadline-miss node is permitted in this proof certificate only
when it is `locked` and has no reachable time-divergent target; if it has a
divergent target, the checker rejects the proof certificate as `bad-state`.
This rule classifies proof certificates only; it is not a task-miss witness.
Refutation still requires FR-364 to replay the concrete miss and its
time-divergent continuation. This keeps
time-locking paths outside FR-416's timed-claim behavior set while FR-417
continues to report time-locks separately.

It states three facts: the initial symbolic state (the zero valuation, let
time elapse unless the initial state is urgent, intersected with its time
invariants) is covered by a node of the initial discrete state; each node's
successor by each identity is covered by its named target under the aLU
simulation for `bounds`; and no node is bad unless it is marked `locked` (a rejecting automaton state, or
for the deadlock item a deadlocked valuation).

### Büchi emptiness certificate

For TT-4 and the time-lock-freedom item, the certificate adds:

- a component index per node, every edge going to a node of equal or greater
  index, so every cycle stays inside one component;
- per component, its reason: it lacks a node of some acceptance set (the
  time-divergence set included), or some weak fairness constraint is enabled
  at every valuation of every node in it and taken by no edge inside it, or,
  for a strong fairness constraint, a nested numbering of the component with
  the nodes that enable it removed;
- for the time-lock-freedom item, per node, a reachable time-divergent target:
  a quiescent node, or a component that does not fail for want of the
  divergence set.

### Refinement part

For the safety half of a refinement whose concrete model is timed, the
certificate is the same value with `refinement` present, the reachability part
read over the refinement product (QSL FR-244 "Refinement form"):

| Member | Content |
| --- | --- |
| `refinement_node` | The refinement node's `node_id` (FR-322). The part itself is `refinement` and an `edges` entry's `node` is an index, so no name stands for two things. |
| `concrete_package`, `abstract_package` | The two packages' identities (FR-322 package `name`). |
| `positions` | Nonempty (an admitted refinement half starts at a positioned initial node, so it certifies at least one position). The certified positions: each `{concrete, history, abstract_states}`, a concrete `StateKey`, the FR-378 history values as FR-181 typed values in history-row order, and the sorted abstract `StateKey`s. |
| `taken` | One entry per element of `edges`, in order: `null` for an edge into or inside the locked region (QSL FR-244), else `{type: "stutter"}` or `{type: "abstract", transitions}` with FR-181 transition identities. |

A node's `automaton` is then an index into `positions`; the locked region's
nodes carry the reserved index `len(positions)`, which names no position, and
the initial node of a time-locked concrete initial state whose `check_initial`
fails is such a `locked` node (QSL FR-245-AC-9).

### Canonical bytes

The bytes of a zone certificate are the RFC 8785 (JCS) serialization, UTF-8,
through ADR-013 §2's one encoder, of the FR-331 `ZoneCertificate` value, with
no byte before or after it. Equal certificates give equal bytes (QSL FR-244-AC-3
and AC-5), and the encoding is a function of the value alone.

- **Member order.** Object members are in ascending UTF-16 code-unit order of
  their names, at every depth: for the certificate, `admitted`, `bounds`,
  `components`, `divergence`, `edges`, `identity`, `initials`, `nodes`,
  `reasons`, `refinement`, `scale`, `task_product`. Array elements keep the order the
  certificate states: `nodes` in discovery order, `edges` in canonical
  transition order, `bounds` by the first node of its state then by clock
  index, `components`, `divergence` and `zone` rows by node or clock index.
- **Number encoding.** An index, a count or a position (`node` and `target` of an `edges` entry,
  `automaton`, `components` entries, `divergence` entries, `admitted`
  entries, `initials`) is a JSON integer from 0 to 2^53 − 1. A scaled constant
  (`scale`, every `bound`, every task-product ceiling, diagonal constant and
  subtraction constant) is an integer string `0` or `-?[1-9][0-9]*`,
  arbitrary precision, with no `+`, no leading zero and no `-0`; `strict` is a
  boolean. The one string spelling of an unbounded entry, and of an absent LU
  bound, is `"infinity"`. A DBM is its rows of entries, entry `[i][j]`
  bounding `x_i − x_j`, index 0 the reference clock. No number is a fraction
  or has an exponent.
- **Option encoding.** An absent optional member (`components`, `reasons`,
  `divergence`, `refinement`, `task_product`, `locked`) is omitted, never `null`; `locked` is
  present only as `true`. In an array a position is significant, so the one
  option inside an array, a `taken` entry that is `None`, is `null`.
- **No embedded digests.** The certificate embeds no digest or hash of the
  certificate itself, of a node or zone, of the checker, the producer or a
  manifest, or of any run output, and has no checksum member. What it carries
  that names the checker's inputs is structural: `identity` (the obligation
  identity), each `StateKey` (the discrete state's own identity, FR-181) and,
  in a refinement form, `refinement_node` (a `node_id`), `concrete_package`
  and `abstract_package` (package identities). The checker compares nothing
  against them; it checks the certificate's rules against the packages it is
  given.
- **Vectors.** The worked certificates ZC-10 to ZC-12, with their bytes, are
  published in `proposals/backend-provider-v1/zone-certificate-vectors.json`
  (TC-371). Their state keys are fixed illustrative values, not
  computed from a model.

## Checker

`check_zone_certificate` recompiles the package, recomputes every successor of
every node with its own DBM operations (canonical form by shortest-path
closure, emptiness as a negative cycle, guard intersection, reset, time
elapse, time-invariant intersection, splitting a disjunctive guard into its
convex parts) in exact arithmetic, checks ordinary timed coverage by the aLU
test and task-product coverage by the refined-region relation,
checks initial coverage and the badness of every node, checks the component
numbering and each component's stated reason, and reads no output of the
search beyond the certificate. It refuses a certificate whose `identity`
differs from the item's with `invalid_runtime_input`/`conflicting-identity`.
A rejected certificate settles `inconclusive`, `certificate-rejected{rule,
at}` (FR-331): `initial-missing` at the State locus `{key}` of
the initial discrete state, when no node of it covers the initial symbolic state (in a refinement form, of the discrete state of concrete initial state `i`, when `initials[i]` does not cover that state's symbolic state, for each `i`); `successor-missing` at
`{node}` when a node's successor by an identity is not covered by its named
target; `bad-state` at `{node}` for a bad node; `backward-edge` at `{edge}`
for an edge to a smaller component index; `witness-fails` at `{component}`
for a component whose reason does not hold, and at `{node}` for a node whose
named target is not a reachable time-divergent target. `node` and `edge` are
positions in `nodes` and `edges`, and `component` is a component index. A
checker stopped by a budget settles `incomplete`, `limit-reached` naming the
budget. Its cost is linear in the certificate's edges times the DBM operations.

## Identity and comparison

- **Identity kind** — *value*, compared componentwise; nodes and edges in
  their stated order.
- **Byte form** — the canonical bytes above; two certificates are equal exactly
  when their bytes are equal.
- **Absence** — a missing member rejects the certificate,
  `certificate-rejected{rule, at}` with rule `malformed` at `{member}`, that
  member's name.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-418-AC-1 | The checker accepts the certificate of the `Rpc` `NoLateReply` proof at `T = 4 ms` by recomputing every successor and coverage. | Test (TC-371) |
| FR-418-AC-2 | A certificate with a node whose successor is not covered by its named target settles `inconclusive`, `certificate-rejected` with rule `successor-missing` at that node; a bad node, `bad-state` at that node; an edge that decreases the component index, `backward-edge` at that edge; a component with no valid reason, `witness-fails` at that component; a Büchi certificate without `reasons`, `malformed` at member `reasons`; a certificate in which no node covers the initial symbolic state, `initial-missing` at the State record of the initial discrete state; and a time-lock-freedom certificate whose node names a target that is not a reachable time-divergent target, `witness-fails` at that node. | Test (TC-371) |
| FR-418-AC-3 | A certificate whose identity differs from the item's is refused `invalid_runtime_input`/`conflicting-identity`. | Test (TC-371) |
| FR-418-AC-4 | Constants are scaled to integers by the least common multiple of their denominators and DBM bounds are arbitrary-precision integers, so a constant of any size checks exactly. | Test (TC-371) |
| FR-418-AC-5 | A time-lock-freedom certificate states a reachable time-divergent target for every node, and the checker verifies each. | Test (TC-371) |
| FR-418-AC-6 | The bytes of a zone certificate are the RFC 8785 UTF-8 serialization of its `ZoneCertificate` value with members in ascending code-unit order and arrays in the stated order, and the bytes of ZC-10, ZC-11 and ZC-12 equal the published bytes, byte for byte, across two runs. | Test (TC-371) |
| FR-418-AC-7 | An index is a JSON integer in 0 to 2^53 − 1 and a scale or a bound an integer string; a bound written as a JSON number, with a leading zero, a `+` or `-0`, or a scale `0`, is not a `ZoneCertificate`, and a bound of 27 digits is carried exactly. | Test (TC-371) |
| FR-418-AC-8 | An absent optional member is omitted: `components`, `reasons`, `divergence`, `refinement` or `task_product` as `null` is not a `ZoneCertificate`, `locked` is present only as `true`, and a `taken` entry that is absent is `null`. | Test (TC-371) |
| FR-418-AC-9 | `identity` is the structural obligation identity, and in a refinement form the part's `refinement_node` is a `node_id` and `concrete_package` and `abstract_package` are package identities; no member carries a digest, hash or checksum of the certificate or of another artifact. | Test (TC-371) |
| FR-418-AC-10 | Every certificate lists `initials` (nonempty), and a refinement-form certificate lists, in its part, `positions` (nonempty) and `taken`, so a refinement part with empty `positions` is not a `ZoneCertificate`; ZC-12's first initial node (`initials[0]`) is a `locked` node with `automaton` equal to the number of positions and its `admitted` starts at the positioned `initials[1]`, a refinement form whose `initials[i]` does not cover concrete initial state `i` is `initial-missing` at that state's State record (the locus is the discrete state; two initial states sharing one discrete state share the locus, and the checker reports the lowest failing index first), `admitted` may start at any positioned `initials[i]`, and a certificate without `initials` or `admitted`, or with an `initial` member, is not a `ZoneCertificate`; nor is a refinement part whose node-id member is named `refinement` or `node` rather than `refinement_node`. | Test (TC-371); the checker clauses by QSL TC-700 and TC-371 ZC-14 |
| FR-418-AC-11 | The `TaskProductCertificate` schema admits `abstraction: bounded-subtraction-diagonal/v1`, the structured `time_progress` object with method `closed-finite-product/v1`, a positive `live_job_cap` and closure label `recompute-all-reachable-successors/v1`, plus the queue-bound, clock-ceiling, diagonal-constraint and subtraction arrays; each task-scheduler edge and subtraction uses a finite `TaskTransitionTemplate`, source-operation edges retain their source identity, and task-search frontier edges retain exact `TaskTransitionIdentity`; the schema refuses an exact task identity, an occurrence or generation added to a template, an unknown method, zero cap or a producer-asserted closure label. This criterion covers wire shape only; the reader's exhaustive finite-product closure and SCC checks are normative in the behavior above. | Test (TC-371) |
| FR-418-AC-12 | For a task-automaton certificate, the reader derives the scheduler policy from the checked claim named by `identity`, recomputes every policy successor and refuses an edge inconsistent with that policy; `task_product` carries no policy override. An EDF certificate with absent task priorities is admitted with no synthesized priorities, and a forged priority or ready order is refused. | Test (TC-371, TC-372) |
| FR-418-AC-13 | In a fixed-work task certificate, every task-scheduler edge and bounded-subtraction row uses a finite `TaskTransitionTemplate`, while source-operation edges retain their source identity; the reader closes the source-plus-scheduler graph modulo capture-avoiding renaming of completed jobs, accepts a periodic cycle whose absolute activation occurrence grows on each traversal, and refuses a template with an out-of-range or mismatched live-job rank/slot. Concrete replay instantiates the template with the exact prefix occurrence and generation, including when a completed slot is reused. | Test (TC-371) |
| FR-418-AC-14 | A task-product certificate may use `closed-no-release-product/v1` only with `live_job_cap: 0`, closure `recompute-all-source-successors-no-task-release/v1`, checked task demand intervals, no scheduler clock metadata and no task-transition edge; the reader recomputes every source successor from every initial state and accepts this closure only when no checked release binding is reachable. It refuses a positive cap, scheduler clocks, or a reachable release under this method. | Test (TC-371; TC-372) |

## Dependencies

- [FR-416](../../functional/temporal/FR-416-evaluate-timed-temporal-claims.md)
  and [FR-417](../../functional/temporal/FR-417-settle-time-lock-freedom.md)
  settle the proofs this certificate carries.
- [FR-331](FR-331-backend-provider-envelope.md) carries the certificate in
  the result.

## References

- QSL ADR-026 EZ-2 to EZ-4, CF-1 to CF-6, OV-12.
- QSL FR-244 (the certificate's members), FR-245 (its checker) and FR-430 (the refinement form); Linear STD-522 and QSL STD-516.
- S. Wimmer and J. von Mutius, TACAS 2020; S. Wimmer, F. Herbreteau and
  J. van de Pol, FORMATS 2020; F. Herbreteau, B. Srivathsan and
  I. Walukiewicz, LICS 2012.
- E. Fersman, P. Krčál, P. Pettersson and W. Yi, “Task Automata: Schedulability,
  Decidability and Undecidability,” *Information and Computation* 205(8), 2007,
  §4 and Theorem 2, DOI 10.1016/j.ic.2007.01.009. This is design evidence for
  the fixed-computation bounded-subtraction/diagonal product; this requirement
  separately states the checker obligations for its finite refined-region
  certificate.
- Owner ruling recorded on QSL-373: a native zone engine with zone
  certificates for checkable proofs.
- Ticket: STD-139 (paired with QSL-373).
