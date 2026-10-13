---
id: SR-9003
title: "ears-conformance review of quire-specification PR 246 (variant-typed members)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@852c515d143bb2fbdce5ed186cd68e686493dfb6; spec/objects/foundation/FR-208-quire-meaning-vocabulary.md, spec/functional/type-model/FR-154-admit-domain-package-model.md"
review_set: subset
---

## Summary

Ticket: QSL-748. EARS and ambiguity review of the new and amended statements. The new
FR-208 statements fit EARS. The admit rule is event-driven ("When a field, an operation
parameter or an operation result has a `typeRef` that names a variant type, the binder
SHALL admit the member"), and the case-literal typing is ubiquitous ("The checker SHALL
give a case literal the static type..."). `quire validate` reports no new grammar
diagnostic on any touched file. Two wording defects follow.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | Removing the last list item left a list with no conjunction. FR-208-AC-15 now reads "no case, a case with no source span, a case whose identity is not ..., a construct of a shape other than `enumeration` are ...", and FR-154-AC-11 reads "... a construct of a shape other than `enumeration` each refuse". Restore "and" before the last item. | spec/objects/foundation/FR-208-quire-meaning-vocabulary.md:403; spec/functional/type-model/FR-154-admit-domain-package-model.md:216 | wrong-requirement |
| FND-002 | low | FR-154's list headed "variants refusals name these members" now contains a bullet that states an admission ("... is admitted and raises no refusal of its own"). A list of refusal loci should not hold one. Move the sentence into the prose above the list, or keep the bullet only for the meanings that are still refused (see SR-9002 FND-008). | spec/functional/type-model/FR-154-admit-domain-package-model.md:108-110 | wrong-requirement |

## Verdict

Two low findings. FR-208-AC-16 and FR-208-AC-17, FR-322-AC-71 (apart from SR-9002
FND-001) and FR-141-AC-7 are direct, falsifiable assertions with named codes, causes and
paths. The unwritable-case bullet is stated as an outcome, "admitted with its type but has
no spelling in source", rather than a deferral, which is acceptable.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 5a72cdc: "and" restored in FR-208-AC-15 and FR-154-AC-11 |
| FND-002 | fixed | 5a72cdc: the admission sentence moved out of the refusal-locus list into prose |
