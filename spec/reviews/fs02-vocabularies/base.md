---
id: SR-523
title: "Base review of FS02 vocabulary closure"
type: SpecReview
analysis: base
scope: "quire-specification #29; FR-047, FR-061, FR-062, FR-091, FR-092, FR-115, FR-240, FR-242, FR-243, FR-249, FR-271, FR-273, FR-283, FR-285 and FR-286"
review_set: subset
evaluated_revision: "task/29 working tree based on cc6cb90"
review_date: "2026-09-13"
---
# Base review of FS02 vocabulary closure

## Summary

PASS. The scoped packet gives each issue #29 decision one authority, preserves
unknown and absent values, and maps every changed acceptance criterion to an
honest planned mutation case without claiming implementation evidence.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No blocking ID, consistency, completeness, ambiguity or planned-test coverage finding remains in the scoped vocabulary packet. | issue #29; FR-273; FR-243; FR-283; FR-240 |

## Checklist disposition

| Issue #29 obligation | Authoritative disposition |
| --- | --- |
| Producer stages | FR-273 is a producer-qualified value: exact interface identity/revision/digest plus one member of that interface's closed vocabulary. The former six phrases are coverage concerns and the eleven generic phrases are not a global enum. Unknown pairs retain exact input and refuse. |
| Digest mismatch refusal | FR-283 owns six exact projection codes. Source validation or recomputed source-digest failure is `invalid-source-result`; only a valid source reaches target/derived `identity-mismatch`. |
| Settlement basis | FR-243 owns exactly five values. Temporal incomplete/refused outcomes carry no lower truth/basis fact and the explicit canonical adapter produces the `unavailable` pair. Absence remains distinct. |
| Canonical result identity | FR-286 fixes a derived, versioned RFC 8785/SHA-256 preimage and validation order. Same identity with different canonical bytes refuses; supersession creates a new linked identity. |
| Participation | FR-285 is a closed `known`/`unknown` value object, not a source status enum. E02 statuses are total derivations and cannot be copied back. |
| Consumer imports | FR-047, FR-061, FR-062 and FR-115 reference the new authorities; FR-240/249 remove their competing open alternatives. |

IDs FR-285 and FR-286 are unused and sequential after FR-284. Every changed or
new acceptance criterion is mapped through TM-003, TM-004 or TM-007 to TC-047,
TC-061, TC-064 or TC-145. The cases cover every selected member, unknown and
absent values, mixed shapes, identity/digest mutations, cross-interface labels,
display-text substitution and projection-loss boundaries. All rows remain
`Planned`; no Rust implementation or trace tag is fabricated.

Security and performance requirements are not introduced by this vocabulary
decision. The material safety property is strict refusal: no fallback,
normalization, first-wins choice, approximation or reader resealing is allowed.
