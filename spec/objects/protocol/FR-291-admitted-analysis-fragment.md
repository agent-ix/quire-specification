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
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
---
# FR-291: Admitted analysis fragment

## Description

When an analysis result names its admitted fragment, the dispatcher SHALL
retain the exact versioned fragment definition and the closed construct/profile
set actually admitted for that claim.

## Properties

- **definition identity** — nonempty owner-qualified fragment identity.
- **revision** — positive immutable revision.
- **digest** — exact canonical definition-byte digest and selected FR-201 domain.
- **claim kinds** — sorted distinct nonempty FR-290 member set supported by the fragment.
- **construct/profile set** — closed sorted identities of the syntax,
  semantic, clock and boundary profiles admitted by this revision.

## Identity and comparison

- **Identity kind** — *authored* by the fragment-definition owner.
- **Key** — (definition identity, revision, digest domain, digest).
- **Key order** — lexicographic for deterministic collections only.
- **Causal order** — explicit successor revision only; key order is not recency.
- **Absence** — an advanced result without a selected admitted fragment is
  refused; no backend name or supported-looking operator supplies one.
- **Digest domain** — the selected FR-201 domain is part of the key.

Changing the claim-kind or construct/profile set requires a new revision and
digest. A result may cite only a claim kind and construct set admitted by the
exact selected fragment.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-291-AC-1 | Mutating identity, revision, digest domain, digest, claim membership or construct/profile membership prevents reuse. | Test (TC-060, TC-061) |
| FR-291-AC-2 | Unknown, duplicate, absent or display-derived members are refused without a partial admitted fragment. | Test (TC-060) |
| FR-291-AC-3 | Backend capability does not enlarge the selected fragment, and fragment membership does not prove a claim. | Test (TC-060) |

## Dependencies

- [FR-060](../../functional/protocol/FR-060-separate-protocol-claims.md)
  requires every advanced result to identify its exact admitted fragment.
- [FR-290](FR-290-protocol-claim-kind.md) defines claim-kind membership.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) defines digest
  domain comparison.

## Status

Proposed shared ruling for `quire-specification#40`.
