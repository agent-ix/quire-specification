---
id: TC-280
title: "Model-owned member types"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-280: Model-owned member types

## Description

Show that a reader derives the type of a `field` or `operation` member whose
declaring node is a model declaration node from the lock-selected domain
package, as FR-322's "Model-owned members" states: it admits the selected
document as evidence, recovers the declaring node's `ModelOwner` by
recomputing the model declaration node keys, resolves the member name among
the declaring type's exposed effective members, own and inherited, and derives
the member type from the resolved record.

## Preconditions

- `proposals/checked-package-v2/model-member-type-vectors.json` carries the
  domain package `acme/orders` `1.0.0` as a projection of the declaration
  records FR-154 intake reads from it (`typeRef`, `interfaceFeatures`,
  value types with the `Int[lo, hi]` they are bound as (TC-196, TC-197), presence, multiplicity, supertypes
  and redefinitions), the model declaration node vectors M1 to M9, the member
  cases, each recorded with its criterion, the wire cases and the selection
  cases. The SHA-256 of the RFC 8785 bytes of its `domain_package` member
  stands for the document's `sha256-jcs` digest.
- The lock selects `acme/orders` `1.0.0` only.
- `node-identity-preimage.schema.json` defines the closed
  `ModelDeclarationNode` preimage.

## Test Procedure

Validate each model declaration node vector against the closed preimage schema
and recompute its JCS SHA-256. Run step 2 over each wire case. For each member
case, recover the declaring node's owner from the selected domain package,
resolve the member and derive its type or refusal, and compare with the
recorded expectation. An operation case also carries its call's receiver type
and argument types, and a reference type names its target by the recomputed
model declaration node key. For each selection case, run FR-322's selection
admission checks over the supplied documents and compare with the recorded
result.

## Vectors

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| MT-01 | `field` `total` on M1 (`Order`), typed value type `Digit`, `Integer` bound to `[0, 9]` | Resolves `Order/total`; type `Int[0, 9]` | FR-322-AC-31 |
| MT-02 | `field` `total` on M5 (`Sub`, supertype `Order`) | Resolves the inherited `Order/total`; type `Int[0, 9]` | FR-322-AC-30 |
| MT-03 | `field` `note`, optional `Boolean` `[1, 1]` | `Option<Boolean>` | FR-322-AC-31 |
| MT-04 | `field` `lines`, `Integer` `[0, 3]`, unordered, unique | `Set<Integer>[0, 3]` | FR-322-AC-31 |
| MT-05 | `field` `history`, `Integer` `[0, 5]`, ordered, not unique | `Sequence<Integer>[0, 5]` | FR-322-AC-31 |
| MT-06 | `field` `invoice`, typed object type `Invoice` | `Reference<Invoice>` over M3 | FR-322-AC-31 |
| MT-07 | `operation` `size` on M1, receiver `Reference<Order>` | Resolves `Order/size`; type `Integer` | FR-322-AC-31 |
| MT-08 | `operation` `size` on M5, which `Sub/size` redefines | Resolves `Sub/size`; type `Integer` | FR-322-AC-30 |
| MT-09 | `operation` `scaled` on M5, redefined with a renamed parameter, called with an `Integer` argument | Resolves `Sub/scaled`; type `Integer` | FR-322-AC-30 |
| MT-10 | `operation` `size` on M7 (`SubSub`, under `Sub`), which both `Sub/size` and `SubSub/size` redefine | Resolves the most derived `SubSub/size`; type `Integer` | FR-322-AC-30 |
| MT-11 | `operation` `scaled` on M1 with an `Integer` argument | Resolves `Order/scaled`; type `Integer` | FR-322-AC-31 |
| MT-12 | `field` `floor`, native `Integer` carrying a `min` constraint | `Integer` | FR-322-AC-31 |
| MT-13 | `field` `tags`, `Integer` `[0, unbounded]`, unordered, unique | `Set<Integer>` with no bounds | FR-322-AC-31 |
| MT-14 | `field` `pick`, `Integer` `[0, 1]`, unordered, unique | `Set<Integer>[0, 1]` | FR-322-AC-31 |
| MA-01 | `field` `missing` on M1 | `ill_typed`/`operator-ineligible` | FR-322-AC-30 |
| MA-02 | `field` `size`, an operation name | `ill_typed`/`operator-ineligible` | FR-322-AC-30 |
| MA-03 | `operation` `total`, a field name | `ill_typed`/`operator-ineligible` | FR-322-AC-30 |
| MA-04 | `field` `total` on M4, the relationship `billedTo` | `ill_typed`/`operator-ineligible` | FR-322-AC-30 |
| MA-05 | `field` `total` on M2, `Order` keyed under version `2.0.0` | `missing_declaration`/`missing-selection` | FR-322-AC-29 |
| MA-06 | `field` `label`, typed `Text` | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| MA-07 | `field` `backlog`, `[1, unbounded]` | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| MA-08 | `operation` `reset`, with no result | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| MA-09 | `field` `code` on M6 (`Both`), declared by both supertypes `Tagged` and `Coded` | `ambiguous_declaration`/`ambiguous-name` | FR-322-AC-30 |
| MA-10 | `operation` `scaled` on M1 with no argument | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| MA-11 | `operation` `scaled` on M1 with a `Boolean` argument | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| MA-12 | `operation` `rate` on M8 (`Flow`, a `systems_interface`) | `ill_typed`/`operator-ineligible` | FR-322-AC-30 |
| MA-13 | `operation` `size` on M1 with receiver `Reference<Invoice>` | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| MW-01 | A `model`/`object_type` wire node with M1's id, its own id as `semantic_type` and an empty body | Recovers `Order` | FR-322-AC-29 |
| MW-02 | MW-01 with a body member | `invalid_package`/`stale-node-key` | FR-322-AC-29 |
| MW-03 | MW-01 with a `declaration` | `invalid_package`/`stale-node-key` | FR-322-AC-29 |
| MW-04 | MW-01 with M4's id as `semantic_type` | `invalid_package`/`stale-node-key` | FR-322-AC-29 |
| MW-05 | MW-01 with a `recursion_group` | `invalid_package`/`stale-node-key` | FR-322-AC-29 |
| MW-06 | A `model`/`object_type` wire node carrying M4's id | `missing_declaration`/`missing-selection` | FR-322-AC-29 |
| MS-01 | The document supplied under the selected digest, identity and version equal | Passes the selection admission checks | FR-322-AC-32 |
| MS-02 | No document supplied | `missing_import`/`missing-selection` | FR-322-AC-32 |
| MS-03 | A document with `Digit`'s upper bound changed to 8, supplied under the selected digest | `stale_dependency`/`byte-digest-mismatch` | FR-322-AC-32 |
| MS-04 | The selection names version `1.0.1` | `invalid_model_binding`/`wrong-model-selection` | FR-322-AC-32 |
| MS-05 | The selection names digest domain `sha256` | `stale_dependency`/`digest-domain-mismatch` | FR-322-AC-32 |

## Expected Results

M1 to M9 validate and recompute their digests. Every wire, member and
selection case gives exactly its recorded result.
