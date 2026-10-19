---
id: FR-147
title: "Evaluate mathematical and bounded integer division"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-007
    type: references
  - target: ix://agent-ix/quire-specification/FR-044
    type: references
  - target: ix://agent-ix/quire-specification/AD-005
    type: references
  - target: ix://agent-ix/quire-specification/FR-349
    type: references
  - target: ix://agent-ix/quire-specification/FR-149
    type: references
---

# FR-147: Evaluate mathematical and bounded integer division

## Description

When `quire.value.complete/v1` admits an integer `div`, `rem` or `mod` expression,
the evaluator SHALL apply its exact selected quotient/remainder law and result
domain.

## Inputs

Integer type, operands, selected Euclidean, floor or truncating division profile and
consumer-domain bounds.

## Outputs

A completed exact integer: the quotient for `div`, the remainder for `rem`
and the Euclidean remainder for `mod`. Otherwise an undefined, refused or
incomplete evaluator outcome. I13 `requires-bound` remains a separate provider
negotiation disposition.

## Behavior

Mathematical integers are unbounded language values. `div`/`rem` use the
selected law and satisfy its quotient identity; operator spellings cannot mix
laws. Bounded consumers prove operand membership and the membership of the one
member the operator exposes. Division by zero is undefined and no backend may
saturate or substitute float division.

The shared grammar recognizes `div`, `rem` and `mod` at product precedence, but
semantic admission requires the declaration's `quire.value.complete/v1` facet.
Without it, each occurrence is a located `unsupported_construct` refusal even
inside an unreachable branch. `/` never denotes integer division; rational `/`
follows FR-044 and FR-149. The required `div`/`rem` law is selected by the
checked package, never inferred from operator spelling or backend availability.

## Division profiles

For dividend `a`, nonzero divisor `b`, quotient `q` and remainder `r`, every
profile satisfies `a = b*q + r` and the following additional law:

| Profile | Quotient | Remainder constraint |
| --- | --- | --- |
| truncating | `q` is the exact rational quotient rounded toward zero | `abs(r) < abs(b)` and a nonzero `r` has the sign of `a` |
| floor | `q` is the exact rational quotient rounded toward negative infinity | `abs(r) < abs(b)` and a nonzero `r` has the sign of `b` |
| Euclidean | `q` is the unique integer satisfying the remainder constraint | `0 <= r < abs(b)` |

For every nonzero divisor, `a div b` and `a rem b` compute the exact `q` and
`r` of the selected profile, and `mod` computes the exact Euclidean `q` and
`r`. Each operator exposes one member: `div` exposes `q`, `rem` exposes `r` and
`mod` exposes the Euclidean `r`. Mathematical-integer evaluation has no
overflow. A bounded consumer checks only the exposed member against its domain;
the unexposed member is never checked, retained or exposed, so it cannot cause
a refusal. Under every profile:

| Operator | Member checked | Refusal when that member is outside the domain |
| --- | --- | --- |
| `div` | `q` of the selected profile | `refused { code: division_out_of_domain, cause: quotient-outside-domain }` |
| `rem` | `r` of the selected profile | `refused { code: division_out_of_domain, cause: remainder-outside-domain }` |
| `mod` | Euclidean `r` | `refused { code: modulo_out_of_domain, cause: outside-domain }` |

A refusal exposes no member. The signed minimum divided by negative one is a
`div` whose quotient lies outside a signed-64 domain; its `rem` is `0` and
completes in that domain. `mod` names only the Euclidean remainder and cannot
inherit a truncating or floor profile.

The checked package retains exactly one of
`quire.value.integer-division.truncating/v1`,
`quire.value.integer-division.floor/v1` or
`quire.value.integer-division.euclidean/v1` for `div`/`rem`; absence or multiple
selections refuses semantic admission. `mod` always selects Euclidean remainder.
Before evaluation, a
finite I13 consumer without complete operand, intermediate and result bounds
returns the per-item `requires-bound` negotiation disposition without narrowing
mathematical integers. That provider disposition is not an evaluator outcome.
`div` and `rem` are each a single-result operation charged at exactly
`integer-division.operands`, `integer-division.arithmetic`,
`integer-division.domain` and `integer-division.result-retain`, in that order,
with the measures defined by `quire.value.accounting/v1`. `mod` is a single-result operation charged at exactly
`integer-modulus.operands`, `integer-modulus.arithmetic`,
`integer-modulus.domain` and `integer-modulus.result-retain`, in that order,
with the measures defined by `quire.value.accounting/v1`. For `div`/`rem` and
`mod`, the zero-divisor undefined check follows the operands charge and
precedes the arithmetic charge; an undefined outcome makes no later charge.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-147-AC-1 | Positive and negative operand vectors give a `div` quotient and a `rem` remainder that together satisfy the selected profile's identity `a = b*q + r` exactly. | Test (TC-192) |
| FR-147-AC-2 | Division by zero is undefined and produces no numeric value. | Test (TC-192) |
| FR-147-AC-3 | I13 negotiation for a finite backend lacking a complete derived domain returns `requires-bound` instead of invoking evaluation or narrowing mathematical integers. | Test (TC-192) |
| FR-147-AC-4 | Positive and negative dividend/divisor vectors distinguish truncating, floor and Euclidean results while preserving `a = b*q + r`. | Test (TC-192) |
| FR-147-AC-5 | `mod` returns the Euclidean remainder independently of the selected `div`/`rem` law; an attempted explicit non-Euclidean `mod` selection is refused, while every zero divisor, including `mod`'s, is undefined after the operands charge and before the arithmetic charge, without emitting a quotient or remainder. `mod` exact-bound accounting consumes its four named `integer-modulus` charges, and denial of any of them returns incomplete without a remainder. | Test (TC-192) |
| FR-147-AC-6 | Exact-bound accounting of `div` and of `rem` consumes their four named `integer-division` charges and succeeds, and denial of a named next integer-division charge returns incomplete with no quotient or remainder; a zero divisor is undefined after the operands charge and before the arithmetic charge. | Test (TC-192) |
| FR-147-AC-7 | In a locked qualification run with a registered SMT backend advertising mathematical-integer semantics, the corpus settles at least one integer-division obligation `proved` through that backend via the I13 negotiation path. | Test (TC-192, TC-266) |
| FR-147-AC-8 | In that same locked run, the corpus settles at least one integer-division obligation `refuted` through that backend via the I13 negotiation path; the AC-3 `requires-bound` finite-backend negotiation disposition is unchanged in a locked run with no such backend registered. | Test (TC-192, TC-266) |
| FR-147-AC-9 | Under each profile, a bounded consumer's `div` whose quotient is outside its domain returns `refused { code: division_out_of_domain, cause: quotient-outside-domain }`, a `rem` whose remainder is outside it returns `refused { code: division_out_of_domain, cause: remainder-outside-domain }`, and a `mod` whose Euclidean remainder is outside it returns `refused { code: modulo_out_of_domain, cause: outside-domain }`; no refusal exposes a member. | Test (TC-192) |
| FR-147-AC-10 | The unexposed member never refuses: over the consumer domain `Int[1,10]`, `10 div y` at `y = 5` returns `2` although the remainder `0` is outside the domain, and over `Int[-10,5]`, `x rem -1` at `x = -10` returns `0` although the quotient `10` is outside the domain. | Test (TC-192) |
| FR-147-AC-11 | In the shared grammar, `div`, `rem` and `mod` parse at product precedence in both a complete-value declaration and a declaration without `quire.value.complete/v1`; semantic admission accepts each well-typed complete-value form under its selected law, but returns a located `unsupported_construct` for each occurrence without that facet, including in an unreachable branch. `/` never becomes integer division; only under complete value may a rational `/` with an expected rational result type promote integer operands exactly under FR-149. | Test |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- [FR-349](../ir-proof/FR-349-register-smt-backend.md) supplies the
  registered SMT backend that discharges this requirement's `proved`/`refuted`
  cases when registered.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
