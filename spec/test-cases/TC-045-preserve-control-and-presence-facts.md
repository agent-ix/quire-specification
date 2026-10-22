---
id: TC-045
title: "Preserve conditional evaluation and exact optional-value facts"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-045
    type: verifies
---
## Description

Accepted portable controls for [FR-045](../functional/foundation/FR-045-preserve-control-and-presence-facts.md).
No executable test or observed coverage is claimed.

## Test Procedure

Use source-bound counters/probes in the eventual test harness to observe operand/initializer evaluations without adding side effects to the language. Exercise all short-circuit truth-table cases, selected branches, repeated immutable aliases and matched/mismatched optional identities. Add an unselected unknown/prohibited construct, remove a probe and exhaust the budget after an antecedent.

## Expected Results

Evaluation and recorded activation agree with source control. Only exact guard identities establish optional safety. Refused/incomplete cases have no fabricated Boolean or participation evidence, and test observation instrumentation does not alter source meaning.
