---
id: SR-13635
title: "QSpec PR #308 (STD-202) FR-432 alias-free order, model-subject resolution, step-row target kinds"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@b9d097109bddaa4843fc8a425152ba2f69f9db7a vs main d080afa; spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md, spec/test-cases/TC-386-explicit-protocol-step-rows.md; context spec/functional/analysis/FR-375-declare-model-refinement.md, quire-spec-language FR-147 at origin/main 97b53c85a"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-432
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-386
    type: reviews
---

## Summary

Ticket: STD-202 (PR #308, head b9d097109bddaa4843fc8a425152ba2f69f9db7a). The three STD-202 defects are addressed in FR-432. The node-kind limit on a step-row right side (attempt, event, send, receive, finish; anything else `missing_declaration`/`missing-name`) and the model-subject skip of protocol-name resolution match QSL FR-147 at quire-spec-language origin/main 97b53c85a word for word in effect; the PR body flags the limit as a planner default. The alias-free order matches FR-375's `abstract_internal`. The defects are in the TC-386 vector IL-05, which cannot fail under the old alias-including order and renames the protocol as well as the alias.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-432-AC-11 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | examined |
| FR-432-AC-12 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | examined |
| FR-432-AC-13 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | examined |
| FR-432 Resolution | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | examined |
| FR-432 model-subject paragraph | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | examined |
| FR-432 step-row right side | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | examined |
| FR-432-AC-7 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | context_only |
| TC-386 SR-10 | spec/test-cases/TC-386-explicit-protocol-step-rows.md | examined |
| TC-386 IL-05 | spec/test-cases/TC-386-explicit-protocol-step-rows.md | examined |
| TC-386 IL-06 | spec/test-cases/TC-386-explicit-protocol-step-rows.md | examined |
| TC-386 IL-01 | spec/test-cases/TC-386-explicit-protocol-step-rows.md | context_only |
| FR-375 abstract_internal | spec/functional/analysis/FR-375-declare-model-refinement.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | IL-05 does not discriminate alias-free from alias-including order: with alias `abs` and protocol `full`, `abs::full::fin` < `lose abs::full::ch` and `full::fin` < `lose full::ch`, so an implementation still ordering by the alias-including spelling passes. AC-11 requires the case where the two orders differ. Use an alias that sorts after `lose` (QSL FR-147-AC-5 uses `spec`: `lose spec::Full::ch` < `spec::Full::fin` alias-including, `Full::fin` first alias-free), or a lower-case protocol name after `l` under an upper-case alias | spec/test-cases/TC-386-explicit-protocol-step-rows.md:46 |
| FND-002 | medium | IL-05 renames the protocol clause `Full` to `full` as well as the alias, yet expects the same list "with equal node ids". AC-11 and FR-375 make only an alias rename identity-neutral; FR-375 identifies the protocol clause by `protocol`, and the alias-free spellings themselves change (`full::fin` vs `Full::fin`), so a correct implementation gives a different list and node id. Rename the alias only (keep `Full`), or drop the node-id equality for the protocol rename | spec/test-cases/TC-386-explicit-protocol-step-rows.md:46 |
| FND-003 | low | Resolution now orders by alias-free spellings, but AC-7 and IL-01 still state the resolved list in alias-including spellings (`Spec::Full::fin` then `lose Spec::Full::ch` "in UTF-8 byte order"), while IL-05 states it alias-free (`full::fin`, `lose full::ch`). Say once whether the resolved list holds classes (shown alias-free) and phrase AC-7 accordingly | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:203 |
| FND-004 | low | "so such a node has no step row of its own to target and is mapped on the concrete side only" is unclear: a concrete timeout or fence class still has its own left-side row; what the clause means is that no right side can name an abstract timeout, fence or parallel node, so those abstract classes are only internal (listed) or visible-but-unmatched. State that directly | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:71-76 |

## Verdict

Not mergeable as-is: FND-001 and FND-002 (TC-386 IL-05) are medium. The FR-432 text itself is sound and agrees with QSL FR-147.

## Dispositions

Disposition pass 1, against head 2d36bd89cd04354984efa21c81a7df2ff2f4c159.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | verified at 2d36bd89cd04 |
| FND-002 | fixed | verified at 2d36bd89cd04 |
| FND-003 | fixed | verified at 2d36bd89cd04 |
| FND-004 | fixed | verified at 2d36bd89cd04 |
