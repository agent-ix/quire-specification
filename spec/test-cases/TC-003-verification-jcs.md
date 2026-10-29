---
id: TC-003
title: "Compare required feature sets"
type: TC
relationships:
  - target: "ix://agent-ix/quire-specification/FR-018"
    type: verifies
---
# TC-003: Compare required feature sets

## Description

Accepted portable contract control for FR-018-AC-4. The JCS canonical-byte and
digest vectors this case once held (FR-018-AC-1 to -3 and -6) are withdrawn with
those criteria.

## Test Procedure

Validate the semantic reference. Exercise requiredFeatures as empty, singleton,
two reordered unique features, duplicate decoded features and an unknown
feature. Resource controls use IT-003's byte and value limits, including exact
limit and one-over-limit; missing dependencies fail setup.

## Expected Results

Feature-set equality is independent of the order written. Duplicate decoded
features refuse before comparison and an unknown feature refuses interpretation.
Resource exhaustion yields incomplete.
