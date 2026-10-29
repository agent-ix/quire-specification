---
id: FR-286
title: "Canonical protocol result key"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-240
    type: specifies
---
# FR-286: Canonical protocol result key

BLOCKED-BY QSpec #513: this requirement's `sha256-jcs` / `quire.verification.jcs` digest identity (rows removed from FR-201).

## Description

When a canonical `ProtocolResult` is produced or read, the producer SHALL identify it by
its structural `resultKey`, so that the identity cannot be authored
independently of the run that produced the result. No hash of the result is
computed.

## Properties

`resultKey` is the closed object `{producer, request, ordinal}`:

- `producer` — the protocol producer's identity and version (`{identity, version}`).
- `request` — the exact request identity the result answers.
- `ordinal` — a `U64String` position that the producer assigns to the result in
  its own result stream for that request. The producer assigns each ordinal once
  and never reuses it for different canonical result members.

All other result members are attributes of the keyed result. Unknown or
duplicate result members are refused by the closed result contract.

## Reader order and comparison

A strict reader SHALL validate in this order: (1) closed result shape,
`contractVersion` and `kind`; (2) the presence and completeness of `resultKey`;
(3) cross-field combination and lineage rules. The reader SHALL refuse the first
failed boundary and SHALL NOT repair or approximate the result. No
separate artifact digest is computed or compared.

- **Identity kind** — *entity*. The key is structural and is not derived from the
  result's bytes.
- **Key** — `resultKey`, compared componentwise.
- **Key order** — none. Lexicographic or timestamp order has no semantic force.
- **Causal order** — only explicit supersession or invalidation lineage. It is
  never inferred from the key, presentation order or time.
- **Absence** — an absent `resultKey` member is refused and never supplied as a
  reader default.

Two accepted canonical results with one `resultKey` SHALL have equal result
members. The reader SHALL check this by comparing the members of each accepted
result with those of every earlier accepted result of the same `request` that
has an equal `resultKey`, and SHALL refuse the later result when any member differs.
The same key with different members is an invalid result, not an
alternate representation. A later contradiction emits a new result with a new
`ordinal`, linked explicitly to the earlier unchanged result by its `resultKey`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-286-AC-1 | Withdrawn: it existed only for the SHA-256 identity computed over the result preimage, which is no longer computed. | Withdrawn |
| FR-286-AC-2 | Changing any `resultKey` member yields a different key; changing only an attribute member leaves the key unchanged; object-member presentation permutations change neither. | Test (TC-061) |
| FR-286-AC-3 | Withdrawn: it existed only for the enclosing artifact digest, which is no longer computed or validated. | Withdrawn |
| FR-286-AC-4 | A result whose `resultKey` equals that of an earlier accepted result of the same request but whose members differ is refused by member comparison, a result with equal key and equal members is accepted once, and a missing or incomplete `resultKey` is refused. | Test (TC-061) |
| FR-286-AC-5 | A superseding or invalidating result preserves the earlier result, carries a new `ordinal` and names the earlier result by its `resultKey`. | Test (TC-061) |

## Dependencies

- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md) owns the
  canonical result and its strict-reader rules.
- [FR-240](FR-240-canonical-protocol-result.md) defines the closed result object.

## Status

Accepted FS02 identity ruling, revised to a structural key. The key and reader
order above are normative; producers and readers refuse deviations without
approximation.
