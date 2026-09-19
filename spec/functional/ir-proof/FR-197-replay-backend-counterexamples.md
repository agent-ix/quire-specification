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
  - target: ix://agent-ix/quire-specification/FR-323
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-352
    type: references
---

# FR-197: Replay proof counterexamples through native execution

## Description

When a counterexample reaches the verification boundary, whether a proof
backend reported it or it did not come from a backend transcript (a corpus
counterexample), the verification boundary SHALL serialize its typed inputs
canonically. The verification boundary SHALL replay those inputs through the
QSL complete-V1 executor
([AD-016](../../assurance/AD-016-semantic-family-extension-path.md) arrow 7)
under the same semantic package and domain.

## Inputs

Counterexample with its `ReplaySource` arm (a backend transcript, or canonical
assignments for a counterexample that did not come from a backend transcript),
package identity, backend run identity for the `Witness` arm, expected
disposition, model domain, claim and native runtime selection.

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
digests, typed input values, pre-state, selected transition or trace
occurrences, expected disposition and complete source maps. A `Witness`-arm
envelope also contains the backend run identity, and its typed input values are
the decode of its transcript; it stores no input beside the transcript. An
`Input`-arm envelope's expected disposition is the one recorded with its
counterexample.
Integers and exact numbers serialize losslessly; IEEE values use width and bits;
sets/bags use canonical semantic ordering without erasing occurrence identity.
Native replay first validates every value against the original domain, then
executes the same claim without backend shortcuts. A minimized envelope is a
new revision linked to its parent. A minimized `Witness`-arm envelope is
accepted only after native and backend replay preserve the failure, and it
carries the `Witness` arm of the backend re-run that preserved the failure; it
never inherits its parent's transcript. Minimizing an `Input`-arm
counterexample preserves the native verdict only and yields `Input`-arm
revisions.

## Replay source

A counterexample's replay source is exactly one `ReplaySource` arm:
`Witness(Witness)`, whose replay input is `Witness::decode` of the transcript
admitted by `Witness::parse`, or `Input(values)`, canonical assignments keyed by
parameter `WireNodeId` for a counterexample that did not come from a backend
transcript (AD-016 Replay source). The replay boundary is the CG replay adapter
together with the QSL layer-6 `replay` facade, within the verification
boundary. A replay agrees when the native verdict equals the envelope's expected
disposition, mapped to a verdict by the outcome-to-verdict map of the QSL #231
replay envelope. A witness-backed field is any stored value, under any name,
that states whether a backend witness exists; `ReplaySource` is the only
carrier of that fact.

- The replay boundary SHALL yield a replay result of one type for the
  `Witness` arm and of a distinct type for the `Input` arm.
- When a replay of the `Witness` arm agrees, the replay boundary SHALL settle
  it `reproduced-with-evaluated-witness` in the `Witness`-arm result type.
- When a replay of the `Input` arm agrees, the replay boundary SHALL settle it
  `reproduced-without-witness` in the `Input`-arm result type.
- When a replay of either arm disagrees, the replay boundary SHALL settle it
  `inconclusive` with a typed cause and SHALL construct no backend-evidence
  verdict.
- The backend-evidence verdict type SHALL have exactly one construction path,
  which takes an agreeing `Witness`-arm result.
- The replay boundary SHALL accept `reproduced-without-witness` in no position
  that accepts a backend-evidence verdict.
- The replay boundary SHALL derive whether a replay is witness-backed by
  matching on `ReplaySource`.
- The replay boundary SHALL store no witness-backed field in any packet or
  replay result.
- Each arm's replay-result type SHALL hold that arm's payload and SHALL hold
  no `ReplaySource`.
- The replay-result reader SHALL decode a `native-run-result/2` record of an
  `Input` replay only into the `Input`-arm result type.
- When a packet holds both arms or neither, the replay boundary SHALL refuse it
  with a typed cause before any replay runs.
- The public `Witness` replay entry SHALL call the QSL `replay` facade and
  SHALL take no caller-supplied executor.
- A `Witness` SHALL be constructed only through `Witness::parse`, including on
  deserialization.
- When an `Input` packet holds a value keyed to no parameter, or a parameter
  has no value, the replay boundary SHALL refuse it with a typed cause before
  execution.

An `Input` replay reproduces an answer supplied with its input, so its
agreement shows that native execution matches the recorded disposition, and a
backend-evidence verdict rests only on a `Witness` replay. Type structure
enforces this.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-197-AC-1 | Canonical serialize/deserialize round-trips every backend-supported typed counterexample exactly. | Test (TC-219) |
| FR-197-AC-2 | Kani counterexamples replay through native execution with the same verdict on the bounded corpus. | Test (TC-219) |
| FR-197-AC-3 | Invalid-domain, decode or verdict mismatch is retained as parity failure and never rewritten as clause success/failure. | Test (TC-219) |
| FR-197-AC-4 | Every complete-V1 value and execution identity round-trips through the canonical counterexample envelope without narrowing. | Test (TC-219) |
| FR-197-AC-5 | Minimization of a `Witness`-arm counterexample creates linked revisions and retains only candidates that preserve domain validity and both replay verdicts. | Test (TC-219) |
| FR-197-AC-6 | An agreeing `Input`-arm replay returns a `reproduced-without-witness` result. | Test (TC-272) |
| FR-197-AC-7 | An agreeing `Witness`-arm replay returns a `reproduced-with-evaluated-witness` result. | Test (TC-272) |
| FR-197-AC-8 | A `reproduced-without-witness` result fails to compile in every parameter, field and return position that accepts a backend-evidence verdict. | Test (TC-272) |
| FR-197-AC-9 | The only construction path of a backend-evidence verdict takes an agreeing `Witness`-arm result. | Test (TC-272) |
| FR-197-AC-10 | The backend-evidence verdict type has no public field and implements none of `Deserialize`, `Default`, `From` or `TryFrom`. | Test (TC-272) |
| FR-197-AC-11 | No packet or replay-result type has a witness-backed field. | Test (TC-272) |
| FR-197-AC-12 | The witness-backed answer, derived by a match on `ReplaySource`, is `true` for every `Witness` packet and `false` for every `Input` packet. | Test (TC-272) |
| FR-197-AC-13 | A disagreeing replay of either arm settles `inconclusive` with a typed cause and yields no backend-evidence verdict. | Test (TC-272) |
| FR-197-AC-14 | A packet that holds both arms or neither is refused with a typed cause before any replay runs. | Test (TC-272) |
| FR-197-AC-15 | A `Witness` is constructed only through `Witness::parse`, including on deserialization. | Test (TC-272) |
| FR-197-AC-16 | The public `Witness` replay entry calls the QSL `replay` facade and takes no caller-supplied executor. | Test (TC-272) |
| FR-197-AC-17 | No replay-result type holds a `ReplaySource`, so a `Witness`-arm result holding an `Input` source cannot be built. | Test (TC-272) |
| FR-197-AC-18 | A `native-run-result/2` record of an `Input` replay decodes only into the `Input`-arm result type and yields no backend-evidence verdict. | Test (TC-272) |
| FR-197-AC-19 | Minimization of an `Input`-arm counterexample creates linked `Input`-arm revisions and retains only candidates that preserve domain validity and the native verdict. | Test (TC-219) |
| FR-197-AC-20 | An `Input` packet with a value keyed to no parameter, or with a parameter that has no value, is refused with a typed cause before execution. | Test (TC-272) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- The serialized one-arm `counterexamples` entry and the replay members of
  [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) and
  [FR-323](../../objects/interfaces/FR-323-native-runtime-envelope.md) are
  QSL ADR-013 QC-6 and QC-8 (TK-06); the replay-result record is
  [FR-352](../foundation/FR-352-mint-native-run-result-witness-wire.md)
  `native-run-result/2` (QSL ADR-013 OQ-2).
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
