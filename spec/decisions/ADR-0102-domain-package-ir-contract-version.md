---
id: ADR-0102
title: "Semantic IR contract version for a domain package"
type: ADR
status: proposed
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: depends_on
  - target: ix://agent-ix/quire-specification/AD-006
    type: depends_on
  - target: ix://agent-ix/filament-core-data/FR-051
    type: depends_on
---
# ADR-0102: Semantic IR contract version for a domain package

## Status

**Open question for the architect.** The producer and the consumer of the same
document name different contract versions.

## Context

[FR-154](../functional/type-model/FR-154-admit-domain-package-model.md),
[AD-006](../assurance/AD-006-model-graph-state-contracts.md) and
[FR-208](../objects/foundation/FR-208-quire-meaning-vocabulary.md) require the
lifted domain package to be a semantic IR `2.0.0` document, and require that
document to carry generalization, abstractness, subsetting, redefinition,
operation frames, populations and an embedded meaning vocabulary.

`filament-core-data` specifies the lift as producing semantic IR v1.1, and
`ix://agent-ix/filament-core-data/FR-051` states that projecting to `2.0.0`
yields `UNKNOWN_CONTRACT_VERSION`. Its normative IR contract covers fields,
multiplicity, units, relationships, operations, opaque clauses and a closed
constraint keyword set; no requirement there defines a supertype list, an
abstractness flag, a subsets or redefines edge, an operation frame, a
population node or a meaning vocabulary.

QSpec's only citation for the `2.0.0` members is a pair of `filament-core-data`
issue numbers, not a requirement id, so the member set Quire admits has no
producer-side requirement to read it from.

## Question

Which contract version does a lifted domain package carry, and which
requirement on the producing side defines the members `FR-154` and `AD-006`
require Quire to admit?

## Consequences

Until the question is answered, `FR-154` admits a document that no
`filament-core-data` requirement produces, and the producing repository refuses
the version Quire names.
