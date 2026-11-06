---
id: ADR-002
title: "Semantic IR contract version for a domain package"
type: ADR
status: accepted
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: depends_on
  - target: ix://agent-ix/quire-specification/AD-006
    type: depends_on
  - target: ix://agent-ix/filament-core-data/FR-051
    type: depends_on
---
# ADR-002: Semantic IR contract version for a domain package

## Status

**Accepted.**

## Decision

A lifted domain package carries semantic IR contract version `2.0.0`.

`filament-core-data` is the producing side and defines the `2.0.0` member set.
That member set includes the members
[FR-154](../functional/type-model/FR-154-admit-domain-package-model.md),
[AD-006](../assurance/AD-006-model-graph-state-contracts.md) and
[FR-208](../objects/foundation/FR-208-quire-meaning-vocabulary.md) require
Quire to admit: a supertype list, an abstractness flag, a subsets edge, a
redefines edge, an operation frame, a population node and an embedded meaning
vocabulary, alongside the fields, multiplicity, units, relationships,
operations, opaque clauses and closed constraint keyword set the contract
already defines.

## Context

The producing repository owns how the Semantic IR 2.0.0 member set is written
and its producer-side schema. Quire owns the semantics with which it admits
those members. A Quire rule may constrain the accepted values of a member
without adding a member or revising the producer-side schema; FR-154's Rational
bound encoding is such a rule over the existing `Constraint.operands.value`
member. Quire cites the producer-side requirement that defines each member.

## Consequences

- `filament-core-data` states the seven members above in its normative IR
  contract at `2.0.0`, and `FR-154`, `AD-006` and `FR-208` cite that
  requirement id.
- `FR-051` classifies diffs over the `2.0.0` member set, including the seven.
- FR-154 owns the accepted Rational bound values inside the existing
  `Constraint.operands.value` member. The Rational numeric/string choice does
  not expand or version the producer-owned member set.
