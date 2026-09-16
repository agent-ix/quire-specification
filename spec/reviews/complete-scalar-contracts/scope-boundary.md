---
id: SR-594
title: "Scope-boundary review of complete scalar contracts"
type: SpecReview
analysis: scope-boundary
scope: "QSpec #68 authority, repository ownership and QSL #118/#119 allocation"
review_set: all
evaluated_revision: "cd4c0a0 plus QSpec #68 working-tree remediation"
review_date: "2026-09-15"
relationships:
  - target: ix://agent-ix/quire-specification/issues/68
    type: reviews
---

## Summary

PASS. The repair closes normative inputs needed by implementation without
moving evaluator behavior into the transport schema or moving composite
equality delivery into QSL #118.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-5941 | high | Resolved: I04 defines only source-authored versioned node preimages, joins and reader validity; AD-005/FR-140–149 retain language meaning, including the evaluator-owned compound-unit value domain. | interface-004; FR-322; AD-005; value-compound-unit.md |
| FND-5942 | medium | QSL remains the reference semantic producer; Contract IR validates and lowers versioned public artifacts without importing QSL internals. | proposals/checked-package-v2/README.md; TC-217 |
| FND-5943 | low | #118 implements reusable scalar equality primitives; #119 owns complete composite/collection/object equality and TC-194 completion. | FR-149; QSL #118/#119 |
| FND-5944 | high | Resolved: runtime compound-unit identities are neither projected into I04 nor assigned source occurrences, semantic-graph nodes or source-map entries. | FR-142; value-compound-unit.schema.json; value-compound-unit-vectors.json |
| FND-5945 | high | Resolved: QSpec defines V2 but does not claim the merged Contract IR V1 consumer implements it; that consumer migration remains downstream delivery work. | interface-004; FR-322; Contract IR #100 |
| FND-5946 | medium | Resolved: QSpec owns the migration wire, the closed migration outcome/refusal-code vocabulary and semantic prohibitions; QSL owns the re-link/re-check producer and Contract IR owns version dispatch, V2 reading/lowering and consumer qualification. The correspondence vector is not a claim that either production implementation already exists. | NFR-074; interface-004; FR-195; TC-217 |
