---
id: FR-296
title: "Admitted observation key"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-110
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-260
    type: specifies
---
# FR-296: Admitted observation key

## Description

When an observation is admitted or read, the binder SHALL identify it by one
structural observation key, so immutable fact populations and corrections can
compare observations without names, arrival order or caller-authored
identifiers. No hash of the observation is computed.

## Properties

The observation key is the tuple `(source schema identity and version, declared
binding identity and version, selected subject identity, source ordinal)`:

- `source schema identity and version` and `declared binding identity and
  version` are those retained by FR-260.
- `selected subject identity` is the FR-287 key of the linked subject.
- `source ordinal` is the `U64String` position the observation source assigns to
  the record in its own immutable record stream. The source assigns each
  ordinal once; a producer SHALL NOT reuse it for different canonical bytes.

The remaining members of the closed observation (typed valuation and units,
visibility, event/ingestion time facts, declared causal relation, clock
interpretation and provenance) are attributes of the keyed observation, not
key members.

## Identity and comparison

- **Identity kind** — *entity*. The key is structural; it is not derived from the
  observation's bytes.
- **Key** — the four-member tuple above. Equality is componentwise.
- **Key order** — none. Populations sort keys componentwise only to obtain
  deterministic set encoding; that order has no temporal or causal meaning.
- **Causal order** — only an explicit declared causal or correction relation;
  event time, ingestion time and population order do not establish causality.
- **Absence** — an absent key member is refused, never filled by a reader default.

Two accepted observations with one key have equal canonical observation members.
A correction preserves the predecessor's members, carries a new source ordinal
and names the exact direct predecessor by key. The same key on unequal
observation members is refused.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-296-AC-1 | Withdrawn: it existed only for the SHA-256 identity computed over the observation preimage, which is no longer computed. | Withdrawn |
| FR-296-AC-2 | Changing any key member yields a different observation key; changing only an attribute member leaves the key unchanged; object-member presentation permutation changes neither. | Test (TC-140) |
| FR-296-AC-3 | The same key on unequal observation members, and a missing or incomplete key, are refused. | Test (TC-140) |
| FR-296-AC-4 | Event time, ingestion time, display name, trace identifier and arrival order cannot independently substitute for the observation key or causal order. | Test (TC-140, TC-143) |
| FR-296-AC-5 | A correction retains predecessor members, carries a new source ordinal and names the exact direct predecessor by key. | Test (TC-143) |

## Dependencies

- [FR-110](../../functional/observation/FR-110-bind-typed-observations.md)
  declares observation admission and retained provenance.
- [FR-260](FR-260-admitted-observation.md) defines the complete observation
  object that this key identifies.

## Status

Proposed shared ruling for `quire-specification#40`.
