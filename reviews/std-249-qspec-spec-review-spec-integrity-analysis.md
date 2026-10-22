---
id: SR-14030
title: 'Spec review of PR #330: binder record of an abstract node (STD-249), spec-integrity analysis'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification@2f0a09d; git diff origin/main...2f0a09d (PR #330, branch std-protocol-249-qspec, based on main, not stacked): spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md, spec/test-cases/TC-386-explicit-protocol-step-rows.md; ticket STD-249'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-432
  type: reviews
- target: ix://agent-ix/quire-specification/TC-386
  type: reviews
---

## Summary

Ticket: STD-249. PR: quire-specification#330, head 2f0a09d, based on main (not stacked). Method: spec-review with the spec-integrity sub-analysis. Tooling: quoin 0.28.3, quire 0.36.2 (engine 0.50.2), spec-artifacts-process@b82e42e.

The PR adds "The binder record of an abstract node" to FR-432 (fields per node kind, the arity refusal `ill_typed`/`operator-ineligible` and the type refusal `ill_typed`/`type-mismatch`), FR-432-AC-14/15 and TC-386 BR-01/02. It answers the ticket's two questions (attempt result; non-record binder) with the two lead defaults the PR body names, and both are buildable: each is a fixed field list the checker can compare an argument list against.

Gates at head: `quire validate --scope . spec/**/*.md` reports 134 failed documents, the count on main, none of them FR-432 or TC-386; `cargo test --test checked_package_v2 --test backend_provider_v1` passes (14 and 117 tests).

What node rows do today (QSL, read at 53cee90bd): `node_target` (qsl-semantics/src/check/refinement.rs:2130-2207) resolves only a three-segment name `<alias>::<protocol>::<node>` naming an attempt, effect, event, commit, send, receive or finish node, and refuses any other spelling `missing_declaration`/`missing-name`. It then types each written argument with no expected type, over `self` as the concrete receiver plus the concrete operation's parameters, and checks no count, as FR-147's old Remaining-work text said. The operation-application path (same file, 2209-2290) already refuses a wrong count `operator-ineligible` at the row and types the receiver and arguments against the abstract types after reference lifting. The grammar (qsl-cst/src/grammar.rs:2116-2123, and QSpec shared-grammar `protocol-step-target`) admits a node target with no list or with one or more `step-argument`s; an empty `()` is not admitted.

Vectors recomputed by hand: a1 = (receiver, x), 2 fields; a2 = (receiver, x, result), 3; ev = (p, q), 2; eb = (b), 1. `a1(self, 1)` 2/2, `a2(self, 1, _)` 3/3, `ev(true, 3)` 2/2 with 3 in Int[0, 9], `eb(true)` 1/1: counts and types agree with admission. `a1(self, 1, 2)` 3/2, `a1(self)` 1/2, `ev(true)` 1/2, `eb(true, false)` 2/1: each a count mismatch. `eb(1)`: Int for Boolean; `ev(true, true)`: Boolean for Int[0, 9]: each a type mismatch. FR-147-AC-14: inc has no parameter and no result, so a1 = (receiver), 1 field; `a1(self)`, `a1(_)` 1/1 and `a1` (no list) admitted; `a1(self, 1, 2)` 3/1; `a1(1)` Int for a reference. The counts are right; FND-001/002 (QSpec) and FND-001 (QSL) are about spellings that never reach the count check.

IDs: FR-432-AC-14/15 and BR-01/02 are unused on main and on every other open QSpec PR; PRs #328 and #329 edit FR-426 but not its binder record text. The QSL half (PR #1191) agrees with this rule, apart from the empty-list row it adds (see SR-14033). The PR body states gates, defaults and remaining work only; it paraphrases no review finding.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-432 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:21-25 | examined |
| FR-432 binder-record section | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:87-107 | examined |
| FR-432 deciding-row paragraph | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:68-71 | examined |
| FR-432-AC-14 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:235 | examined |
| FR-432-AC-15 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:236 | examined |
| TC-386 BR-01 | spec/test-cases/TC-386-explicit-protocol-step-rows.md:48 | examined |
| TC-386 BR-02 | spec/test-cases/TC-386-explicit-protocol-step-rows.md:49 | examined |
| TC-386 Test Procedure | spec/test-cases/TC-386-explicit-protocol-step-rows.md:20-21 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-432-AC-14 and TC-386 BR-01 write every row as `node A::a1(...)`, `node A::ev(...)` and so on, two segments, with `A` the protocol and no model alias. FR-432 gives the right side as `node <A>::<protocol>::<node>(…)`, IL-07 writes `node Spec::Full::a1(self)`, and the QSL checker resolves only `<alias>::<protocol>::<node>` (qsl-semantics/src/check/refinement.rs:2146-2168), refusing any other spelling `missing_declaration`/`missing-name`. So each row the AC says is admitted refuses `missing-name` as written, and the fixture names no abstract model alias or `abstract <alias>::A` header (TC-386's procedure fixes the header `abstract Spec::Req`, with only SR-09 excepted). Fix: name the model (for example `Spec`, with `Evt` as `Spec::Evt`), head the vector `abstract Spec::A`, write the rows `node Spec::A::a1(self, 1)` and so on, and add BR-01/BR-02 to the procedure's exceptions. | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:235 |
| FND-002 | high | FR-432-AC-15 and TC-386 BR-02 use the same two-segment spellings (`node A::a1(self, 1, 2)`, `node A::eb(1)` and the rest). Under FR-432's `node <A>::<protocol>::<node>` form and the checker's resolution (qsl-semantics/src/check/refinement.rs:2146-2168) each refuses `missing_declaration`/`missing-name` before any count or type check, so no row reaches the `operator-ineligible` or `type-mismatch` refusal the AC states. Fix with FND-001: three-segment names under a named abstract header. | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:236 |
| FND-003 | medium | The new section says "Each argument expression is typed against its field's type" and gives the attempt `receiver` field the reference type of the abstract operation's owner, but does not say what the expressions read or that reference lifting applies. FR-375's step rows state both for an application: arguments are typed against the abstract types after reference lifting, over the concrete receiver as `self`, the concrete arguments by name and the concrete pre-state, and the QSL application path does this (qsl-semantics/src/check/refinement.rs:2240-2266). Read literally, `self` in `node A::a1(self, 1)` is a concrete reference, not the abstract owner's reference type, and refuses `type-mismatch`, which contradicts FR-432-AC-14. Fix: state that each argument is typed against its field's type after reference lifting, over the same scope FR-375 gives an application's arguments. | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:87-107 |
| FND-004 | low | The deciding-row paragraph still says a node right side's "arguments are the abstract binder record's fields in declaration order, `_` leaving one open". The new section gives the order by node kind (an attempt's receiver, parameters, result is not a declared order) and admits a row with no argument list. Fix: point the sentence at "The binder record of an abstract node" instead of restating the order. | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:68-71 |
| FND-005 | low | The table's `event` row does not say that it covers `effect` and `commit` nodes. A right side may name them (the QSL checker admits Effect and Commit, refinement.rs:2153-2161, and FR-426's `event` step is taken at an effect, domain event or commit node), but the table names only node kinds `attempt`, `event`, `finish`, `send`, `receive`. Fix: write the row as `event` (an `effect`, `event` or `commit` node), `finish`. | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:87-107 |
| FND-006 | low | "the reference type of the operation's owner" does not say whether the owner is the type the attempt names (`on M::Cell::inc`) or the type that declares an inherited operation. The two give different receiver types when `inc` is declared on a supertype. The QSL application path uses the named type (`operation_named`, refinement.rs:2222). Fix: say "the reference type of the type the attempt's operation name names". | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:87-107 |

## Verdict

Not mergeable as written. FND-001 and FND-002 are high: every row of the new vectors uses a two-segment node name that the right-side form and the checker refuse as `missing-name`, so neither AC can hold. FND-003 is medium (the typing scope and reference lifting are unstated, which makes AC-14's `self` rows mistyped when read literally). FND-004 to FND-006 are low. The two lead defaults are consistent, leave no unbuildable AC on their own and match the ticket.

## Dispositions

Re-checked at head 3d88428 (fix commit 85c4afe, then a merge of origin/main). Gates at head 3d88428: `quire validate --scope . spec/**/*.md` reports 134 failed documents, none of them FR-432 or TC-386 (touched files add none); `cargo test --test checked_package_v2 --test backend_provider_v1` passes (117 and 14 tests). The head contains origin/main. FR-432-AC-14/15 and TC-386 BR-01/02 are used by no other open QSpec PR. The PR body states behaviour, defaults, gates and remaining work and paraphrases no review finding. Vectors recomputed at the new head: BR-01 a1 (receiver, x) 2/2, a2 (receiver, x, result) 3/3, ev (p, q) 2/2 with 3 in Int[0, 9], eb (b) 1/1, `a1` with no list admitted; BR-02 a1(self, 1, 2) expected 2 actual 3, a1(self) 2/1, ev(true) 2/1, eb(true, false) 1/2, all as written. FR-375's step-row text (FR-375:321-326) states the same scope the new sentence cites. No other spec file outside reviews/ contradicts the added effect and commit kinds: FR-426's step-kind tables list step kinds, and an effect or commit node is taken as an `event` step there.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fixed text at the new head: \| FR-432-AC-14 \| Over the abstract protocol `A` of the abstract model `Spec`, headed `abstract Spec::A`, with attempt `a1` on `twice(x: Int[0, 9])` (no result), attempt `a2` on `quad(x: Int[0, 9])` with result `Int[0, 99]`, event `ev as (e: Spec::Evt)` with `Evt{p: Boolean, q: Int[0, 9]}` and event `eb as (b: Boolean)`, the binder record of `a1` is (`receiver`, `x`), of `a2` is (`receiver`, `x`, `result`), of `ev` is (`p`, `q`) and of `eb` is the one field `b`; the rows `node Spec::A::a1(self, 1)`, `node Spec::A::a2(self, 1, _)`, `node Spec::A::ev(true, 3)`, `node Spec::A::eb(true)` and `no... |
| FND-002 | fixed | Fixed text at the new head: \| FR-432-AC-15 \| In the protocol of FR-432-AC-14, each refusal is `ill_typed`/`operator-ineligible` at the node retaining `expected` (the field count) and `actual` (the argument count): `node Spec::A::a1(self, 1, 2)` with 2 and 3, `node Spec::A::a1(self)` with 2 and 1, `node Spec::A::ev(true)` with 2 and 1, and `node Spec::A::eb(true, false)` with 1 and 2; `node Spec::A::eb(1)` refuses `ill_typed`/`type-mismatch` at the argument with `expected` `Boolean` and `actual` the type of `1`, and `node Spec::A::ev(true, true)` with `expected` `Int[0, 9]` and `actual` `Boolean`. \| Test (TC-386) \| |
| FND-003 | fixed | Fixed text at the new head: Each argument expression is typed against its field's type after reference lifting, over the scope FR-375 gives the arguments of an abstract-operation target: the concrete receiver as `self`, the concrete operation's arguments by parameter name and the concrete pre-state. An argument list |
| FND-004 | fixed | Fixed text at the new head: `node <A>::<protocol>::<node>(…)`, whose arguments are the node's binder record as stated in "The binder record of an abstract node"; `stutter`; or |
| FND-005 | fixed | Fixed text at the new head: \| `effect`, `event`, `commit`, `finish` \| when the node's binder type `T` is a record type, one field per field of `T`, in declaration order; otherwise exactly one field \| each field's declared type; otherwise `T` \| |
| FND-006 | fixed | Fixed text at the new head: \| `attempt` \| `receiver`; one field per parameter of the node's operation, in declaration order; `result`, present only when the operation declares a result \| the reference type of the type the attempt's operation name names (`on M::Cell::inc` names `M::Cell`, even when a supertype declares the operation); each parameter's declared type; the declared result type \| |
