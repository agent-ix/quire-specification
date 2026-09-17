---
id: SR-591
title: "Dependency review of complete scalar contracts"
type: SpecReview
analysis: dependency
scope: "QSpec #68 semantic-definition, producer/consumer and delivery dependencies"
review_set: all
evaluated_revision: "cd4c0a0 plus QSpec #68 working-tree remediation"
review_date: "2026-09-15"
relationships:
  - target: ix://agent-ix/quire-specification/issues/68
    type: reviews
---

## Summary

PASS. QSpec owns meaning and immutable definition selection, I04 owns the public
checked semantic node boundary, QSL owns checking/evaluation, and Contract IR
consumes only merged versioned I04 contracts. The already-merged Contract IR
V1 reader remains valid for frozen V1; V2 consumption is a separately visible
downstream implementation dependency rather than a silent change to that reader.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-5911 | high | Resolved: the complete-value root now requires accounting and an exact rule manifest; optional scalar profiles are selected sibling leaves rather than reverse dependencies. | value-complete.md; value-complete-rules.json; definitions/README.md |
| FND-5912 | medium | Resolved: semantic node identity was placed in I04 instead of being invented independently by QSL or Contract IR. | interface_004; FR-322; TC-217 |
| FND-5913 | low | QSL #118 retains scalar primitives/TC-185–193; QSL #119 retains composite equality closure and TC-194. | FR-149; Plan-013 Task-048/049 |
| FND-5914 | high | Resolved: source-authored enum/dimension/unit identities remain I04 nodes, while evaluator-created compound-unit values use the separate `quire.value.compound-unit/v1` domain and do not introduce an I04 producer dependency. | value-compound-unit.md; FR-142; interface_004 |
| FND-5915 | medium | Resolved: the qualification catalog enumerates every byte dependency; F1 re-review separates it from per-package selection, where Unicode, IEEE and the three mutually exclusive integer-division profiles are conditional on `text_bearing`, `ieee_operation` and `integer_div_rem`. | complete-value-lock.json; value-complete.md; definitions/README.md |
| FND-5916 | high | Resolved in the contract: the V1 consumer remains pinned to the unchanged V1 wire, while QSL #118 targets the explicit V2 successor. A production V2 Contract IR reader remains downstream implementation work and is not claimed by this QSpec review. | interface_004; checked-package-v1; checked-package-v2; QSL #118; Contract IR #100 |
| FND-5917 | high | Resolved: V1 migration is not a converter dependency. It reruns the V2 producer from the exact V1-locked source/definition/model inputs, then supplies a typed total correspondence or a closed typed refusal to the V2 consumer, so Contract IR does not invent refusal codes. The existing V1 reader stays intact; the V2 dispatcher/lowerer remains downstream Contract IR work. | NFR-074; interface_004; FR-195; TC-217; migration-correspondence-vectors.json |
