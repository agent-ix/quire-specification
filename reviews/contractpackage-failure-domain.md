---
id: SR-5651
title: "ContractPackage decision: failure-domain review"
type: SpecReview
analysis: failure-domain
scope: "interface_012, FR-330, TC-931; FR-322 and Contract IR FR-035 reference context"
review_set: all
---

## Summary

No open findings remain. Local references, dependency references, recursive graph meaning, and package-admission failure now have distinct rules.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No open findings; this sentinel row records the schema-required empty disposition. | FR-330 |

## Analysis

The initial same-package-only reference wording contradicted FR-322 selected-dependency function callees. The corrected text preserves package-qualified dependency identity and explicitly disallows flattening it into a local node. Explicit recursion groups remain legal while serialized values remain finite. Absent and wrong-tag protocol targets are negative controls. A stale selected dependency rejects admission before lowering; sibling-disposition preservation applies only to valid admitted requests.

## Validation

Scoped validation of the three changed documents passed. Whole-spec validation failed on 135 pre-existing documents, including FR-200 and FR-201 structural-model errors; it is not reported as passing. The review used quoin 0.28.3 and quire 0.36.2 (engine 0.50.2). No product build ran.

