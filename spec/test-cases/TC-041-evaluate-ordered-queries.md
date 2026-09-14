---
id: TC-041
title: "Evaluate bounded queries with occurrence-preserving semantics"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-041
    type: verifies
---
## Description

Accepted portable controls for [FR-041](../functional/foundation/FR-041-evaluate-ordered-queries.md).
The matrix remains Planned; no executable test or observed coverage is claimed.

## Test Procedure

Use the synthetic declared domains and sequence [2,2,3], plus empty/singleton/max-length and duplicated-reference inputs. Observe binder occurrence/short-circuit controls. Independently substitute an incompatible domain/unit, allow six items, remove completeness, exhaust work/output budgets, and propose a sum with an invalid intermediate prefix.

## Expected Results

Every query preserves the specified ordering, duplicate and identity semantics. Invalid type/definedness cases refuse before completed evaluation; missing inputs and resource exhaustion remain distinct incomplete outcomes. No smaller observed sample silently narrows the authored input domain.
