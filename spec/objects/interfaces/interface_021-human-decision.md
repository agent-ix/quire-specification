---
id: interface_021
title: "I21 attributed human decision"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-338
    type: references
---

# [interface_021] I21 attributed human decision

## Contract

```yaml
name: HumanDecision
associated_types: [DecisionId, ActorId, Scope, Disposition, Rationale, Supersession]
operations:
  - { name: record_decision, inputs: [actor authority, exact evidence scope, disposition, rationale], output: immutable attributed decision, semantics: accepts rejects or limits an exact qualification or release scope }
  - { name: supersede, inputs: [prior decision, new decision, authority], output: linked decision history, semantics: never mutates the prior record }
invariants:
  - no test proof score or agent promotes itself
  - a decision cannot claim broader scope than its selected evidence
```
