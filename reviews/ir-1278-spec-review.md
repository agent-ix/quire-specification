---
id: SR-202611022230
title: "Spec review of QSpec PR #606: temporal_clause_names per claim occurrence"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; spec/objects/interfaces/FR-322-checked-package-artifact.md (Temporal clause names, AC-342..354); spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md (declared name, AC-16); spec/test-cases/TC-233, TC-303; proposals/checked-package-v2/schema.json, temporal-clause-name-vectors.json, seven positive fixtures; tests/checked_package_v2.rs (decide_temporal_clause_names)"
review_set: base
---

## Summary

Ticket: IR-1278 (also QSL-1544); PR quire-specification#606 at head a188eabcf17cc1409a901445284ac7474aeb37e7, diff `origin/main...HEAD` (base b1feb4841).

The carrier is a sound design and implementable. `temporal_clause_names` is a closed, sorted sidecar keyed by (declaration, claim occurrence), modelled on `schedulability_claims`, and it leaves the node and `node_id` name-free. I read the Contract IR reader at main (de8919b0): `schedulability::validate_carrier` already keys claim occurrences by ordinal and checks join, occurrence, repeated key, order and coverage, and `temporal_binding::validate` runs before the structural and clause-body steps, so the stated slot (after the clock-binding checks, before the FR-370 joins) is available. A second `temporal_bindings` row for one declaration refuses today with `invalid_package`/`timed-node-reference` at `/temporal_bindings/{i}` (the `seen.insert` check), exactly CN-R9's locus.

`cargo test --all-targets` at the head passes with no failures. Mutation spot-checks on `decide_temporal_clause_names`: disabling the canonical-order check fails CN-R7, and disabling the coverage check fails CN-R1. I walked CN-R1..R9 on paper under the stated order and each reaches its stated refusal and locus. The seven positive fixtures carry one name row per claim occurrence and validate against the schema. No FR-322/FR-370 AC id or CN vector id collides with main, #600 or #602; FR-322-AC-342..354 are free. `quire validate` on FR-322 reports the missing `id` and `record-invalid` errors, which are in the file's frontmatter and not in the diff. FR-370, TC-233 and TC-303 validate. #602 conflicts textually with this PR (see FND-005).

The FR-337 resolution, one `temporal_bindings` row per clause node, rests on a premise that is not true for the clock configuration (see the failure-domain review SR-202611022231, FND-001). The findings below are the ones local to this PR's own ACs, vectors and tests.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-322-AC-352 states four orderings (join, repeated key, order, coverage) but only join-before-coverage has a vector (CN-R8). Nothing witnesses repeated-key-before-order or order-before-coverage, so a reader that swaps those passes every vector. The AC also bundles four independent claims in one row | spec/objects/interfaces/FR-322-checked-package-artifact.md:4340; proposals/checked-package-v2/temporal-clause-name-vectors.json (CN-R8) | correct-requirement-no-evidence |
| FND-002 | medium | The text and the test decider run the checks as whole passes over all rows (join for every row, then repeated keys, then order, then coverage). The reader this models (`schedulability::validate_carrier`) runs join, occurrence, repeated key and order per row, then coverage. The two differ on a package with a repeated key at row 1 and a bad declaration at row 2: the reader refuses the repeated key, the decider the declaration. The wording is copied from the schedulability section, so the ambiguity is inherited, but this AC makes the order normative and the test decides it the other way | spec/objects/interfaces/FR-322-checked-package-artifact.md:3418-3427; tests/checked_package_v2.rs (decide_temporal_clause_names) | wrong-requirement |
| FND-003 | low | FR-322-AC-349 refuses an `occurrence` that "is not a `claim` occurrence". CN-R5 names an ordinal the node does not have. No vector names an ordinal the node has under another role, so the claim-role filter is not exercised | spec/objects/interfaces/FR-322-checked-package-artifact.md:4337; proposals/checked-package-v2/temporal-clause-name-vectors.json (CN-R5) | correct-requirement-no-evidence |
| FND-004 | low | CN-S9 is listed under `schema_refusals` with `expected.code` `invalid_semantic_graph`, and FR-322-AC-342 claims the reader refusal at `/semantic_graph/nodes/{N}/declaration`. The test only asserts the schema refuses the package and compares the code string; no check reaches the reader locus. The reader half of AC-342 is asserted by a string in the vector. Separately the vector note and TC-233 cite "QSL PR #2156", which is process history and not system behaviour | proposals/checked-package-v2/temporal-clause-name-vectors.json:221; spec/objects/interfaces/FR-322-checked-package-artifact.md:4330 | correct-requirement-no-evidence |
| FND-005 | low | #602 (STD-650) conflicts with this PR: it adds `fixtures/protocol-node-base.json` from an older base and edits `schema.json` and the README. If that fixture holds a `temporal_clause`, whichever PR lands second needs name rows in it, or `every_positive_fixture_names_each_claim_occurrence_of_each_temporal_clause` fails | proposals/checked-package-v2/fixtures/protocol-node-base.json | missing-requirement |

## Verdict

Mergeable after the owner ruling on the shared-clause clock configuration (SR-202611022231 FND-001) is recorded and the text is corrected. The carrier, refusals, loci, id allocation and fixtures are sound and the suite is green. FND-001 and FND-002 should be fixed in this PR (add two vectors, state the reader order per row or per pass, and align the decider). FND-003 to FND-005 are small. No process language beyond the one PR number in FND-004 was added.
