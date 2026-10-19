---
id: SR-13322
title: "Base checklist review of QSpec PR #287 (STD-211)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@1e274e7; spec/functional/analysis/FR-375-declare-model-refinement.md, spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md, spec/functional/analysis/FR-377-decide-model-refinement.md, proposals/quire-v1/shared-grammar.md, proposals/quire-v1/definitions/native-diagnostics.md, spec/test-cases/TC-310-refinement-declaration-admission.md, spec/test-cases/TC-311-population-valued-mapping-expressions.md, spec/test-cases/TC-315-refinement-undetermined-and-replay.md, spec/test-cases/TC-316-ring-buffer-data-refinement.md"
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

Ticket: STD-211. Base checklist (ID formats, duplicates, AC-to-TC coverage, matrix visibility) over `git diff origin/main...1e274e7`. `quire validate` exits 0 on the three FRs and four TCs (the two proposal files have no frontmatter, as on main). `quire matrix --scope .` lists every new criterion (FR-375-AC-15..18, FR-376-AC-6..8, FR-377-AC-9..11). No id collides with merged #285 (FR-375-AC-12..14, TC-310 mutations 14-17) or with open #283 (FR-322-AC-75/76, TC-277 BP-14), #286 (FR-149-AC-21, TC-194 E35 only) or #288.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-375-AC-15 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-AC-16 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-AC-17 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-AC-18 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-376-AC-5 | spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md | examined |
| FR-376-AC-6 | spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md | examined |
| FR-376-AC-7 | spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md | examined |
| FR-376-AC-8 | spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md | examined |
| FR-377-AC-9 | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| FR-377-AC-10 | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| FR-377-AC-11 | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| TC-310-m20 | spec/test-cases/TC-310-refinement-declaration-admission.md | examined |
| TC-310-m21 | spec/test-cases/TC-310-refinement-declaration-admission.md | examined |
| TC-311-steps | spec/test-cases/TC-311-population-valued-mapping-expressions.md | examined |
| TC-315-c7 | spec/test-cases/TC-315-refinement-undetermined-and-replay.md | examined |
| TC-315-c8 | spec/test-cases/TC-315-refinement-undetermined-and-replay.md | examined |
| TC-316-row | spec/test-cases/TC-316-ring-buffer-data-refinement.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-377-AC-11 has three clauses but TC-315 covers one. Case 7 holds "an unmapped subtype object whose key sorts first" (the object clause); no case maps a reference to an unmapped object, and no case reads the mapped state's relationship extent (the expected results cite AC-11 only for case 7). The ring-buffer fixture also declares no subtype of `Ring`, and TC-315 does not add one. Add the subtype to the fixture (QSL TC-1086 uses `SubRing` and a `next: Reference<Ring>` field) and vectors for the reference clause and the extent clause. | spec/functional/analysis/FR-377-decide-model-refinement.md:192; spec/test-cases/TC-315-refinement-undetermined-and-replay.md:44-49,72-74 |
| FND-002 | medium | Case 7's first run cannot be built as written. The ring-buffer `Queue` has one field row (`items`), so there is no "earlier row"; and a per-evaluation meter budget of zero (the TC-555 / QSL FR-140-AC-5 device) applies to every row, so the `items` row is itself incomplete and its `only` is never reached. The fourth run ("an evaluator stop injected in the earlier row") is fault injection that a black-box vector cannot perform. Name a fixture variant with a second field row, a way to make only that row refused or incomplete (for example a refused form in it), and drop or re-scope the injected stop. | spec/test-cases/TC-315-refinement-undetermined-and-replay.md:44-49,72-74 |
| FND-003 | low | The last clause of FR-375-AC-17 and TC-310 mutation 20 expect `Some(1)` in a state clause to be "an ordinary call that resolves as any other", which states no outcome. Say what the vector expects (for example `missing_declaration`/`missing-name` at `Some` when no function `Some` is declared, and a call when one is). | spec/functional/analysis/FR-375-declare-model-refinement.md:398; spec/test-cases/TC-310-refinement-declaration-admission.md:113-116,192-195 |

## Verdict

Not mergeable on this lens until FND-001 and FND-002 give FR-377-AC-9 and AC-11 constructible vectors. Every other new criterion parses, names a TC, and that TC carries a matching step: FR-375-AC-15/16 (TC-310 mutations 18/19), AC-18 (mutation 21), FR-376-AC-5 (TC-311 step 2, TC-316), AC-6/7/8 (TC-311 steps 6/7/8), FR-377-AC-10 (TC-315 case 8). The `convert<Integer>` spelling is applied in all three copies of the ring row (FR-376-AC-5, TC-311, TC-316), no stale `s.index = (self.head` copy remains, and it matches QSL #1038's ADR-020 §8 text. The sum bound matches QSL FR-139 (#1038).

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | The rewritten case 7 fixture puts the `also` row in the variant used by every run, but `also = only(s in R::slots where s.ring = self: s.value)` matches both slots of `r` and is undefined at the held state. Read that way, `also` (the lower field name) is the reported undefined row in (a), (b) refutes instead of being undetermined for `SubRing`, and (d) refutes instead of being undetermined for the `next` row, so the expected results for (a), (b) and (d) are wrong. Only (c) says "with the `also` row". Run (d)'s ring `a` (type `Ring`) also has no stated `head`/`size`, so whether its `items` row is defined is unstated. Say that `also` is added only in (c), and give ring `a`'s state in (d). | spec/test-cases/TC-315-refinement-undetermined-and-replay.md:44-57,78-84 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | The refusing half of FR-375-AC-18 (and FR-001-AC-7, FR-090-AC-10) and TC-310 mutation 21 say "a row using a form outside `quire.model.complete/v1` and its `requires` closure" without naming one, and expect "naming the layer" without saying which. Name the form (for example a `protocol` reference or a temporal operator in a row) and the layer the refusal names. | spec/test-cases/TC-310-refinement-declaration-admission.md:127-130,210-212; spec/functional/analysis/FR-375-declare-model-refinement.md:416 |

## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | The chosen out-of-layer example cannot reach the layer check. A field row is `member-name '=' expr`, and `eventually` is a reserved word that appears only in the temporal-clause productions (`temporal-unary-unbounded` and the bounded temporal grammar), not in `expr` or `primary`. A row `items = eventually ...` is therefore a syntax error, `invalid_syntax`/`unexpected-token` at `eventually`, before any `unsupported_construct`/`expression-form` refusal naming `quire.model.complete/v1`. FR-375-AC-18, FR-001-AC-7, FR-090-AC-10 and TC-310 mutation 21 all expect the latter. Use a form that `primary` admits but the model.complete closure does not, for example the protocol reference `protocol Spec::Twice`, a `primary` admitted only under `quire.protocol.complete/v1` (shared-grammar.md:941). | spec/functional/analysis/FR-375-declare-model-refinement.md:284-287,428; spec/functional/foundation/FR-001-select-native-profile.md:98; spec/functional/temporal/FR-090-select-temporal-profile-and-clock.md:121; spec/test-cases/TC-310-refinement-declaration-admission.md:127-132,218-220; proposals/quire-v1/shared-grammar.md:161-175,352-365,939-943 |

## New findings (disposition pass 4)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | medium | `protocol Spec::Twice` now parses (`protocol-reference` is a `primary`), but in mutation 21's unit it does not resolve, and no text puts the layer check before resolution. Mutation 21 uses the ring-buffer unit (aliases `Q` and `R`); `Twice` is a protocol clause of the CasRefinesCounter fixture's abstract package (TC-310 mutation 13), and there is no `Spec` alias in the ring-buffer unit at all. FR-463 resolves a reference "in the unit's own declaration scope, to a protocol declaration of the unit"; "a name offered only by an import does not resolve" and refuses `missing_declaration`/`missing-name` at the reference (FR-463-AC-4). The reference also types as the clause's Boolean, so in a non-Boolean row it would add `ill_typed`/`type-mismatch`. Either declare a local protocol in the mutation 21 unit (for example `protocol Fill`), write it in a Boolean position, and use `protocol Fill`; or state in FR-375 or FR-001 that the `expression-form` facet refusal is decided before name resolution and typing, so no other diagnostic is reported for the row. | spec/functional/analysis/FR-375-declare-model-refinement.md:285-287,428; spec/functional/foundation/FR-001-select-native-profile.md:98; spec/functional/temporal/FR-090-select-temporal-profile-and-clock.md:121; spec/test-cases/TC-310-refinement-declaration-admission.md:75-78,127-134,215-222; spec/functional/protocol/FR-463-refer-to-a-declared-protocol-in-an-expression.md (Resolution, Refusals, AC-4); proposals/quire-v1/shared-grammar.md:941-944 |

## New findings (disposition pass 5)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | medium | The local `Fill` cannot be built from "the body of TC-376's `Fill`, over the unit's own model `R`". TC-376's body names `M::Cell`, its operations `M::Cell::inc` and `M::Cell::setTwo`, the contract clauses `IncPre`, `IncPost`, `TwoPre` and `TwoPost`, a role `on M` and a protocol profile alias `p`. The ring-buffer model `R` (TC-311, TC-316) has types `Ring` and `Slot` with operations `put` and `take`, no `Cell`, no `inc` or `setTwo`, no such contract clauses, and the mutation 21 unit declares no protocol profile alias. Rewritten over `R`, every one of those names refuses `missing_declaration`/`missing-name` in `Fill` itself, so the unit carries refusals besides the expected `expression-form` one at the row and the vector cannot tell them apart. The rest is sound: the row now names a local protocol, which FR-463 resolves; a protocol reference types as the clause's Boolean, so `ready = protocol Fill` conforms to `ready: Boolean`; and adding `ready` with its row leaves the other rows, the hidden-field set and the step rows unchanged. Write `Fill` out over `R` (for example one `attempt` of `R::Slot::put` with its own named pre/post clauses and a protocol profile alias selecting `quire.protocol.complete/v1`), or point at a fixture that already declares a protocol over `R`. | spec/test-cases/TC-310-refinement-declaration-admission.md:130-135,220-223; spec/test-cases/TC-376-two-branch-join-worked-example.md:22-38; spec/test-cases/TC-316-ring-buffer-data-refinement.md:22-40; spec/functional/protocol/FR-463-refer-to-a-declared-protocol-in-an-expression.md (Resolution; Type and value) |

## Dispositions

Round 1, against head 5a66bf9.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 5a66bf9: the fixture now declares `SubRing` and `next: Reference<Ring>`; case 7 (d) covers a reference to an unmapped object and (e) reads the relationship extent; expected results cite FR-377-AC-11 |
| FND-002 | fixed | 5a66bf9: the zero-budget run and the injected evaluator stop are gone; each run now uses an `only` that is undefined at the held state and the `SubRing` object. The `also` row's scope across the runs is now ambiguous: see new FND-004 |
| FND-003 | fixed | 5a66bf9: AC-17 and TC-310 mutation 20 now expect `missing_declaration`/`missing-name` at `Some` with no function declared and a call with one |

Round 2, against head 1f15551.

| FND | outcome | reason |
| --- | --- | --- |
| FND-004 | fixed | 1f15551: `also` is now added only in run (c), which drops the `SubRing` object; ring `a` and `q` in (d) have `head = 0`, `size = 0`; expected results for (c) name object `r` |

Round 3, against head 7e85fb7.

| FND | outcome | reason |
| --- | --- | --- |
| FND-005 | fixed | 7e85fb7: AC-18, FR-001-AC-7, FR-090-AC-10 and TC-310 mutation 21 now name the form (`eventually`) and the layer (`quire.model.complete/v1`). The chosen form cannot reach the layer check: see new FND-006 |

Round 4, against head 73aad4e.

| FND | outcome | reason |
| --- | --- | --- |
| FND-006 | fixed | 73aad4e: the example is now `protocol Spec::Twice`, a `primary` the grammar admits only under `quire.protocol.complete/v1`, so the row parses and the form is outside the model.complete closure. The name does not resolve in the mutation 21 unit: see new FND-007 |

Round 5, against head f52e145.

| FND | outcome | reason |
| --- | --- | --- |
| FND-007 | fixed | f52e145: the row is now `ready = protocol Fill`, naming a protocol the unit declares, in a Boolean field, so FR-463 resolution and typing no longer produce their own refusals. The `Fill` body itself is not constructible over `R`: see new FND-008 |
