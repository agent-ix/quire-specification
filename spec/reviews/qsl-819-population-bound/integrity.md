---
id: SR-12048
title: "QSL-819 QSpec PR #268 spec review: the population link of a reaches relationship_end member"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@29987ff9816cead0237f607bb5fd0c1d02a449f4; spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, proposals/checked-package-v2/schema.json, proposals/checked-package-v2/node-identity-preimage.schema.json, proposals/checked-package-v2/fixtures/adverse.json"
review_set: subset
---

## Summary

Ticket: QSL-819. PR quire-specification#268, reviewed at 29987ff9816cead0237f607bb5fd0c1d02a449f4. The review covers FR-322 "Population link of a `reaches`", the amended dependency rule in "Body terms", the `relationship_end` pointer to the new section, FR-322-AC-73, the TC-233 paragraph, the `relationship_end` variant in both V2 schemas, and the seven `reaches_population_cases`. FR-322-AC-73 is unique on origin/main and on every remote branch.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS. Each of the seven vectors was checked by hand against the rule:

- The absent-key case keeps the published member: exactly `kind`, `declaration` and `name`.
- The `max` `3` and `max` `0` cases name a `model_population` node over `Set<Reference<Order>>`. `Order` is the type of the `order` end, the member's role, in `Order-order-Order`. Both are admitted.
- The `integer_range` case, the absent node, and the `model_population` over `Invoice` all refuse `ill_typed`/`operator-ineligible`. `Invoice` is declared in the selected acme/orders document, so the appended node passes the owner join and the refusal comes from the link check alone.
- The case with the node omitted from `dependencies` is a graph-shape refusal (`invalid_semantic_graph`).

Both schemas add `population` only to the `relationship_end` variant, as an optional `NodeId`, so `null` and every other member kind fail the schema. The absent-means-unchanged rule holds because the key is absent, not `null`, in the preimage. An absent node refusing `ill_typed` rather than `missing-name` is stated explicitly. It follows the rule that graph shape compares `dependencies` without resolving them, which the protocol carrier also uses. The navigate refusal has no vector. That was the author's choice and the planner accepted it. Cross-repo: the node shape, T = the type of the end the role names, and the least-ordinal occurrence rule match QSL FR-365 "The population link" and FR-380 "The population node". The role names the near end, and FR-365/AC-57 make the destination end the same type. `quire validate` fails on FR-322 with `[value_object] required 'id'` and `fields` missing, but `main` 2fb2473 already fails the same way. This PR does not introduce it.
