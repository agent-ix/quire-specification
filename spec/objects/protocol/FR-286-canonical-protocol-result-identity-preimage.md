---
id: FR-286
title: "Canonical protocol result identity preimage"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-240
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
---
# FR-286: Canonical protocol result identity preimage

## Description

When a canonical `ProtocolResult` is produced or read, its `resultId` SHALL be
derived from one exact versioned RFC 8785 preimage so that the identity cannot
be authored independently of the complete closed result it names.

## Properties

For a complete closed result object `R`, let `R_without_resultId` be `R` with
exactly its `resultId` member removed. The identity is the textual value
`sha256-jcs:<hex>`, where `<hex>` is 64 lowercase hexadecimal digits of SHA-256
over the RFC 8785 canonical UTF-8 bytes of exactly this value:

```text
IdentityPreimage := {
  "identityVersion": "quire.protocol.result-identity/v1-draft.1",
  "result": R_without_resultId
}
```

`R_without_resultId` denotes the JSON object value defined above, not a string
or wire token. RFC 8785 is applied to the JSON object on the right-hand side of
`:=`; the notation itself is not hashed. The preimage includes every other
result member and excludes only `resultId`.
Unknown or duplicate result members are refused by the closed result contract
before identity computation. The enclosing artifact digest is separate: it is
computed over the complete canonical result bytes including `resultId`, lives in
the enclosing reference outside those bytes, and does not equal or replace the
result identity merely because both select `quire.verification.jcs`.

## Reader order and comparison

A strict reader SHALL validate in this order: (1) closed result shape,
`contractVersion` and `kind`; (2) recompute and compare `resultId`; (3) recompute
and compare the enclosing artifact digest; (4) validate cross-field combination
and lineage rules. The reader SHALL refuse the first failed boundary and SHALL
NOT repair, approximate or reseal the result.

- **Identity kind** — *derived* from the exact preimage above.
- **Key** — the complete textual `resultId`, including its domain prefix.
- **Key order** — none. Lexicographic or timestamp order has no semantic force.
- **Causal order** — only explicit supersession or invalidation lineage. It is
  never inferred from identity, presentation order or time.
- **Absence** — an absent `resultId` is refused and never computed as a reader
  default.
- **Digest domain** — `quire.verification.jcs` under FR-201. Equal digest bytes
  under another domain are unequal.

Two accepted canonical results with one `resultId` SHALL have byte-identical
RFC 8785 canonical result bytes. Same identity with different canonical bytes is
an invalid result, not an alternate representation. A later contradiction emits
new bytes and therefore a new result identity, linked explicitly to the earlier
unchanged result.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-286-AC-1 | Independent RFC 8785/SHA-256 computation over the exact versioned preimage yields the emitted `resultId`. | Test (TC-061) |
| FR-286-AC-2 | Any change to a result member other than `resultId` changes the recomputed identity; object-member presentation permutations do not. | Test (TC-061) |
| FR-286-AC-3 | The enclosing digest covers complete canonical result bytes including `resultId`, remains outside those bytes and is validated after `resultId`. | Test (TC-061) |
| FR-286-AC-4 | Same `resultId` with different canonical result bytes, a missing identity or an identity from another domain is refused. | Test (TC-061) |
| FR-286-AC-5 | A superseding or invalidating result preserves the earlier bytes, has a newly derived identity and carries explicit lineage. | Test (TC-061) |

## Dependencies

- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md) owns the
  canonical result and its strict-reader rules.
- [FR-240](FR-240-canonical-protocol-result.md) defines the closed result object.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) defines the selected
  JCS digest domain.

## Status

Accepted FS02 identity ruling. The preimage and reader order above are
normative; producers and readers refuse deviations without approximation.
