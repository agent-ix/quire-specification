---
id: SR-7836
title: "Integrity analysis of allInstances coverage accounting"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@2923f8a0debaee0102d8889aed52925c9174bf2f; proposals/quire-v1/definitions/value-accounting.md"
review_set: subset
---

## Summary

Checked the changed rule for consistency, atomicity, and testability against FR-153's coverage predicate and FR-151's effective-type conformance relation. The rule preserves the exact closure condition, leaves member walking and charging order explicit, and introduces no conflicting charge point or new charge row. No integrity defect found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No issues found | - |

## Scope

- {id: accounting-allInstances, path: "proposals/quire-v1/definitions/value-accounting.md", lines: "414-430", role: examined, excerpt: "Before its first charge, each query decides whether its `T` conforms to at least one declared population member type using the effective view; this decision makes no charge. FR-151 defines conformance for the same effective type, so `T` conforms to itself. The query then walks every admitted member of `p` in canonical reference-key order, charging one `population.visit` before each member walked whether or not its most-specific type conforms to `T`, and selects the member, without a charge, exactly when that type conforms to `T`."}
- {id: FR-153, path: "spec/functional/type-model/FR-153-query-closed-model-environments.md", lines: "62-77", role: examined, excerpt: "Subtype closure for a queried type `T` holds exactly when every effective type that conforms to `T` is covered by the population declaration's member types; otherwise `incomplete_population`/`unclosed-subtypes`. A member type covers itself and every type that conforms to it."}
- {id: FR-151, path: "spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md", lines: "31-36", role: examined, excerpt: "Type `S` conforms to type `T` exactly when `S` and `T` are the same effective type or a chain of declared supertypes leads from `S` to `T`."}
- {id: accounting-charge-row, path: "proposals/quire-v1/definitions/value-accounting.md", lines: "44-47", role: examined, excerpt: "| population | one `population.visit` before each member walked by `allInstances<T>(p)`, conforming or not, then `collection.bound` and `collection.result-retain` |"}
