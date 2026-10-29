---
id: TC-975
title: "Compare required feature sets"
type: TC
relationships:
  - target: "ix://agent-ix/quire-specification/FR-486"
    type: verifies
---
# TC-975: Compare required feature sets

## Description

Portable contract control for FR-486-AC-1.

## Test Procedure

Validate the semantic reference. Exercise requiredFeatures as empty, singleton,
two reordered unique features, duplicate decoded features and an unknown
feature. Resource controls use IT-003's byte and value limits, including exact
limit and one-over-limit; missing dependencies fail setup.

## Expected Results

Feature-set equality is independent of the order written. Duplicate decoded
features refuse before comparison and an unknown feature refuses interpretation.
Resource exhaustion yields incomplete.
