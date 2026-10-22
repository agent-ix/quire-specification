---
id: SR-13820
title: "Integrity review of QSpec PR #327 (attempt type after 'attempts N of'), STD-158"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@6d395af97ee35847031fdf3f413c29b1e98e9a2a; git diff origin/main...6d395af (PR #327, branch std-158-attempts-inline-type): proposals/quire-v1/shared-grammar.md, spec/objects/protocol/FR-239-compensation-retry-policy.md, spec/test-cases/TC-380-compensation-worked-example.md; ticket STD-158"
review_set: subset
date: 2026-10-22
relationships:
  - target: ix://agent-ix/quire-specification/FR-239
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-380
    type: reviews
---

## Summary

Ticket: STD-158. PR: quire-specification#327, head 6d395af. Method: quoin spec-review, spec-integrity-analysis sub-analysis. Tooling: quoin 0.28.3, quire 0.36.2 (engine 0.50.2), spec-artifacts-process@b82e42e.

Repo placement is correct: the EBNF (proposals/quire-v1/shared-grammar.md), FR-239 and TC-380 are in quire-specification; TC-1058 and FR-381 are in quire-spec-language, and TC-1058 row 15c already expects the checker refusal, so it needs no change. The PR takes ticket option (a). FR-239-AC-8 and TC-380 CP-11 are new, unique ids. `Set<M::Total>[0, 3]`, `Integer` and `M::Mode` are each admitted by the complete-V1 `type-ref` (shared-grammar.md:317-330, including the optional `[uint, uint]` suffix on `Set`). The QSL checker (qsl-semantics protocol_obligation.rs `attempt_type`) refuses a resolved non-record type with `invalid_model_binding`/`malformed-declaration`, as claimed. The QSL parser still takes `P::TypeName` after `attempts N of` (qsl-cst grammar.rs:1681-1684), so `Integer` and `Set<...>` do not parse yet. The PR says that is code-lane work. Two integrity defects are reported below.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| compensation production | proposals/quire-v1/shared-grammar.md | examined |
| attempt-type paragraph | proposals/quire-v1/shared-grammar.md | examined |
| type-ref production | proposals/quire-v1/shared-grammar.md | context_only |
| FR-239 | spec/objects/protocol/FR-239-compensation-retry-policy.md | examined |
| FR-239-AC-8 | spec/objects/protocol/FR-239-compensation-retry-policy.md | examined |
| TC-380 | spec/test-cases/TC-380-compensation-worked-example.md | examined |
| FR-427 | spec/functional/protocol/FR-427-check-protocol-compensation.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The bounded `compensation` production in "Declarations and expressions" now uses `type-ref`, but that section does not define `type-ref`. Its type positions use `type-name` and `parameter-type = 'Boolean' \| type-name`. `type-ref` is defined only in the "Complete-V1 facet grammar", whose productions "extend or replace the same-named bounded productions above". So the bounded grammar has an undefined nonterminal. The new paragraph also sits in the bounded section but describes complete-only forms such as `Set<M::Total>[0, 3]`. The grammar already handles the same change for `channel` differently: it keeps `type-name` in the bounded production and redefines `channel` with `type-ref` in the complete section (line 521-523). Fix: keep `type-name` in the bounded production, or add `attempt-type = type-name ;` there. Then replace it in the complete section (`attempt-type = type-ref ;`, or a replacing `compensation` production), and move the paragraph next to it. | proposals/quire-v1/shared-grammar.md:204, 242-248, 317 |
| FND-002 | medium | FR-239-AC-8 specifies source-grammar and checker behaviour: which `type-ref` forms parse after `attempts N of`, and the checker refusal. FR-239's only statement is "When compensation attempts are evaluated, the conformance evaluator SHALL refuse attempts beyond the authored finite retry bound". FR-239 is the retry-policy value object, and its Description says nothing about the attempt type, the grammar or the checker. So the AC goes beyond its FR's statement. Fix: add a statement sentence to FR-239 that covers the attempt-type rule (only a record or event type is admitted; every other `type-ref` is refused at check). Alternatively, place the AC under FR-427, which checks compensation templates. | spec/objects/protocol/FR-239-compensation-retry-policy.md:20, 80 |

## Verdict

Findings. The grammar placement and the AC's home need fixing. Otherwise the change is consistent: ids are unique, no ACs, rows or vectors are deleted, and no other statement in the repo (choreography-surface.md, FR-427, TC-948) contradicts the new text. The touched files add no `quire validate` failures: FR-239's three diagnostics are identical on origin/main, and TC-380 is clean. `cargo test --test checked_package_v2 --test backend_provider_v1` passes (117 + 14).
