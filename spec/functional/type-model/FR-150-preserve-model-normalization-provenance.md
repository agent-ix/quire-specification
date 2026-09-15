---
id: FR-150
title: "Preserve original and effective model declarations"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
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

## Normalization derivation

The correspondence layer applies, in order: producer-version decoding,
qualified-identity formation, declared inheritance expansion, explicit
subsetting/redefinition, and profile-selected canonicalization. Each derived
fact records the input fact identities, rule identifier, rule version and
ordinal. A later phase may reference but cannot erase an earlier derivation.

Two applicable rules conflict when they derive unequal kind, type, multiplicity,
direction, endpoint or declaration identity for the same effective member and
neither rule is declared to override the other. Conflicts are refused with both
derivation paths. Canonical ordering affects serialization only; it never
changes producer identity or resolves semantic ambiguity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-150-AC-1 | An inherited/redefined member links its normalized identity to every contributing original declaration and rule. | Test (TC-195) |
| FR-150-AC-2 | Two equal effective shapes from distinct original identities remain distinguishable. | Test (TC-195) |
| FR-150-AC-3 | Missing producer revision, ambiguous rule order or cross-domain digest substitution refuses normalization. | Test (TC-195) |
| FR-150-AC-4 | Replaying the ordered derivation from immutable original declarations reproduces the effective model byte-for-byte. | Test (TC-195) |
| FR-150-AC-5 | Conflicting derivations report both rule paths and expose no chosen effective member. | Test (TC-195) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
