---
id: SR-11740
title: "spec review of quire-specification PR 265 (process and state-machine owners as clause anchors)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@71f8ac5ec0f95f3e16c3dbf3394b517ec3806286; spec/functional/foundation/FR-013-validate-operation-frames.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/objects/interfaces/FR-340-checked-package-frame-location.md, spec/objects/interfaces/FR-341-checked-package-state-clause-body.md, spec/objects/interfaces/FR-342-checked-package-operation-anchor-body.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, spec/test-cases/TC-280-model-owned-member-types.md, proposals/checked-package-v2/schema.json, proposals/checked-package-v2/node-identity-preimage.schema.json, proposals/checked-package-v2/model-owner-form-vectors.json, proposals/checked-package-v2/README.md"
review_set: subset
---

## Summary

Ticket: QSL-815. One pass of spec-review with the integrity and consistency
sub-checks folded in, run against merged QSpec FR-013, FR-154, FR-208 and
FR-322, and against QSL main (QSL #901: FR-104, FR-105 and FR-154 text, and the
emitter's model form mapping in `qsl-semantics/src/check/lowering/model.rs`).
Only major issues block. Mediums and lows are recorded below and do not block.

Checked against QSL #901. A clause context is an object type, a process or a
state machine. On a process, `self` is the run, typed `Reference<M::T>`. The
anchor and frame `semantic_type` are the declaring node. A process operation's
frame holds no receiver-scoped entry. FR-340, FR-341 and FR-342 now state all
of this, and FR-013 says the same.

The author's choices:

1. Refusing receiver-scoped members on a process or state-machine frame in the
   wire reader too is consistent with QSpec FR-013 on main, which already names
   both forms, and with QSL FR-154's intake refusal. The reader rule is a
   second line for packages from another producer. Accepted.
2. Spelling a state machine's `self` as `Reference<C>` matches QSL FR-104 ("The
   checker SHALL type `self` as `Reference<M::T>`") and #901 ("`self` the
   machine's context"). Accepted.
3. `model`/`persistence_interface` in the owner-bearing forms is required, not
   scope creep. QSL's `record_form` emits `(Model, "persistence_interface")`
   with a `ModelOwner`, and the old `OwnerPresenceRule` refused an owner on that
   form, so a QSL-emitted package failed the V2 schema. The PR keeps the
   persistence interface out of anchor contexts, frame `semantic_type`s,
   `Reference<O>` targets and the `object` family (AP-08 and FP-06 refuse it).
   That agrees with QSL's position that a persistence interface has no object
   receiver. FR-322-AC-25 (an `operation` member resolves on a persistence
   interface) is about operation-application members, not anchors, so it does
   not conflict.
4. Deref and field projection over a process or state machine follow from the
   `object` family. Nothing else breaks: `object` is still admitted only at
   `field_owner` and the clause anchor, and step 3 resolves the members of a
   process or state machine along supertypes of the same meaning.
5. No digest anywhere else pins the published hashes. I grepped the seven
   digests the PR removes in QSL `origin/main` and in quire-contract-ir,
   quire-contract-codegen, quire-contract-runtime, quire-integration,
   quire-driver, quire-verification-contracts and quire-semantic. None appears.
   QSpec HEAD has no stale copy either.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-322 step 4 (member type) now says a field, parameter or result `typeRef` naming "an object type `O`, a process `O` or a state machine `O`" derives `Reference<O>`. Merged QSpec FR-154 says a field, parameter or result that names a state machine or process keeps its refusal at intake. FR-322-AC-65 restates that. The two rules contradict. The widening is not needed for this ticket: `self` gets its type from the "Derived type forms" `Reference<O>` target rule (line 845), not from step 4. Revert the step-4 row to "an object type `O`". | spec/objects/interfaces/FR-322-checked-package-artifact.md:809; spec/functional/type-model/FR-154-admit-domain-package-model.md:124-131 |
| FND-002 | low | Recorded. QSL FR-105's node table still describes a field `modifies` entry as an "(object type node, field name) pair", although QSL FR-105-AC-8 puts `(Flow node, n)` there. This is QSL-side wording, and QSpec FR-340 is correct. | spec/objects/interfaces/FR-340-checked-package-frame-location.md:56-60 |

## Verdict

One blocking finding: FND-001. Everything else listed in scope is clean and
matches QSL #901 and merged QSpec. That covers FR-342's context, its order and
its operation join, FR-341's anchor, `self` and AC-11/AC-12, FR-340's
`semantic_type`, field declaring node and receiver refusal (AC-19/AC-20), the
revised AC-3, AC-7 and AC-12, the FR-013 pointer, the FR-322 owner-presence
rule, the preimage enum, the derived `Reference<O>` target and the `object`
family. All OF, FP, AP, CP and RF vector expectations agree with the FR text.

Re-keying method: done with the repo's own `rekey_stale_nodes`, not typed by
hand. I recomputed three values independently by JCS and SHA-256. Both
published fixtures' `package_id`s match, and so do their identity projections.
Both frame node keys match (`77ea72c8…` and `91c4226e…`). The
`dependency-selection-vectors.json` `package_id` `331fed52…` matches.

Cross-repo merge sequencing (not a defect in this PR): the schema now requires
`deletes_scoped` and a field-entry `scope`, which FR-340-AC-16 and AC-17 on main
already did. The quire-contract-ir main reader still accepts only
`term`/`modifies`/`creates`/`deletes`, at `v2/frame.rs:109`. QSL's
`QSPEC_DIR`-gated frame conformance test builds field entries without `scope`.
After this merges, QSL `make conformance` against QSpec main refuses the
published fixtures until IR-816 lands and QSL adapts.

## New findings (disposition pass 1)

Reviewed at agent-ix/quire-specification@281e94e04e6b8b464657d335fa503f569cab1565.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | high | FR-340-AC-2 still says "every field entry's `declaration` names a declared dependency that is a `model`/`object_type` or `model`/`record_value_type` node". This PR widened the field-entry declaring node to `model`/`process` and `model`/`state_machine` (Properties, Field entries, AC-3, AC-19: "A field entry whose `declaration` is a process or a state machine is admitted in an object type's frame too"), but left AC-2 unchanged. AC-2 and AC-19 now contradict each other. Add `model`/`process` and `model`/`state_machine` to AC-2's field-entry clause. | spec/objects/interfaces/FR-340-checked-package-frame-location.md:306, 323 |

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed 281e94e | FR-322 step 4's `Reference<O>` row reads "an object type `O` of the same domain package" again; the "Derived type forms" `Reference<O>` target row keeps process and state machine, which is where `self` takes its type. This agrees with FR-154 and FR-322-AC-65. |
| FND-002 | deferred: QSL-side wording | QSL FR-105's node-table text belongs to quire-spec-language, not this repo. QSpec FR-340 is correct. |
