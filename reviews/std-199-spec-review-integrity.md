---
id: SR-13067
title: "Integrity review of QSpec PR #285 (STD-199)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@4d0b395; proposals/checked-package-v2/schema.json, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/objects/interfaces/FR-323-native-runtime-envelope.md, spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md, spec/objects/foundation/FR-272-typed-diagnostic-cause.md, spec/functional/analysis/FR-375-declare-model-refinement.md, proposals/quire-v1/definitions/native-diagnostics.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-432
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-323
    type: reviews
---

## Summary

Ticket: STD-199. Checks internal consistency of the lock schema, FR-322/FR-323 shapes, the diagnostic vocabulary rows and FR-375's node body text.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322-AC-75 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| schema-RI | proposals/checked-package-v2/schema.json | examined |
| FR-323-AC-17 | spec/objects/interfaces/FR-323-native-runtime-envelope.md | examined |
| FR-323-shape | spec/objects/interfaces/FR-323-native-runtime-envelope.md | context_only |
| FR-271-uc | spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md | examined |
| FR-271-unimpl | spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md | context_only |
| FR-272-AC-20 | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| FR-375-node | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new `RefinementInternal.internal` schema carries `uniqueItems: true`, so a repeated spelling (necessarily an identical string) fails the V2 schema and never reaches the reader, yet FR-322's prose, FR-322-AC-75 and TC-233's expected results require a repeated spelling to refuse `invalid_package`/`conflicting-definition` with both entries as loci. The TC-233 repeated-spelling vector cannot pass as written. Either drop `uniqueItems` from `internal` (the outer array correctly lacks it) or move the repeated spelling to the schema-failure list. | proposals/checked-package-v2/schema.json:28; spec/objects/interfaces/FR-322-checked-package-artifact.md:125-140,1583; spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:404-409 |
| FND-002 | medium | FR-323 now says a replay request carries `refinement_internals` on `package` and on each `dependencies` entry, but the closed shapes it lists in the same paragraph are unchanged: `package` is still `{package_id, contract_version, sources, dependencies}` and each entry `{identity, package_id, sources}`. A closed-envelope reader following those shapes refuses the new member. AC-17 also locates an unresolving spelling only under `/package/refinement_internals`, not under `/package/dependencies/<n>/refinement_internals`. Add the optional member to both shapes and state the dependency pointer. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:24-39,454 |
| FND-003 | medium | FR-271 now holds two codes for one meaning: `unsupported_construct` (cause `not-yet-implemented`, "a recognized declaration form the producer has not built") and the existing `unimplemented_capability` ("valid complete-V1 meaning is not implemented by the selected producer"). STD-199 asked QSpec to choose between them; the PR picks the first but adds no rule saying when `unimplemented_capability` applies instead, and FR-452 classes only the first as `unsupported`. The three descriptions of the new cause also differ (FR-271 "declaration form", FR-272 "valid complete-V1 feature", native-diagnostics "unbuilt feature"), while `refinement-internal-list` is a host input, not a declaration form. State the split (or retire `unimplemented_capability`) and use one description. | spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md:38,79,189; spec/objects/foundation/FR-272-typed-diagnostic-cause.md:67; proposals/quire-v1/definitions/native-diagnostics.md:62 |
| FND-004 | low | The sentence adding `abstract_internal` orders it by UTF-8 bytes of its spellings, and the next sentence says "Every array is sorted ascending by the node-key digest of its first named declaration"; `abstract_internal` is an array of strings with no node key. Scope the "every array" rule to the row arrays. | spec/functional/analysis/FR-375-declare-model-refinement.md:271-276 |
| FND-005 | low | "two packages whose sources differ only by the lock member's presence" misstates the vector: the sources are identical and only the supplied list (hence the lock member) differs. Say "two compilations of the same sources, one with a non-empty list and one without". | spec/objects/interfaces/FR-322-checked-package-artifact.md:1583 |

## Verdict

Not mergeable on this lens until FND-001 (schema contradicts AC-75) and FND-002 (closed replay shapes omit the new member) are fixed. The lock member's other rules are consistent: `minItems: 1` matches "absent, never `[]`", the outer array correctly omits `uniqueItems` so a repeated `refinement` reaches the reader, the member stays out of `IdentityPreimage`, and FR-272-AC-20 matches the native-diagnostics and FR-272 rows.
