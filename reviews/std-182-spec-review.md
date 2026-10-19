---
id: SR-13054
title: "Base review of PR #284: refinement header selects an abstract protocol subject"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@04cc1b9 (base a2230e1); shared-grammar.md refinement-header paragraph; FR-375 Surface syntax, Protocol subject, Checked-package node, AC-11; FR-432 abstract-side selection paragraph, AC-6; TC-310 mutation 13 and Expected Results; TC-386 SR-09; checked against agent-ix/quire-spec-language@776c9ed76 FR-147, FR-310, ADR-020 RM-9/QS-11, TC-552, TC-889"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-432
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-310
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-386
    type: reviews
---

## Summary

Ticket: STD-182. PR quire-specification#284 (head 04cc1b9, diff
`origin/main...HEAD`, base a2230e1). The refinement header becomes
`'abstract', abstract-subject` with `abstract-subject = ident, [ '::',
member-name ]`; the node body gains `protocol` for a protocol subject;
FR-375-AC-11, FR-432-AC-6, TC-310 mutation 13 and TC-386 SR-09 are new.
`quire validate` over the four changed spec files exits 0. The diff holds
no code, so `code-review` and `gap-analysis` do not apply; the statements
(Description sections) are unchanged, so EARS analysis is skipped.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-432-AC-6 says a `node Spec::Req::Ask` row under a model-subject header `abstract Spec` refuses `missing_declaration`/`missing-name`. The paired QSL FR-147 (Behavior and AC-2, on main) refuses the same case `invalid_model_binding`/`malformed-declaration` ("While the abstract side is a model subject, a right side naming a protocol node SHALL refuse ..."). The name does resolve (the node exists in the abstract package), so `missing-name` is also the weaker fit. One of the two ACs must fail against one implementation. QSpec should pick one code and QSL align (or the reverse), in this PR. | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:118; spec/test-cases/TC-386-explicit-protocol-step-rows.md:39; quire-spec-language spec/functional/FR-147-check-a-refinement-whose-abstract-side-is-a-protocol.md:91-92,140 |
| FND-002 | medium | TC-310 mutation 13 cannot show that `protocol` enters the node id. It compares the `abstract Spec::Twice` node (written "with the step rows FR-432 requires") with the plain `abstract Spec` header, whose rows may differ, so the ids can differ through the rows alone. AC-11's "protocol name changed" case needs a second protocol clause that exists, which the fixture does not give (`Thrice` refuses, so no node). Needed: identical rows using only operation targets (as QSL `CasTwice` does) under `Spec::Twice`, under `Spec::Twice2` (an identical second clause) and under `Spec`, all admitted, with three distinct ids. | spec/test-cases/TC-310-refinement-declaration-admission.md:69-75,107-114; spec/functional/analysis/FR-375-declare-model-refinement.md:250 |
| FND-003 | medium | AC-11's near-miss `abstract Spec::inc` is labelled "(an operation)", but in the TC-310 fixture `inc` is an operation of type `Counter`, spelled `Spec::Counter::inc`; `Spec::inc` is just an unknown name and tests nothing beyond `Spec::Thrice`. The real collision is a two-segment model-name that does resolve: `Spec::Counter` (a type) or `Spec::counters` (a population), which has exactly the `abstract-subject` shape. The Protocol subject paragraph names "a type", but no AC vector exercises it. | spec/functional/analysis/FR-375-declare-model-refinement.md:123-126,250; spec/test-cases/TC-310-refinement-declaration-admission.md:20-23,72 |

## Verdict

Not mergeable until FND-001 is resolved (a cross-repo contradiction in a
normative AC); FND-002 and FND-003 should be fixed in the same round.

What holds: the production is unambiguous under LL(1) (a lone `ident` is
followed by `concrete`; `::` commits to the protocol form) and matches how
the corpus writes other model-qualified names (`model-name = ident, '::',
member-name`; `qualified-name = ident, { '::', member-name }`), including
admitting reserved words after `::`. QSL FR-310's `Protocol(alias, clause)`
form and FR-147's `CasTwice` header `abstract Spec::Twice concrete Impl`
parse under it, and FR-147's `abstract Spec::Thrice` refusal agrees. The new
body key follows FR-322's absent-not-null convention, and no golden digest
in this repo pins a refinement node body (the repo has no refinement code or
fixtures). No AC, TC vector or file collides with open QSpec PRs #283, #274,
#273, #267, #207, #200 or #59. The PR description's "Out of scope" note about
QSL `internal`/`visible` rows is stale: QSL #992 (merged) already removed
them from FR-147, FR-148, TC-552 and ADR-020 RM-9/QS-11; nothing in the diff
depends on that note.
