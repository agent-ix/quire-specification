---
id: SR-532
title: "Dependency review of FS02 protocol residual rulings"
type: SpecReview
analysis: dependency
scope: "quire-specification #31 owner rulings against current Spec Language, Protocol and Observation implementation reality"
review_set: subset
evaluated_revision: "quire-specification dcc22bd; quire-spec-language 6eccb05; quire-protocol e53fb55; quire-observation 41e4919"
review_date: "2026-09-13"
---
# Dependency review of FS02 protocol residual rulings

## Summary

PASS. The packet consumes existing native wire facts and previously accepted
FR-287/288 identity authorities. It records, without approximating, where
Protocol and Observation are incomplete. No ruling depends on or reallocates a
TL-* implementation. The overlapping FR-281/result work is assigned to #40 and
will be consumed only after its owner merges it.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Planned consumer correction: Protocol's FIFO identity retains only binder/key/anchor positions. Equal positions under semantically changed expressions can compare equal, so FR-216 requires the exact qualified typed expression definition instead. | qprotocol `src/link/mod.rs`; FR-054; FR-216 |
| FND-002 | high | Planned consumer correction: Protocol's `ControlShape` omits guards, constraints, await/event/finish semantics, values and compensations and reduces exact numbers to a collision-prone `u32` fingerprint. FR-220 refuses those approximations for exact graph consistency. | qprotocol `src/link/mod.rs`; FR-052; FR-220 |
| FND-003 | high | Planned producer/consumer correction: Spec Language has no internality field and emits no concrete participant input; Protocol builds `ParticipantId` from static requirement coordinates. FR-050/217 require a versioned binding carrying a concrete FR-287 participant and refuse slot substitution or default internality. | SL `src/syntax/composed.rs`, `src/protocol_artifact/wire.rs`; qprotocol `src/link/mod.rs`; FR-050; FR-217 |
| FND-004 | medium | Planned Protocol work: Spec Language publishes compensation definition, effect requirement, retry maximum/relation and commit selection, while Protocol's compensation algebra is absent. FR-233/237/239 state the exact consumer behavior and claim no implementation evidence. | SL `src/protocol_artifact/native/runtime/compensations.rs`; qprotocol task #5; FR-233; FR-237; FR-239 |
| FND-005 | high | Planned Observation correction: current matching uses unqualified endpoints, ignores relationship identity and treats two exact replays as ambiguity. FR-262 requires qualified endpoints and exact-key replay idempotence. | qobservation `src/lib.rs`; FR-111; FR-262; qobservation #18/#13 |
| FND-006 | medium | Coordination fixed: the active #40 branch was based before accepted FR-287/288 allocation and touched FR-281/shared result files. #31 relinquished FR-281, documented the collision on #40 and leaves that branch untouched. | qspec #31; qspec #40; FR-281 |

## Dependency allocation

| Owner | Existing usable fact | Remaining owned adoption |
| --- | --- | --- |
| Spec Language | Closed `unordered`/FIFO wire, typed FIFO binder/expression, inclusive delivery interval, complete typed control/compensation records, `commit Node`/`never`. | Add versioned role internality and concrete participant binding only under its own ticket; do not mutate old contracts or default. |
| Protocol | Nominally distinct IDs, explicit linked channels and causal structures. | Replace position/fingerprint approximations; consume FR-287/288; implement registration/retry/commit under existing #4/#5 tasks. |
| Observation | Explicit relationship kind/source/target and typed incomplete/refusal shapes. | Consume authority-qualified endpoints and relationship identity, fix replay matching, and preserve exact outcome semantics under #18/#13. |
| #40 owner | Active claim/temporal/observation authority specification. | Rebase past #28, renumber colliding objects, merge FR-281/result edits; #31 does not duplicate them. |

Dependency order remains FIFO/graph identity before Protocol compensation
evaluation; relationship qualification can proceed independently in
Observation. None of those edges requires Agent B's TL-* library lane.
