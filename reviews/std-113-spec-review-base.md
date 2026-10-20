---
id: SR-13418
title: "STD-113 quire-specification PR 296 base spec review of the Quantity type reference"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@f78981550 vs refs/remotes/origin/main 7c66d67; proposals/quire-v1/shared-grammar.md, spec/functional/type-model/FR-142-evaluate-quantities-and-units.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-187-quantity-and-unit-conversion.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-142
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-187
    type: reviews
---

## Summary

Ticket: STD-113. Base review of the spec-only PR that adds the `Quantity<qualified-name>`
type-ref form to the shared grammar, a "Quantity type reference" section and AC-11/AC-12
to FR-142, a forwarding sentence to FR-322's derived-type paragraph, and TC-187 rows U30/U31.

Independent verification of the "no wire/schema change" claim:

- Type annotations: holds. The V2 schema's `NodeRef` is untyped, a `value`/`parameter` node's
  `semantic_type` and an application's `result_type` may name any node, and FR-322 already
  gives a `scalar_type`/`unit` node the family `quantity` (FR-322:1297) and admits `quantity`
  in `any_value`. The kernel `Quantity { magnitude: Rational, unit: UnitId }` with the
  declared arm `UnitId::declared(NodeKey)` (quire-exact src/quantity.rs:25, src/identity.rs:145)
  matches "the kernel quantities with an exact rational magnitude and the unit id of `u`".
  QSV FR-124 (merged, origin/main 9270383) needs no change. QSL FR-094 "Quantity type nodes"
  already keys a declared-unit quantity type as the unit node itself.
- Quantity literals: does not hold (FND-001).
- Domain-package member types: does not hold (FND-002).

Gates: `quire validate` on the four touched files gives the same results at head and at
main (FR-142 and TC-187 pass; FR-322 and shared-grammar.md fail identically on both, for
pre-existing reasons). `cargo test --test checked_package_v2 --test backend_provider_v1`:
108 + 11 passed. `quire matrix`: the only change is two new untagged rows, FR-142-AC-11 and
AC-12, the same as the other untagged FR-142 checker ACs.

code-review was not run: the diff contains no code.

## Examined scope

- FR-142#quantity-type-reference (examined): Complete-V1 source names the type of a quantity in a declared unit as `Quantity<u>` ... Units and dimensions are two namespaces, and `Quantity<...>` is the one type form that looks up the unit namespace. (five bullets: denotation/no node; literal typing and type-mismatch; missing-name; no magnitude bound; no `Quantity<D>`)
- FR-142-AC-11 (examined): With the dimension `T` and units `s` (root) and `ms` (target `s`, scale `1/1000`), a parameter `d: Quantity<ms>` admits the literal `3 ms` and refuses `3 s` with `ill_typed`/`type-mismatch` at the literal; `d: Quantity<T>` and `d: Quantity<nothing>` each refuse `missing_declaration`/`missing-name` at the name.
- FR-142-AC-12 (examined): A function `f(d: Quantity<ms>): Quantity<ms>` returning `d` checks; returning `d` converted to `s` as `Quantity<ms>` refuses `ill_typed`/`type-mismatch`; and `Quantity<ms>` annotations introduce no checked-package node, naming the node of `ms`.
- FR-322#derived-type-forms-and-forwarding (examined): A `Quantity<u>` annotation likewise introduces no node: it names the `scalar_type`/`unit` node of `u` ... (FR-142).
- shared-grammar type-ref (examined): `| 'Quantity', '<', qualified-name, '>'`
- TC-187 U30, U31 (examined)
- FR-322 literal family admission, FR-322:1349-1352 (context_only)
- FR-322 "Model-owned members" step 4, FR-322:830-866 (context_only)
- shared-grammar complete-V1 reserved words, shared-grammar.md:72-91, and "Native declaration names are unique within the linked package", shared-grammar.md:960-962 (context_only)
- QSL FR-056, FR-091 "Dimension and unit declarations", FR-093, FR-094 "Quantity type nodes" at quire-spec-language refs/remotes/origin/main 812c954 (context_only)
- QSV FR-124 at quire-semantic-value origin/main 9270383; quire-exact Quantity/UnitId (context_only)

AC-11 and AC-12 checked by hand: `ms` targeting `s` with scale `1/1000` is a valid non-root
unit of `T` (`s = 1/1000 × ms`); `3 s` is a declared quantity in another unit of the same
dimension, so it refuses under the type-mismatch bullet; `T` is a dimension and `nothing` is
undeclared, so both refuse under the missing-name bullet; AC-12's converted result has type
`Quantity<s>`, not `Quantity<ms>`, so it refuses. Both ACs follow from the section's text.

## Verdict

**Not mergeable as is.** The type-annotation half of the claim holds, but the PR also states a
literal typing rule and names parameter types that the checked package and FR-322 cannot carry
today, while claiming no wire change. One high and four medium findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | A quantity literal has no checked-package form, so the "no wire change" claim does not hold for the literal rule. FR-142 says a literal `n u` has type `Quantity<u>`, and AC-11/U30 admit `3 ms`. FR-322 admits a `literal` term only when its `value_kind` admits the type's family, and no `value_kind` admits `quantity` (FR-322:1349-1352). The reference reader does the same: tests/checked_package_v2.rs:1094-1108 returns no descriptor for a literal typed by a unit node. QSL FR-093 has no quantity-literal lowering row either, although QSL FR-094 calls the unit node the quantity's "literal `type`". Any lowered body holding `3 ms` is therefore refused `operator-ineligible`. Either add a `value_kind`-to-`quantity` admission (FR-322 text and the reader) with a vector, or define how a quantity literal lowers, and drop the "no ... changes" claim. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:161-172; spec/objects/interfaces/FR-322-checked-package-artifact.md:1349-1352; tests/checked_package_v2.rs:1094-1108 |
| FND-002 | medium | A model or domain-package member cannot have a quantity type. FR-322 "Model-owned members" step 4 maps a member `typeRef` only through its table (natives, Int/Text value types, records, variants, object types). It refuses every other declared type `ill_typed`/`operator-ineligible`, and the table has no unit row. The semantic-IR domain package has no unit declaration a `typeRef` could name. In-repo TC-350 PD-01 (`Server::attempt(d)`, a model operation parameter) needs a quantity parameter, and QSL PR #1086's FR-185-AC-8 and its "QSL intake FR-056 resolving a domain-package parameter typeRef that names a unit" follow-up assume one. They need a new `typeRef` target form and an FR-322 step-4 row. The PR neither adds them nor bounds its scope to native source declarations. | spec/objects/interfaces/FR-322-checked-package-artifact.md:830-866; spec/test-cases/TC-350-probabilistic-declarations.md:26 |
| FND-003 | medium | FR-142 says "Units and dimensions are two namespaces", but QSpec defines no namespaces. shared-grammar.md:960-962 says "Native declaration names are unique within the linked package", which reads as one namespace for every native declaration. The separate-namespace rule exists only in QSL FR-091. The PR does not say whether a unit may share a name with a dimension, a record, an alias or an enum, or what a bare type-ref naming a unit (`d: ms`) does. "The one type form that looks up the unit namespace" implies `missing-name` but never says so. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:156-159; proposals/quire-v1/shared-grammar.md:960-962 |
| FND-004 | medium | `Quantity` is not added to the complete-V1 reserved words, unlike `Reference` and `Population` (shared-grammar.md:72-75). It stays an `ident`, so `record Quantity {...}` or `type Quantity = ...` is legal. A type-ref `Quantity` then matches the `qualified-name` alternative while `Quantity<...>` matches the new one, which differs from how every other built-in type constructor is treated. Reserve `Quantity` in the complete-V1 facet list. | proposals/quire-v1/shared-grammar.md:72-75,316 |
| FND-005 | medium | FR-142 never states how source writes a conversion into a unit, yet AC-12 depends on "`d` converted to `s`". The grammar has only `convert<type-ref>(e)`, and since `Quantity<...>` is the only type form that looks up units, the spelling must be `convert<Quantity<s>>(d)`. On main, QSL FR-091 says units are used by name outside type forms, so implementers could write `convert<s>(d)`. State the conversion spelling in the section, and say which other type-ref positions (`Option<Quantity<u>>`, collections, record fields, aliases) take the form. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:167-172,198 |
| FND-006 | low | AC-11 says the parameter "admits the literal `3 ms`" but not the expression that supplies the literal (a call argument `f(3 ms)`, a `let` with annotation, or a random-parameter support). The locus "at the literal" therefore has no fixed host expression. Name the call form, as AC-12 does. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:197 |

## Evidence

- quire validate head/main: /private/tmp/claude-501/-Users-peter-dev/be6a486a-dbff-4734-b40c-5d7654187d9c/scratchpad/r296 (per-file runs; identical results)
- matrix: /private/tmp/claude-501/-Users-peter-dev/be6a486a-dbff-4734-b40c-5d7654187d9c/scratchpad/r296/m-head.tsv vs m-main.tsv
