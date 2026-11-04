---
id: SR-202611022032
title: "Spec review of PR #603: protocol_locations is application-only (FR-322-AC-310, QSL-852)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@90afa09b (final); git diff origin/main...HEAD (PR #603): FR-322 protocol_locations paragraph, FR-322-AC-310, TC-233, vector PL-R3-leaf-row, tests/checked_package_v2.rs; ticket QSL-852"
review_set: subset
date: '2026-11-02'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
---

## Summary

Ticket: QSL-852. The PR adds the application-only rule to FR-322's protocol_locations paragraph, FR-322-AC-310 (TC-233), vector PL-R3-leaf-row and a test arm. The rule sits in the right clause: FR-340 is the frame body and does not mention protocol_locations. No id collisions with main or open PRs (AC-310 free, PL-R3 free). `cargo test --test checked_package_v2` passes (162/162). `quire validate` output for FR-322 and TC-233 is identical on main and the PR (exit 3 both, same diagnostics). Mutation check: changing the vector path from literal to application, or its expected_code, fails the test arm, so the arm is not vacuous against vector edits; but it exercises no reader.

## Verdict

PASS: mergeable at 90afa09b. FND-001..005 are fixed across three disposition rounds; no finding is open.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 protocol_locations paragraph | spec/objects/interfaces/FR-322-checked-package-artifact.md:1099-1131 | examined |
| FR-322-AC-310 | spec/objects/interfaces/FR-322-checked-package-artifact.md:4108 | examined |
| FR-322-AC-291 | spec/objects/interfaces/FR-322-checked-package-artifact.md:4081 | context_only |
| PL-R3-leaf-row | proposals/checked-package-v2/checked-term-data-vectors.json:2520 | examined |
| TC-233 | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-310 is backed only by a vector self-consistency check; no reader is run, so no implementation can fail it. | tests/checked_package_v2.rs:20586 |
| FND-002 | medium | PL-R3-leaf-row defines no node graph; the leaf is only a path-segment name, so the refusal is indistinguishable from AC-291's absent-node refusal. | proposals/checked-package-v2/checked-term-data-vectors.json:2520 |
| FND-003 | low | AC-310 is compound, overlaps AC-291, and exercises only literal, not Local or reference. | spec/objects/interfaces/FR-322-checked-package-artifact.md:4108 |

## Open decision

Code and locus invalid_package/invalid-value at the row's application member match EQ-R1 and PC-R2; FR-322-AC-291 records none for the non-application case. Not a defect; the planner should confirm the choice.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | PL-R3 graph nodes carry only node_id, node_tag and semantic_form and are never validated against the schema's ExpressionNode/ValueNode, and the decider refuses every `value` node while the rule text names only literal and parameter. | proposals/checked-package-v2/checked-term-data-vectors.json; tests/checked_package_v2.rs:decide_protocol_locations |

## Dispositions

Round 1, head bc207688.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Test-local decide_protocol_locations is run on each PL-R3 vector. Mutation: disabling the refusal gives Ok(()) vs Err for PL-R3-leaf-literal and fails the test; removing the reference-as-leaf clause also fails. Restored; 162/162 pass. |
| FND-002 | fixed | Each PL-R3 vector carries a graph holding the admitted leaf and an admitted expression/call; the test asserts the row names a graph node, so absent/foreign (AC-291) is excluded; value/literal, value/parameter and expression/reference are real forms in schema.json (ValueNode, ExpressionNode enums). |
| FND-003 | fixed | AC-310 rule, AC-311 refusals (4 vectors incl. leaf copying enclosing location at index 1), AC-312 positive rows; paragraph states it refines AC-291 and keeps absent/foreign under AC-291. |
| FND-004 | still-open | Low; new this round, not yet routed. Mergeable without a fix. |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | "Application" now means every non-reference ExpressionNode form (unary, binary, let, quantify, ...), a wider reading than AC-291's "expression/application node"; a design decision not in the planner ruling. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1124-1135 |

## Dispositions (continued)

Round 2, head a1bd441e.

| FND | outcome | reason |
| --- | --- | --- |
| FND-004 | fixed | Graph nodes are validated against SemanticNode in the test; the leaf set is every ValueNode form plus expression/reference and the rule text and decider agree; the test loops over the schema's ValueNode and ExpressionNode enums. Mutations (record_value treated as application; let treated as leaf) both go red. |
| FND-005 | still-open | Low, new this round; mergeable. Owner to confirm that every non-reference expression form is an application for protocol_locations. |

## Dispositions (round 3)

Round 3, head 90afa09b.

| FND | outcome | reason |
| --- | --- | --- |
| FND-005 | fixed | The rule now says a valid row names an expression/application node exactly as FR-322-AC-291 states and does not widen application; the leaf code is the only code the rule assigns; let, conditional, absent and foreign nodes stay under AC-291 with no new code. The decider returns a refusal only for leaves (value nodes and expression/reference) and continues otherwise. Mutations (parameter dropped from the leaf set; leaf test inverted) both go red; 162/162 pass. |
