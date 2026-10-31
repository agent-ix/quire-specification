---
id: FR-201
title: "Canonical identity domain vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-015
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-150
    type: specifies
---
# FR-201: Canonical identity domain vocabulary

## Description

When two identities are compared, the reader SHALL compare them componentwise
by their named components ("Structural identities" below), so that no identity
is derived from a hash of bytes and no identity is compared by anything other
than its components.

## Identity and comparison

- **Identity kind** — *value*, compared componentwise.
- **Key** — the named components of the identity, matched exactly. Spellings are
  not normalized aliases.
- **Key order** — the component order stated below, for collection comparison
  only. It carries no precedence, preference or causal meaning.
- **Absence** — an absent component compares only to absence and is never
  defaulted.
- **Causal order** — not applicable. An identity participates in no causal
  structure.

## Structural identities

A structural identity is a value of named components compared componentwise,
never a digest and never derived from a hash. Two structural identities are
equal exactly when every component is equal; no other member of the record
that carries an identity contributes to it. Every component below is a value
defined elsewhere in this specification; no identity adds a package string, a
model selection identity or a digest. Components compare in the order stated,
strings by their UTF-8 bytes, arrays element by element with a proper prefix
first and integers numerically.

| Identity | Exact members, in comparison order | Components |
| --- | --- | --- |
| `NodeId` | `package`, `path`, `ordinal` | The structural key of a checked-package node, FR-322 "Structural node identity": `package` a string, `path` an array of strings and the structural keys of operand nodes, `ordinal` an integer in `0..=9007199254740991`. This is the identity value. A checked package's wire writes each operand as an index segment into its own graph and never nests a node id ("Operand index segments"), and a reader rebuilds this value from it. |
| `EffectiveId` | `owner_effective_type`, `original` | The effective declaration identity of FR-150: `owner_effective_type` is `null` (compared first) or an `EffectiveId`; `original` is the original declaration key `{package, node}`, compared by `package` then `node`. |
| `MemberId` | `owner`, `member` | One record, event or object member. `owner` is the arm `{"node_id": NodeId}` for a member of a declaration node (a `composite_type`/`record` node, a `model`/`record_value_type` or `model`/`event_type` node), or the arm `{"effective_id": EffectiveId}` for a member of an effective declaration (the `EffectiveId` of the effective type that owns the member); `member` is the declared field identifier, byte for byte (FR-143 "Declarations and identity"). The arm `node_id` compares before the arm `effective_id`, then the arm value componentwise, then `member`. |
| `VariantId` | `owner`, `case` | One enumeration or sum case. `owner` is the `NodeId` of the declaration node (the `scalar_type`/`enum`, `composite_type`/`union` or `model`/`variant_type` node); `case` is the case or member identifier, byte for byte (FR-141, FR-441). It is the pair that the path of the `value`/`enum_value` node carries. |
| `UnitId` | one arm: `declared`, or `terms` | The arm `declared` holds the `NodeId` of the declared unit node (FR-142). The arm `terms` holds the ascending array of `{unit, exponent}`, `unit` a `NodeId` of a canonical root unit, `exponent` a nonzero mathematical integer written as a canonical decimal string (FR-142, the evaluator-owned compound unit value); the empty array is the dimensionless unit. The arm `declared` compares before the arm `terms`; `terms` elements compare by `unit` then `exponent` numerically. |
| `UniverseId` | the `NodeId` itself | The `NodeId` of the declaration node of the least root type of the object universe (FR-150 "Object universe"); for the context universe of a state-machine context, the `NodeId` of the most-specific machine's model declaration node (FR-208 "Context universe key"). |
| `PopulationId` | the `NodeId` itself | The `NodeId` of the `relation`/`population` node that carries the declared population (FR-322 "Population references"). The binding, anchor and declared maximum are not components. |

An object's identity stays the authored object identity within its universe and
type, the FR-204 triple (universe, type, object identity), and an obligation
identity is (the claim or subject `node_id`, the identity of the model that
owns it, an ordinal) (FR-418-AC-9). An effective declaration identity is
`{owner_effective_type, original}` and carries no ordinal.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-201-AC-1 | Withdrawn: the identity-domain label vocabulary is removed with the digest domains it labelled. | Withdrawn |
| FR-201-AC-2 | Withdrawn: it compared digests across domain labels, and no digest domain remains. | Withdrawn |
| FR-201-AC-3 | Withdrawn: it concerned a digest with an absent domain, and no digest remains. | Withdrawn |
| FR-201-AC-4 | Withdrawn: the lock-file digest and package fingerprint domains are removed. | Withdrawn |
| FR-201-AC-7 | Withdrawn: the byte-equality test of the exposed constants against their files is removed. | Withdrawn |
| FR-201-AC-8 | Withdrawn: it existed only for the cache key, which is removed with the result cache. | Withdrawn |
| FR-201-AC-9 | Package selection is exact under the closed triggers `text_bearing`, `ieee_operation` and `integer_div_rem`, and each accepted and refused selection vector is admitted or refused with exactly its recorded selection code. | Test (TC-233) |
| FR-201-AC-10 | Two obligations of one clause, application, function, parity claim, function-value parity claim or per-step record whose records differ only in a member outside the triple (`source_span`, a resource ceiling, an embedded oracle, a symbol, the subject path, the results, the solver, `unwind` or an option) have equal obligation identities; changing the subject `node_id`, the owning model identity or the ordinal gives a different identity. | Test (TC-952) |
| FR-201-AC-11 | Each of `MemberId`, `VariantId`, `UnitId`, `UniverseId` and `PopulationId` is the exact member list of "Structural identities": two values are equal exactly when their arms and components are equal, a change of any single component gives a different identity, and the comparison order of that table orders a collection of them. | Test (TC-233) |

## Dependencies

- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity discipline.
- [FR-150](../../functional/type-model/FR-150-preserve-model-normalization-provenance.md)
  defines the effective declaration identity.
- QSL ADR-013 O-09 and [AD-016](../../assurance/AD-016-semantic-family-extension-path.md)
  arrow 5 define the obligation identity;
  [FR-196](../../functional/ir-proof/FR-196-generate-kani-contracts.md) and
  [FR-197](../../functional/ir-proof/FR-197-replay-backend-counterexamples.md)
  generate and replay the obligations it identifies.

## Status

Accepted complete-V1 structural identity contract.
