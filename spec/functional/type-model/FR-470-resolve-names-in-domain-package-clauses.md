---
id: FR-470
title: "Resolve names in a domain package's own clauses"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
  - target: ix://agent-ix/quire-specification/FR-208
    type: references
---

# FR-470: Resolve names in a domain package's own clauses

## Description

When QSL checks a domain clause, it SHALL resolve every qualified name in the
clause against the clause's own domain package through the self alias of the
clause's owner, independently of the unit that imports the package.

A **domain clause** is clause text that a domain package itself carries: a
`Clause` with `language: quire` of a type definition (an invariant or a
transition guard, FR-208) and each `pre` and `post` entry of an `Operation`.
In a spec artifact each is one `quire` fence under its own heading, with no
header and no import. The **owner** of a domain clause is the type definition
that declares it; for an operation's `pre` or `post` entry it is the type that
declares the operation.

## Inputs

- The admitted original declarations of one domain package (FR-154), including
  every type definition bound to `quire.meaning.model.namespace/v1` with its
  `members` (FR-208).
- One domain clause, its owner and its FCD `origin`.

## Outputs

- For each qualified name in the clause, the one declaration key it resolves
  to, or a located refusal.

## Behavior

### Clause text

A domain clause is exactly one clause expression of the shared grammar. It
carries no `quire` header, `profile`, `import`, `model` or declaration form: a
clause whose text begins with one of these refuses
`invalid_syntax`/`unexpected-token` at that token. The domain package is the
only model a domain clause can name, so it needs no import, and an import of
the package's own identity and digest could not be written: the digest is
computed over bytes that contain the clause.

### Self alias

The **self alias** of an owner is the artifact id of the namespace whose
`members` list the owner. FR-208 allows a type to be a member of at most one
namespace, so an owner has at most one self alias. In a business spec this is
the `domain` artifact that declares the owner's bounded context.

- The self alias denotes the clause's own domain package. A qualified name
  whose first segment is the self alias resolves its remaining segments over
  the whole package by artifact id and member name, exactly as a unit resolves
  them after its model alias (shared grammar, model-complete): `N::T` names the
  type definition declared by the artifact with id `T`, `N::T::m` names member
  `m` of `T` or, when `T` is a variant type, its case `m`. `T` need not be a
  member of `N`.
- An owner that no namespace lists has no self alias. Every qualified name in
  its clauses refuses `missing_declaration`/`missing-name` at the first
  segment. A clause that names only `self`, operation parameters, `result`,
  its own binders and native forms is unaffected.
- The artifact title of a namespace is presentation only (FR-154) and is never
  a self alias. A first segment equal to a namespace's title and not to its id
  refuses `missing_declaration`/`missing-name`.
- The id of a namespace that does not list the owner is not the owner's self
  alias. A qualified name whose first segment is such an id refuses
  `missing_declaration`/`missing-name`.

### Independence from the importing unit

The model aliases, profile aliases and native declarations of the unit that
imports the domain package are not visible in a domain clause. A first segment
that is one of the unit's model aliases and not the owner's self alias refuses
`missing_declaration`/`missing-name`. Consequently a domain clause resolves to
the same declaration keys, and gives the same check result, under every unit
that imports the package, whatever alias the unit's `model` declaration
chooses.

### Refusals

Every refusal of this requirement names the clause's IR node identity (for an
operation entry, the operation's identity with the entry's kind and position),
the unresolved segment or the absent node identity, and the clause's FCD
`origin`, as FR-272 states for a `missing-name` during model intake. A
qualified name whose segments after the self alias name no node of the package
refuses `missing_declaration`/`missing-name` with the absent node identity,
for example `ix://test/orders/variant/OrderStatus-Shipped`. Each refused name
is reported once, in source order within the clause.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-470-AC-1 | An invariant of `Order`, a member of namespace `OrderManagement`, that names `OrderManagement::OrderStatus::Draft` resolves that name to the case key `(test/orders, ix://test/orders/variant/OrderStatus-Draft)`, and `OrderManagement::OrderStatus::Placed` to `(test/orders, ix://test/orders/variant/OrderStatus-Placed)`, with no refusal. | Test |
| FR-470-AC-2 | A `pre` entry of operation `Order/place` naming `OrderManagement::OrderStatus::Draft` resolves to the same case key as AC-1, through the self alias of `Order`, the type that declares the operation. | Test |
| FR-470-AC-3 | The package of AC-1 imported by one unit as `model M = …` and by another as `model Shop = …` gives byte-equal resolved keys and check results for every domain clause; in both units a domain clause naming `M::OrderStatus::Draft` or `Shop::OrderStatus::Draft` refuses `missing_declaration`/`missing-name` at the first segment, naming the clause's IR node identity and its `origin`. | Test |
| FR-470-AC-4 | A domain clause naming `OrderContext::OrderStatus::Draft`, where `OrderContext` is the title and `OrderManagement` the id of the namespace, refuses `missing_declaration`/`missing-name` at `OrderContext`. | Test |
| FR-470-AC-5 | An invariant of `Audit`, which no namespace lists, naming `OrderManagement::OrderStatus::Draft` refuses `missing_declaration`/`missing-name` at `OrderManagement`, while its invariant `self.entries >= 0` checks with no refusal. | Test |
| FR-470-AC-6 | With a second namespace `Billing` listing `Invoice`, an invariant of `Invoice` naming `Billing::OrderStatus::Draft` resolves to the case key of AC-1, and one naming `OrderManagement::OrderStatus::Draft` refuses `missing_declaration`/`missing-name` at `OrderManagement`. | Test |
| FR-470-AC-7 | A domain clause naming `OrderManagement::OrderStatus::Shipped`, a case the variant type does not declare, refuses `missing_declaration`/`missing-name` naming the absent node `ix://test/orders/variant/OrderStatus-Shipped`; two such names in one clause give two refusals in source order. | Test |
| FR-470-AC-8 | A domain clause whose text begins with `model M = "test/orders" digest "…";`, with `import`, with `profile` or with a `quire` header refuses `invalid_syntax`/`unexpected-token` at that first token, and no name in the clause is resolved. | Test |

## Dependencies

- **Upstream**: [FR-154](./FR-154-admit-domain-package-model.md) admits the
  domain package and its declaration keys;
  [FR-208](../../objects/foundation/FR-208-quire-meaning-vocabulary.md) defines
  namespace membership, invariants and guards;
  [AD-006](../../assurance/AD-006-model-graph-state-contracts.md) places
  clauses in spec artifacts.
- **Downstream**: [TC-953](../../test-cases/TC-953-domain-clause-name-resolution.md)
  verifies every criterion.
