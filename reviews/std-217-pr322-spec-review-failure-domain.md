---
id: SR-13694
title: "Failure-domain review of QSpec PR #322 (STD-217) refinement step occurrences"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@dcc70c07d1a0905407a5242fbbbb5620727aae33 vs main 6ab82c5; spec/objects/interfaces/FR-322-checked-package-artifact.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
---

## Summary

Ticket: STD-217 (PR #322, head dcc70c07d1a0905407a5242fbbbb5620727aae33). Checks the reader rule for refusal loci and edge cases it leaves open. The zero-row case (`n = 0`, only ordinal 0) follows from the text and needs no finding.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 refinement step occurrences | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-89 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-52 | spec/objects/interfaces/FR-322-checked-package-artifact.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The rule says which owner entry is reported first, but not when the check runs relative to the reader's other `source_map` and graph checks (schema, graph shape, FR-322-AC-52's owner/source agreement, stale node keys), nor in which order several refinement owners are checked. A package with two defective owners, or with an AC-52 defect and a step-occurrence defect, has no single specified refusal. Name the stage and the owner order (for example ascending node-id digest, as the carrier checks use). | spec/objects/interfaces/FR-322-checked-package-artifact.md:670-672 |
| FND-002 | medium | The rule requires the `generated` ordinals, but not the `declaration` occurrence that the ordinal-0 region is defined from. If an owner has no `declaration` entry, the region the initial occurrence "requires" is undefined and no refusal is named. State that the `declaration` occurrence is required, and its refusal locus (`/source_map`, as for a missing ordinal), and add it to FR-322-AC-89. | spec/objects/interfaces/FR-322-checked-package-artifact.md:658-672,1804 |

## Verdict

Two medium findings; fix in this PR.
