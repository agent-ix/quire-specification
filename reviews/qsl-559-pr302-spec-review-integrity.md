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

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | medium | The send record is "recorded" as the step's result, but no output defines where it goes. It is not in the state key, by design. It is not a member of FR-426's transition identity, and it is not in FR-433's counterexample wire, which lists the post-state digest, blocked threads, rest points, binders, queues and memory, but no send record. "Data for the reader, like the blocked-thread list" does not hold, because the blocked list is a named member of the deadlock counterexample. FR-170-AC-6, FR-426-AC-11 and CH-07 to CH-10 all assert on the outcome and on `displaced`, and none of them has a place to read it. Name a carrier, for example a per-step `send_record` member in FR-433 that replay recomputes from the pre-state and post-state, or say that it is derived from those states and give the derivation. | spec/functional/protocol/FR-426-apply-protocol-steps.md:111-122; spec/functional/protocol/FR-433-replay-protocol-counterexamples.md |
| FND-009 | low | FR-170-AC-6 still says "`displaced` the oldest queued message" and "the newest", while the FR-170 statement and FR-426 now define `displaced` as the queue entry (send node, sending thread path and payload). | spec/functional/protocol/FR-170-evaluate-channel-queue-semantics.md:62 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The send binder now holds the payload of the `carries` type that FR-466 rule 5 gives it. The send record is the step's result and is not a binder value (FR-425:134-136, FR-426:108-122). This now differs from ADR-027 ST-3 and QSL FR-206, which bind the binder to the record, so QSL needs a matching amendment. |
| FND-002 | fixed | FR-434's gate row now covers `fence` ("for a `fence` it is the model's fence rule"), and FR-429's `gate` row says the same. Reporting `gate`, rather than a separate fence cause, is acceptable: ADR-027 SE-1(e) defines the fence rule as the gate for `fence`, and the fence has no other enabling condition. |
| FND-003 | fixed | FR-170 and FR-426 both define `displaced` as the queue entry. FND-009 records the residual wording in AC-6. |
| FND-004 | fixed | FR-429 now defines the qualified channel name once: the declaring protocol's qualified name, `::`, then the handle, rendered as FR-433's `protocol` member is. FR-425 and FR-170 refer to that definition. |
| FND-005 | fixed | The send record is out of the state key, and FR-426 gives its form: `{"type":"send-record",...}` with `outcome` as text and `displaced` as a `queue-entry` or `null`. FND-008 records that it has no carrier. |
| FND-006 | fixed | FR-425's list of rest points now includes `fence`. |
| FND-007 | fixed | FR-425 now says the environment is keyed in `observations` by (binder node, thread path), not in the thread's `control` entry. |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-010 | medium | Two rules now give different outcomes for a missing or extra `send_record`. FR-433's wire section says "a missing, extra or mistyped member refuses before replay" (FR-433-AC-5). The new replay step 2 says that a `send` step without a `send_record`, or another step with one, refuses `stale_dependency`/`revision-mismatch` during replay, and FR-433-AC-8 and the new TC-387 vector assert that code. Whether a step is a `send` can be read from its transition identity without replaying, so a missing or extra record is a wire-shape defect. Keep `stale_dependency`/`revision-mismatch` for a recorded record that differs from the recomputed one, which parallels the `post_state` digest mismatch and is acceptable. Route a missing or extra member to the existing pre-replay refusal, and align AC-8 and the vector with that. | spec/functional/protocol/FR-433-replay-protocol-counterexamples.md:63; spec/functional/protocol/FR-433-replay-protocol-counterexamples.md:75-78 |

## Dispositions (round 2)

| FND | outcome | reason |
| --- | --- | --- |
| FND-008 | fixed | The send record is now the `send_record` member of a `send` step in FR-433's counterexample wire, and replay recomputes and compares it (FR-426:121-125, FR-433:56 and step 2, FR-433-AC-8, TC-387). FND-010 records the conflict between refusal codes for a missing or extra record. |
| FND-009 | fixed | FR-170-AC-6 now says that `displaced` is "the oldest queue entry" or "the newest queue entry". |

## Dispositions (round 3)

| FND | outcome | reason |
| --- | --- | --- |
| FND-010 | fixed | FR-433 step 2 now refuses `stale_dependency`/`revision-mismatch` only for a recorded `send_record` that differs from the recomputed one. A missing `send_record` on a `send` step, or one present on any other step, is a wire-shape defect and refuses before replay, as FR-433-AC-5 requires. FR-433-AC-8 and TC-387 RP-11 now assert the same codes. |
