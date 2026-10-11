---
id: TC-192
title: "Integer division profiles"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-147
    type: verifies
---

# TC-192: Integer division profiles

## Description

Evaluate signed vectors, zero divisors and missing finite-backend domains under each division law.

## Test Procedure

Select language `ix:native`, edition `1-draft`, and the
`quire.value.complete/v1` definition. For `div`/`rem`,
select exactly one of the three integer-division
definitions in FR-147, through
[`complete-value-lock.json`](../../proposals/quire-v1/definitions/complete-value-lock.json).
For `(a,b)` in
`(7,3)`, `(7,-3)`, `(-7,3)` and `(-7,-3)`, evaluate `a div b` for `q` and
`a rem b` for `r` as two separate expressions and compare `(q,r)` with this
table:

| Profile | `(7,3)` | `(7,-3)` | `(-7,3)` | `(-7,-3)` |
| --- | --- | --- | --- | --- |
| truncating | `(2,1)` | `(-2,1)` | `(-2,-1)` | `(2,-1)` |
| floor | `(2,1)` | `(-3,-2)` | `(-3,2)` | `(2,-1)` |
| Euclidean | `(2,1)` | `(-2,1)` | `(-3,2)` | `(3,2)` |

For every cell, independently assert `a = b*q + r`, the profile's remainder
sign/range law, and that each expression completes with exactly its one
member. Then execute:

| Vector | Operation/domain | Expected value/disposition |
| --- | --- | --- |
| DIV-01 | Each profile's `div` and `rem` with divisor zero | undefined; neither quotient nor remainder is present |
| DIV-02 | `mod(-7,3)` while each of the three `div`/`rem` definitions is selected | `2` every time because `mod` is independently Euclidean |
| DIV-03 | Mutate the checked definition closure so `mod` claims the truncating or floor definition | semantic admission returns `refused { code: invalid_package, cause: incompatible-definition }`; no expression is evaluated |
| DIV-04 | Mathematical `(-9223372036854775808) div -1` and `(-9223372036854775808) rem -1` | exact `9223372036854775808` and `0` |
| DIV-05 | DIV-04 in signed-64 consumer domain, under each profile | the `div` returns `refused { code: division_out_of_domain, cause: quotient-outside-domain }` and exposes no quotient; the `rem` completes with `0` because the unexposed quotient is not checked |
| DIV-06 | Signed-64 `MIN div 1`, `MAX div 1`, then `MIN div -1`, whose quotient is one over `MAX` | endpoints succeed; the out-of-domain quotient returns `refused { code: division_out_of_domain, cause: quotient-outside-domain }` |
| DIV-07 | At I13 negotiation, omit operand, intermediate and result bounds in three separate finite-consumer requests | each returns per-item `requires-bound`; evaluation is not invoked |
| DIV-08 | Run truncating `7 div 3` and, separately, `7 rem 3`, each with `ScalarLimitsV1 { integer_bits: 3, decimal_digits: 0, scale_expansion: 0, text_input_bytes: 0, text_scalars: 0, normalized_scalars: 0, unit_edges: 0, value_occurrences: 2, work_units: 4, result_units: 1 }`; then deny `integer-division.result-retain` for each | the exact tuple returns `2` and `1` respectively, each after exactly `integer-division.operands`, `integer-division.arithmetic`, `integer-division.domain` and `integer-division.result-retain`; each denied final charge is exactly `incomplete { limit_kind: work_units, limit: 3, consumed: 3, next_charge: 1, charge_point: integer-division.result-retain }` with no quotient or remainder |
| DIV-09 | Omit the division definition, select two laws, or substitute another definition identity | semantic admission returns `refused { code: invalid_package }` before evaluation |
| DIV-10 | Run `mod(-7,3)` with `ScalarLimitsV1 { integer_bits: 3, decimal_digits: 0, scale_expansion: 0, text_input_bytes: 0, text_scalars: 0, normalized_scalars: 0, unit_edges: 0, value_occurrences: 2, work_units: 4, result_units: 1 }`; then deny each of `integer-modulus.operands`, `integer-modulus.arithmetic`, `integer-modulus.domain` and `integer-modulus.result-retain` in turn | the exact tuple returns `2` after exactly those four charges and no `integer-division.*` charge; the denials return `incomplete { limit_kind: work_units, limit: k, consumed: k, next_charge: 1, charge_point: <denied point> }` for `k` = 0, 1, 2 and 3 respectively, with no remainder |
| DIV-11 | Run truncating `7 div 0` with DIV-08's tuple and `mod(7,0)` with DIV-10's tuple, each with `work_units: 1`; then each with `work_units: 0` | `work_units: 1`: undefined after the operands charge, with no arithmetic charge; `work_units: 0`: `incomplete { limit_kind: work_units, limit: 0, consumed: 0, next_charge: 1, charge_point: integer-division.operands }` and `integer-modulus.operands` respectively |
| DIV-12 | Run `mod(-7,3)` for a bounded consumer result domain `Int[0,1]` with DIV-10's tuple except `result_units: 0`; then `work_units: 2` | `refused { code: modulo_out_of_domain, cause: outside-domain }` after `integer-modulus.domain` because remainder `2` is outside `[0,1]`; `integer-modulus.result-retain` is not charged, so it is not incomplete. With `work_units: 2`: `incomplete { limit_kind: work_units, limit: 2, consumed: 2, next_charge: 1, charge_point: integer-modulus.domain }`. (A signed-64 domain cannot refuse the Euclidean remainder of signed-64 operands, so this bounded domain is narrower.) |
| DIV-13 | Run DIV-08 with `integer_bits: 2` and `work_units: 0` | both counters are short at `integer-division.operands`; the first in `ScalarLimitsV1` field order is reported: `incomplete { limit_kind: integer_bits, limit: 2, consumed: 0, next_charge: 3, charge_point: integer-division.operands }` |
| DIV-14 | With the FR-349 SMT backend registered advertising mathematical-integer semantics, submit one mathematical-integer division obligation that holds and one that a mutated divisor domain violates; I13 negotiation returns the `supported` disposition for both | negotiation's `supported` disposition is not itself a verdict: the backend then evaluates the holding obligation to the verdict `proved` and the violated obligation to `refuted` with a replayable counterexample; mathematical integers are never narrowed to reach either verdict |
| DIV-15 | Under each profile, over consumer domain `Int[1,10]`, run `10 div 3` and `10 rem 7`; over `Int[1,10]`, run `10 rem 5` | `10 div 3` returns `3`; `10 rem 7` returns `3`; `10 rem 5` returns `refused { code: division_out_of_domain, cause: remainder-outside-domain }` after `integer-division.domain` because remainder `0` is outside `[1,10]`, with no remainder exposed and no `integer-division.result-retain` charge |
| DIV-16 | Under each profile, over consumer domain `Int[1,10]`, run `10 div y` at `y = 5`; over `Int[-10,5]`, run `x rem -1` at `x = -10` | `10 div y` returns `2`, not a refusal, although the unexposed remainder `0` is outside `[1,10]`; `x rem -1` returns `0`, not a refusal, although the unexposed quotient `10` is outside `[-10,5]` |
| DIV-17 | Parse `1 + 7 div 3 * 2`, `7 rem 3`, and `-7 mod 3` in declarations selecting `quire.value.complete/v1`, and repeat each source in a declaration selecting only `quire.state.core/v1` | All parse with product precedence and original operator spans. Under the complete facet and one selected division law, `div`/`rem` use that law and `mod` returns Euclidean `2`; under the state-only facet, each operator receives a located `unsupported_construct` before evaluation. |
| DIV-18 | Place each of `div`, `rem` and `mod` in the dead branch of `if true then 1 else (7 <operator> 3)` under the state-only facet; separately use `7 / 3` with and without expected result type `Rational[0,7;1,3]` under state-only and complete-value selections | Each dead-branch integer operator is still a located `unsupported_construct`; `/` never performs integer division. State-only refuses integer operands even with a rational expectation. Complete value refuses without that expectation; with it, FR-149 promotes the operands to `7/1` and `3/1` and FR-044 admits exact result `7/3` after nonzero/domain checks. No expression is silently reinterpreted as `div`. |

Tag the profile-admission vectors DIV-17 and DIV-18 with
`#[trace("FR-147-AC-11", "TC-192")]`; DIV-18 additionally carries
`#[trace("FR-044-AC-3", "FR-044-AC-9", "TC-192")]` when implemented. These
are planned test bindings, not executed results.

Generate bounded signed operand pairs for all three laws, emphasizing zero,
unit divisors, sign quadrants and finite-domain endpoints. The oracle computes
the unique exact `(q,r)` satisfying `a = b*q + r` plus the selected remainder
law, then checks only the exposed member: `q` for `div`, `r` for `rem`; `mod`
always uses the Euclidean remainder. Generate cases where only the unexposed
member is outside the consumer domain and require completion. Shrink operand magnitude while
preserving divisor-zero, sign quadrant, domain overflow or selected-law
distinction. Deny each applicable named integer-division charge independently
and require incomplete output with no member.

## Expected Results

Every signed vector produces the tabled pair and satisfies its law. Zero,
profile misuse, missing bounds, finite-domain overflow and exhaustion remain
distinct typed dispositions. Each completed `div`, `rem` or `mod` exposes
exactly its one member; only that member's domain membership can refuse, with
cause `quotient-outside-domain` for `div` and `remainder-outside-domain` for
`rem`. No failing case exposes a member, uses host overflow or substitutes
floating-point division; independent sibling results remain intact. I13 negotiation yields a disposition
(`supported`/`requires-bound`/`unsupported`/`invalid-input`), never a verdict;
in a locked run with the FR-349 SMT backend registered, negotiation returns
`supported` and at least one mathematical-integer division obligation then
settles the verdict `proved` and at least one settles `refuted` through it;
the I13 `requires-bound` finite-backend negotiation disposition is unchanged
in a locked run with no such backend registered.
