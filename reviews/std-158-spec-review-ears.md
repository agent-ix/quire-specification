---
id: SR-13821
title: "EARS review of QSpec PR #327 (attempt type after 'attempts N of'), STD-158"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@6d395af97ee35847031fdf3f413c29b1e98e9a2a; spec/objects/protocol/FR-239-compensation-retry-policy.md (FR-239-AC-8); ticket STD-158"
review_set: subset
date: 2026-10-22
relationships:
  - target: ix://agent-ix/quire-specification/FR-239
    type: reviews
---

## Summary

Ticket: STD-158. PR: quire-specification#327, head 6d395af. Method: quoin spec-review, spec-ears-analysis sub-analysis. Tooling: quoin 0.28.3, quire 0.36.2 (engine 0.50.2), spec-artifacts-process@b82e42e.

The FR-239 statement did not change. It is event-driven EARS: "When compensation attempts are evaluated, the conformance evaluator SHALL ...". The new AC-8 is testable and names its code, cause, span and retained members. However, it lists four examples instead of stating the rule.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-239 | spec/objects/protocol/FR-239-compensation-retry-policy.md | context_only |
| FR-239-AC-8 | spec/objects/protocol/FR-239-compensation-retry-policy.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-239-AC-8 lists three refused forms (`Set<M::Total>[0, 3]`, `Integer`, a variant type) and one admitted form (a record type). It does not state the rule the grammar paragraph gives: the checker admits only a record or event type and refuses every other `type-ref`. Event-type admission, and refusal of `Boolean`, an object type or a scalar model name, are not required by any QSpec AC. Packing four cases into one sentence also makes the AC compound. Fix: state the rule in the AC ("the checker admits a record or event type and refuses every other `type-ref` with ..."), and keep the forms as examples or as TC-380 vectors. | spec/objects/protocol/FR-239-compensation-retry-policy.md:80 |

## Verdict

One low finding. The wording describes only behaviour, with no changelog, ruling or review language.
