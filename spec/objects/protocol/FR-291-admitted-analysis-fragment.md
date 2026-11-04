---
id: FR-291
title: "Admitted analysis fragment"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-060
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
---
# FR-291: Admitted analysis fragment

## Description

When an analysis result names its admitted fragment, the dispatcher SHALL
retain the exact fragment definition identity and the closed construct/profile
set actually admitted for that claim.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| definition_identity | String | 1..1 |  |
| claim_kinds | String | 1..1 |  |
| construct_profile_set | String | 1..1 |  |

## Field sources

- **definition identity** — nonempty owner-qualified fragment identity.
- **claim kinds** — sorted distinct nonempty set of FR-290 protocol-family members supported by the fragment; the fragment selects the FR-290 vocabulary `quire.capability-kind/v1`.
- **construct/profile set** — closed sorted identities of the syntax,
  semantic, clock and boundary profiles admitted by this fragment.

## Identity and comparison

- **Identity kind** — *authored* by the fragment-definition owner.
- **Key** — definition identity.
- **Key order** — lexicographic for deterministic collections only.
- **Causal order** — none; key order is not recency.
- **Absence** — an advanced result without a selected admitted fragment is
  refused; no backend name or supported-looking operator supplies one.

Changing the claim-kind or construct/profile set requires a new definition
identity. A result may cite only a claim kind and construct set admitted by the
exact selected fragment.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-291-AC-1 | Mutating identity, claim membership or construct/profile membership prevents reuse. | Test (TC-060, TC-061) |
| FR-291-AC-2 | Unknown, duplicate, absent or display-derived members are refused without a partial admitted fragment. | Test (TC-060) |
| FR-291-AC-3 | Backend capability does not enlarge the selected fragment, and fragment membership does not prove a claim. | Test (TC-060) |

## Dependencies

- [FR-060](../../functional/protocol/FR-060-separate-protocol-claims.md)
  requires every advanced result to identify its exact admitted fragment.
- [FR-290](FR-290-protocol-claim-kind.md) defines claim-kind membership.

## Status

Proposed shared ruling for `quire-specification#40`.
