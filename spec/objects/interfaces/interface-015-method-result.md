---
id: interface-015
title: "I15 verification method result"
type: interface
---

# [interface-015] I15 verification method result

## Contract

```yaml
name: MethodResult
associated_types: [MethodRunId, MethodOutcome, Witness, Counterexample, Provenance]
operations:
  - { name: validate_result, inputs: [method request, provider result], output: typed validated result or provider failure, semantics: checks subject claim assumptions domain and evidence correspondence }
invariants:
  - proved refuted inconclusive unsupported incomplete and failed are distinct
  - candidates and heuristic findings are not proofs
```
