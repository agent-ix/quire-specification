---
id: FR-130
title: "Interpret composed-package references without flattening clause profiles"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-005
    type: implements
  - target: ix://agent-ix/quire-specification/FR-030
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-032
    type: depends_on
---
## Description

When binding a semantic reference to a composed linked package, the reader SHALL
interpret its outer package definition separately from the exact semantic
profiles of its constituent declarations.

## Inputs

An exact shared-reference envelope, selected package artifact interpretation,
declared payload wire, source/model/definition closure, inner declarations and
explicit required-feature set.

## Outputs

A reference to the fully linked static package retaining each inner profile,
or a typed version, role, feature-set, canonical-domain or dependency refusal.

## Behavior

The [package-reference candidate](../../proposals/quire-v1/definitions/package-reference.md)
defines the outer interpretation, four declaration/family features and explicit
null canonical identity. The outer definition identifies a container contract;
it cannot identify the particular package without its content and inner closure.
Package features describe declarations, not backend support, executed paths or
full-release conformance. A payload reader must support the actual payload wire;
envelope schema acceptance alone cannot admit it. Historical selections retain
their original meanings and do not migrate to this candidate implicitly.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-130-AC-1 | A three-family package uses its selected outer package interpretation while preserving every inner profile; substituting a constituent's profile for the outer one refuses, including for a one-family composed package. | Test (TC-030) |
| FR-130-AC-2 | Selecting the package artifact interpretation as a clause's using profile refuses rather than granting state, temporal or protocol forms. | Test (TC-030) |
| FR-130-AC-3 | The exact unique feature set matches declared kinds; predicate-only does not claim family.state, an extra known family.temporal on a state-only package refuses, and unique set reordering preserves interpretation. | Test (TC-030) |
| FR-130-AC-4 | Changing an inner profile changes the static package subject even when the outer definition identity is unchanged; outer feature flags cannot establish implementation or full-v1 conformance. | Test (TC-030) |
| FR-130-AC-5 | This candidate refuses an unknown payload wire or a non-null whole-package canonical identity, including an unrelated IR/JCS digest, despite a structurally valid reference envelope. | Test (TC-030) |
| FR-130-AC-6 | A partial compilation cannot claim a fully linked package through this interpretation; independent checked child references and scoped results remain available under FR-030/031. | Test (TC-030) |

## Dependencies

- [Exact package linking](FR-030-bind-composed-definitions.md).
- [Language/wire evolution](FR-032-preserve-language-evolution.md).
- [Planned shared matrix](../composed-foundation/tests.md).

Proposed under FS05/L2 and the existing compiler #35 scope. The requirement and
definition do not implement or qualify a strict package reader.
