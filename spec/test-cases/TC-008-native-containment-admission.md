---
id: TC-008
title: "Separate acyclic values from deferred reference behavior"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-019
    type: verifies
---
## Description

Exercise acyclic record paths and the replacement reference/equality boundary.
Planned; the original ConfigVersion reference/bounds gate remains unresolved.

## Test Procedure

Read an explicitly typed acyclic record field and compare admitted scalar values.
Offer recursive containment, record/object equality, opaque-scalar navigation,
reference dereference and reachability, including unreachable expressions.
Retain the original minimum-only ConfigVersion/parent declaration when checking
its gate; inspect that intake neither removes parent nor invents a maximum.

## Expected Results

Exact acyclic field paths and scalar equality succeed. Deferred operations and
invalid containment refuse with their original source/model locus. The original
ConfigVersion case remains a design gate; historical bounded native successes
cannot be relabelled as replacement-profile correspondence.
