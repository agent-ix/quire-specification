---
id: SR-13323
title: "Integrity review of QSpec PR #287 (STD-211)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@1e274e7; spec/functional/analysis/FR-375-declare-model-refinement.md, spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md, spec/functional/analysis/FR-377-decide-model-refinement.md, proposals/quire-v1/shared-grammar.md, proposals/quire-v1/definitions/native-diagnostics.md, spec/test-cases/TC-047-emit-typed-located-causes.md; spec/functional/foundation/FR-001-select-native-profile.md, spec/functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md, spec/functional/expressions/FR-147-evaluate-integer-division-domains.md, spec/functional/type-model/FR-149-apply-complete-equality-matrix.md, spec/objects/foundation/FR-272-typed-diagnostic-cause.md (context)"
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

Ticket: STD-211. Checks the new rules against the requirements they lean on (FR-001 profile selection, FR-143 field presence, FR-147 division admission, FR-149 equality, FR-272 causes) and the grammar text against its own productions and TC vectors. Engine evidence from quire-spec-language origin/main a2d68af2e (read-only).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-375-value-profile | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-AC-18 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-option-rows | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-377-AC-10 | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| FR-377-option-map | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| GRAM-forms | proposals/quire-v1/shared-grammar.md | examined |
| FR-376-AC-8 | spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md | examined |
| GRAM-dotdot | proposals/quire-v1/shared-grammar.md | examined |
| DIAG-only | proposals/quire-v1/definitions/native-diagnostics.md | examined |
| FR-376-AC-6 | spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md | examined |
| FR-376-AC-7 | spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md | examined |
| TC-047-row | spec/test-cases/TC-047-emit-typed-located-causes.md | examined |
| FR-147 | spec/functional/expressions/FR-147-evaluate-integer-division-domains.md | context_only |
| FR-001 | spec/functional/foundation/FR-001-select-native-profile.md | context_only |
| FR-143 | spec/functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md | context_only |
| FR-272 | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The value-profile rule contradicts the requirements it relies on and names the wrong layer. (1) FR-147 says admission of `div`/`rem`/`mod` "requires the declaration's `quire.value.complete/v1` facet", and FR-001 admits a declaration under the layer its `using` names; a refinement's `using` names the infinite-trace profile, so under both texts its rows have no complete-value facet however the unit selects layers. The PR adds a unit-level selection without amending FR-001 or FR-147, and still says rows follow FR-147 "as anywhere else". (2) The forms it cites are complete-model, not complete-value: FR-149 compares `Reference` values only when "the package selects `quire.model.complete/v1`" (the ring row's `s.ring = self`), and native-diagnostics' Undefined reasons, including the new `only-not-one`, apply only to a package selecting `quire.model.complete/v1`. A unit selecting value.complete alone passes AC-18 and then refuses `s.ring = self`. (3) Engine: QSL derives one unit-wide `complete_value` flag from any function or state clause whose `using` selects complete value (qsl-semantics/src/check/mod.rs:927-937); refinement rows inherit it (refinement.rs:1030 `Typer::new`, check.rs:1666). A unit that selects complete value in its header but has no such clause has rows whose `mod` refuses `profile_prohibited_operator`; QSL #1038 left this to the owner (its choice 7) and dropped its own rule. The ticket's open question is not answered consistently with FR-001/FR-147; the owner should rule (a refinement's rows are admitted under the selected layer closure containing `quire.model.complete/v1`, with FR-001 and FR-147 amended to say so, or rows inherit a named declaration's facet). | spec/functional/analysis/FR-375-declare-model-refinement.md:279-287,399; spec/functional/expressions/FR-147-evaluate-integer-division-domains.md:47-52,118; spec/functional/foundation/FR-001-select-native-profile.md:54-70; spec/functional/type-model/FR-149-apply-complete-equality-matrix.md:157; proposals/quire-v1/definitions/native-diagnostics.md:226-236; QSL qsl-semantics/src/check/mod.rs:927-937, refinement.rs:1030 |
| FND-002 | medium | AC-18 (and the Value profile paragraph) refuse `unsupported_construct` with no cause. FR-272 requires one of `declaration-form`, `expression-form` or `not-yet-implemented`, each with its payload, and FR-001's rule for a declaration outside the selected layer is `declaration-form` at the declaration naming the selected layer. Name the cause and payload (which layer is named when the unit selects none), and say whether the support-set and `wrong-selection-role` refusals of the same declaration are also reported. | spec/functional/analysis/FR-375-declare-model-refinement.md:283-284,399; spec/objects/foundation/FR-272-typed-diagnostic-cause.md:67; spec/functional/foundation/FR-001-select-native-profile.md:65-67 |
| FND-003 | high | The `null` row rule describes a value no well-typed row can produce, while the stated `?` reading silently maps `null` to absent. FR-143: "`null` is never a value of `Option<T>`, and `none` is never a field state"; explicit `null` exists only as the state of a `?` field slot. A row for a `?` field must be `Option<T>` and a row for any other field has the field's type, so no row can evaluate to `null`: FR-377-AC-10's null clause and TC-315 case 8 have no constructible vector. Meanwhile FR-375 reads a concrete `?` field as `if present(self.f) then Some(value(self.f)) else none`, and FR-143's `present` is false for explicit `null`, so a concrete `null` slot maps to an absent abstract field: the conflation the paragraph says it avoids. An abstract `?` field's `null` state is unreachable by any row. The not-yet-implemented refusal it prescribes also names no feature, which FR-272's payload and FR-375's closed feature set require. QSL's rule (FR-140 in #1038) follows its kernel `Value::Null`; the QSL ask was for QSpec to rule. State how a concrete `null` slot maps (to abstract `null`, or a refused row with a named feature), and drop the unreachable `null`-valued row case. | spec/functional/analysis/FR-377-decide-model-refinement.md:75-83,191; spec/functional/analysis/FR-375-declare-model-refinement.md:267-277,398; spec/functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md:152-160; spec/test-cases/TC-315-refinement-undetermined-and-replay.md:50-53; spec/objects/foundation/FR-272-typed-diagnostic-cause.md:67 |
| FND-004 | medium | The disambiguation sentence misreads `sum` without `where`. It says that after the binder's `model-name`, "`where` or `)` reads the population form, `:` reads the quantifier over a collection expression". The population `sum` always has `:` after the binder, and the grammar has no `sum(` quantifier (only `sum<T>(`), so `sum(x in P: e)` would be read as a quantifier that does not exist. TC-311 step 1 uses exactly `sum(s in R::slots: s.value)`. The rule holds only for `count`, `exists` and `all`; for `sum` the `<` test alone separates the forms. Restrict the sentence to the three. | proposals/quire-v1/shared-grammar.md:682-687; spec/test-cases/TC-311-population-valued-mapping-expressions.md:30-35 |
| FND-005 | medium | AC-8 requires both that "the six forms ... outside a refinement declaration" refuse `unsupported_construct`/`expression-form` and that `only`, `seq`, `where` and `Some` "remain identifiers outside these productions". Outside a refinement the grammar recognizes none of `only(x in P where c: e)` or `seq(i in a .. b: e)`: `only`/`seq`/`where` are identifiers there and `..` is admitted only in `seq`, so the text is a syntax error (`invalid_syntax`) at `in` or `..`, not an `expression-form` refusal. State that the `expression-form` refusal applies to `count`/`sum`/`exists`/`all` population forms and a bare population name, and that `only`/`seq` spellings outside a refinement are syntax errors (or parse them everywhere and refuse them, and drop "remain identifiers"). | spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md:103; proposals/quire-v1/shared-grammar.md:687-692; spec/test-cases/TC-311-population-valued-mapping-expressions.md:43-44 |
| FND-006 | low | TC-047's `not-yet-implemented` row enumerates the feature names as "(`history-row`, `protocol-abstract-side`, `refinement-internal-list`)"; FR-375 now says its list of five is "the closed set". Add `nested-reference-row` and `abstract-relationships` or reword the row as examples. | spec/test-cases/TC-047-emit-typed-located-causes.md:38; spec/functional/analysis/FR-375-declare-model-refinement.md:180-188 |

## Verdict

Not mergeable on this lens until FND-001 and FND-003 are ruled on and FND-002, FND-004 and FND-005 are fixed. Consistent: `only-not-one` is placed in the closed Undefined-reason table in order, with a payload matching FR-376-AC-7 and QSL FR-139/FR-140 (#1038); the sum bound `[min(0, n·lo), max(0, n·hi)]` is correct for negative and positive summands and its vector (`[1, 2]` over two objects gives `[0, 4]`) matches QSL FR-139-AC-6; the `..` token rule matches the productions; `count`, `sum`, `exists` and `all` are reserved as stated; FR-376-AC-5's `convert<Integer>` rationale matches FR-149-AC-21 as merged, and open #286 changes only ordering operators, so it does not affect it.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | high | The row-layers mechanism (option A) rests on a profile declaration that cannot be written. FR-375 says the `profile` declaration the refinement's alias resolves to "selects `quire.temporal.infinite-trace/v1` and may name layers such as `quire.value.complete/v1` and `quire.model.complete/v1`", and the new FR-001 bullet speaks of an infinite-trace alias "whose `profile` declaration also names layers". The grammar gives a profile declaration exactly one identity (`profile = 'profile', ident, '=', string, ';'`, shared-grammar.md:110), FR-001 selects one layer per declaration naming it, and no temporal-profile definition carries layers or `requires` edges (FR-090, FR-250; `header_selectable_layers` has no temporal row). So the first half of FR-375-AC-18, FR-001-AC-7 and TC-310 mutation 21 have no spellable vector, and every refinement, including TC-316's and TC-311's `RingIsQueue` (`using inf`) and FR-376-AC-5's `s.ring = self` and `mod` rows, falls in the "alias naming no layer" case and refuses. The baseline for that case is also undefined: FR-001 gives no admitted-form set for a declaration whose alias names no layer, so "naming the selected layer" names nothing and it is unstated whether a plain `value = self.value` row (state core) is admitted. Define how an alias names layers (for example a profile declaration with a layer list, or the infinite-trace definition requiring `quire.model.complete/v1`), state the baseline layer set when it names none, and make TC-316's and TC-310's fixtures spell it. | spec/functional/analysis/FR-375-declare-model-refinement.md:267-279,410-414; spec/functional/foundation/FR-001-select-native-profile.md:65-68,85; proposals/quire-v1/shared-grammar.md:110; spec/test-cases/TC-316-ring-buffer-data-refinement.md:30-39 |

## Dispositions

Round 1, against head 5a66bf9.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 5a66bf9: planner option A applied: rows are admitted under the alias's layers; FR-001 (new bullet, AC-7), FR-147, FR-149 and the native-diagnostics Undefined-reasons scope are amended to say so, and `quire.model.complete/v1` is named for `Reference` comparison and the reasons. The contradiction and the wrong layer are resolved; the alias mechanism itself cannot be spelled: see new FND-007 |
| FND-002 | fixed | 5a66bf9: the refusal is now `unsupported_construct`/`expression-form` at the expression naming the selected layer, per FR-001; the layer named when the alias names none is undefined: see new FND-007 |
| FND-003 | fixed | 5a66bf9: the `null`-row rule is removed from FR-375, FR-377-AC-10 and TC-315 case 8; the implicit `if present` reading is replaced by "a row reads a concrete `?` field only through `present` and `value` (FR-143)", so any mapping of an explicit `null` slot is the author's written expression |
| FND-004 | fixed | 5a66bf9: the sentence now separates `count`/`sum` by `<` alone and applies the `where`/`)`/`:` test to `exists` only, so `sum(x in P: e)` reads as the population form |
| FND-005 | fixed | 5a66bf9: AC-8, the grammar note and TC-311 step 8 now refuse `count`/`sum`/`exists`/`all` population forms and a bare population name with `expression-form` outside a refinement, and make `only`/`seq` spellings there `invalid_syntax`/`unexpected-token` at `in` |
| FND-006 | fixed | 5a66bf9: TC-047's list now holds the four support-set features; `abstract-relationships` is no longer one |
