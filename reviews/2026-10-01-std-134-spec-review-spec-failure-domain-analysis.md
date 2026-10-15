---
id: SR-811
title: "Partial-order reduction soundness under fairness (FR-383, ADR-021 POR-10/POR-11)"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@6a46fe220e02d93679279e2fef7f390fd57d448a; spec/functional/analysis/FR-383-explore-under-state-space-reduction.md, spec/test-cases/TC-323, TC-325, TC-326; design input quire-spec-language ADR-021 at spec/368-por 1e48e7a7 (POR-1 to POR-11, §2, §7.4, §7.5)"
review_set: all
review_date: "2026-10-01"
---

## Summary

Ticket: STD-134 (paired QSL-368), PR quire-specification#169.

Dedicated check of the fairness-preserving partial-order reduction: the POR-10
visibility rule, the POR-11 soundness and completeness argument, the §7.4/§7.5
explanation, the `Gate` vector, and the L3 corollary that the POR-7 closure
already keeps behaviour-used writers so POR-10 matters only for never-taken
ample transitions. Both the QSpec FR-383 text and the QSL ADR-021 argument
were checked.

Method: hand proof check plus a small explicit-state simulator of POR-4 to
POR-10 with the breadth-first revisit proviso and a weak/strong fairness SCC
filter (scratchpad `porsim/`). The simulator reproduces the §7.5 `Gate`
vector exactly (without POR-10: ample `{arm}`, false `proved`; with POR-10:
ample `{toggle}`, `refuted`; empty fairness: `{arm}`, `refuted`). Random
search over 80,000 small models with static, complete footprints found no
verdict difference between the full and the POR-10-reduced product for weak
and strong, single and multi-member classes; without POR-10 differences
appear within a few hundred models. So the POR-10/POR-11 construction is
sound when the enabling footprint is complete. It is not complete as
written (FND-001, FND-002), and each gap has a concrete false `proved`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Enabling footprint is the precondition read footprint, but enabledness (ADR-018 FA-2) also needs an admissible post-state, so C1 and POR-10 miss writers that enable a transition through its postcondition; two concrete false `proved` results | FR-383:95-121; ADR-021 POR-7, POR-10, POR-11 |
| FND-002 | high | Footprints carry no receiver or argument membership: `deletes`/`creates` write only (population, membership) and R(t) never reads it, so deleting a receiver is "independent" of that receiver's transitions; concrete false `proved` | FR-383:71-104; ADR-021 POR-1, POR-4, POR-10 |
| FND-003 | low | ADR-021 §7.5 corollary is false for multi-member classes: the POR-7 closure keeps writers of one member's enabling footprint together, not writers of different members of a `whole` or thread class; POR-11 relies on POR-10 for those | ADR-021 §7.5 first paragraph; POR-11 |
| FND-004 | low | POR-11's "an extra transition of K would be enabled forever in σ" is argued on the rewritten suffix, not on σ; it holds only by an induction on the first K-extra that the text omits | ADR-021 POR-11 |

### FND-001 counterexamples

Enabledness per ADR-018 FA-2 is "precondition holds and at least one
post-state satisfies its postcondition". POR-7, POR-10 and FR-383 (line 102,
"the enabling footprint (the precondition read footprint)") read only the
precondition.

(a) Safety, C1. Gate `g` with `armed` (true) and `alarm` (false), Cell `a`
with `v: Int[0,1]` (0), `g.cell = a`. `disarm()`: `modifies self.armed`, post
`not self.armed`. `boom()`: `modifies self.alarm`, pre `self.armed`, post
`self.alarm and pre(self.cell.v) = 1`. `set()` on `a`: pre `self.v = 0`, post
`self.v = 1`. Claim `always holds(not g.alarm)`. Unreduced: `set`, `boom`
reaches `alarm`, refuted. Reduced (canonical order `disarm` before `set`): the
candidate seeded by `disarm` is `{disarm, boom}` with `boom` disabled, whose
enabling footprint `{(g, armed)}` has no other writer; enabled part
`{disarm}`, invisible, new successor, so `A = {disarm}`; `boom` is then
disabled forever and the run reports `proved`. Simulated.

(b) Fairness, POR-10. The §7.5 `Gate` with `go`'s guard moved to its
postcondition: pre `not self.done`, post `self.done and pre(self.c) and
pre(self.d)`. `go`'s enabling footprint is `{done}`, so `arm`/`arm2` are not
fairness-visible, `{arm}` is chosen at `(F,F,F,F)`, and the run reports
`proved` while the subject is refuted by `toggle, toggle`. Simulated.

Fix: define the enabling footprint as every location FA-2 enabledness reads:
the precondition read footprint plus every pre-state location the
postcondition reads (with frame and invariant reads where they decide
successor existence). The simplest correct choice is `R(t)`. Add (a) and (b)
as vectors to TC-323 and TC-325.

### FND-002 counterexample

Job population `{j, k}`, fields `armed`, `cancellable`, `bad`, `peer`;
`j.armed = j.cancellable = true`, `k` neither, `j.peer = k`. `boom()`:
every-object `modifies bad`, pre `self.armed`, post `self.peer.bad`.
`cancel()`: `deletes Job`, pre `self.cancellable`, deletes the receiver.
Claim `always holds(not k.bad)`. W(cancel(j)) = (Job, membership); R(boom(j))
= {(j, armed), (j, peer)}, W(boom(j)) = (Job, bad, any). By POR-4 the two are
independent, though `cancel(j)` disables `boom(j)`. `boom(j)` is visible, so
`A = {cancel(j)}` at the initial state, then no transition is enabled and the
run reports `proved`; the subject refutes with `boom(j)`. Simulated. The same
gap leaves a `creates` step invisible under POR-10 when it adds a member to a
`whole` class.

Fix: R(t) and the enabling footprint include (population, membership) for the
receiver's population and for each reference argument's population, and a
`deletes` write meets every field location of the deleted type.

### FND-003 counterexample to the corollary

`fair weak whole go` over Gates `p`, `q`; `go` pre `self.on`; `flip()`
toggles `self.on` with a receiver-scoped frame. The candidate seeded by
`flip(p)` is `{flip(p), go(p)}` and never holds `flip(q)`. From `(p.on,
q.on) = (T, F)`, `flip(p), flip(q)` passes `(F, F)` where the class is
disabled, and the swapped order passes `(T, T)` where it is enabled. The
class's enabledness depends on the order of two writers that the behaviour
takes, and only POR-10 keeps that order. The corollary holds for
single-identity `each` classes only. The §7.5 rule text (POR-10) is right;
the explanation should not say POR-10 matters only for never-taken
transitions.

### FND-004

At stage i of the standard construction, the remaining suffix includes the
effects of earlier extras, so a K-extra β is enabled forever on that suffix,
not on σ. It transfers to σ because, inductively, no earlier extra is in K,
and an extra outside K cannot write β's enabling footprint (POR-10). POR-11
should state that induction. It also depends on the enabling footprint being
complete, which FND-001 and FND-002 show it is not.

## Verdict

POR-10 is the right rule, and POR-11's completeness step holds once the
enabling footprint is complete. As written, with the enabling footprint
equal to the precondition read footprint and no membership locations, it
does not hold, and FR-383 inherits both gaps. The `Gate` vector is correct
and reproduces. FR-383's C0 to C3, the breadth-first proviso and the
candidate rule otherwise match ADR-021 POR-6 to POR-8. Not mergeable until
FND-001 and FND-002 are fixed in FR-383 and in ADR-021.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | The membership write of a `deletes` or `creates` entry is "membership of the deleted (created) object", which a static per-identity footprint cannot instantiate when the object is not the receiver or a reference argument; the location model has no any-object membership form, so an implementation that resolves it to the receiver brings back FND-002 | spec/functional/analysis/FR-383-explore-under-state-space-reduction.md:79-84 |

FND-005 counterexample: TC-323 case 6 with a third Job `m` (`cancellable`,
`target = j`) and `cancel()` deleting `deref(self.target)` in place of its
receiver. The identity `cancel(m)` fixes no deleted key statically. If the
footprint resolves "the deleted object" to the receiver `m`, then
`cancel(m)` and `boom(j)` are independent, `A = {cancel(m)}`, `j` is
deleted, and the run reports `proved` while the subject refutes. Fix: add a
(population, membership of any object) location, which meets every
membership location of the population and every field location of its
type. Use it for a `deletes` entry unless the deleted object is the
receiver or a reference argument, and for every `creates` entry, since the
engine chooses the created key.

## Dispositions

Round 1, reviewed at ebfdad1c48b8bc3d30e84fd6597d55117bc37dee.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ebfdad1: the enabling footprint is R(t), covering precondition reads, postcondition pre-state reads and membership; vectors TC-323 case 5 and TC-325 case 3 re-simulated: unreduced and reduced both `refuted`, 6 states each, ample set `{toggle}` at `(F,F,F,F)` |
| FND-002 | fixed | ebfdad1: R(t) holds receiver and argument membership, and a `deletes` write meets the deleted type's fields; vector TC-323 case 6 re-simulated: both `refuted`, 4 states each, `{cancel(j)}` is not an ample set. The unresolved-object case is new FND-005 |
| FND-003 | deferred | routed to quire-spec-language #568 (QSL-368), which owns ADR-021 §7.5; nothing in this PR to change |
| FND-004 | deferred | routed to quire-spec-language #568 (QSL-368), which owns the POR-11 text; nothing in this PR to change |

Round 2, reviewed at efc24277229291b6772974701e52a83a2cc96516.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-005 | fixed | efc2427: FR-383 adds (population, membership of any object) for a `deletes` not on the receiver or a reference argument and for every `creates`, meeting every membership and field location of the type; vector TC-323 case 7 re-simulated: unreduced and reduced both `refuted`, 4 states each, `{cancel(m)}` is not an ample set (resolving to the receiver instead gives the false `proved`, 2 states) |

## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | FR-013 still says a deleted object is permitted "exactly when its most-specific type conforms to some `deletes` entry", which a `deletes self` or `deletes p` entry (naming no type) never satisfies; read literally, scoped deletes permit nothing | spec/functional/foundation/FR-013-validate-operation-frames.md:29 |
| FND-007 | low | FR-383-AC-6 has a stray ".;" joining two sentences ("counts it.; a constraint that evaluates undefined ...") | spec/functional/analysis/FR-383-explore-under-state-space-reduction.md:262 |

FND-006 fix: "A created object is permitted exactly when its most-specific type
conforms to some `creates` entry; a deleted object exactly when its
most-specific type conforms to some type-wide `deletes` entry, or it is the
receiver under `deletes self`, or the argument bound to `p` under
`deletes p`." FND-007 fix: replace ".;" with "; ".

## Dispositions (round 3)

Round 3, reviewed at 39268afed5980496c1db5f634998e1094735a379. No finding had
a latest outcome of still-open or no outcome, so this round adds no rows.
FND-001, FND-002 and FND-005 were re-verified under the final rules: the
coder's `final_q2_r3.py` and my own scripts agree that TC-323 cases 5 to 9
and TC-325 cases 2 and 3 settle the same verdict reduced and unreduced. Cases
8 and 9 reproduce the false `proved` under the old rules (3 states each) and
agree under the final rules (5 and 6 states). FND-003 and FND-004 stay
deferred to quire-spec-language #568.
