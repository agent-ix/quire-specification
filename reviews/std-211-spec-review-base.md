---
id: SR-13372
title: "Base review of QSpec PR #292 (STD-211): failure precedence across a refinement step"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@e78332c; spec/functional/analysis/FR-377-decide-model-refinement.md, spec/test-cases/TC-315-refinement-undetermined-and-replay.md; ticket STD-211"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-377
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-315
    type: reviews
---

## Summary

Ticket: STD-211. PR: quire-specification#292 at e78332c (e78332ce731d91e077cc66b834bbee948eda0926), scoped to `git diff refs/remotes/origin/main...e78332ce7` (origin/main d053fb2). The diff is spec-only, over two files: FR-377 gains "Failure precedence across a step" and the budget paragraph, FR-377-AC-12/13 are added, and TC-315 gains step 9 with cases (a) to (h) and verifies edges to AC-12/13. Base review: I checked each case of step 9 by hand against the new rules, and checked the rules against QSpec FR-375, FR-377-AC-9, FR-379, FR-363 and FR-383, and against QSL main 9567ed6 (FR-141-AC-9/10/11 and FR-142/144/145). EARS: the FR-377 requirement statement is unchanged and stays event-driven ("When a refinement item is checked, the analyzer SHALL ..."). The new Behavior prose describes the system rather than the change, and each new AC is a single checkable outcome. A separate `spec-ears-analysis` was not run, because no requirement statement changed. Gates: `cargo test --test checked_package_v2 --test backend_provider_v1` passes on the head (9 and 108). `quire validate --scope .` (quire 0.36.2, engine 0.50.2; the rollout note names 0.36.1/0.50.1, so the binary is newer, not older) exits 0 on FR-377 and on TC-315, with only the duplicate-archetype, duplicate-inverse-edge and inline-schema warnings, the same as on main. A whole-tree `quire validate --scope . 'spec/**/*.md'`, with paths normalised, is byte-identical between the head and main (134 failing documents, all already on main). `quire matrix` lists FR-377-AC-12/13 `untagged`, as AC-1..11 are on main too, because QSpec tags no Rust test for FR-377 and its implementation tests live in QSL. Method `code-review` was not run, because no code or test file changed; the Cargo gates stand in for it.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-377-failure-precedence-across-a-step | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| FR-377-step-budget-paragraph | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| FR-377-AC-12 | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| FR-377-AC-13 | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| TC-315-step-9 | spec/test-cases/TC-315-refinement-undetermined-and-replay.md | examined |
| TC-315-case-9-expected | spec/test-cases/TC-315-refinement-undetermined-and-replay.md | examined |
| TC-315-relationships | spec/test-cases/TC-315-refinement-undetermined-and-replay.md | examined |
| FR-377-AC-9 | spec/functional/analysis/FR-377-decide-model-refinement.md | context_only |
| FR-377 | spec/functional/analysis/FR-377-decide-model-refinement.md | context_only |
| TC-316-declaration | spec/test-cases/TC-316-ring-buffer-data-refinement.md | context_only |
| FR-363-undecided-successor | spec/functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md | context_only |
| QSL-FR-141-AC-9 | quire-spec-language:spec/functional/FR-141-decide-one-concrete-step-against-the-abstract-model.md | context_only |
| QSL-FR-141-AC-10 | quire-spec-language:spec/functional/FR-141-decide-one-concrete-step-against-the-abstract-model.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The step-9 fixture is refused before it runs. TC-316's abstract `enq(x: Int[0, 1])` has one parameter, so the row `put -> enq(self.ring, A1, A2)` has one argument too many, and FR-375 refuses a wrong argument count `ill_typed`/`operator-ineligible` (FR-375 line 326). The row also fixes the receiver as `self.ring`, but the procedure asks for `R` to be chosen as incomplete or undefined. TC-316 has no history field for `H` and no unmapped subtype for `P`. Nothing says the step must create the `P` object, or make `P` fail only at `s'`. Without that, `map(s, h)` already fails at the pre-state, and the failure is reported one position earlier than at this step. Every case of step 9 is therefore unbuildable as written. Give step 9 its own fixture, as QSL FR-141-AC-10's `Reads` does: an abstract operation with two parameters, a receiver expression that can be chosen, a bounded history field with an update on `put`, and a post-state row or object that fails only after `put`. | spec/test-cases/TC-315-refinement-undetermined-and-replay.md:66-76 |
| FND-002 | medium | Cases (a) to (h) never test the order between the read groups. Only `A1` before `A2` (c) and `H` before `P` (d) are observable. Every other case has exactly one undefined read, or ranks undefined over undetermined, which holds in any order. An implementation that reads post history and post-state rows before the receiver and arguments (order H, P, R, A1, A2) passes all eight cases, although it breaks AC-12's "order receiver, arguments as written, post history, post-state rows". Add cross-group cases, for example: `R` and `A1` both undefined, reporting `R`; `A1` and `P` both undefined, reporting `A1`; `A1` incomplete and `H` out of type, reporting `A1`. QSL FR-141-AC-10 has the same six outcomes and the same gap. | spec/functional/analysis/FR-377-decide-model-refinement.md:218 |
| FND-003 | medium | The new text does not separate a budget stop from an undecided contract. FR-363 "Undecided successor" calls an abstract postcondition over a candidate post-state undecided when its evaluation "returns an incomplete outcome". That settles `inconclusive`/`undecided-successor`, and FR-379 applies the rule "during expansion or a step check" for either subject. The PR instead makes a budget the abstract subject exhausts while deciding a step a stop of the run, and "therefore" derives this from the read order. The read order does not imply it: it only says when the step rules run. Meanwhile a concrete row that exceeds its fresh meter is incomplete, so undetermined. So two meter exhaustions are classified differently, and no rule says why. TC case (g) uses "per-evaluation meter", which is a QSL term that QSpec never defines (QSpec says "fresh meter", and FR-363's Stopped row says "work meter"). AC-13's exclusions also leave out `undecided-successor`. State the rule as a rule, not as a consequence: exhausting the abstract subject's evaluation meter or candidate limit while deciding a step is a budget stop (the FR-363 Stopped row), not an incomplete outcome of a contract, and an incomplete mapping read stays `mapping-undetermined`. QSL FR-141 already states this. Add `undecided-successor` to AC-13's never-list. | spec/functional/analysis/FR-377-decide-model-refinement.md:120-125 |
| FND-004 | low | AC-13 and cases (g)/(h) cover only an undefined read outranking the budget. The body says that "a failed read outranks it", which also covers an undetermined read (expect `mapping-undetermined`) and a faulted read (expect `failed`). Neither is in the AC or the TC. Case (g) also expects bare `limit-reached`, although FR-379 spells the cause `limit-reached{limit, value, setting}`. A stop on a different limit, such as `max_states`, would still pass, whereas QSL FR-141-AC-9 pins `EvaluationMeter` and B. Name the limit and its value in (g), and add an undetermined-read-plus-budget case. | spec/functional/analysis/FR-377-decide-model-refinement.md:219 |

## Verdict

Not mergeable. One high finding: the step-9 fixture is refused at admission (wrong argument count against TC-316's one-parameter `enq`), and its `R`, `H` and `P` have nothing to bind to. Two medium: the cross-group read order of AC-12 is never observable in (a) to (h), and the budget-stop rule is derived rather than stated, which leaves it in tension with FR-363 and FR-379's undecided-successor rule. One low: AC-13 and case (g) are under-pinned. Correct by hand, once a buildable fixture exists: (a) A1, (b) P, (c) A1, (d) H mapping-undetermined, (e) P mapping-undetermined, (f) failed, (g) incomplete/resource-incomplete/limit-reached, and (h) A1. Each of these follows from the new body and from AC-12/13, and each matches QSL FR-141-AC-9/10. The verifies edges and the Verification column agree.
