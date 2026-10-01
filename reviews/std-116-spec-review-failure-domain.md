---
id: SR-873
title: "Failure-domain review — STD-116/121 checked-package abstraction relation node"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@896aa54d5cc4fdc17325c918959fe130294adcd4; PR #171; FR-451, proposals/checked-package-v2/schema.json, node-identity-preimage.schema.json, TC-412"
review_set: subset
---
# Failure-domain review — STD-116/121 checked-package abstraction relation node

## Summary

Ticket: STD-116. Strict-reader failure modes of the new
`correspondence`/`abstraction_relation` node. The node id is a canonical
content identity (JCS SHA-256 of version and body), which is the allowed
digest kind. Identity, order, target and member checks are ordered, and
package-wide key uniqueness is checked after per-node checks.

## Verdict

**PASS with one medium finding.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The reader never refuses two `fields` entries with the same `name` in one object entry: the order rule is "sorted ascending" (not strictly), step 4 checks membership only, and the schema has no uniqueness; FR-340 states its arrays duplicate-free, FR-451 does not, so a hand-built package can bind one model field to two Rust fields. | spec/objects/interfaces/FR-451-checked-package-abstraction-relation-body.md:79-82,106 |
