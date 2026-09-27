---
id: SR-725
title: "Gap analysis — STD-111 state clause, operation anchor, frame and reaches_field"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@7160d58dcf7a7182ca64c6f61c5aa10df336869d; PR #163 diff: FR-322 (Reaches over a field, AC-17, AC-38), FR-340, FR-341, FR-342, TC-233, TC-280, TC-281, spec/complete-v1/tests.md, tests/checked_package_v2.rs, tests/model_member_types.rs, proposals/checked-package-v2"
review_set: subset
---
# Gap analysis — STD-111 state clause, operation anchor, frame and reaches_field

## Summary

Ticket: STD-111. Every new or changed criterion maps to a TC row in
`spec/complete-v1/tests.md` and carries a `#[trace]` tag on a real test:
FR-322-AC-38 to TC-281; FR-340-AC-1..12, FR-341-AC-1..4/6..9 and
FR-342-AC-1..3/5 to TC-233; FR-340-AC-13, FR-341-AC-5 and FR-342-AC-4 to
TC-280. All five ticket items are covered by spec and tests:
state-clause body, operation-anchor body, frame `semantic_type`/role
with (declaring node, field name) entries, `reaches_field`, and the clause
operation and member kind.

The gap is in what the full reader (`read_package_semantics`) checks. That
is the reader TC-233 calls normative. The model-resolution half of each
new rule (field-name resolution, anchor operation resolution and
pre/postcondition signatures) exists only in the standalone TC-280
oracles over the member-type vectors. The two halves never run together
on one package. As a result, the published clause package's anchor
operation and precondition signature are never checked against a model,
and FR-340's precedence for resolution defects is never exercised by a
reader.

## Verdict

**Changes requested.** Two medium findings and one low finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The full reader's frame step never resolves a field entry's `name`. `frame_body_defects` checks only the declaring node's tag/form and dependency membership. FR-340 "Field entries" makes a name-resolution refusal (`missing-name`, `missing-selection`, `stale-node-key`, `ambiguous-name`) a meaning-join defect that takes part in the frame precedence. A frame naming `(Account, "no_such_field")` is admitted by `read_package_semantics`. The `undeclared-field-entry` vector covers only a declaring node missing from `dependencies`. FR-340-AC-6's by-name tie-break is tested only by sorting hand-built `FrameDefect` values. Either add field resolution to the reader step, with a vector where a resolution defect meets a canonical-order defect, or scope FR-340's precedence text to what the reader resolves. | tests/checked_package_v2.rs:2447-2510; spec/objects/interfaces/FR-340-checked-package-frame-location.md:76-97,190-201 |
| FND-002 | medium | The full reader's state step leaves out FR-342 step 4 (Operation) and, for a precondition or postcondition, FR-341 step 3 (Signature) beyond `self`. `anchor_refusal` stops after the `semantic_type` agreement. `clause_refusal` checks only the `self` type and the invariant's arity. A precondition missing an operation parameter, or an anchor naming a nonexistent operation, is admitted by `read_package_semantics`. TC-233's procedure says it reads "arity" mutations, but only invariant arity is exercised. FR-341-AC-5 and FR-342-AC-4 are covered in isolation by TC-280, but no test checks the published clause package's own anchor/precondition against a model. | tests/checked_package_v2.rs:2672-2745; spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:135-138; spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:108-112 |
| FND-003 | low | FR-341-AC-7's second half ("runs ... before the operation step") has no vector. The tests show frame-before-anchor and anchor-before-clause, but no package carries a state defect and an operation defect together. | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:116-118; tests/checked_package_v2.rs:7611-7791 |
