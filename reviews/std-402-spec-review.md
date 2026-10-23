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
