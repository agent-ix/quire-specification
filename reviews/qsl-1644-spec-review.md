---
id: SR-617004
title: 'Spec review of PR #618: FR-370 window aggregate atom quire.op.temporal.aggregate (QSL-1644)'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@9d8696de; git diff origin/main...9d8696de (PR #618): FR-370 "Window aggregate atom", FR-370-AC-17, FR-370-AC-18, reader-order paragraph, TC-303 step 15, schema.json OperationIdentity, checked-package-v2 README; QSL FR-194, FR-337-AC-15, FR-515, FR-516, FR-532 (quire-spec-language origin/main, read only)'
review_set: subset
date: '2026-11-04'
relationships:
- target: ix://agent-ix/quire-specification/FR-370
  type: reviews
- target: ix://agent-ix/quire-specification/TC-303
  type: reviews
---

## Summary

Ticket: QSL-1644. The node shape matches the QSL contract. FR-194 `AggregateTerm` has `kind` (with `Quantile(q)` and `Fraction(predicate)`), `value` (absent for count and fraction), `filter`, `window` (`Positions{a,b}` or `Duration{a,b}`) and `min_count` (at least 1, or 0 for count and sum). FR-516 adds the comparison (`<`, `<=`, `=`, `>=`, `>`) and the converted threshold. Each maps to exactly one FR-370 binding (`kind`, `q`, `predicate`, `value`, `filter`, `window_kind`/`lower`/`upper`, `min_count`, `comparison`, `threshold`). Nothing is lost and nothing is added.

The author's reason for flattening holds. In schema.json, the `binding` items of `GroupTerm` take only a `LeafTerm` value, so a nested `window` group inside the argument is not expressible. The value/predicate split also follows FR-194's own model more closely than FR-515's single `expression`.

The refusal causes, loci and the both-faults order agree with FR-194/FR-414. The window-kind refusal is located at `window_kind` rather than at a source bound, which is the right choice for a package reader. The open issue is the unit of the `duration` bounds. It is the one place where IR admission and QSL emission could diverge.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-370 Window aggregate atom (binding table, reader steps 1-5) | spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md:172 | examined |
| FR-370 reader-order paragraph | spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md:391 | examined |
| FR-370-AC-17 | spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md:415 | examined |
| FR-370-AC-18 | spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md:416 | examined |
| TC-303 step 15 and expected result 15 | spec/test-cases/TC-303-checked-package-temporal-clause-body.md:73 | examined |
| FR-414 Windows | spec/functional/temporal/FR-414-evaluate-window-aggregates.md:83 | context_only |
| QSL FR-194, FR-337-AC-15, FR-515 AggregateUse, FR-516, FR-532 | quire-spec-language origin/main | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Duration window bounds are "quantity literals of one unit, converted exactly by the checker" but the unit is not named, while FR-414 says the bounds are converted "to the binding's unit" and bounded-profile intervals are ticks of the clock's timestamp unit. As written, `past[0, 300 s]` can be emitted as 0..300 s or 0..300000 ms, which gives two node identities. The reader cannot check the bounds against the clause's clock unit, and it admits a `duration` window in a non-time unit such as metres. Fix: name the unit (the clause's timestamped-event clock unit), refuse any other unit or a non-Time dimension `ill_typed`/`unit-mismatch` at `M/value` of `lower`, and add a reader case for it | spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md:191 |
| FND-002 | low | The identity sentence lists "kind, q, expression, window, normalized min_count, comparison or threshold" and omits `filter` and `predicate`. The preceding clause ("the bindings are inputs of the node's structural identity") covers them, but the list reads as exhaustive. Fix: say "a change of any binding" | spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md:200 |

## Verdict

Conditional on FND-001. The shape, binding names, order, presence rules and refusal ladder are sound and complete against FR-194/FR-414. Without FND-001, quire-contract-ir's admission and QSL-1643's emitter have no stated canonical unit for duration windows.

Follow-up for the QSL lane, not a defect of this PR: FR-515 `AggregateUse { kind, filter, window, min_count, expression: &CheckedNodeId, comparison, threshold }` has a single non-optional `expression`. A `count` has no expression, and a `fraction` carries `predicate` rather than `value`, so FR-515/FR-337-AC-15 need `value: Option<..>` plus the predicate, and FR-370-AC-17/AC-18 as the wire-member citation. FR-532 "Window aggregates" ("QSpec defines no wire members") becomes stale once this merges.
