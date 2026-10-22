---
id: interface_014
title: "I14 linked verification plan"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-332
    type: references
---

# [interface_014] I14 linked verification plan

## Features

| Feature | Kind |
| --- | --- |
| link_plan | operation |

## Contract

```yaml
name: VerificationPlan
associated_types: [PlanId, MethodSelection, DomainSelection, EvidenceObligation]
operations:
  - { name: link_plan, inputs: [plan source, CheckedPackage, provider manifests], output: dependency-closed executable plan or refusal, semantics: resolves clauses methods assumptions domains bounds dependencies and evidence }
invariants:
  - plan selection does not alter language admission
  - unsupported dependency cycles are refused before execution
```
