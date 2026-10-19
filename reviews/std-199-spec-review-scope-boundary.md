---
id: SR-13070
title: "Scope-boundary review of QSpec PR #285 (STD-199)"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-specification@4d0b395; spec/functional/analysis/FR-375-declare-model-refinement.md, spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-310-refinement-declaration-admission.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-432
    type: reviews
---

## Summary

Ticket: STD-199. Checks where the list, its identity effect and the support set are allocated: compile input vs source, node body vs identity preimage, producer capability vs request.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-375-node | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-unbuilt-may | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| TC-310-m16 | spec/test-cases/TC-310-refinement-declaration-admission.md | examined |
| FR-375-internal | spec/functional/analysis/FR-375-declare-model-refinement.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `abstract_internal` stores source spellings that embed the refinement's local abstract alias (`Spec::Twice::finish`), while every other member of the node body is key-addressed (`abstract` and `protocol` are `PackageNodeKey`s, rows carry node keys). Renaming the alias (`as Spec` to `as S`), with the list renamed to match, changes the node id, `package_id` and obligation identities with no change of meaning. Store resolved classes by node key (with the class keyword) in the node, keeping the spelling only in the input and the lock. | spec/functional/analysis/FR-375-declare-model-refinement.md:271-280; spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:123-126 |
| FND-002 | medium | The support set is "a capability of the producer and not a request parameter", yet AC-14 is verified by Test (TC-310) in "a producer whose support set omits one feature at a time". Once a producer builds all three features no test can construct that producer, so AC-14 can only pass vacuously or through an unstated test seam. QSL FR-135-AC-5 scopes the same vector to the build's actual state ("it does not select a support set"). State the seam (a test-only producer configuration) or verify AC-14 by Inspection/Analysis per producer. | spec/functional/analysis/FR-375-declare-model-refinement.md:158-166,303; spec/test-cases/TC-310-refinement-declaration-admission.md:89-91 |

## Verdict

Mergeable on this lens after FND-001 and FND-002 are ruled on. Clean: hosting the list as a compilation input (no grammar row, no profile parameter), recording it as a lock-only member outside the identity preimage, and carrying it on replay are consistent allocations; the identity effect enters through the node, as QSL asked.
