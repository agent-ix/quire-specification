---
id: FR-141
title: "Evaluate text and declaration-qualified enumerations"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/AD-005
    type: references
---

# FR-141: Evaluate text and declaration-qualified enumerations

## Description

When checking text or enumeration values, the semantic kernel SHALL apply the
selected text normalization and declaration-qualified member identity rules.

## Inputs

Text profile, text bounds, enum declarations, literals and comparison requests.

## Outputs

Typed text/enum values, comparison results or located refusals.

## Behavior

Text equality and ordering operate on the selected Unicode scalar/normalization
profile and enforce declared length bounds. Enumeration members are identified
by declaration identity plus member identity; source order defines ordering
only when the declaration explicitly selects ordered enumeration semantics.
Evolution never reuses a removed member identity for different meaning.

## Text and enumeration profiles

`unicode-scalars` compares the decoded scalar sequence without normalization;
`nfc`, `nfd`, `nfkc` and `nfkd` apply exactly the named Unicode normalization
form at the Unicode version pinned by the profile; `binary-utf8` compares the
original valid UTF-8 bytes. Length is scalar count for the first five profiles
and byte count for `binary-utf8`. Ordering is lexicographic over normalized
scalar values or unsigned bytes respectively. No locale, case folding,
collation table or grapheme segmentation is implicit.

An enumeration value is (`declaration identity`, `member identity`). Unordered
enumerations admit only equality/inequality. An `ordered enum` additionally
uses declaration order for comparison; insertion or reordering therefore
creates a new declaration revision. Optional display strings never participate
in equality or ordering.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-141-AC-1 | Canonically equivalent text compares according to the selected normalization profile and distinct text remains distinct. | Test (TC-186) |
| FR-141-AC-2 | Equal member spellings from different enum declarations are not equal. | Test (TC-186) |
| FR-141-AC-3 | Missing text bounds, invalid scalars, unordered enum ordering or reused member identity refuses. | Test (TC-186) |
| FR-141-AC-4 | Each text profile applies its exact length and lexicographic comparison domain; equal display text under different profiles remains differently typed. | Test (TC-186) |
| FR-141-AC-5 | An unordered enumeration comparison other than equality refuses, while an ordered enumeration follows declaration order only. | Test (TC-186) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
