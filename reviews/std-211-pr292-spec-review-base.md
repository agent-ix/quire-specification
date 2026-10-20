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

## New findings (disposition pass 1)

Against head 7a78721 (7a787210427712f969a0ef1f88f9aa133156ba22). `cargo test --test checked_package_v2 --test backend_provider_v1`: 9 and 108 pass. `quire validate --scope .` on FR-377, FR-379 and TC-315 exits 0, with output identical to main once the known warnings are filtered out. The whole-tree `quire validate --scope . 'spec/**/*.md'` is byte-identical to main after path normalisation. `quire matrix` lists FR-377-AC-12/13 and FR-379-AC-1..7 as `untagged`, the same as main, because the implementation tests live in QSL. Mirroring `Reads` in the QSL engine was not feasible: QSL main 9567ed6 does not implement `check_step` (FR-141) yet, since no `check_step` symbol exists under crates/ or tests/. So I type-checked the fixture by hand against FR-375: one receiver plus two arguments for `set(x, y)` has the right count; `r(self)` gives `Reference<Impl::Reg>`, which is lifted to `Reference<Spec::Reg>`; `a1`/`a2` give `Int[0, 3]`; the history row is bounded, with a literal initial value. I re-derived every case by hand (results below). Round 1 cases, re-derived by hand on the fixture as intended, with `Extra` present only in (e): (a) `A1`; (b) `P`; (c) `A1`; (d) `H`, `mapping-undetermined`; (e) `P`/`Extra`, `mapping-undetermined`; (f) `failed`; (g) `limit-reached`; (h) `A1`; (i) `A1`, `mapping-undetermined`; (j) `failed`; (k) `R`; (l) `A1`; (m) `A1`, `mapping-undetermined`; (n) `H`; (o) `undecided-successor`. Each agrees with FR-377-AC-12/13 and with QSL FR-141. Cases (k)-(n) now force the total order R < A < H < P: no other permutation passes all four. FND-006 records why (g) and (o) fail on the fixture as written.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | high | The fix commit overwrites two committed review records from PR #287. `reviews/std-211-spec-review-base.md` held SR-13322 and `reviews/std-211-spec-review-integrity.md` held SR-13323, each with dispositions rounds 1-7. At 7a78721 both are replaced by this PR's SR-13372/13373, which deletes 199 lines of review history from main. The cause is mine: I named the SR files for this PR with the same `std-211-spec-review-<method>` slug that STD-211's PR #287 review already used. Restore both files from main. Commit this PR's reviews under the distinct slugs `reviews/std-211-pr292-spec-review-base.md` and `reviews/std-211-pr292-spec-review-integrity.md`; the reviewer scratch copies now carry those names. | reviews/std-211-spec-review-base.md:1-48 |
| FND-006 | high | `store()` always creates the `Extra` object of an unmapped subtype, because the fixture states it unconditionally. Then every case's post-state `map(s', h')` holds an unmapped-type read (FR-377-AC-11), and that read is undetermined. So the step rules are never decided in any case. Cases (g) and (o) cannot reach the abstract `set` contract: (g) settles `inconclusive`/`mapping-undetermined` (naming the `Extra` object row), not `limit-reached`, and (o) settles `mapping-undetermined`, not `undecided-successor`. The other cases still give the expected result, because each has an earlier or an undefined read. The wording also gets the unmapped read wrong: `map` reads the `Extra` object's object row, not `P` "evaluated on" it. The fixture also does not say which type `Extra` subtypes, or that the source population admits it. Fix: create `Extra` only in case (e), for example with a variant of `store`; name its supertype `Impl::Reg` in the population; and describe the unmapped read as the `Extra` object's object-row read. | spec/test-cases/TC-315-refinement-undetermined-and-replay.md:66-88 |
| FND-007 | low | The fixture text has three loose ends. (1) It writes "rows `v = self.v`" and also "the field row `P` (`v = p(self)`)". Two rows for `v` refuse `invalid_model_binding`/`conflicting-binding`, so say that `P` is the `v` row. (2) "`H` set to 3" reads as a literal update `3`, which FR-151 conformance may refuse statically against `Int[0, 2]`. Write an update whose value reaches 3, for example `pre(self.h) + 3`, as TC-315 case 2 does. (3) Case (o) is "as (g)", so it keeps (g)'s meter-exhausting postcondition, and neither spec says whether an undecided precondition is reported before the postcondition runs. Say "as (g) but with the postcondition cheap". | spec/test-cases/TC-315-refinement-undetermined-and-replay.md:66-88 |


## New findings (disposition pass 2)

Against head 377969c (377969c91bdc4b945f723d5de8649a1309f17439). Main's SR-13322/13323 files are byte-identical to origin/main again. The diff against main adds only the two `std-211-pr292-*` review files, and they equal the round-1 reviewer copies. `cargo test --test checked_package_v2 --test backend_provider_v1`: 9 and 108 pass. `quire validate --scope .` exits 0 on FR-377, FR-379, TC-315 and both review files, with no errors once the known warnings are filtered out. The whole-tree validate is byte-identical to main after path normalisation. `quire matrix` lists FR-377-AC-12/13 as untagged, the same as main. Type rules, checked by hand against FR-375 and FR-149: the `v` row is now the only row; its `if` has arms `0` and `<post>`, an integer expression over `Int[0, 3]`, which is admitted as TC-315 case 1's `2 div self.tmpA` is. `h + 3` is decided at run time, since FR-149's static unproved-range rule covers only `=`/`!=` operands and FR-375 decides statically only for literals, so it leaves `Int[0, 2]` at run time. The step row has one receiver and two arguments for `set(x, y)`. With `v` starting at 0 and `store` setting it to 1, `P` takes its `0` arm at the pre-state and `<post>` only at the post-state. Every one of the fifteen cases holds by hand: (a) `A1`; (b) `P`; (c) `A1`; (d) `H`, `mapping-undetermined`; (e) the `Extra` object row, `mapping-undetermined`, since `Extra` exists only in (e) and every read is defined; (f) `failed`; (g) `limit-reached{limit, M, setting}`, since nothing else fails; (h) `A1`; (i) `A1`, `mapping-undetermined`; (j) `failed`; (k) `R`; (l) `A1`; (m) `A1`, `mapping-undetermined`; (n) `H` (`1 div pre(h)` at `h = 0`) before `P` (`1 div 0` at `v = 1`); (o) `undecided-successor`, with a cheap postcondition and the precondition incomplete without meter exhaustion. The budget paragraph now gives every clause evaluation of either subject a fresh meter of one budget and decides the outcome by which side ran out, which matches QSL FR-120 and FR-141-AC-7/9. The limit is named in QSpec terms (`limit-reached{limit, value, setting}`, an FR-461 `model_check.*` setting), and no setting name is invented.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | low | Two wording slips in the fixture. Neither changes a result. (1) "`P` (the field row after `store`) is read only at the post-state" is false: the `v` row is read at every state, the pre-state included, where it takes the `0` arm. What holds is that `P` fails only at the post-state. (2) "`H` is `h`", "`1 div h`" and "`h + 3`" use a bare `h`, but FR-375 has an update read the history field's prior value as `pre(self.h)`. Write `pre(self.h)`, `1 div pre(self.h)` and `pre(self.h) + 3`. | spec/test-cases/TC-315-refinement-undetermined-and-replay.md:74-80 |

## Dispositions

Round 1, against head 7a78721.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | The arity, the receiver that can be chosen, the history field and the post-state-only unmapped object are fixed. But nothing yet makes `P` (`v = p(self)`) undefined, incomplete or faulted only at `s'`. `P` is read at every state, the initial state included, so a `p` that fails at `s` reports at position 0 (FR-377-AC-1), not at the step. That affects (b), (d), (f), (j), (l) and (n). Say that `p` fails only after `store` writes `v`, for example `p(self) = 1 div (1 - self.v)` with initial `v = 0` and `store` setting `v = 1`. |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |

Round 2, against head 377969c.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-005 | fixed | - |
| FND-006 | fixed | - |
| FND-007 | fixed | - |
