---
id: SR-13587
title: "Spec review (integrity) of quire-specification PR #302: protocol checking rules aligned with QSL ADR-027"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@cf0af5ff8eb2a11406f410cb1bdcb49ad358ea22; PR #302, git diff origin/main...cf0af5f: spec/functional/protocol/FR-170, FR-425, FR-426, FR-429; read against FR-050, FR-288, FR-433, FR-434, FR-436 and FR-466"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-170
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-425
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-426
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-429
    type: reviews
---
# Spec review (integrity) of quire-specification PR #302

## Summary

Tickets: QSL-559, QSL-562. This pass checks whether the new statements agree
with the QSpec requirements they depend on: FR-050 and FR-288 (identity),
FR-434 (the memory seam), FR-436 (`tso`) and FR-466 (checked control bodies
and binder scope).

**Consistent:**

- the fence row with FR-436's table;
- nested cancellation with FR-425's rule for removing a parallel instance;
- the compensation thread's empty environment with FR-466's compensation
  stages;
- fork, `await` and `all`-join copying with FR-466's `In` and `Out` table.

SR-13588 FND-002 covers the one divergence from FR-466 that the
environment rule creates.

FR-426 keeps "the channel's protocol instance is not finished" for
`duplicate` and `lose`. QSL ST-5 does not have that condition, but the two do
not conflict. In QSL (AE-3) and in QSpec FR-428, an instance is removed with
its queues in its `finish` step, so no entry is left for ST-5 to act on and
the QSpec guard is redundant there. FR-425 also keys `instance-state finished`
and FR-426 rests the root at `end`, which reads as though a finished `on
origin` instance stays in the state. Under that reading the guard is what
keeps `duplicate` and `lose` from making a finished state non-terminal.
Either way the enabled set is the same, so the condition should stay. That
FR-425 and FR-428 disagree on whether a finished instance is kept is an
existing defect outside this diff.

## Verdict

Changes requested: one high and three medium findings. The high one also
affects QSL: ADR-027 ST-3 binds a send record, while QSL FR-218-AC-11 and
TC-1056 type the send binder as the payload. It needs one ruling for both
repositories.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Two requirements now contradict each other on the type of a send binder. FR-466 rule 5 gives the binder of a `send` the channel's `carries` type, and the source writes `send Sent via Pay as (sent: M::Plain)`. FR-425, FR-426 and FR-170 now bind that binder to a send record `{channel, payload, outcome, displaced}`. A `check` or later constraint that reads `sent.f` type-checks against the payload, but at run time it finds a record with no field `f`. Choose one: either the source binder stays the payload and the send record is a separate observation, or FR-466 types the binder as the send record. ADR-027 ST-3 and QSL FR-218-AC-11 have the same split. | spec/objects/interfaces/FR-466-checked-package-protocol-control-bodies.md:105; spec/functional/protocol/FR-426-apply-protocol-steps.md:106-113; spec/functional/protocol/FR-425-define-protocol-subjects-and-state-keys.md:134-135 |
| FND-002 | medium | The closed cause list gives a waiting `fence` no cause. FR-429's `gate` cause is "a step disabled only by its memory model's gate", and FR-434's gate covers only `fork`, `join`, `send`, `receive` and `attempt`. FR-426 enables a fence by "its memory model's fence rule alone", which is not that gate. ADR-027 SE-1(e) does make the fence rule the gate for `fence`. Add `fence` to FR-434's gate row, as SE-1(e) does, or name the fence in FR-429's `gate` row. | spec/functional/protocol/FR-429-report-protocol-deadlocks.md:128; spec/functional/memory/FR-434-select-memory-models-for-parallel.md:117; spec/functional/protocol/FR-426-apply-protocol-steps.md:100 |
| FND-003 | medium | FR-170 says `displaced` is "the queued message a `head` or `tail` selector removed". FR-426 says it is "the queued entry", and an entry is (send node, sending thread path, payload). The state key holds this record under `observations`, so an implementation that stores the payload and one that stores the entry produce different key bytes. Use one term in both, preferably the entry that FR-426 and ADR-027 ST-3 use. | spec/functional/protocol/FR-170-evaluate-channel-queue-semantics.md:48; spec/functional/protocol/FR-426-apply-protocol-steps.md:110-113 |
| FND-004 | medium | The qualified channel name is "the qualified protocol-definition identity (FR-050) followed by the channel's declaration-local handle", and it is serialized as the string `"channel":"<qualified-name>"`. But the FR-050 and FR-288 identity is a structured key of four members (source document key with its digest, declaration handle, owner and clause, selection closure), not a name, and "followed by" defines no rendering. FR-433's subject uses `"protocol":"<qualified-name>"`, the protocol's qualified name. Say that the channel's `<qualified-name>` is the declaring protocol's qualified name joined to the handle by the `::` qualified-name rule, and keep FR-050 as the identity it keys. Otherwise two implementations will render different key bytes. ADR-027 PS-7 has the same wording. | spec/functional/protocol/FR-429-report-protocol-deadlocks.md:111-116; spec/functional/protocol/FR-425-define-protocol-subjects-and-state-keys.md:173-177; spec/objects/protocol/FR-288-qualified-protocol-definition-identity.md |
| FND-005 | medium | The send record has no state-key encoding. `observations` holds "the binder's record", which FR-181 encodes as `{"type":"record","name":"<qualified-name>",...}`. Neither the record's name, nor the encoding of `outcome` (enum member or text), nor the value of a `displaced` that is none (null, or an absent field) is stated. These bytes enter the key and decide which states coalesce. | spec/functional/protocol/FR-425-define-protocol-subjects-and-state-keys.md:193; spec/functional/protocol/FR-426-apply-protocol-steps.md:106-113 |
| FND-006 | low | FR-425's list of rest points leaves out `fence` (ADR-027 PS-3 includes it), while FR-426 now has a thread that waits at a fence whose rule is false. | spec/functional/protocol/FR-425-define-protocol-subjects-and-state-keys.md:118-122 |
| FND-007 | low | "Each live thread's control state holds a binder environment" can be read as putting the environment in the key's `control` member. The key actually carries environments in `observations`, and `thread-state` has no environment field. Say "the thread's binder environment, keyed in `observations`". | spec/functional/protocol/FR-425-define-protocol-subjects-and-state-keys.md:136-138 |
