---
id: ADR-003
title: "Model intake and formal type system authority"
type: ADR
status: proposed
relationships:
  - target: ix://agent-ix/quire-specification/AD-010
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-154
    type: depends_on
  - target: ix://agent-ix/quire-spec-language/FR-005
    type: depends_on
---
# ADR-003: Model intake and formal type system authority

## Status

**Open question for the architect.** Two repositories each assign the same two
roles to the other.

## Context

QSpec places the formal value and type system in subsystem S2, owned by
`quire-spec-language` and `quire-contract-model`, and places Contract IR
downstream of the checked package:
[FR-195](../functional/ir-proof/FR-195-lower-complete-contract-ir.md) lowers an
already-admitted `quire.checked-package/v2` into contract IR, and
`ix://agent-ix/quire-contract-ir/AD-003` agrees that native Quire is the sole
semantic authority.

`ix://agent-ix/quire-spec-language/FR-005` states instead that formal
declarations are constructed through Contract IR's DeclarationEnvironment API
and that "Contract IR FR-013 owns the formal type system", inverting that
direction for linking and checking.

The same requirement admits exactly one Contract IR binder and no general
Filament model authority, and states no Filament reader or universal
typed-model adapter.
[FR-154](../functional/type-model/FR-154-admit-domain-package-model.md) assigns
exactly that model intake to native Quire model linking, and
`quire-spec-language` carries no artifact for a domain package, a model
selection or a package digest.

## Question

Which repository's requirement owns the formal type system used during linking
and checking, and which repository owns intake of a lifted domain package?

## Consequences

Until the question is answered, an implementation of model linking has two
specifications: one that builds declarations through Contract IR and admits no
domain package, and one that admits a domain package and treats Contract IR as
a lowering target.
