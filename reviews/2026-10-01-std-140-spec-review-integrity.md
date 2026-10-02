---
id: SR-841
title: "Integrity review of PR #173: protocol transition system and weak memory"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@559a409fa92622132b84dc6df275d8fb612ddb5a; PR #173 diff against origin/main; spec/functional/protocol/FR-052, FR-425..FR-433; spec/functional/memory/FR-434..FR-439; FR-049, FR-181, FR-220, FR-228; ADR-006; TC-052, TC-210, TC-375..TC-394; checked against quire-spec-language ADR-027 (qsl-396-protocol-ts) and ADR-025 (qsl-372-memory)"
review_set: subset
---
# Integrity review of PR #173

## Summary

Ticket: STD-140 (protocol transition system), with STD-138 (weak memory).
This pass checks the PR's requirements against each other, against the
shared grammar, and against ADR-027 and ADR-025. The PR follows both ADRs
closely: state key, step kinds, transition identity, folding, fairness
targets, scheduler constraints, counted steps, explicit refinement rows,
deadlock opt-out, instance budget (default 3, stated in every result), the
TSO and RA rules, race freedom, memory budgets (default 4, stated in every
result) and the precedence of limit causes all match. No fixed caps were
found. Depth is not used as a limit kind. No pins, ledgers or compat paths
were added. The PR also removes the ecosystem-lock pin and the sampler
version pin from FR-181 and TC-210.

Three contradictions would stop an implementer: `finish` enabledness, the
repeat grammar, and the obligation identity of the `memory` clause. The
loop resolution in ADR-006 has not had an owner ruling. Its "neither
refuses" leg is an admission gate that the PR's own test vectors cannot
satisfy.

### Loop resolution (ADR-006)

Lane Q5 decided that a `repeat` needs a maximum, or an invariant plus a
variant, or both.

- **Dropping the mandatory maximum is right.** It follows the no-fixed-caps
  ruling and ADR-027 PS-6, and it removes the contradiction with
  FR-171-AC-2. A stated maximum is model meaning, so it is not a cap.
- **Refusing a loop that has neither is not supported.** The explicit-state
  engine does not need loops to terminate. Cycles close through the state
  key, and behaviours are maximal paths, lassos included. FR-171 asks for an
  invariant and variant only "for termination claims". FR-171-AC-2 already
  settles an unproved termination claim as inconclusive or refused. Requiring
  a variant at admission forces authors to write a termination proof for
  loops that are meant to run for ever. TC-391 LT-11 (a spin loop) and
  TC-393 MB-01 (a store loop) do exactly that and attach a variant that
  cannot exist (FND-005).
- **Recommendation for the owner.** A `repeat` carries a progress edge. An
  authored maximum is optional model meaning. An invariant and variant are
  optional, and only termination claims read them (FR-171-AC-2). Refuse a
  zero-progress loop only. Until the owner rules, ADR-006 should not merge
  as a decision.

## Verdict

**Not mergeable as it stands.** Four high findings and four medium findings.
The rest of the protocol and memory design is consistent with ADR-027 and
ADR-025.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-426's `finish` row is enabled whenever a record satisfies the finish constraint and marks the instance finished. FR-427 (an instance finishes only after every compensation thread ends), FR-427-AC-2, FR-429, TC-377 JP-02 and the TC-380 state counts all require `finish` to wait until every thread has left `running`. ADR-027 ST-9 has the same gap. | spec/functional/protocol/FR-426-apply-protocol-steps.md:91; spec/functional/protocol/FR-427-check-protocol-compensation.md:83,97; spec/test-cases/TC-377-join-policies-at-thresholds.md:32 |
| FND-002 | high | FR-052 says the shared grammar's complete `repetition` production admits either termination premise. The grammar was not edited: proposals/quire-v1/shared-grammar.md:466 still requires `max` and also `invariant` and `variant`, and :201 requires `max`. FR-339 makes that grammar normative, so a conforming parser refuses loops that FR-052 admits. | spec/functional/protocol/FR-052-represent-bounded-control.md:51; proposals/quire-v1/shared-grammar.md:201,466 |
| FND-003 | high | FR-434-AC-2 gives a source default `tso` and a source `sc` with a `tso` selection the same obligation identity. FR-425 puts the protocol clause in the subject and so in the identity, and FR-052-AC-4 makes graph content identity-bearing. The two sources differ in their `memory` clause. No FR excludes the `memory` clause from the clause's identity contribution. | spec/functional/memory/FR-434-select-memory-models-for-parallel.md:102,134; spec/functional/protocol/FR-425-define-protocol-subjects-and-state-keys.md:62,179 |
| FND-004 | medium | Loop rule (ADR-006, FR-052-AC-3): refusing a loop with neither a maximum nor an invariant and variant is an admission gate that the engine does not need. FR-171 reads the variant only for termination claims. FR-228 still requires a *proved* invariant/variant at admission, but FR-052 says an authored one is admitted and the proof is an obligation. The owner has not ruled. | spec/functional/protocol/FR-052-represent-bounded-control.md:39,96; spec/objects/protocol/FR-228-bounded-control-construct.md:33; spec/decisions/ADR-006-protocol-bounded-control-profile.md |
| FND-005 | high | TC-393 MB-01 (branch `w` stores in a loop until the buffer budget is hit) and TC-391 LT-11 (a reader spinning on a flag) are written as invariant-and-variant loops with no maximum, but both must run without bound. No well-founded variant decreases on every back edge, so under FR-052 and FR-171 these vectors are not admissible as written. | spec/test-cases/TC-393-memory-budgets-and-verdicts.md:22; spec/test-cases/TC-391-litmus-conformance-vectors.md:48 |
| FND-006 | medium | The owner ruled that an undefined evaluation in any engine settles refuted with cause UndefinedEvaluation. FR-427 maps any recovery-predicate value other than true to `unrecovered`, so an undefined value is silently counted as unrecovered. The activation guard, the `retire` predicate `p`, attempt and event constraints, and `terminal when P` (asserted "total" with no check named) have no undefined path. | spec/functional/protocol/FR-427-check-protocol-compensation.md:69; spec/functional/protocol/FR-429-report-protocol-deadlocks.md:81 |
| FND-007 | medium | FR-432's Description requires "exactly one explicit step row for every concrete protocol step class" and refuses two rows. FR-432-AC-2 admits a node row and an operation row together for the same attempt node, with the node row taking precedence. | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:21,113 |
| FND-008 | low | FR-432's grammar uses `fork`, `join`, `activate` and `memory` as row keywords, but its reserved-word list names only `cend`, `duplicate`, `lose`, `spawn`, `retire` and `node`. `fork` is reserved nowhere in the shared grammar. | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:59 |
| FND-009 | low | FR-434, FR-435, FR-429, FR-430 and FR-432 each define a production that "replaces" or "extends" a shared-grammar production inside the FR. None of them edits shared-grammar.md, so source recognition now has two owners. | spec/functional/memory/FR-434-select-memory-models-for-parallel.md:48; spec/functional/protocol/FR-429-report-protocol-deadlocks.md:63 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-010 | low | The shared grammar's `protocol-step-target` copies the alternatives of the refinement declaration's `step-target` (FR-375, on another lane's open branch) instead of naming `step-target`, so the two productions can drift. | proposals/quire-v1/shared-grammar.md:513-516 |
| FND-011 | low | FR-052 still refuses "termination conditions that cannot be reached within the declared finite graph". TC-376 FL-07 and TC-393 MB-01 rely on a `while { true }` loop whose exit is never taken, so `finish` is never reached. FR-052 should say this refusal is structural (graph edges), not guard evaluation. | spec/functional/protocol/FR-052-represent-bounded-control.md:45-47; spec/test-cases/TC-376-two-branch-join-worked-example.md:56 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-012 | low | TC-392's guarded message-passing choice guards on `r1`, the register written by `r1 := flag`, rather than on the flag load's binder, and its `skip` case spells inequality `<>`. FR-435 refuses a guard that reads a model location, and the shared grammar's token is `!=`. Written literally, the vector refuses at check time instead of exercising DR-01/DR-02. | spec/test-cases/TC-392-race-freedom-item.md:21-23 |

## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-013 | low | FR-052's step-node list (event, send, receive, attempt, effect, commit, fence) leaves out `await` and `parallel`, whose `timeout`, `fork` and `join` are steps (FR-426). A polling loop such as `repeat … await W … match receive R then … timeout sequence { }` takes a `timeout` step on every iteration, yet its timeout cycle passes through no listed node, so FR-052 refuses it as zero-progress. FR-425 states the actual property: every state-graph cycle through a repeat takes a step. | spec/functional/protocol/FR-052-represent-bounded-control.md:39-41 |

## Dispositions

Round 1, reviewed at 47c10e8ea16abb2545234dc09dedefca37bbf273.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 47c10e8 |
| FND-002 | fixed | 47c10e8 |
| FND-003 | fixed | 47c10e8 |
| FND-004 | fixed | 47c10e8 (against the team-leader loop decision accepted by the plan lead) |
| FND-005 | fixed | 47c10e8 |
| FND-006 | fixed | 47c10e8 |
| FND-007 | fixed | 47c10e8 |
| FND-008 | fixed | 47c10e8 |
| FND-009 | fixed | 47c10e8 |

Round 2, reviewed at d493d475b7a86caa7433bd3d3fce95596850c010.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-010 | fixed | d493d47 |
| FND-011 | fixed | d493d47 |

Round 3, reviewed at c358412e7dff0f29fa007f5ccb24042086787736.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-012 | fixed | c358412 |

Round 4, reviewed at d173de56000c884ffe3fe43a963d72eb4609c758.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-013 | fixed | d173de5 |
