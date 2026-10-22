---
id: SR-11741
title: "code review of quire-specification PR 265 test oracles (process and state-machine owners)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@71f8ac5ec0f95f3e16c3dbf3394b517ec3806286; tests/checked_package_v2.rs, tests/model_member_types.rs, proposals/checked-package-v2/fixtures/positive-all-families.json, proposals/checked-package-v2/fixtures/positive-clause-operations.json, proposals/checked-package-v2/dependency-selection-vectors.json, proposals/checked-package-v2/node-identity-vectors.json"
review_set: subset
---

## Summary

Ticket: QSL-815. This is the code-review pass with the rust-review lane, over
the two Rust test oracles and the re-keyed data.

`frame_body_defects` admits a process or state-machine `semantic_type`. It
pushes a `Modifies` meaning-join defect at the entry's declaration for a
`receiver` field entry, and a `DeletesScoped` defect at the frame node for
`{kind: "receiver"}`. Both use `malformed-declaration`, so they rank through the
existing precedence. `ANCHOR_CONTEXTS` is shared by the anchor `context` join
and the invariant anchor join. The `object` family, model-declaration, owner
join and reference-target predicates add process and state machine, and the
owner join also adds persistence interface. The `Reference<O>` target and the
`object` family leave the persistence interface out.

The two new reader tests rewrite the published packages through
`rekey_stale_nodes` (plus `rekey_abstraction_nodes`). They assert through
`read_package_semantics`, the normative reader path. Two focused runs passed
(log /home/peter/dev/worktrees/logs/qspec265-focused.log, EXIT=0):
`tc_233_a_process_or_state_machine_*` and
`tc_280_*process_or_state_machine*`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Recorded, pre-existing. With a model index, the wire oracle's `anchor_refusal` resolves the operation through `mm_declared_member`, which reads own members only. So an inherited-only operation (AP-03 `(SubFlow, "go")`) gets `missing-name` there. FR-342-AC-4 and AC-7 give `malformed-declaration`. The TC-280 oracle (`anchor_operation`) decides it correctly, and no wire vector exercises the case. | tests/checked_package_v2.rs:3486-3520, 3151 |

## Verdict

Clean for this PR's scope. The oracle changes match FR-340, FR-341 and FR-342
as revised. No tautological assertion: every new test checks recorded
expectations, or a specific node, against the reader. The re-keyed data comes
from repo tooling, and the three values I recomputed match. Panics are limited
to test `expect`s on authored fixtures.

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | accepted-no-change | Low, pre-existing, recorded only. No wire vector exercises an inherited-only operation through `anchor_refusal`, and the TC-280 oracle decides it correctly. Re-checked at 281e94e: the new check 5 uses the same `mm_declared_member` lookup, which only matters once the operation has resolved. |
