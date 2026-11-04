---
id: FR-140
title: "Evaluate exact decimal values"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/AD-005
    type: references
---

# FR-140: Evaluate exact decimal values

## Description

When checking a decimal expression, the semantic kernel SHALL represent every
value as an exact signed coefficient and declared nonnegative scale.

## Inputs

Decimal types with coefficient/scale bounds, literals, operands and explicit
conversion requests.

## Outputs

A completed exact/rounded decimal with optional typed loss, or an undefined,
refused or incomplete evaluator outcome. I13 dispositions are not evaluator
outputs.

## Behavior

Literal spelling determines an exact coefficient/scale before normalization.
Equality compares mathematical values; declared representation constraints
remain validation conditions. Arithmetic derives exact intermediate scales and
requires a non-`exact` rounding mode only at a rounding step, when the exact
value is not an integer multiple of `10^-T` for target scale `T`; a value
outside the declared range refuses under every mode. No binary float is used as
an intermediate.

An omitted rounding spelling in a wire or semantic form selects strict `exact`; a source spelling never omits it. A successful rounded
operation carries `DecimalLoss { exact_numerator, exact_denominator,
rounded_coefficient, rounded_scale, mode }`; the exact rational difference is
therefore reconstructable and a message string is never the loss authority.
The exact rational is canonical: its denominator is positive, numerator and
denominator have greatest common divisor one, and zero is exactly `0/1`.
`exact_numerator/exact_denominator` is the exact mathematical value of the
operation before rounding, independent of the target scale; it is neither the
discarded difference nor a scaled coefficient-space intermediate.
Strict `exact`, division by zero, domain refusal and resource exhaustion carry
no decimal value and remain distinct outcomes.

## Decimal domain and operations

A decimal representation is (`coefficient: Integer`, `scale: u32`) and denotes
`coefficient × 10^-scale`. Mathematical normalization removes trailing decimal
zeros while `scale > 0`; every zero normalizes to (`0`, `0`). Equality and
ordering use normalized mathematical values. Source and result provenance retain
the pre-normalized coefficient/scale.

In `Decimal[lo, hi; smin, smax; mode]`, `lo` and `hi` are the inclusive
coefficient bounds, `smin` and `smax` are the inclusive scale bounds and `mode`
is the rounding spelling. In source the spelling is required: the source grammar's only `Decimal` form ends `; mode ]`, and a `Decimal[..]` written without a mode is a syntax refusal at parse, not an implied `exact`. The mode-free spelling `Decimal[lo, hi; smin, smax; exact]` is the domain notation (diagnostic payloads, FR-272) and the wire and semantic form of a type whose mode is omitted, which carries the default `exact`. The type is well formed only when
`lo <= hi`, `smin <= smax` and `smax <= u32::MAX`; any other declaration refuses
type checking with `refused { code: ill_typed }`. For a value `v` with
normalized representation (`c`, `s`), the membership scale is
`s* = max(s, smin)` and the membership coefficient is the exact integer
`c* = v × 10^s*`; `v` is a member exactly when `s* <= smax` and
`lo <= c* <= hi`. Membership is therefore a function of the mathematical value
only: equal values have equal membership, and a pre-normalized coefficient or
scale never decides it. This differs from `Rational` bounds, which check the
normalized numerator and denominator, because a decimal scale is
representational: `Decimal[0,10000;2,2;exact]` admits `1` and `1.1` as `1.00` and
`1.10`. Membership is decided without materializing `c*` (for example by
comparing digit counts and exact magnitudes), makes no accounting charge and is
total for every well-formed `smin` and `smax` up to `u32::MAX`. The target
scale of a result is `smax`.

Each operand's retained representation is its literal coefficient/scale as
written or, for a computed result, (`v × 10^T`, `T`) at that result's target
scale `T`. An FR-478 text-to-decimal conversion is the one computed result
retained as spelled, like a literal: a spelled scale at most `T` keeps its
spelled coefficient/scale, so `1.5` into `Decimal[0,1000;0,2;exact]` retains
(`15`, `1`), and only a spelled scale above `T` is retained at `T`; operations and accounting use retained representations, never a
normalized form. Addition/subtraction align to the greater operand scale using exact powers of
ten. Multiplication multiplies coefficients and adds scales. Unary negation
negates the coefficient. Division by a divisor whose normalized coefficient is
zero is undefined. Otherwise division into target scale `T` forms the exact
rational `N/D`, where `N = dividend coefficient × 10^max(0, T + divisor scale -
dividend scale)` and `D = divisor coefficient × 10^max(0, dividend scale -
divisor scale - T)`; `N/D` is the exact quotient in units of `10^-T`.
A rounding step occurs exactly when the exact mathematical result is not an
integer multiple of `10^-T`, that is, when at least one discarded digit is
nonzero; discarded digits that are all zero are not a rounding step, select no
rounding mode and produce no `DecimalLoss`. At a rounding step, `exact`
refuses as `refused { code: inexact_decimal, cause: nonzero-discarded-digit }`,
while `toward-zero`, `toward-positive`, `toward-negative`,
`nearest-even` and `nearest-away` produce a coefficient at scale `T`. Nearest
modes compare twice the discarded magnitude to one target unit; ties choose an
even coefficient or the greater absolute coefficient respectively. Every result
is kept at its retained representation (`v × 10^T`, `T`) after any rounding
step and then checked for membership by value; a
nonmember, other than a `sum` seed or running total, refuses as
`refused { code: decimal_out_of_domain, cause: outside-domain }` with no value
and is never re-rounded to a coarser scale,
clamped or widened. A `sum` seed or running total is not the expression's
result: FR-145 requires every running total to lie in `N`'s domain, so one
outside it leaves the fold with no value in `N` even when a later summand would
bring the total back inside, and FR-145 makes that a located undefined outcome,
not a refusal.

An explicit conversion of a `Rational[n1,n2;d1,d2]` value with `d2 > 1`, reduced
`n/d`, into `Decimal[c1,c2;s1,s2;m]` is exactly the division of `(n, 0)` by
`(d, 0)` into target scale `T = s2` under mode `m`: `N = n × 10^T`, `D = d`, the
same rounding step, `DecimalLoss` (whose exact value is `n/d`) and membership
rule, and the accounting decimal division schedule. A rational source whose
denominator bound is one converts exactly as the integer `n`. The rounding
conversion is lossy, so FR-149 never admits it as an equality conversion. FR-149
classifies it, and every decimal-to-decimal conversion to a smaller maximum
scale, as a scale reduction (`quire.op.numeric.convert_rounding`), the only
exact-numeric conversion that rounds; an FR-478 text-to-decimal conversion
rounds by this same rounding step under a non-`exact` target mode. A conversion that changes only the admitted range, such
as `Int[0,300]` into `Decimal[0,20000;2,2;exact]`, is an FR-149 range narrowing
(`quire.op.numeric.narrow`): its membership is an FR-146 obligation, and it
never rounds or clamps.

Decimal typing derives no default type. The operands of `+`, `-`, `*`, `/` and
unary `-` and the result have one named `Decimal[..]` type, whose `smax` is the
target scale; operands of distinct decimal types are
`refused { code: ill_typed, cause: type-mismatch }` until an explicit
`convert` selects one. A `decimal(c,s)` literal takes exactly its unique
expected type: the other operand's static type when that operand is not a
literal, a declared field, parameter, result or annotated type of its position,
a `convert` target, or the unique expected type of the enclosing decimal
operation, which that operation passes to its operands. A literal without one, such as `decimal(15,1) +
decimal(225,2)` as an unannotated `let` right side or both operands of `=`, is
`refused { code: ill_typed, cause: ambiguous-literal }`; no coefficient range,
scale, rounding mode or target scale is inferred from the literal spellings.

Operand membership applies to source operands. A `decimal(c,s)` literal that
is not a member of its unique expected type under the membership rule above, for
example `decimal(150,2)` in a type whose `smax` is 0, is
`refused { code: ill_typed, cause: type-mismatch }` at type checking, the
literal's value needing a scale or coefficient its expected type does not
admit, before any accounting charge and with no value; membership charges
nothing. (`decimal_out_of_domain` stays the evaluator's value refusal for a
computed decimal result.) No source form feeds an operation an operand outside the shared type.
A retained operand may nevertheless carry a scale above the target scale `T`
when it is a member whose normalized scale is at most `smax` (for example
`decimal(100,2)` in a scale-0 type), so the accounting behavior for retained
operands wider than the target is expressible in source. TC-185 also names
evaluator-boundary vectors that supply retained operands to the evaluator
directly; each such vector states that it is not a source form and names its
source-admitted counterpart. A `Rational` to `Decimal` conversion in source is
`convert<R>(q)` on a `Rational`-typed value, not a decimal division of integer
literals, because the integer operands of that division need not be members of
`R`.

Under `quire.value.accounting/v1`, the evaluator charges the named decimal
points and counters before a power-of-ten expansion, coefficient operation or
result retention. The first unavailable charge returns incomplete accounting
without attempting a smaller or floating approximation. The zero-divisor
undefined check follows `decimal.operands` and precedes
`decimal.scale-expansion`; strict `exact` refusal at a rounding step follows
`decimal.arithmetic` and precedes `decimal.rounding`; membership refusal
follows `decimal.rounding`, or `decimal.arithmetic` when no rounding step
occurs, and precedes `decimal.result-retain`. Undefined and refused outcomes
make no later charge.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-140-AC-1 | `1.0` and `1.00` compare mathematically equal while retaining their declared representation provenance. | Test (TC-185) |
| FR-140-AC-2 | Exact representable arithmetic returns the mathematical result without rounding. | Test (TC-185) |
| FR-140-AC-3 | A result that is not an integer multiple of `10^-T` refuses under `exact` with `inexact_decimal`/`nonzero-discarded-digit`, while a non-`exact` mode returns the rounded value plus a canonical-rational loss record; a result outside the declared range refuses under every mode. | Test (TC-185) |
| FR-140-AC-4 | The six rounding modes produce their declared result on positive and negative half-way values and `exact` refuses every nonzero discarded digit. | Test (TC-185) |
| FR-140-AC-5 | Division by zero is undefined, while a value whose membership coefficient `c*` or membership scale `s*` is outside the target domain is refused with `decimal_out_of_domain`/`outside-domain`, except a `sum` seed or running total, which is FR-145's located undefined outcome; neither produces a value or uses a floating intermediate. A declaration with `lo > hi`, `smin > smax` or `smax > u32::MAX` refuses type checking with `ill_typed`. Membership charges nothing and is total up to scale `u32::MAX`. | Test (TC-185, TC-233) |
| FR-140-AC-6 | Exact-bound accounting succeeds and denial of a named next decimal charge returns incomplete without a value or an implementation-specific retry; zero-divisor, strict-`exact` and membership outcomes occur at their defined charge positions, and all-zero discarded digits make no `decimal.rounding` charge. | Test (TC-185) |
| FR-140-AC-7 | A `Rational` to `Decimal` conversion follows the decimal division rule and schedule at target scale `smax`, decimal operands and result share one named type, and a `decimal(c,s)` literal without a unique expected type is `ill_typed` with cause `ambiguous-literal`. | Test (TC-185) |
| FR-140-AC-8 | A `decimal(c,s)` source literal that is not a member of its unique expected type (`decimal(150,2)` in `Decimal[-9,9;0,0;exact]`, `decimal(1234,3)` in `Decimal[-1000,1000;0,1;exact]`) is `refused { code: ill_typed, cause: type-mismatch }` at type checking before any charge, while a member literal spelled at a scale above `smax` (`decimal(100,2)` in `Decimal[-9,9;0,0;exact]`) is admitted and retained as spelled. | Test (TC-185) |
| FR-140-AC-9 | A source `Decimal[lo, hi; smin, smax]` written without a rounding mode (as a type annotation, a `convert` target or a `Quantity` carrier) is a syntax refusal at parse and reaches no type check, while `Decimal[lo, hi; smin, smax; exact]` checks; the mode-free spelling stays the domain notation of diagnostic payloads and denotes mode `exact` in a wire or semantic form that omits it. | Test (TC-185) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
