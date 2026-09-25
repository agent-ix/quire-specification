---
id: SR-625
title: "Spec review — STD-100 #156 review ruling delta"
type: SpecReview
analysis: failure-domain
scope: "STD-100; PR #156 delta f320263..2e0a975: FR-322 Model-owned members, FR-150, native-diagnostics 1-draft.4 order, TC-280 vectors"
review_set: subset
---
# Spec review — STD-100 #156 review ruling delta

## Summary

A base checklist and failure-domain review covered commit 2e0a975, which
applies the #156 review ruling. That ruling covers:

- integer ranges from value types only;
- `[0, unbounded]` collections;
- step 2's fixed node members;
- selection causes outside the exact cause set;
- the 1-draft.4 order;
- anonymous structural nodes;
- the dispatch receiver and parameter type nodes;
- the FR-150 clause.

The review found no high findings, two medium and seven low. The follow-up
commit fixes F1 to F5. F6 to F9 are recorded with their dispositions below.

## Verdict

**PASS after fixes.** The delta applies the ruling faithfully. The repinned
FR-150 and rules-manifest digests match their bytes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | QSpec named no source for a value type's `[lo, hi]`. Fixed: the row cites the value types bound as `Int[lo, hi]` that TC-196 and TC-197 declare, and TC-280 states its projection. | FR-322 step 4; TC-280 |
| FND-002 | medium | The `model_member` definition excluded a node carrying a `declaration`, which step 2 refuses in MW-03. Fixed: a node whose id equals a model declaration key is a declaring node. | FR-322 `model_member` |
| FND-003 | low | The "separate set" wording was not disjoint by cause name. Fixed: exclusivity is keyed on (code, cause) pairs. | FR-322 `read` |
| FND-004 | low | Native-diagnostics item 2 listed its causes grouped by code. Fixed: they are in FR-154's check order. | native-diagnostics.md |
| FND-005 | low | AC-29 named only the body-member variant. Fixed: AC-29 names all four fixed members. | FR-322-AC-29 |
| FND-006 | low | The oracle compares type descriptors, not node ids, so an equal declared node is not exercised. Accepted: the reference targets are recomputed keys, and a declared-node vector needs FR-092's structural preimage, which is QC-18's. | tests/model_member_types.rs |
| FND-007 | low | A malformed model node that no member names is not refused by step 2. Accepted: step 2 is scoped to member declaring nodes. | FR-322 step 2 |
| FND-008 | low | The oracle's absent-member check treats an explicit `null` as present. Accepted: the V2 schema admits `declaration` only as an object, so the wire never spells it `null`. | tests/model_member_types.rs |
| FND-009 | low | QSL `model_field` types `[0, 1]` as optional, where QSpec's Presence row gives `K<E>[0, 1]`. Recorded for a QSL ticket, per the ruling. | QSL assemble.rs |
