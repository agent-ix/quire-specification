---
id: SR-202611022231
title: "Failure-domain review of QSpec PR #606: shared clauses, clock configuration and consumer APIs"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification; spec/objects/interfaces/FR-322-checked-package-artifact.md (Clock bindings of a shared clause, AC-353, AC-354); proposals/checked-package-v2/temporal-clause-name-vectors.json (CN-P2, CN-R9); read against Contract IR temporal_binding.rs and QSL emit.rs, FR-337, FR-515, FR-517, FR-518"
review_set: subset
---

## Summary

Ticket: IR-1278 (and IR-1279, QSL-1544); PR quire-specification#606 at head a188eabcf17cc1409a901445284ac7474aeb37e7.

The PR answers IR-1279 with one `temporal_bindings` row per clause node, on the stated ground that the clock name, the selected `temporal_profile` law and the clock configuration are all in the clause body. The first two are. The clock configuration is not: in Contract IR the `ClockConfiguration` is validated only for kind against the lock's profile selection and the row's clock name (`temporal_binding.rs` `validate`), and in QSL it comes from a `TemporalClockSelection` keyed by the declaration's own source span (`emit.rs` `temporal_binding_rows`), given in the request (FR-517, QSL-1557). Two structurally equal declarations therefore share a node and may select different periods, epochs or units. The package can carry only one row for the node, so the second configuration has no representation.

QSL today emits one row per declaration (`graph.temporal_clauses()`), so structurally equal declarations already produce the repeated row the reader refuses. FR-517 and the handoff fixtures never contain such a pair, so that has not surfaced.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | "Clock bindings of a shared clause" says every occurrence of one node has the same clock name, profile and clock configuration, because those are arguments of the clause body. The clock configuration is not a body argument, so the claim is false and AC-353 ("that row is the binding of every occurrence") is not justified for it. The spec neither states that twins with different configurations are unrepresentable and refused by the producer, nor carries a per-occurrence configuration. The planner's literal "one binding per occurrence" would need the occurrence in `TemporalBinding`. Choosing one row per node silently narrows what QSL can compile, and no one has ruled on it | spec/objects/interfaces/FR-322-checked-package-artifact.md:3436-3446,4341; /Users/peter/dev/qsl-606-ref/qsl-package/src/emit.rs:5498-5545; /Users/peter/dev/qci-606/crates/quire-contract-model/src/checked_package/v2/temporal_binding.rs:256-345 | wrong-requirement |
| FND-002 | medium | FR-515 keys the temporal subject by clause node id and offers a singular `name() -> &str`, and keys the predicate view by `PredicateSelection { clause, leaf }`. With per-occurrence names, a node-keyed view has several names and a predicate selection cannot say which occurrence's name it wants. The PR does not say how a consumer selects an occurrence, only that the reader exposes the name of a (declaration, occurrence) pair. QSL FR-515, FR-518 row 8 and AC-11, and the selection sidecar `clause.name` axis need an occurrence key. This is an adoption gap, not a defect in the carrier | spec/objects/interfaces/FR-322-checked-package-artifact.md:3431-3434; /Users/peter/dev/qsl-606-ref/spec/functional/FR-515-read-the-checked-predicate-and-temporal-subject-from-a-checked-package.md:101,127,173 | missing-requirement |

## Verdict

The carrier handles the shared-node identity problem titan raised correctly. The binding half is under-justified: FND-001 needs an owner ruling (twins must share a clock configuration and QSL refuses a difference, or the binding becomes per-occurrence) before the text is final. FND-002 lists work QSL must do and is not a blocker for merging the spec.
