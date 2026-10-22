---
id: SR-14742
title: "gap analysis of STD-237 changed ACs against TC rows and the harness (QSpec PR #347)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@6d280af1f597c7cc709cb2f1e673d2e5f467e7ae; spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, tests/checked_package_v2.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-233
    type: reviews
---
# SR-14742: gap analysis of STD-237 (QSpec PR #347)

## Summary

Ticket: STD-237. Plan completion: not assessed. Mapping:

- FR-322-AC-37 (changed) maps to the TC-233 dependency-call paragraph and its
  expected result, and to
  `tc_233_a_dependency_call_is_admitted_and_its_mutations_refuse_at_their_loci`,
  traced `#[trace("TC-233", "FR-322-AC-37", "FR-322-AC-85")]`.
- FR-322-AC-85 (verification changed from Inspection to Test) maps to the
  same test, which checks the `package`, `node` and term loci at
  `arguments/0`, `arguments/1` and the body root.

`cargo test --test checked_package_v2 --test backend_provider_v1` passes
(118 and 14). Five harness mutations each make the new test fail on the
intended vector: dropping the callee check (`term-as-a-non-callee-argument`),
treating every signature as package-independent
(`function-returns-a-declared-record`), moving the `missing-name` locus to
the term (`node-carries-no-declaration`), skipping the supply check
(`no-dependency-supplied`), and dropping the `declaration` filter
(`node-carries-no-declaration`). The library is added to
`positive_fixtures()`, so every all-positive check covers it.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The test never checks the `invalid_semantic_graph` locus that AC-37 now states. `dependency_call_outcome` maps every `ReadRefusal::GraphShape(id)` to `/semantic_graph/nodes/{index}/dependencies`, but the oracle's `GraphShape` carries only a node id. So a graph-shape refusal of node 3 for any other reason, such as a contradictory `semantic_form`, would still match the `target-listed-in-dependencies` vector's locus. Give the oracle a locus or cause that separates a dependency-list refusal from other graph-shape refusals, and assert it. | tests/checked_package_v2.rs:5988-5994; spec/objects/interfaces/FR-322-checked-package-artifact.md:1787 |

## Verdict

Both changed ACs have a TC row and a traced test, and that test fails when
any of the five checked rules is removed. The one gap is that the harness
writes the graph-shape locus itself (FND-001). This is a minor harness-hygiene
finding and does not block merge.
