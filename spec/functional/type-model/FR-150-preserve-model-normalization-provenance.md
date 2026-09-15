---
id: FR-150
title: "Preserve original and effective model declarations"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
  - target: ix://agent-ix/quire-specification/FR-035
    type: references
---

# FR-150: Preserve original and effective model declarations

## Description

When normalizing an authoritative model export, the correspondence layer SHALL
retain each original declaration identity, effective declaration identity and
ordered transformation provenance.

## Inputs

Exact producer export, producer/interface version and selected normalization
profile.

## Outputs

Immutable original-to-effective correspondence or typed ambiguity/refusal.

## Behavior

Normalization may derive normalized inherited/redefined views but never replaces
the original identity or source locus. Every derived member records its source
declarations and rule. Equal normalized shapes from distinct originals remain
distinct unless the model contract declares identity equivalence.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-150-AC-1 | An inherited/redefined member links its normalized identity to every contributing original declaration and rule. | Test (TC-195) |
| FR-150-AC-2 | Two equal effective shapes from distinct original identities remain distinguishable. | Test (TC-195) |
| FR-150-AC-3 | Missing producer revision, ambiguous rule order or cross-domain digest substitution refuses normalization. | Test (TC-195) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
