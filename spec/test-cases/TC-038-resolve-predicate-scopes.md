---
id: TC-038
title: "Resolve predicate names under explicit lexical scopes"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-038
    type: verifies
---
## Description

Planned controls for [FR-038](../functional/FR-038-resolve-predicate-scopes.md).
These expected cases have not been implemented or executed.

## Test Procedure

Link two source units with local alias M bound to different models and distinct predicate declarations. Introduce duplicate predicate names, duplicate parameters, a nested shadow, and case-mismatched calls. Place x in a let initializer and quantifier domain before it is bound, then in their valid bodies. Compare a valid forward acyclic call and a predicate body attempting implicit self/result/pre access.

## Expected Results

Aliases remain source-unit local and every admitted use has one exact declaration. Invalid scope uses and hidden environment access refuse with original loci. Reusing x in distinct predicate declarations is admitted; forward-call order alone does not change meaning.

