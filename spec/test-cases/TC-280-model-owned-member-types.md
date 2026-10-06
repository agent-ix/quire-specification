---
id: TC-280
title: "Model-owned member types"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-340
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-341
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-342
    type: verifies
---

# TC-280: Model-owned member types

## Description

Show that a reader derives the type of a `field` or `operation` member whose
declaring node is a model declaration node from the lock-selected domain
package, as FR-322's "Model-owned members" states: it admits the selected
document as evidence, reads the declaring node's `ModelOwner` from its
`owner` member, checks the node's key against its `ModelDeclarationNode`
preimage, looks up the declaration that owner names, resolves the member name among
the declaring type's exposed effective members, own and inherited, and derives
the member type from the resolved record.

## Preconditions

- `proposals/checked-package-v2/domain-package-acme-orders.json` is the
  selected evidence: the FCD semantic IR 2.0.0 domain package `acme/orders`
  `1.0.0` (FR-154), authored by hand with FCD-minted identities (types
  `ix://acme/orders/<Id>`, members `<type>/<member>`, relationships
  `ix://acme/orders/relationship/<Name>-<verb>-<Target>`). Its `sha256-jcs`
  digest is the digest every published selection of `acme/orders` carries.
- `proposals/checked-package-v2/model-member-type-vectors.json` carries the
  `domain_package` member, the projection of the document's declarations
  that FR-154 intake reads, derived from the document (a test derives it and
  compares) (`typeRef`, `interfaceFeatures`,
  value types with the `Int[lo, hi]` they are bound as (TC-196, TC-197) and with
  the `Text[min, max; profile]` they are bound as, presence, multiplicity, supertypes
  and redefinitions), the model declaration node vectors M1 to M12, the member
  cases, each recorded with its criterion, the wire cases and the selection
  cases. The selection cases patch and digest the document, not the projection.
  The value type `OrderTotal` is the alias FR-095 mints for `Order`'s
  constrained field `total` (`<Name><Field>`), with its constraints under
  `constraint/Order-total-min` and `-max`.
  Status: the vectors drop the redefined-operation cases (MT-08, MT-09, MT-10,
  AN-02) and the text-profile cases (MT-15, MA-14, MA-15, MA-16); AGE-2226
  (filament-core-data: operation `redefines`, text profile, value-type
  constraint slots) restores them against the next FCD revision.
- The wire cases are hand authored: each is a single model declaration
  node, a shape QSL emits only inside a package, keyed by QSL FR-094 exactly
  as emitter output would be. The model declaration nodes of the positive
  fixtures are QSL emitter output (TC-233).
- The lock selects `acme/orders` `1.0.0` only.
- `node-identity-preimage.schema.json` defines the closed
  `ModelDeclarationNode` preimage.

## Test Procedure

Validate each model declaration node vector against the closed preimage schema
and recompute its JCS SHA-256. Run FR-322's owner presence rule, the
derived-shape key stage's owner join and currency check, and step 2 over
each wire case; each wire case's node carries the `owner` its row names. For each member
case, look up the declaration the declaring node's owner names in the
selected domain package,
resolve the member and derive its type or refusal, and compare with the
recorded expectation. An operation case also carries its call's receiver type
and argument types, and a reference type names its target by the recomputed
model declaration node key. For each selection case, run FR-322's selection
admission checks over the supplied documents and compare with the recorded
result. For each frame field case, look up the declaration the declaring node's
owner names and
resolve the name among its exposed fields (FR-340 "Field entries"). For each
anchor case, resolve the operation among the context's exposed operations
and require it to be declared by the context itself (FR-342). For each clause
signature case, derive the list `self`, then the result for a postcondition
of an operation with one, then the operation's parameters (FR-341), and
compare it with the case's parameter types.

## Vectors

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| MT-01 | `field` `total` on M1 (`Order`), typed value type `OrderTotal`, `Integer` bound to `[0, 9]` | Resolves `Order/total`; type `Int[0, 9]` | FR-322-AC-31 |
| MT-02 | `field` `total` on M5 (`Sub`, supertype `Order`) | Resolves the inherited `Order/total`; type `Int[0, 9]` | FR-322-AC-30 |
| MT-03 | `field` `note`, optional `Boolean` `[1, 1]` | `Option<Boolean>` | FR-322-AC-31 |
| MT-04 | `field` `lines`, `Integer` `[0, 3]`, unordered, unique | `Set<Integer>[0, 3]` | FR-322-AC-31 |
| MT-05 | `field` `history`, `Integer` `[0, 5]`, ordered, not unique | `Sequence<Integer>[0, 5]` | FR-322-AC-31 |
| MT-06 | `field` `invoice`, typed object type `Invoice` | `Reference<Invoice>` over M3 | FR-322-AC-31 |
| MT-07 | `operation` `size` on M1, receiver `Reference<Order>` | Resolves `Order/size`; type `Integer` | FR-322-AC-31 |
| MT-11 | `operation` `scaled` on M1 with an `Integer` argument | Resolves `Order/scaled`; type `Integer` | FR-322-AC-31 |
| MT-12 | `field` `floor`, native `Integer` carrying a `min` constraint | `Integer` | FR-322-AC-31 |
| MT-13 | `field` `tags`, `Integer` `[0, unbounded]`, unordered, unique | `Set<Integer>` with no bounds | FR-322-AC-31 |
| MT-14 | `field` `pick`, `Integer` `[0, 1]`, unordered, unique | `Set<Integer>[0, 1]` | FR-322-AC-31 |
| MA-01 | `field` `missing` on M1 | `ill_typed`/`operator-ineligible` | FR-322-AC-30 |
| MA-02 | `field` `size`, an operation name | `ill_typed`/`operator-ineligible` | FR-322-AC-30 |
| MA-03 | `operation` `total`, a field name | `ill_typed`/`operator-ineligible` | FR-322-AC-30 |
| MA-04 | `field` `total` on M4, the relationship `Order-order-Order` | `ill_typed`/`operator-ineligible` | FR-322-AC-30 |
| MA-05 | `field` `total` on M2, `Order` owned by the unselected `acme/billing` | `missing_declaration`/`missing-selection` | FR-322-AC-29 |
| MA-06 | `field` `label`, typed native `Text` with no bounds | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| MA-07 | `field` `backlog`, `[1, unbounded]` | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| MA-08 | `operation` `reset`, with no result | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| MA-09 | `field` `code` on M6 (`Both`), declared by both supertypes `Tagged` and `Coded` | `ambiguous_declaration`/`ambiguous-name` | FR-322-AC-30 |
| MA-10 | `operation` `scaled` on M1 with no argument | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| MA-11 | `operation` `scaled` on M1 with a `Boolean` argument | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| MA-12 | `operation` `rate` on M8 (`Flow`, a `systems_interface`) | `ill_typed`/`operator-ineligible` | FR-322-AC-30 |
| MA-13 | `operation` `size` on M1 with receiver `Reference<Invoice>` | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| RE-ends-1 | `relationship_end` `order` on M4, the relationship `Order-order-Order` | Resolves the end `order` | FR-322-AC-56 |
| RE-ends-2 | `relationship_end` `next` on M4 | Resolves the end `next` (`targetEnd.role`) | FR-322-AC-56 |
| RE-ends-3 | `relationship_end` `invoiced` on M10, the relationship `Order-invoiced-Invoice` with no `targetEnd.role` | Resolves the end `invoiced` | FR-322-AC-56 |
| RE-ends-4 | `relationship_end` `next` on M10 | `missing_declaration`/`missing-name` at `member.name`: M10 has no inverse end | FR-322-AC-56 |
| RE-ends-5 | `relationship_end` `invoice` on M10 | `missing_declaration`/`missing-name` | FR-322-AC-56 |
| RE-ends-6 | `relationship_end` `order` on M1, an object type | `ill_typed`/`operator-ineligible` at `member.declaration` | FR-322-AC-56 |
| RE-ends-7 | `relationship_end` `follows` on M11, the `source-to-target` relationship `Order-follows-Order` | Resolves the end `follows` | FR-322-AC-56 |
| RE-ends-8 | `relationship_end` `followedBy` on M11 (the named inverse, traversed target to source) | `ill_typed`/`operator-ineligible` at `member.name`: the direction is `source-to-target` (FR-152) | FR-322-AC-56 |
| RE-ends-9 | `relationship_end` `twin` on M12, whose source and target roles are both `twin` | `ambiguous_declaration`/`ambiguous-name` at `member.name` | FR-322-AC-56 |
| MW-01 | A `model`/`object_type` wire node with M1's id, M1's owner (`acme/orders`, `ix://acme/orders/Order`), its own id as `semantic_type` and an empty body | Resolves `Order` | FR-322-AC-29 |
| MW-02 | MW-01 with a body member | `invalid_package`/`stale-node-key` | FR-322-AC-29 |
| MW-03 | MW-01 with a `declaration` | Fails the V2 schema; `malformed_wire` | FR-322-AC-29 |
| MW-04 | MW-01 with M4's id as `semantic_type` | `invalid_package`/`stale-node-key` | FR-322-AC-29 |
| MW-05 | MW-01 with a `recursion_group` | `invalid_package`/`stale-node-key` | FR-322-AC-29 |
| MW-06 | A `model`/`object_type` wire node carrying M4's id and M4's owner (`acme/orders`, `ix://acme/orders/relationship/Order-order-Order`) | `missing_declaration`/`missing-selection` at the owner join: `Order-order-Order` is a relationship, and the join runs over every owner-bearing node before either key pass, so the stale key (M4's id on an object-type node) is never reached; the outcome equals MW-07's | FR-322-AC-29, FR-322-AC-55 |
| MW-07 | A `model`/`object_type` wire node carrying M4's owner, keyed by its own `model`/`object_type` preimage | `missing_declaration`/`missing-selection` at the owner join: `Order-order-Order` is a relationship | FR-322-AC-29, FR-322-AC-55 |
| MW-08 | A `model`/`object_type` wire node with M2's id and M2's owner (`acme/billing`, `ix://acme/billing/Order`) | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-29 |
| MW-09 | MW-01 with no `owner` | Fails the V2 schema; `malformed_wire` | FR-322-AC-29 |
| MW-10 | A `model`/`object_type` wire node with owner (`acme/orders`, `ix://acme/orders/Nope`), keyed by its own preimage, that no member reaches | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-55 |
| MW-11 | A clause function wire node (`clause` binding `precondition`) owned by (`acme/orders`, `ix://acme/orders/Order/size`), keyed by its own preimage (FR-094 C1) | Passes the owner join | FR-322-AC-55 |
| MW-12 | A clause function wire node (`clause` binding `invariant`) owned by (`acme/orders`, `ix://acme/orders/Order`), keyed by its own preimage | Passes the owner join | FR-322-AC-55 |
| MW-13 | MW-11 owned by `ix://acme/orders/Order/missing`, rekeyed | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-55 |
| MW-14 | MW-11 owned by `ix://acme/orders/Order/total`, a field, rekeyed | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-55 |
| MW-15 | A `model`/`object_type` wire node carrying M4's id (the relationship's) and M1's owner (`acme/orders`, `ix://acme/orders/Order`), its own id as `semantic_type` and an empty body | `invalid_package`/`stale-node-key`: the owner joins, the key was computed under another tag | FR-322-AC-29 |
| MW-16 | A clause function (`clause` binding `precondition`) owned by the object type `ix://acme/orders/Order`, keyed by its own preimage | `missing_declaration`/`missing-selection` at the owner join: only an invariant is owned by an object type | FR-322-AC-55 |
| MW-17 | A clause function (`clause` binding `body`) owned by `ix://acme/orders/Order/size`, keyed by its own preimage | Passes the owner join | FR-322-AC-55 |
| MW-18 | A clause function (`clause` binding `invariant`) owned by the operation member `ix://acme/orders/Order/size`, keyed by its own preimage | `missing_declaration`/`missing-selection` at the owner join: an invariant is owned by an object type | FR-322-AC-55 |
| MW-19 | A clause function whose `clause` binding is `postcondition`, owned by `ix://acme/orders/Order/size` | `missing_declaration`/`missing-selection` at the owner join: a postcondition is FR-341's `state_clause` node, not a clause function kind | FR-322-AC-55 |
| MW-20 | MW-17 with two `clause` bindings | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-55 |
| MW-21 | MW-17 with no `clause` binding | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-55 |
| MW-22 | A clause function (`clause` binding `invariant`) owned by the systems interface `ix://acme/orders/Flow` | `missing_declaration`/`missing-selection` at the owner join: an invariant is owned by an object type with no `interfaceFeatures` | FR-322-AC-55 |
| MS-01 | The document supplied under the selected digest, identity equal | Passes the selection admission checks | FR-322-AC-32 |
| MS-02 | No document supplied | `missing_import`/`missing-selection` | FR-322-AC-32 |
| MS-03 | A document with `OrderTotal`'s upper bound changed to 8, supplied under the selected digest, identity equal | `stale_dependency`/`content-mismatch` naming the selected and the recomputed digest | FR-322-AC-43 |
| MS-05 | The selection names digest domain `sha256` | `stale_dependency`/`digest-domain-mismatch` | FR-322-AC-32 |
| MS-06 | The selection names identity `acme/invoices` | `invalid_model_binding`/`wrong-model-selection` | FR-322-AC-32 |
| FF-01 | Frame field entry `(M1, "total")` | Resolves `Order/total` | FR-340-AC-13 |
| FF-02 | Frame field entry `(M5, "total")` | Resolves the inherited `Order/total` | FR-340-AC-13 |
| FF-03 | Frame field entry `(M1, "missing")` | `missing_declaration`/`missing-name` | FR-340-AC-13 |
| FF-04 | Frame field entry `(M1, "size")`, an operation name | `missing_declaration`/`missing-name` | FR-340-AC-13 |
| FF-05 | Frame field entry `(M6, "code")` | `ambiguous_declaration`/`ambiguous-name` | FR-340-AC-13 |
| FF-06 | Frame field entry `(M2, "total")` | `missing_declaration`/`missing-selection` | FR-340-AC-13 |
| AN-01 | Anchor `(M1, "scaled")` | Resolves `Order/scaled` | FR-342-AC-4 |
| AN-03 | Anchor `(M5, "reset")`, which `Sub` only inherits | `invalid_model_binding`/`malformed-declaration` | FR-342-AC-4 |
| AN-04 | Anchor `(M1, "missing")` | `missing_declaration`/`missing-name` | FR-342-AC-4 |
| AN-05 | Anchor `(M1, "total")`, a field name | `missing_declaration`/`missing-name` | FR-342-AC-4 |
| AN-06 | Anchor `(M2, "size")` | `missing_declaration`/`missing-selection` | FR-342-AC-4 |
| AN-07 | Anchor `(M8, "rate")`, a `systems_interface` context | `invalid_model_binding`/`malformed-declaration` | FR-342-AC-4 |
| CS-01 | Precondition of `Order.scaled` binding `[Reference<Order>, Integer]` | Admitted | FR-341-AC-5 |
| CS-02 | Postcondition of `Order.scaled` binding `[Reference<Order>, Integer, Integer]` | Admitted | FR-341-AC-5 |
| CS-03 | Postcondition of `Order.reset` binding `[Reference<Order>]` | Admitted: no result | FR-341-AC-5 |
| CS-04 | Precondition of `Order.scaled` binding `[Reference<Order>]` | `ill_typed`/`operator-ineligible` | FR-341-AC-5 |
| CS-05 | Postcondition of `Order.scaled` binding `[Reference<Order>, Integer]` | `ill_typed`/`operator-ineligible` | FR-341-AC-5 |
| CS-06 | Precondition of `Order.scaled` binding `[Reference<Sub>, Integer]` | `ill_typed`/`operator-ineligible` | FR-341-AC-5 |
| CS-07 | Precondition of `Order.scaled` binding `[Reference<Order>, Boolean]` | `ill_typed`/`operator-ineligible` | FR-341-AC-5 |
| CS-08 | Invariant of `Order` binding `[Reference<Order>]` | Admitted | FR-341-AC-5 |
| CS-09 | Invariant of `Order` binding `[Reference<Order>, Integer]` | `ill_typed`/`operator-ineligible` | FR-341-AC-5 |

## Expected Results

M1 to M12 validate and recompute their digests. Every wire, member,
selection, frame field, anchor and clause signature case gives exactly its
recorded result.

## Relationship declaration cases

`fixtures/adverse.json` `relationship_document_cases` patch the selected
document and are read under FR-154's relationship declaration refusals
(FR-154-AC-10); each refusal is reported at the selection row. Ids: `relationship-document-admitted`, `relationship-without-target-role-admitted`, `relationship-with-generated-origin-admitted`, `relationship-identity-owner-nested`, `relationship-identity-foreign-package`, `relationship-identity-wrong-slot`, `relationship-identity-missing`, `relationship-two-identities-missing`, `relationship-direction-missing`, `relationship-direction-null`, `relationship-direction-unsupported`, `relationship-category-missing`, `relationship-category-unsupported`, `relationship-composite-wrong-type`, `relationship-origin-missing`, `relationship-origin-both-branches`, `relationship-source-role-missing`, `relationship-target-role-null`, `relationship-target-role-empty`, `relationship-target-role-non-string`, `relationship-source-end-names-another-owner`, `relationship-target-end-type-unresolved`, `relationship-target-end-type-names-a-relationship`, `relationship-target-end-type-names-a-value-type`, `relationship-end-multiplicity-malformed`, `relationship-end-multiplicity-reversed`, `relationship-identity-repeated-under-two-owners`, `relationship-malformed-role-beats-unresolved-end-type`, `relationship-unresolved-end-type-beats-reversed-multiplicity`, `relationship-duplicate-identity-beats-reversed-multiplicity`.
