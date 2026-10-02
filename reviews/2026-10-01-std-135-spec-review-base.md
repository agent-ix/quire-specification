---
id: SR-820
title: "Base review of STD-135/STD-136: state-graph claims and hyperproperties over model behaviours"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@c54d91e1a7e9c69eba77c5f4b1505d192a9cc0e9; git diff origin/main...HEAD (PR #168): spec/functional/analysis/FR-390..FR-392, FR-394..FR-399, FR-401, FR-403, spec/objects/interfaces/FR-393, FR-400, FR-402, amended FR-181, FR-191, FR-290, FR-331, FR-341 (infinite-trace), shared-grammar.md, TC-330..TC-349, spec/complete-v1/tests.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-390
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-391
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-392
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-393
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-394
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-395
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-396
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-397
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-398
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-399
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-400
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-401
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-402
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-403
    type: reviews
---
# SR-820: Base review of STD-135/STD-136: state-graph claims and hyperproperties over model behaviours

## Summary

Ticket: STD-135 (with STD-136). PR agent-ix/quire-specification#168, draft.

Checked the PR diff against QSL ADR-022 (worktree qsl-369-possible) and
ADR-023 (worktree qsl-370-hyper), the wave B/C owner rulings, AC
testability and AC-to-TC coverage. Every new and amended AC has a TC and a
matrix row. The new FR and TC files pass `quire validate`. The four
failures on amended FR-331, FR-290, FR-341 and tests.md are the same
structural failures main already has. Ticket ids appear only in References.
No new pin, ledger, provenance record, compat path or fixed cap is added.
FR-181 drops the recorded generator version and the "same ecosystem lock"
wording. Every limit added (`witness_samples`, `max_witness_set`,
`max_relation_tuples`) is a caller-set budget with a published default.
Reaching one names the limit and how to raise it. Undefined → refuted
(`undefined-evaluation`) appears in both families.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Phase-0 sampling can change a verdict when an undefined evaluation is present. FR-392 settles a `possible` item from sampled witnesses and skips exhaustive exploration. It says that on a completed run a sampled `proved` "is the verdict exploration alone gives". FR-391-AC-10 says an undefined predicate at an explored node refutes "also when a witness or other evidence exists". Take a model where every initial state has a short witness and an undefined node sits off every walk (TC-349 SG-03 is one). It settles `proved` with the default 64 walks and `refuted` with `witness_samples` 0. ADR-022 has the same gap (§3 "Determinism" against GV-7), so fixing it needs an ADR amendment or a ruling. | FR-392:53-64, FR-391:75, FR-391:92, FR-391:184 |
| FND-002 | high | TC-348 UE-02 adds a match conjunct `(lookup<V::Vault>(vaults, rw) absent undefined).l @ a = 0` and expects `refuted`, `undefined-evaluation`. That conjunct reads state. FR-395 Admission refuses a match conjunct that reads state, and TC-337 HB-07 expects that refusal for `v.l @ a = 0`. A vector cannot pass both. Build the undefined match from a step-label read, as TC-340 VS-03 does. | TC-348:36, FR-395:138-139, TC-337:34 |
| FND-003 | medium | FR-391 puts the Undefined row first and says it settles "whatever the rest of the graph holds". ADR-022 GV-7 says "between refuting evidence, the first in discovery order settles it", and STD-131's FR-363 also ranks an undefined evaluation and a violation by canonical order. Suppose a trap is discovered before an undefined node. ADR-022 gives V-10 with a `Trap`, and FR-391 gives `undefined-evaluation`. | FR-391:75, FR-391:92 |
| FND-004 | medium | FR-399 says "the analyzer reports the first refuting evidence it finds". That is not canonical. ADR-023 HV-8 fixes the order: for HP-1, the first tuple in the product of canonical edge orders; for HP-2, HP-3 and HP-6, the first product state in canonical breadth-first order, as ADR-018 UE-4 states. Without it, two conforming engines can return different counterexamples. | FR-399:94-95 |
| FND-005 | medium | The `undefined` evidence arm (FR-393) and counterexample kind (FR-400) carry no undefined-evaluation value. The FR-394 and FR-401 replay rules compare against "the item's `undefined-evaluation` cause", but neither FR's Inputs include the item's result. So the evidence does not replay on its own. ADR-022 GX-1 has `Undefined{stem, undefined}`, and ADR-023 HV-8 says "its `undefined` member holds the `UndefinedEvaluation`". | FR-393:58, FR-394:28-30, FR-394:55, FR-400:55, FR-401:28-30, FR-401:57 |
| FND-006 | medium | FR-394 owns replay of every `GraphEvidence` arm. Its `witness` row evaluates `P` at a last node, and its `trap` row explores a closure for "the target predicate". Neither covers an HP-5 claim, whose witness is a fair lasso with a temporal body and whose trap re-explores a product with the fairness filter. FR-396 states that replay inline, so one arm has two replay rules, and no FR-394 AC covers the HP-5 case. | FR-394:52-53, FR-396:93-103 |
| FND-007 | medium | FR-392 Inputs name "the `witness_samples` and maximum depth members of the request's model-check limits". The owner ruling says depth is never a limit kind, and ADR-022/ADR-018 treat `max_depth` as a method parameter. | FR-392:30-31 |
| FND-008 | medium | Three ACs have clauses that no TC vector tests. FR-393-AC-3 has a misplaced or out-of-range `loop_entry`, but TC-335 GE-08 has no such vector. FR-400-AC-3 has an out-of-range `loop_entry` or `position`, but TC-345 HR-07 has none. FR-396-AC-3 has a single candidate that does not discharge the form settling `unsupported`, but TC-342 HF-08 only records kinds. | FR-393:96, TC-335:35, FR-400:96, TC-345:34, FR-396:115, TC-342:35 |
| FND-009 | low | TC-342 HF-06 reads `v.l @ b` but binds no `v`: it has no `behaviours (v: V::Vault)`, unlike HF-05. Under FR-395 Admission it refuses before it can settle `refuted`. | TC-342:33 |
| FND-010 | low | TC-330 SG-R08 expects a syntax error for `possible reaches using v { true }` "because `possible` is reserved". The error comes from `reaches`, a reserved word in an ident position. As written, the vector does not test what its rationale says. | TC-330:38 |

## Verdict

**Not mergeable as it stands.** FND-001 makes a requirement contradict
another AC, and the fix needs an ADR-022 decision. FND-002 is a vector that
cannot pass. FND-003 to FND-008 are real gaps against ADR-022 and ADR-023.

The rest of the PR follows both ADRs closely. That covers the state-graph
family with `E(P)`, traps and path counting, open and closed nodes, the
V-9/V-10 rows, the reduction rows, phase-0 indexing `i × witness_samples + r`
with its published default of 64, and trap replay by fresh exploration. On
the hyper side it covers lockstep, the `μ_U`/`μ_E` split, the HP-1 to HP-6
classification, the HC-2 witness set and its replay, projection alignment,
the code/model claim split, copy-swap and obligation identity. The rulings
hold: no caps, no pins, no compat paths, and ticket ids only in References.

Merge items, not findings:

- `cargo test --locked` fails only `tc_233_complete_value_lock_catalogs_every_artifact_and_separates_package_selection`
  and `tc_233_complete_value_rule_manifest_binds_every_selected_rule_byte_for_byte`
  (the shared-grammar digest). #174 deletes them. The other 54 tests in that
  binary and every other binary pass.
- FR-181-AC-7 is also added by #170 (receiver identity) and #175 (workload
  draws). The wave-C table assigns AC-7 to STD-135, so those PRs renumber.
  FR-181-AC-5 is also amended by #169. Reconcile on merge.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-011 | medium | Reading `a.step.receiver` at a terminal-stutter step or a receiver-less step evaluates undefined (`absent-step-receiver`). Under undefined → refuted, the natural same-inputs match `a.step.op = b.step.op and a.step.receiver = b.step.receiver` refutes every model with a reachable terminal state. TC-348 UE-02 shows this on the secure vault. Arguments are `Option<T>`, and ADR-023 HM-5 pairs stuttering components on equal operations. Give `receiver` the same Option reading and drop `absent-step-receiver`, or record a ruling that refuting is intended. | FR-395:74-77, native-diagnostics.md:264, TC-348:38, TC-340:28 |

## Dispositions

Round 1, reviewed at 2b3fae1 (diff origin/spec/wave-b-q1-temporal...HEAD).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 2b3fae1 |
| FND-002 | fixed | 2b3fae1 |
| FND-003 | fixed | 2b3fae1 |
| FND-004 | fixed | 2b3fae1 |
| FND-005 | fixed | 2b3fae1 |
| FND-006 | fixed | 2b3fae1 |
| FND-007 | fixed | 2b3fae1 |
| FND-008 | fixed | 2b3fae1 |
| FND-009 | fixed | 2b3fae1 |
| FND-010 | fixed | 2b3fae1 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-012 | low | FR-399-AC-8 tests "a match or a body" that evaluates undefined. With `receiver` and arguments read as Option, and with a match barred from reading state, no admitted match can reach either catalogued undefined reason (`absent-key` needs a population, `precondition-false` a dispatched call). So no test can exercise the match branch, and TC-340 no longer traces to AC-8. Keep the normative rule general, "the claim (body or match) evaluates undefined", as FR-363 states it once for every engine, but limit AC-8 to the body. | FR-399:75, FR-399:87, FR-399:145 |

## Dispositions (round 2)

Round 2, reviewed at 703edc4 (diff origin/spec/wave-b-q1-temporal...HEAD).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-011 | fixed | 703edc4 |


## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-013 | medium | FR-363 (main, from #170) says an operation guard, a `terminal when` predicate or a state constraint that evaluates undefined at a reachable state refutes every item over the subject, and binds every model engine. The state-graph and hyper families cover only the claim. FR-391's Undefined row reads "a predicate of the claim (`P`, `Q`, `X` or `Y`)", and its Witnessed and Exhaustive rows require only "no undefined evaluation of a predicate of the claim". So a model with an undefined guard on a reachable state settles `proved` by the table. FR-393's `where.predicate` must be one of P, Q, X or Y and carries no FR-364 `transition`, so an undefined guard has no encoding. FR-399 and FR-400 say only "the body or a match". Extend the Undefined rows and both `undefined` arms to the subject's guard, `terminal when` and state constraint, with FR-364's `transition` key for a guard. | FR-391:79, FR-391:82, FR-391:83, FR-393:64-67, FR-393:75-76, FR-399:77, FR-399:90, FR-400:62 |
| FND-014 | medium | A replay that a limit stops settles two ways. FR-394 settles a stopped trap or product re-exploration `incomplete` with cause `limit-reached{limit, value, setting}`, as FR-391 and FR-399 settle a stopped run. FR-401 settles a witness-set recomputation that reaches `max_witness_set` `inconclusive` with `replay-refused`, and AC-4 and TC-345 HR-06 test that. FR-401's Outputs list no `incomplete`. Settle it `incomplete` with `limit-reached{limit, value, setting}` naming `model_check.max_witness_set`, as FR-394 does. | FR-401:38-40, FR-401:77-79, FR-401:105, TC-345:33, FR-394:70-73 |
| FND-015 | medium | No vector tests the cancelled outcome. FR-399 has a "caller cancelled the run" row (`incomplete`, cause `cancelled`), and FR-399-AC-1 says every row settles exactly its label, but none of TC-338, TC-339, TC-340 or TC-342 cancels a run. FR-391 and FR-394 state the cancelled outcome, but neither FR-391-AC-5 nor FR-394-AC-6 names it, and no TC-330..349 vector cancels. Add a cancelled vector for each family and name `cancelled` in FR-391-AC-5 and FR-394-AC-6. | FR-399:83, FR-399:145, FR-391:124-126, FR-391:211, FR-394:72-73, FR-394:87 |
| FND-016 | low | FR-364 calls the undefined-evaluation envelope "the one shape … for every engine" and lists its `where` keys. FR-393 adds a `where.predicate` key and FR-400 adds `where.variables`, and neither key is in FR-364's table. FR-364 lets an engine use the plural forms of its keys, not add new ones. `locus` already names the expression. Either add the two keys to FR-364's key table in this PR or drop them. | FR-393:75-76, FR-400:72-73, FR-364:92-102 |

## Dispositions (round 3)

Round 3, reviewed at 51649878fc34b720fea9579a57854203458cdb50 (diff origin/main...HEAD, merge base aa0318663f018bf3a6e092fa2cec933009b184d7).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-012 | fixed | b90f870 |

Also checked at this sha. The PR changes none of FR-361..FR-370, so they are main's text. The grammar reuses #170's `fairness` production, and spec.md and tests.md add no FR-36x or TC-29x/30x rows, so there are no duplicate hunks from #170. Wire causes are kebab-case. FR-181-AC-13 and FR-331-AC-17 do not collide with #169 (725f3ce), #173 (2b020e9), #175 (b6d7d89, FR-181-AC-9..12, FR-331-AC-13..16) or #176 (44ca4ad). FR-393 and FR-400 use FR-364's envelope and cite it (except FND-016). A stopped run settles `incomplete` with `limit-reached{limit, value, setting}` and a cancelled run with `cancelled` in FR-391, FR-394 and FR-399 (except FND-014). FR-391 has `well-definedness-unchecked`, FR-392 has `max_walk_steps` as a named limit, and FR-394 witness replay settles through FR-391. `witness_samples` and `seed` are sampling settings, not limits (FR-331, FR-392). No default numbers are published: the remaining 64s are request values that the vectors set explicitly. FR-461 is cited, and it is on open #176. `cargo test --locked --no-fail-fast` exits 0 (lib 0, model 4, v2 59, member 10, shared 7). The target dir was deleted.

## New findings (disposition pass 4)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-017 | medium | FR-399-AC-10 covers a guard, the `terminal when` predicate and a state constraint of a component's subject. TC-348 has a vector only for the guard (UE-08, UE-09), and none for a `terminal when` predicate or a state constraint that evaluates undefined in a hyper item. FR-391-AC-14 has all three on the state-graph side (SG-09, SG-11, SG-12). | FR-399:169, TC-348:42-43, TC-348:47 |
| FND-018 | low | A blank line at TC-349:44 splits the Vectors table. SG-09 to SG-14 follow it with no header row, so they render as paragraph text and not as table rows. These are the vectors that back FR-391-AC-14 and the FR-393-AC-5 guard clauses. Delete the blank line. | TC-349:44-50 |

## Dispositions (round 4)

Round 4, reviewed at 81efef7dc76f72e752a53c495b4c4f2c999b021e (delta 5164987..81efef7; merge base aa0318663f018bf3a6e092fa2cec933009b184d7).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-013 | fixed | 81efef7 |
| FND-014 | fixed | 81efef7 |
| FND-015 | fixed | 81efef7 |
| FND-016 | fixed | 81efef7 |

FR-364 is now edited by this PR, but only to add the two `where` keys that the FND-016 fix chose. No other FR-361..370 text changes. `cargo test --locked --no-fail-fast` exits 0 (model 4, v2 59, member 10, shared 7), and target was deleted. `quire validate` passes on the changed FR and TC files.

## Dispositions (round 5)

Round 5, reviewed at ece9f6b (delta 81efef7..ece9f6b, prose only; cargo not run, per the dispatch).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-017 | fixed | ece9f6b |
| FND-018 | fixed | ece9f6b |

I traced UE-10 and UE-11 by hand on the TC-338 secure vault. HB-01 is HP-2 over one subject, and FR-403 admits a state constraint for it.

UE-10. At joint position 0 every component has `l = 0`, so the guard `self.l = 0` holds, both `step` instances are enabled, and `terminal when` is not read. The body holds. Under the match, the joint successors are `(step(0), step(0))`, which keeps `l = 0` and is enabled, and `(step(1), step(1))`, which gives `l = 1` with no enabled transition. So the first undefined evaluation in canonical breadth-first order is the `terminal when` read at joint position 1. Component `a` comes first in quantifier order. The traces are one `step(1)` each and there is no `transition`. That is correct.

UE-11. The constraint reads `lookup` of the absent `w` at every state, so it is undefined first at `a`'s initial state at joint position 0. The traces have no steps and there is no `transition`. That is correct.

The TC-349 table is now contiguous: SG-08 is followed directly by SG-09. No new findings.
