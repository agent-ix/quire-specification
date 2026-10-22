---
id: SR-726
title: "Spec review (integrity) — STD-111 state clause, operation anchor, frame and reaches_field"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification; PR #163 diff against origin/main: FR-322, FR-340, FR-341 (new), FR-342 (new), interface_004, spec.md, TC-233, TC-280, TC-281, spec/complete-v1/tests.md, proposals/checked-package-v2/README.md, schema.json, node-identity-preimage.schema.json; cross-checked against FR-208, FR-013, AD-016, docs/qualification-dependencies.md"
review_set: subset
---
# Spec review (integrity) — STD-111 state clause, operation anchor, frame and reaches_field

## Summary

Ticket: STD-111. This is an integrity pass over the PR #163 spec diff.

**Dangling references from removing the three node forms: none.** At the
reviewed sha, a repository-wide search (excluding `reviews/`, which is
history) finds `field_declaration`, `operation_declaration` and
`clause_member_declaration` only in FR-340-AC-12 and in the three negative
schema cases in `tests/checked_package_v2.rs` that assert the forms are
refused. `schema.json`'s `ModelNode` enum, FR-340's prose and ACs, TC-233's
procedure and the frame vectors were all rewritten. FR-208's "one
[declaration] per field, relationship, operation and clause member"
(FR-208:76-77) is about IR/effective-view declarations, not wire nodes, so
it is consistent with "a member is named by its declaring node and its
name". AD-016's frame references name only the `state`/`frame` node.

These all agree with each other: the reader order across FR-322,
FR-340, FR-341, FR-342, interface_004 and README (declaration → frame →
state [placement → anchors → clauses] → operation); the catalog rev/digest
in FR-322, the README, `docs/qualification-dependencies.md` and the test
constant; the `state_clause` member and operator class in both schemas;
the TM rows; and the `spec.md` contains edges.

## Verdict

**Changes requested.** Two medium findings and two low findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The `value`/`parameter` node form is added (FR-322, `ValueNode`) and is the type carrier for every clause parameter. Its body and structural-node preimage are "specified separately" (FR-341 Status), but no owning FR or ticket is named. The published clause fixture already commits to a concrete body, `aggregate[binding("name", text), binding("level", integer)]`, which no spec states. Its occurrence role (`expression`) is also unspecified. Node identity is content-derived, so two producers encoding `self` differently get different node and package ids. Specify the body/role in FR-341 (or a sibling FR) with an AC, or name the owning ticket in the Status line. | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:179-182; spec/objects/interfaces/FR-322-checked-package-artifact.md:113-117; proposals/checked-package-v2/fixtures/positive-clause-operations.json:64 |
| FND-002 | medium | FR-342 says "One `operation_anchor` node and one `frame` node stand for one (declaring object type, operation name)", but no rule or refusal enforces it. Two anchors for the same `(C, op)` with different frames, a frame bound by two anchors or by none, and a precondition and postcondition of one operation anchored at two different anchor nodes are all admitted by the stated reader order. Add the uniqueness refusal (cause, locus, position in the state step) with an AC, or restate the sentence as producer guidance with no reader meaning. | spec/objects/interfaces/FR-342-checked-package-operation-anchor-body.md:50-52 |
| FND-003 | low | The `reference_edge` row of the constraint table says "as 'Reaches over a field' below decides", but that section comes earlier in FR-322 (line 500; the table row is at line 590). Change "below" to "above". | spec/objects/interfaces/FR-322-checked-package-artifact.md:590 |
| FND-004 | low | FR-340-AC-12 bundles three independent checks: the `semantic_type` refusal and its precedence, the `generated` occurrence-role schema rule, and the absence of three `model` forms. The last is stated as a negative that names removed alternatives. Split it into atomic ACs and drop the negative clause; the closed `ModelNode` enum in the schema already states what is. | spec/objects/interfaces/FR-340-checked-package-frame-location.md:230 |
