---
id: SR-13325
title: "Failure-domain review of QSpec PR #287 (STD-211)"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@1e274e7; spec/functional/analysis/FR-375-declare-model-refinement.md, spec/functional/analysis/FR-377-decide-model-refinement.md, spec/test-cases/TC-315-refinement-undetermined-and-replay.md; QSL FR-140/FR-135 in quire-spec-language PR #1038 (7a40570) (context)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-376
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-377
    type: reviews
---

## Summary

Ticket: STD-211. Looks for unstated failure orders, undefined failure kinds and edge interactions in the mapping rules, and compares them with QSL FR-140/FR-135 as proposed in quire-spec-language #1038.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-377-precedence | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| FR-377-AC-9 | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| FR-375-node | spec/functional/analysis/FR-375-declare-model-refinement.md | context_only |
| FR-375-AC-7 | spec/functional/analysis/FR-375-declare-model-refinement.md | context_only |
| FR-375-lifting | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-option-rows | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-step-rows | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-AC-15 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | "an evaluator stop on a broken invariant" is not a QSpec outcome. QSpec evaluation yields a value, `undefined`, refused or incomplete; "stopped" in QSpec is a run limit (FR-363's Stopped row, FR-379's budget row), and an internal fault "settles the item `failed`, never a value" (FR-383). FR-377 imports QSL's `EvalFailure::Stopped` (FR-140 in #1038) and makes it `map` undetermined, which FR-379 settles `inconclusive`/`mapping-undetermined`, not `failed`. A reader cannot tell whether a meter or budget stop is this stop. Either define the internal fault and settle it as FR-383 does, or leave it to the producer and drop it from the precedence (keeping undefined over undetermined). | spec/functional/analysis/FR-377-decide-model-refinement.md:92-94,190; spec/functional/analysis/FR-383-explore-under-state-space-reduction.md:190-193; spec/functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md:104 |
| FND-002 | medium | The order rests on "each array in the node's own order", but FR-375 sorts only the body's top-level arrays (`populations`, `objects`, `steps`, `histories`, `assume`, `ensure`); the field rows inside an `objects` entry have no stated order. When two field rows of one object fail at one state, the reported row is not fixed, which is what the rule exists for. Consistency with FR-375-AC-7 is otherwise fine: node order is source-independent, so the AC-9 claim that source order changes nothing holds once field-row order is fixed. Cross-repo: QSL #1038 FR-140 says "population rows in declaration order ... field rows in declaration order"; the PR body (choice 5) asks QSL to read that as node order, but #1038's text says otherwise and no QSL ticket records the change. State the field-row order in FR-375's node (for example by field node-key digest) and file the QSL wording change. | spec/functional/analysis/FR-377-decide-model-refinement.md:85-91; spec/functional/analysis/FR-375-declare-model-refinement.md:366-369; QSL spec/functional/FR-140-evaluate-the-refinement-mapping-at-a-state.md (#1038, map_state order bullet) |
| FND-003 | medium | Option rows and the nested-reference rule interact without a stated outcome. An abstract `?` field of reference type takes a row of type `Option<T>`, so `Some(self.ring)` and the concrete `?` reading of a reference field have type `Option<Reference<T_C>>`, a reference nested in an option, which the lifting paragraph sends to `nested-reference-row`; every current producer then refuses every optional reference field except, perhaps, by `none` (whose type holds `Reference<T_A>`, not `T_C`). Optional references are ordinary (QSL FR-120-AC-13's `Option<Reference<Counter>>`, FR-140-AC-6's `next`). Say whether `Option<Reference<T_C>>` is lifted like a top-level reference (one level of option) or uses the feature, and give AC-15/AC-17 a vector for it. | spec/functional/analysis/FR-375-declare-model-refinement.md:252-258,267-272,396-398 |
| FND-004 | medium | Nested references in step-row arguments and history updates are left undefined in the spec. Step rows type each argument "after reference lifting", but the new top-level-only rule and `nested-reference-row` cover field rows only ("A field row whose type holds ..."), so a `Sequence<Reference<T_C>>` step argument for a `Sequence<Reference<T_A>>` parameter is neither lifted nor refused. The PR body records this (choice 1: "Step arguments and history updates are not addressed") but the spec does not. Extend the rule and the feature to step arguments and history updates, or say they refuse `ill_typed`/`type-mismatch` until defined. | spec/functional/analysis/FR-375-declare-model-refinement.md:252-258,289-299 |

## Verdict

Not mergeable on this lens until FND-001 and FND-002 are fixed, since they leave the reported row and the settled verdict implementation-defined. Clean and consistent with QSL #1038: undefined outranks every undetermined row; the first undefined row in order is reported; an unmapped concrete subtype object or reference makes `map` undetermined and no object is dropped; the mapped state holds no relationship extent; the spans of the two new support-set refusals (the row's expression; the abstract alias) match QSL FR-135-AC-6/7.

## Dispositions

Round 1, against head 5a66bf9.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 5a66bf9: the evaluator stop is removed from the precedence; an internal fault settles the item `failed` per FR-383 (planner ruling); AC-9 states it |
| FND-002 | fixed | 5a66bf9: FR-375's node now sorts field rows within an `objects` entry by field name bytes and FR-377 reads them in that order, so the reported row is fixed and FR-375-AC-7 holds for field rows. QSL #1038 (head 7a40570, open) still says "declaration order", returns at once on `Stopped`, keeps the `null` row and lists `abstract-relationships` as a support-set feature; those need a QSL revision |
| FND-003 | fixed | 5a66bf9: `Option<Reference<T_C>>` (`Some(self.ring)`) is named as using `nested-reference-row`, with an AC-15 clause and a TC-310 mutation 18 vector |
| FND-004 | fixed | 5a66bf9: a nested reference in a step argument or history update is not lifted and refuses `ill_typed`/`type-mismatch`; AC-15 and TC-310 mutation 18 cover the step argument |
