---
id: SR-12104
title: "Base review of QSpec PR #273: FR-149 arithmetic operand against a bounded integer"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@09bf86dc79fec4c70b36f57e18bbe6b92599e3ec; PR #273 diff against origin/main: spec/functional/type-model/FR-149-apply-complete-equality-matrix.md, spec/test-cases/TC-194-complete-equality-matrix.md; context FR-044, FR-146, FR-206, FR-322, TC-191 P09, proposals/state-core/state-semantics.md sections 3 and 6; QSL FR-381-AC-10, TC-1057 fixture R, TC-1058 rows 16 and 22, QSL PR #963 typing.rs admit_sum_into_peer and lowering.rs Coerce"
review_set: subset
---
# Base review of QSpec PR #273 (STD-157)

## Summary

Ticket: STD-157. The PR adds an FR-149 section that coerces a `+`, `-`, `*` or
unary `-` operand of `=`/`!=` into the other operand's `Int[lo,hi]` with a
range obligation, three ACs (FR-149-AC-19..21) and two TC-194 vectors (E33,
E34). Reviewed at head 09bf86d (one commit). Ran the base checklist; the
integrity analysis is in its own SpecReview (SR-12105).

Checked and clean:

- Ids. FR-149-AC-19..21 follow AC-18, and E33/E34 follow E32. No other open
  QSpec PR (#274, #267, #207, #200, #59) touches FR-149 or TC-194, so there
  is no id collision.
- Author question 1 (`tally`). QSL TC-1057 fixture R declares `Tally =
  Int[0, 3]`. The retry binders are `M::Attempt{tally: Tally}` and the
  recovery binder is `M::Receipt{..., tally: Tally}`. FR-322-AC-31 lowers an
  integer value type to its `Int[..]`, so `Int[0,3]` is right. The intervals
  `[1,3]` (guarded) and `[1,4]` (unguarded) and the refusal locus at
  `earlier.tally + 1` / `recovered.tally + 1` match TC-1058 rows 16 and 22.
- Soundness of the obligation. FR-146 comparison facts flow from
  `earlier.tally < 3` into the right operand of `and`, so the path refines to
  `[0,2]` and the sum to `[1,3]`, inside `[0,3]`. A proved narrowing never
  changes the value, and an unproved one refuses, so the coercion cannot turn
  a true comparison false or the reverse. It only replaces an `ill_typed`
  refusal with a proved check or an `unproved-range` refusal.
- Author question 2 (div/rem). Leaving out `div` and `rem` matches QSL. QSL's
  `admit_sum_into_peer` fires only on `NodeKind::Arithmetic`, which is `Add`,
  `Subtract` and `Multiply`. `div`/`rem` are `NodeKind::IntegerDivision`.
  FR-381-AC-10 itself only uses `+`. Unary `-` does not match (FND-002).
- Oracle strength against an absent exception. Without the exception, E33's
  first case gives `ill_typed`, not "checks", and its second and third give
  `ill_typed`, not `unproved-range`, so all three fail. An implementation that
  clamps, or that drops the obligation, fails E33's second and third cases.
  An implementation that coerces any `Integer` operand fails E34's path, call,
  `div` and `let` cases. One that also coerces under ordering fails E34's
  `<=` case.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The section's premise that "FR-146 types `Integer`" an `Int[..]` sum is stated nowhere in FR-146. QSpec's own typing text says the opposite: state-semantics section 3 gives arithmetic the operands' declared integer type, with the literal typed from the field. FR-044 and FR-206 say operands and results share their named numeric type. Under that rule, E34's `earlier.tally + 1 <= later.tally` and the `let` case are `unproved-range`, not `ill_typed`. | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md:175; spec/test-cases/TC-194-complete-equality-matrix.md:69; proposals/state-core/state-semantics.md:76; spec/functional/foundation/FR-044-check-composed-numeric-definedness.md:31 |
| FND-002 | medium | QSpec includes unary `-`, but QSL does not. QSL's `admit_sum_into_peer` matches only `NodeKind::Arithmetic` (`Add`/`Subtract`/`Multiply`), and integer unary `-` is `NodeKind::Negate`, typed `Integer`. So `later.tally = -x` checks under QSpec but QSL refuses it `ill_typed`. No vector exercises unary `-`, so neither side catches the gap. | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md:179-181; qsl-semantics/src/check/check/typing.rs:2105 (QSL main) |
| FND-003 | medium | The vectors only test `+`, only with `=`, and only with the arithmetic on the right. The FR's `-`, `*`, unary `-`, `!=`, arithmetic-on-the-left and parenthesized forms are untested, and so are its `rem`, `count` and `Decimal[..]` exclusions. An implementation that handles only `x = y + k` passes every row. | spec/test-cases/TC-194-complete-equality-matrix.md:68-69; spec/functional/type-model/FR-149-apply-complete-equality-matrix.md:309-311 |
| FND-004 | medium | Author question 3. The FR does not say how the checked package records the coercion. QSL lowers it to a `quire.op.numeric.narrow` application (`Operator::Convert`, member `Int[lo,hi]`, at the arithmetic's location). That node is the same one an explicit `convert<Int[lo,hi]>(e)` under `=` would produce, which this FR's own table refuses, because an unbounded `Integer` source converts only to `Integer`. A package consumer cannot tell them apart. | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md:181-183; spec/functional/type-model/FR-149-apply-complete-equality-matrix.md:155 |
| FND-005 | medium | It is unclear what "does not extend to ... function arguments, or to bindings" means. TC-191 P09 already checks `a * b` over `Int[0,3]` against a declared `Int[0,9]` result, with a range obligation. QSL's `Typer::coerce` range-coerces into every `Int[..]` slot, and QSL FR-381-AC-3 expects a capture `M::Tally = view.tally + 1` to give `unproved-range`. A reader cannot tell whether `let x: Int[0,3] = y + 1` is `ill_typed` or range-checked. | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md:198-199; spec/test-cases/TC-191-total-pure-function-checking.md:37 |
| FND-006 | low | "Writing `convert` around every such sum" and "ill_typed without an explicit `convert`" both suggest that `convert<Int[lo,hi]>` would be accepted. The table refuses that form. The only legal explicit spelling is `convert<Integer>` on the `Int[..]` side, and that carries no range obligation. | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md:175-176,195-196 |
| FND-007 | low | E34 expects `refused { code: ill_typed }` with no cause, unlike E31/E32. A refusal with the wrong cause passes. | spec/test-cases/TC-194-complete-equality-matrix.md:69 |
| FND-008 | low | E33 declares an inline `Int[0,3]` field rather than QSL fixture R's named `Tally = Int[0,3]`, so the alias path QSL actually exercises is not covered. | spec/test-cases/TC-194-complete-equality-matrix.md:68 |
| FND-009 | low | "Adds no charge beyond the equality schedule" does not say that the `+` still charges its own integer-arithmetic schedule (TC-190 Q12). AC-19 and E33 can be read as a five-unit total for the whole relation. | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md:186-187,309 |

## Verdict

Not mergeable as written. The integrity contradiction (SR-12105 FND-001)
must be resolved: either FR-044/FR-206 and FR-149's own "no implicit
conversion" sentences are amended, or the exception is rewritten as an
expected-type rule that does not contradict them. FND-001 (the typing
premise) and FND-002 (unary `-` against QSL) decide whether the spec actually
matches QSL. FND-003 to FND-005 are real gaps that should be fixed in the same
round. The lows are optional.
