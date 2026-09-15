---
id: interface-011
title: "I11 effect decision and receipt"
type: interface
---

# [interface-011] I11 effect decision and receipt

## Contract

```yaml
name: EffectDecisionReceipt
associated_types: [DecisionId, AuthorityGrant, IdempotencyKey, EffectRequest, Receipt]
operations:
  - { name: authorize, inputs: [typed decision, authority grant, adapter manifest], output: authorized request or refusal, semantics: validates subject action and scope }
  - { name: reconcile, inputs: [request, idempotency key, receipt], output: one attributed outcome, semantics: duplicate delivery cannot create another effect request }
invariants:
  - evaluators emit decisions but perform no effects
  - transport receipt is not business effect evidence
```
