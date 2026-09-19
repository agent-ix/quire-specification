---
id: FR-197
title: "Replay proof counterexamples through native execution"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
  - target: ix://agent-ix/quire-specification/FR-114
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-037
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-031
    type: references
  - target: ix://agent-ix/quire-specification/AD-016
    type: references
---

# FR-197: Replay proof counterexamples through native execution

## Description

When a proof backend reports a counterexample, the verification boundary SHALL
serialize its typed inputs canonically and replay them through the QSL
complete-V1 executor ([AD-016](../../assurance/AD-016-semantic-family-extension-path.md)
arrow 7) under the same semantic package and domain.

## Inputs

Backend counterexample, package/run identity, model domain, claim and native
runtime selection.

## Outputs

Canonical counterexample with minimization lineage, backend/native verdict pair
and agreement/parity-failure result.

## Behavior

Serialization preserves exact value kinds, object/trace identities, state
anchors, bounds and source correspondence. Minimization creates linked revisions
and must preserve domain validity and failure. Agreement requires the same clause
verdict; decode, domain or verdict mismatch is an explicit backend parity failure.

## Replay envelope

The canonical envelope contains schema/profile versions, package and claim
digests, backend run identity, typed input values, pre-state, selected transition
or trace occurrences, expected backend disposition and complete source maps.
Integers and exact numbers serialize losslessly; IEEE values use width and bits;
sets/bags use canonical semantic ordering without erasing occurrence identity.
Native replay first validates every value against the original domain, then
executes the same claim without backend shortcuts. A minimized envelope is a
new revision linked to its parent and is accepted only after native and backend
replay preserve the failure.

## Replay source

A counterexample's replay source is exactly one `ReplaySource` arm:
`Witness(Witness)`, whose replay input is `Witness::decode` of the admitted
transcript, or `Input(values)`, the stored values of a counterexample that ran
no backend (AD-016 Replay source).

- The replay boundary SHALL yield a replay result of one type for the
  `Witness` arm and of a distinct type for the `Input` arm.
- When a replay of the `Input` arm agrees, the replay boundary SHALL settle it
  `reproduced-without-witness` in the `Input`-arm result type.
- The replay boundary SHALL construct a backend-evidence verdict only from a
  `Witness`-arm result.
- The replay boundary SHALL derive whether a replay is witness-backed by
  matching on `ReplaySource`, and SHALL store no witness-backed field in any
  packet or replay result.

An `Input` replay reproduces an answer supplied with its input, so its
agreement is never evidence that a backend counterexample reproduces natively.
Type structure enforces this; a documentation comment or a Boolean field does
not.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-197-AC-1 | Canonical serialize/deserialize round-trips every backend-supported typed counterexample exactly. | Test (TC-219) |
| FR-197-AC-2 | Kani counterexamples replay through native execution with the same verdict on the bounded corpus. | Test (TC-219) |
| FR-197-AC-3 | Invalid-domain, decode or verdict mismatch is retained as parity failure and never rewritten as clause success/failure. | Test (TC-219) |
| FR-197-AC-4 | Every complete-V1 value and execution identity round-trips through the canonical counterexample envelope without narrowing. | Test (TC-219) |
| FR-197-AC-5 | Minimization creates linked revisions and retains only candidates that preserve domain validity and both replay verdicts. | Test (TC-219) |
| FR-197-AC-6 | An agreeing `Input`-arm replay returns a `reproduced-without-witness` result. | Test (TC-272) |
| FR-197-AC-7 | A `reproduced-without-witness` result fails to compile in every parameter, field and return position that accepts a backend-evidence verdict. | Test (TC-272) |
| FR-197-AC-8 | Every public constructor of a backend-evidence verdict accepts only a `Witness`-arm result. | Test (TC-272) |
| FR-197-AC-9 | No packet or replay-result type has a witness-backed field. | Test (TC-272) |
| FR-197-AC-10 | The witness-backed answer, derived by a match on `ReplaySource`, is `true` for every `Witness` packet and `false` for every `Input` packet. | Test (TC-272) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
