---
id: SR-16284
title: "spec-review of package-qualified dimension names (quire-specification PR 407)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@ed1f83a1; spec/functional/type-model/FR-142-evaluate-quantities-and-units.md; spec/objects/interfaces/FR-322-checked-package-artifact.md; spec/test-cases/TC-187-quantity-and-unit-conversion.md; spec/test-cases/TC-280-model-owned-member-types.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-142
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---

# Spec review of package-qualified dimension names (quire-specification PR 407)

## Summary

Ticket: STD-402. One spec-review pass over the PR's own delta (`git diff the head of #418...ed1f83a1`), checked for the no-drop/no-narrowing/no-deferral rule, testable vectors (key vectors derived by hand), id uniqueness across open PR heads, and schema/vector consistency.

Examined, clean:

- Resolution is sound: bare names reach only source dimensions, and `M::Name` reaches a package's dimensions. Identity is the node key, maps are over base-dimension keys (R::s / F::s is not dimensionless), duplicates inside one owner still refuse, and the checked package needs no new member. Moving bare package-dimension names to `M::Name` is a spelling change, not a narrowing.
- Gates at ed1f83a1: quire validate 86; cargo 136 + 24 passed. The own delta is unchanged since ea1ddad0.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | STD-402's scope is "so two domain packages that each declare Time compose; resolution, identity, equality and quantity typing across packages". The PR makes cross-owner base dimensions permanently unrelated. `3 R::s = 3 F::s`, passing an `acme/rpc` duration to an `acme/flow` operation, or any conversion between them is refused for good. The PR body names an equivalence declaration Peter "may want" as a new construct. That is a named follow-up plus a permanent refusal, so the packages are admitted together but cannot exchange time quantities. Fix: spec the cross-owner relation in this PR (for example a source `dimension R::Time = F::Time` equivalence, or a unit conversion edge between owners' roots) with vectors, or have Peter rule on it. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | missing-requirement |
| FND-002 | medium | The two-step pivot expression is not written, so U36's last vector cannot be run as specified. Spell it, for example `convert<Quantity<dsec>>(convert<Quantity<<compound>>>(3 R::s))` with the compound unit named. | spec/test-cases/TC-187-quantity-and-unit-conversion.md | missing-requirement |
| FND-003 | low | FR-322-AC-105 sits between AC-99 and AC-100. Move it to numeric order. | spec/objects/interfaces/FR-322-checked-package-artifact.md | wrong-requirement |

## Verdict

Not mergeable: FND-001 (high) block the PR; FND-002 (medium) should be fixed in the same round.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-004 | high | The identification edge sets the two packages' canonical roots equal with scale 1. Nothing requires those roots to be the same size. Suppose `acme/rpc`'s Time root is `s` and `acme/flow`'s is `ms`. Then `dimension R::Time ~ F::Time;` makes 1 F::ms = 1 R::s, so `convert<Quantity<F::ms>>(3 R::s)` returns 3 F::ms instead of 3000. Root coherence constrains derived roots within one graph, not the base roots of two owners. Fix: the declaration must state the root ratio, for example `dimension R::Time ~ F::Time by 1 R::s = 1000 F::ms;`, or relate two named units of equal size. Then the edge carries that exact scale. Add a vector with unequal roots. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | wrong-requirement |

## Dispositions

Round 1, at 12296a48. Standing-rule check: the equivalence is not restrictive. It relates any two dimension nodes, either owner, transitively; derived dimensions follow by equal maps. Hand-derived: U36's pivot ((3 R::s)*(1 R::s))/(1 R::s) is 3 of the compound R::s^1, and converting it to dsec (map R::Time 1, scale 1) gives 3 dsec. AC-23: convert<F::ms>(3 R::s) = 3000 F::ms over 2 edges; 1500 F::ms = 3/2 R::s; 3 R::s / 3 F::s = 1.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | ## Dimension equivalence ... `dimension R::Time ~ F::Time;` ... Direct conversion between two declared units is admitted when their dimension nodes are equal or in one equivalence class |
| FND-002 | fixed | the expression `let c = ((3 R::s) * (1 R::s)) / (1 R::s) in convert<Quantity<dsec>>(c)` |
| FND-003 | fixed | FR-322-AC-105 now sits in numeric order after FR-322-AC-104. |
