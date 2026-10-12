---
id: SR-7835
title: "Base spec review of allInstances coverage accounting"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@2923f8a0debaee0102d8889aed52925c9174bf2f; proposals/quire-v1/definitions/value-accounting.md"
review_set: subset
---

## Summary

Reviewed the changed allInstances accounting paragraph and its normative cross-references in QSpec FR-153 and FR-151. The conformance formulation is semantically equivalent to FR-153's subtype-coverage definition because FR-151 makes conformance reflexive and defines it by transitive supertype reachability; no defect found in the normative change.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No issues found | - |

## Scope

- {id: accounting-allInstances, path: "proposals/quire-v1/definitions/value-accounting.md", lines: "414-430", role: examined, excerpt: "Before its first charge, each query decides whether its `T` conforms to at least one declared population member type using the effective view; this decision makes no charge. FR-151 defines conformance for the same effective type, so `T` conforms to itself. The query then walks every admitted member of `p` in canonical reference-key order, charging one `population.visit` before each member walked whether or not its most-specific type conforms to `T`, and selects the member, without a charge, exactly when that type conforms to `T`."}
- {id: FR-153, path: "spec/functional/type-model/FR-153-query-closed-model-environments.md", lines: "62-77", role: examined, excerpt: "Subtype closure for a queried type `T` holds exactly when every effective type that conforms to `T` is covered by the population declaration's member types; otherwise `incomplete_population`/`unclosed-subtypes`. A member type covers itself and every type that conforms to it. Object closure is decided at binding admission before any admission charge, subtype closure before the query's first charge, and no partial collection is returned."}
- {id: FR-151, path: "spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md", lines: "31-36", role: examined, excerpt: "Type `S` conforms to type `T` exactly when `S` and `T` are the same effective type or a chain of declared supertypes leads from `S` to `T`. `S` is strictly narrower than `T` exactly when it is a proper descendant of `T` in that closed graph; equal shapes of distinct declarations never make one narrower."}
- {id: FR-153-AC-5, path: "spec/functional/type-model/FR-153-query-closed-model-environments.md", lines: "97-99", role: examined, excerpt: "All-instances includes closed subtype populations exactly once and emits no partial population when closure is unknown."}
