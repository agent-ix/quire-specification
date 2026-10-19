---
id: SR-13068
title: "EARS review of QSpec PR #285 (STD-199)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@4d0b395; spec/functional/analysis/FR-375-declare-model-refinement.md, spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/objects/interfaces/FR-323-native-runtime-envelope.md, spec/objects/foundation/FR-272-typed-diagnostic-cause.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-432
    type: reviews
---

## Summary

Ticket: STD-199. The PR edits no FR statement sentence; it adds normative body prose and criteria. `quire validate` reports no EARS diagnostic on FR-375 or FR-432. One permissive clause found.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-375-unbuilt-may | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-internal | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-432-spell | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | examined |
| FR-432-defects | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | "A producer may not have built every form this requirement ... specify" introduces optional conformance with a permissive "may" and no optional-feature trigger. EARS form: "Where the producer's support set omits <feature>, the checker shall refuse a declaration that uses it `unsupported_construct`/`not-yet-implemented` ...". As written it reads as permission, not a requirement. | spec/functional/analysis/FR-375-declare-model-refinement.md:158-166 |

## Verdict

Mergeable on this lens; FND-001 is wording. The internal-list rules in FR-432 and the lock and replay rules in FR-322/FR-323 are stated as determinate behaviour with named codes and loci.

## Dispositions

Round 1, against head 986087c.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 986087c: rewritten in EARS optional-feature form |
