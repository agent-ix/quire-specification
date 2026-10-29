---
id: FR-017
title: "Select exact verification artifact roles"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-004"
    type: traces_to
  - target: "ix://agent-ix/quire-specification/FR-014"
    type: depends_on
---
# FR-017: Select exact verification artifact roles

## Description

When a verification reference is matched to a requested role, the shared reference reader shall require the exact selected artifact kind.

## Inputs

An explicitly selected `ix.shared-reference/2-draft` artifact or semantic envelope,
its `ix.artifact-ref/3-draft` reference and the
consumer's expected role. The amendment is defined in the
[wire contract](../../../proposals/shared-reference-2-draft/README.md).

## Outputs

A checked reference or a distinct invalid-wire, unsupported-version,
identity-mismatch or stale refusal. Exhaustion yields
incomplete with no accepted reference, as specified by
[NFR-002](../../non-functional/NFR-002-classify-resource-incompleteness.md).

## Behavior

The six added kinds are trace, fault-model, review-procedure, oracle, observation
and review-disposition. Each names the artifact itself; none grants truth,
independence, reviewer authentication or evidence sufficiency. No nearby generic
kind substitutes for a requested role. The fourteen existing kinds retain their
meanings. Authority, identity and wire remain mandatory. Unknown kinds/versions and implicit cross-version joins refuse.
The 1-draft envelope and 1/2-draft artifact contracts retain their original bytes
and closed vocabulary. This amendment adds no migration inference.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-017-AC-1 | Each of the six new kinds is structurally representable under the selected versions and matches the same requested role. | Test (TC-001) |
| FR-017-AC-2 | Substituting a nearby generic kind or any other added kind for the requested role refuses with identity-mismatch, even when bytes and every other reference field agree. | Test (TC-002) |
| FR-017-AC-3 | Unknown kinds, extra members and omitted required fields refuse; changing only the envelope or artifact version never selects a fallback decoder. | Test (TC-002) |
| FR-017-AC-4 | Exact property, subject, binding, dependency-closure and run references fail matching after one-axis authority, identity or wire changes. | Test (TC-002) |
| FR-017-AC-5 | The closed state-core shared-reference 1-draft schema refuses each new role. | Test (TC-002) |

## Dependencies

- [US-004](../../usecase/US-004-retain-evidence-meaning.md) and [FR-014](FR-014-read-exact-wire.md).
- [NFR-003](../../non-functional/NFR-003-qualify-reference-bindings.md) constrains identity preservation.
- [IT-003](../../integration/IT-003-verification-reference-amendment.md) requires independent consumer qualification.

## Status

Accepted by FS05 as the exact internal verification-reference role amendment.
Consumer-specific implementation and integration acceptance remain separately
observed work.
