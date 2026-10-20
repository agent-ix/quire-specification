---
id: SR-13056
title: "Failure-domain review of PR #284: resolving the abstract protocol subject"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@04cc1b9; FR-375 Inputs, Selection, Protocol subject, Checked-package node; FR-425 Protocol subject (context)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
---

## Summary

Ticket: STD-182. Looks for identity confusion in resolving `alias::name`
to a protocol clause, and in the node identity the selection produces.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `name` "resolves to a checked protocol clause of the abstract model", but protocol clauses belong to packages, not models (FR-425: "one checked protocol clause of its package"), and a clause's roles may act on any model alias of that package. FR-375's Inputs admit a unit with two `model` imports, where the package holding the abstract model's clauses can be the declaring unit itself; then `abstract Spec::P` could select a clause `P` whose roles act on `Impl`. State the lookup scope (the abstract subject package) and whether the clause must act on the aliased model, with a refusal for one that does not. | spec/functional/analysis/FR-375-declare-model-refinement.md:38-40,112-115,118-126; spec/functional/protocol/FR-425-define-protocol-subjects-and-state-keys.md:55-56 |

## Verdict

Fix FND-001 in the same round as SR-13054. Otherwise the identity edges
hold: the protocol is in the body as its own `PackageNodeKey`, so the same
model under two protocols, or under none, gives different ids; a model
subject omits the key rather than writing `null`, matching FR-322; parse
refusals come before any resolution, so a syntax error never depends on
which names exist.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The fix requires a selected clause to act on the aliased abstract model only when the abstract model is the unit's own. When the abstract model comes from a dependency package that imports more than one model, a clause of that package whose roles act only on another of its models is still selectable as `Spec::P`. The same identity confusion remains on that path. Stating the "acts on the aliased model" rule for every abstract subject package closes it. | spec/functional/analysis/FR-375-declare-model-refinement.md:121-130 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 262aae1 scopes the lookup to the abstract subject package (citing FR-425) and refuses `invalid_model_binding`/`malformed-declaration` for a declaring-package clause that acts only on the concrete or another model. The dependency-package path is FND-002. |
