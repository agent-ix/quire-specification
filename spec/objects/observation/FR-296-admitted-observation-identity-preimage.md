---
id: FR-296
title: "Admitted observation identity preimage"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-110
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-260
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
---
# FR-296: Admitted observation identity preimage

## Description

When an observation is admitted or read, its observation identity SHALL be
derived from one exact RFC 8785 preimage so immutable fact populations
and corrections can compare observations without names, arrival order or
caller-authored identifiers.

## Properties

For the complete closed admitted-observation object `O`, let
`O_without_observationId` be `O` with exactly its `observationId` member removed.
BLOCKED-BY QSpec #513: observation identity digest (`sha256-jcs`, `quire.verification.jcs`).

The identity is `sha256-jcs:<hex>`, where `<hex>` is SHA-256 over the RFC 8785
canonical UTF-8 bytes of:

```text
{
  "identityVersion": "quire.observation.identity/v1-draft.1",
  "observation": O_without_observationId
}
```

The preimage includes the source schema identity, binding and subject
identities, typed valuation and units, visibility, event/ingestion time facts,
declared causal relation, clock interpretation, and every provenance field in
the selected closed observation contract. It excludes only `observationId`.
Unknown or duplicate fields are refused before derivation.

## Identity and comparison

- **Identity kind** — *derived*.
- **Key** — the complete textual `observationId`, including `sha256-jcs:`.
- **Key order** — none. Populations sort identities by their canonical UTF-8
  bytes only to obtain deterministic set encoding; that order has no temporal
  or causal meaning.
- **Causal order** — only an explicit declared causal or correction relation;
  event time, ingestion time and population order do not establish causality.
- **Absence** — an absent identity is refused, never recomputed as a
  reader default.
- **Digest domain** — `quire.verification.jcs` under FR-201.

Two accepted observations with one identity have byte-identical canonical
observation bytes. A correction preserves the predecessor bytes, changes at
least one preimage member, obtains a new identity and names the exact direct
predecessor. Same identity on unequal canonical bytes is refused.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-296-AC-1 | Independent RFC 8785/SHA-256 computation over the exact preimage yields the admitted observation identity. | Test (TC-140) |
| FR-296-AC-2 | Every semantic or provenance member mutation changes the identity; object-member presentation permutation does not. | Test (TC-140) |
| FR-296-AC-3 | Equal identity on unequal canonical bytes, missing identity, and a cross-domain identity are refused. | Test (TC-140) |
| FR-296-AC-4 | Event time, ingestion time, display name, trace identifier and arrival order cannot independently substitute for observation identity or causal order. | Test (TC-140, TC-143) |
| FR-296-AC-5 | A correction retains predecessor bytes, derives a new identity and names the exact direct predecessor. | Test (TC-143) |

## Dependencies

- [FR-110](../../functional/observation/FR-110-bind-typed-observations.md)
  declares observation admission and retained provenance.
- [FR-260](FR-260-admitted-observation.md) defines the complete observation
  object whose identity is derived.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) defines the JCS
  identity domain and cross-domain comparison.

## Status

Proposed shared ruling for `quire-specification#40`.
