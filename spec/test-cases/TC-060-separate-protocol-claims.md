---
id: TC-060
title: "Separate conformance and advanced analysis claims"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-060
    type: verifies
---
## Description

Verify that monitorability, conformance, projection, realizability, refinement and composition have independent premises and dispositions.

## Test Procedure

Use an unprojectable but globally monitorable protocol, a satisfiable trace without a causal implementation strategy and local contracts with circular assumptions. Request every claim from a backend supporting only finite replay/conformance.

## Expected Results

Global results remain available. Each advanced claim is refused, unsupported or separately decided with its exact fragment and premises; satisfiability and circular assumptions establish no stronger success.
