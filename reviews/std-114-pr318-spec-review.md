---
id: SR-13671
title: 'Spec review of QSpec PR #318: FR-151 effective precondition combination (STD-114)'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@233a142a4; spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md refinement rule and FR-151-AC-17; spec/test-cases/TC-196-conformance-redefinition-and-dispatch.md D09; ticket STD-114'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-151
  type: reviews
- target: ix://agent-ix/quire-specification/TC-196
  type: reviews
---

## Summary

Ticket: STD-114. PR quire-specification#318, head 233a142a4d4a530d8cb8588501633abed15f39e0. Review set: base checklist plus the EARS, integrity and failure-domain lenses, in this one document.

The D09 vector works. Over D01, `d1` has type `D`, which links to `B/size`, so the selected effective precondition is `own(B/size) or eff(A/size)` = `(PB1 and PB2) or PA`. Both readings, computed by hand:

| Leg | PB1 | PB2 | PA | (a) `(PB1 and PB2) or PA` | (b) `PB1 or PB2 or PA` | Flat conjunction |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | true | false | true | true | true | false |
| 2 | true | false | false | false | true | false |

Leg 2 is the only leg that separates (a) from (b). Leg 1 gives the same result under (a) and (b), so it works as a control and also rules out a flat conjunction. AC-17 puts its "which a flat disjunction would not give" claim on leg 2, which is right.

The vector can be written in the merged grammar: `('pre' | 'post'), ident, 'using', ident, 'on', operation-name, block` (proposals/quire-v1/shared-grammar.md:121-125). `PB1` and `PB2` are valid identifiers (line 25), and `{ true }` and `{ false }` are Boolean blocks, the same form D06 uses.

The new terms `own(m)` and `eff(r)` are defined once, at FR-151:121-125. The PR adds no new links and no status wording. No open QSpec PR touches FR-151 or TC-196, so FR-151-AC-17 and TC-196 D09 cannot collide with another PR. The QSL code already implements reading (a): `view_clauses` conjoins a member's own `pre` clauses (qsl-semantics/src/check/dispatch_link.rs:309-324), and `effective_terms` disjoins the result with the parent's terms, own term first (checked_dispatch.rs:460-540).

Gates at the PR head:

- `quire validate --scope . 'spec/**/*.md'` (quire 0.36.2, engine 0.50.2): exit 3, 134 documents fail. That is the stated pre-existing count, and neither touched file is among them. Validating the two touched files alone exits 0.
- `cargo test --test checked_package_v2 --test backend_provider_v1`: 114 and 14 tests pass.

Other places that mention an effective precondition: FR-181, 361, 362, 363, 377, 383, 397, 406, 426 and 468, value-accounting.md:435, shared-grammar.md:1001 and native-diagnostics.md:245 use the term without restating how it is combined. One restatement was missed: FR-208 (FND-006).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-151 refinement rule (effective precondition) | spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md | examined |
| FR-151-AC-17 | spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md | examined |
| TC-196 D09 | spec/test-cases/TC-196-conformance-redefinition-and-dispatch.md | examined |
| TC-196 D06 | spec/test-cases/TC-196-conformance-redefinition-and-dispatch.md | context_only |
| TC-196 D08 | spec/test-cases/TC-196-conformance-redefinition-and-dispatch.md | context_only |
| FR-151 dispatch call-graph and runtime paragraphs | spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md | context_only |
| FR-208 redefinition sentence | spec/objects/foundation/FR-208-quire-meaning-vocabulary.md | examined |
| FR-006 short-circuit behaviour | spec/functional/foundation/FR-006-preserve-control-evaluation.md | context_only |
| model-complete Redefines row | proposals/quire-v1/definitions/model-complete.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The rule fixes how the clauses are grouped but not the order in which a member's own clauses are conjoined. Under FR-006, `and` evaluates its left operand first and skips the right one when the left is false, and value-accounting.md:435 charges the precondition's own evaluation, so the order changes the outcome. Example: `PB1` undefined and `PB2` false. `PB1 and PB2` is undefined, but `PB2 and PB1` is `undefined`/`precondition-false`. The two orders also give different charges and different `incomplete` charge points. Implementations already disagree: QSL's dispatch path conjoins in source-declaration order (dispatch_link.rs:309-324, `unit_clauses_for`), while QSL FR-120's simulator conjunction uses ascending clause name and depends only on the set of outcomes. FR-151 otherwise forbids source or declaration order from deciding anything. Fix: state the conjunct order, for example ascending clause name in UTF-8 bytes, or ascending declaration key. Add a TC-196 row where a different order would give a different result, for example `PB1` undefined and `PB2` false. | spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md:121-127; proposals/quire-v1/definitions/value-accounting.md:435 |
| FND-002 | low | FR-151-AC-17 bundles three claims: the grouping, the two D09 outcomes, and "a member with no own clause has the effective precondition `true`". D09 does not exercise the third claim. TC-196 D06's second leg does, but D06 is not tied to AC-17. Fix: drop the third clause from AC-17, since it is already stated in the rule, or name D06 as AC-17's evidence too. | spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md:253 |
| FND-003 | low | D09's first-leg oracle, "the first call proceeds through `B/size`", states neither the result of `I` nor the charges. D01 and D06 state `true`, the body value `2` and the work units, and the TC procedure compares the complete typed result and accounting. The second leg leaves out D06's "without selecting `A/size`". Fix: expect `true` through `B/size` (body `2`, `2 >= 1`) with its charges, and add "without selecting `A/size`" to the second leg. | spec/test-cases/TC-196-conformance-redefinition-and-dispatch.md:69 |
| FND-004 | low | EARS: the new obligation is written as a description ("is", "are conjoined ... never thrown into one flat disjunction") with no SHALL. Elsewhere in the same Conformance rules section, obligations use SHALL (FR-151:49-77). "Thrown into" is informal wording. Fix: "The checker SHALL form the effective precondition of an operation member `m` as ...", and drop the "never thrown" clause, since the formula already says it. | spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md:121-127 |
| FND-005 | low | "`r1` to `rn` are the members `m` redefines" implies a member can redefine several members. But `redefines` names one field or operation (model-complete.md:170), and QSL keeps one parent per member. The list form makes a reader ask what order several `eff(ri)` are evaluated in, which nothing answers. Fix: write `own(m) or eff(r)` for the one redefined member `r`, `own(m)` when there is none, and keep "formed the same way" for the recursion. | spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md:122-125; proposals/quire-v1/definitions/model-complete.md:170 |
| FND-006 | low | The author's sweep missed FR-208. It still says "Redefinition follows FR-151: the effective precondition is the disjunction and the effective postcondition the conjunction over redefined members". That paraphrase can be read as the flat disjunction (b) that this PR rules out. Fix: say "is formed as FR-151 defines (own clauses conjoined, then disjoined with the redefined member's)", or just "follows FR-151". | spec/objects/foundation/FR-208-quire-meaning-vocabulary.md:214-217 |

## Verdict

Not mergeable as is, because of FND-001: the order of a member's own conjuncts is left open, and under short-circuit evaluation and metering that order changes the call's outcome and its charges. Choosing an order is an owner-level choice, like reading (a) itself. If the owner defers it, file a follow-up and the PR can merge with FND-002 to FND-006 fixed. Apart from that, the change is correct. D09 can be written in the merged grammar, its second leg really separates reading (a) from reading (b), `own(m)` and `eff(r)` are defined once, and the touched files add no validation failures.
