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
---

# FR-197: Replay proof counterexamples through native execution

## Description

When a proof backend reports a counterexample, the verification boundary SHALL
serialize its typed inputs canonically and replay them through native
`runtime::execute` under the same semantic package and domain.

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

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-197-AC-1 | Canonical serialize/deserialize round-trips every backend-supported typed counterexample exactly. | Test (TC-219) |
| FR-197-AC-2 | Kani counterexamples replay through native execution with the same verdict on the bounded corpus. | Test (TC-219) |
| FR-197-AC-3 | Invalid-domain, decode or verdict mismatch is retained as parity failure and never rewritten as clause success/failure. | Test (TC-219) |
| FR-197-AC-4 | Every complete-V1 value and execution identity round-trips through the canonical counterexample envelope without narrowing. | Test (TC-219) |
| FR-197-AC-5 | Minimization creates linked revisions and retains only candidates that preserve domain validity and both replay verdicts. | Test (TC-219) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
