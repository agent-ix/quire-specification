---
id: SR-528
title: "Dependency review of FS02 protocol identity rulings"
type: SpecReview
analysis: dependency
scope: "quire-specification #28 identity authorities and implemented consumer reality"
review_set: subset
evaluated_revision: "quire-specification 6a50ab6; quire-spec-language 6eccb05; quire-protocol e53fb55; quire-observation 41e4919"
review_date: "2026-09-13"
---
# Dependency review of FS02 protocol identity rulings

## Summary

PASS. The specification separates accepted owner rulings from partial existing
implementations. It reuses SL's admitted package/declaration/handle records and
observation's caller-owned identities, while leaving incompatible qprotocol
slot-derived runtime IDs as explicit downstream work. No TL-* ownership or file
is assigned or changed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Planned consumer correction: qprotocol derives workflow/message/delivery/attempt/effect IDs from static binding-requirement coordinates, which identify slots rather than concrete runtime subjects. FR-050/051 forbid the substitution and claim no implementation coverage. | qprotocol `src/link/mod.rs`; FR-050; FR-051 |
| FND-002 | medium | Partial evidence only: SL validates declaration, workflow spelling, node, role and repeat ordinals inside one admitted artifact, but its concrete workflow value lacks FR-287 authority/kind qualification. FR-049/FR-221 require the portable qualifier and later binder boundary. | SL `src/protocol_artifact/occurrence.rs`; FR-049; FR-221; FR-287; FR-288 |
| FND-003 | medium | Partial evidence only: qobservation preserves opaque identity and distinct delivery/receipt kinds, but authority selection is request-level and its subject-kind set omits other protocol runtime subjects. Expansion belongs to downstream observation/protocol integration. | qobservation `src/lib.rs`; FR-210; FR-287 |

## Implemented-reality inventory

| Repository/revision | Existing fact consumed by the ruling | Remaining work, not claimed here |
| --- | --- | --- |
| quire-spec-language `6eccb05` | Compiled package retains exact source identity/revision/digest, definition and model selections, formal requirement/clause, declaration-local handles and binding requirements. Occurrence binding retains declaration, caller workflow, node, role and exact outer-to-inner ordinals with typed refusals. | Exchange/adopt the complete FR-288 qualifier and FR-287 workflow reference at the consumer boundary. |
| quire-protocol `e53fb55` | Nominal ID newtypes preserve exact identity bytes and keep display separate; link-time declaration scopes include formal requirement coordinates. | Stop treating `requirement_identity` products as concrete runtime IDs; bind producer-supplied FR-287 values and separately validate their relationships. |
| quire-observation `41e4919` | Caller-owned non-empty opaque `Identity`, a typed `SubjectKind`, exact producer/package selections, and separate `Delivery`/`Receipt` subjects already exist. | Carry the complete authority-qualified reference and needed protocol kinds without collapsing producer-specific vocabularies into an invented global wire enum. |

## Dependency allocation

FR-202 and FR-030 supply exact source and semantic-selection inputs to FR-288.
FR-035 supplies the common producer boundary to FR-287. FR-050/051/057/058 and
the object layer consume those two values. The `specifies`/`references` edges are
authority links, not a circular implementation prerequisite: definitions of the
nouns land here; executable consumers follow under their own tickets.

The change touches only quire-specification. Agent B remains the exclusive owner
of TL-* libraries; this packet neither edits that lane nor makes progress depend
on unmerged TL work.
