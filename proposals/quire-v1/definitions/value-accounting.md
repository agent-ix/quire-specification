# Complete scalar accounting definition

Definition identity: `quire.value.accounting/v2`.
This is a leaf definition selected by `quire.value.complete/v1`; it has no
reverse dependency on that root.

Replacement draft: the complete-value and complete-model selected closures
must adopt this accounting artifact with the replacement closed model
identity schema, vectors and strict reader atomically. Existing closure bytes
do not select it. Scalar and population charge semantics below are unchanged;
model identity preparation and phase-4 preidentity order are amended explicitly.
Public limit-object names, members, incomplete records and API names remain
unchanged. This draft is not permission for an independently released meter,
mixed selected closure or implementation cutover.

## Limit object

Every complete-value request carries `ScalarLimitsV1` with exact unsigned
64-bit (`u64`) limits for `integer_bits`, `decimal_digits`, `scale_expansion`,
`text_input_bytes`, `text_scalars`, `normalized_scalars`, `unit_edges`,
`value_occurrences`, `work_units` and `result_units`. Omission is a refused
request. Every consumed counter is also an exact `u64`; a semantic size or next
charge greater than `u64::MAX` returns incomplete at that named counter before
narrowing, allocation or arithmetic. Zero is a real limit and never means
unlimited.

## Ordered charge points

The following identifiers and order are normative. A charge is made before the
named expansion, operation or retention. The first unavailable charge returns
`incomplete { limit_kind, limit, consumed, next_charge, charge_point }` and no
completed result. That record is closed; its members are defined under
[Incomplete record](#incomplete-record).

| Operation family | Ordered charge points |
| --- | --- |
| decimal | `decimal.operands`, `decimal.scale-expansion`, `decimal.arithmetic`, `decimal.rounding`, `decimal.result-retain` |
| text | `text.input-bytes`, `text.decode-scalars`, `text.normalize-input`, one `text.normalize-output` before each emitted scalar, `text.result-retain` |
| enum | `enum.identity-read`, `enum.result-retain` |
| unit | `unit.identity-read`, one `unit.edge` before each source/target edge, `unit.rational-arithmetic`, `unit.target-domain`, `unit.result-retain` |
| integer division | `integer-division.operands`, `integer-division.arithmetic`, `integer-division.domain`, `integer-division.result-retain` |
| integer modulus (`mod`) | `integer-modulus.operands`, `integer-modulus.arithmetic`, `integer-modulus.domain`, `integer-modulus.result-retain` |
| IEEE | `ieee.operands`, conditional `ieee.exact-intermediate`, conditional `ieee.round`, `ieee.result-retain` |
| equality | `equality.plan-form`, `equality.plan`, one `equality.pair` for each planned semantic occurrence-path pair, then `equality.result-retain` |
| function call | one `function.call` before binding the evaluated arguments of each checked pure-function call |
| collection | one `collection.element` before each constructor element expression or one `collection.visit` before binding each visited occurrence, one `collection.member-walk` then one `collection.member-test` for each membership comparison, then `collection.bound` and `collection.result-retain`, in the FR-144/FR-145 order |
| composite construction | `composite.result-retain` after the last record field, tuple argument or union payload argument completes, and at once for a nullary union member |
| integer arithmetic | `integer-arithmetic.operands`, `integer-arithmetic.arithmetic`, `integer-arithmetic.result-retain` for each `Integer` or `Int[..]` `+`, `-`, `*` and unary `-` |
| rational arithmetic | `rational-arithmetic.operands`, `rational-arithmetic.arithmetic`, `rational-arithmetic.normalize`, `rational-arithmetic.result-retain` for each `Rational[..]` `+`, `-`, `*`, `/` and unary `-`, and each `Integer` or `Int[..]` `/` producing a `Rational[..]` |
| numeric ordering | `ordering.operands`, `ordering.arithmetic`, `ordering.result-retain` for each `Integer`, `Int[..]`, `Rational[..]` or `Decimal[..]` `<`, `<=`, `>` and `>=` |
| Boolean connective | `boolean.result-retain` for each `and`, `or`, `implies` and `not` |
| model reference | `model.deref` for each `deref(...)`; `model.navigate` for each runtime `.name` field or relationship-end navigation, followed for a multi-valued end by one `collection.visit` per target, `collection.bound` and `collection.result-retain` |
| graph | `graph.expand` before each node is discovered (the start included), one `graph.edge` before each edge target is tested, then `graph.result-retain` for each `reaches(...)` |
| lookup | `lookup.key`, then `lookup.result-retain` when a value is returned, for each `lookup<T>(p, r) absent m` |
| population | one `population.visit` before each member walked by `allInstances<T>(p)`, conforming or not, then `collection.bound` and `collection.result-retain` |
| dispatch | `dispatch.select` for each dispatched `receiver.name(args)` call, followed by the callee's `function.call` |
| frame | for each field write of an invocation, in ascending (object reference key, field declaration key) order: one `frame.write` before the write is tested against the frame's `modifies` grants, then one `frame.redefines-link` before each `redefines` link followed from the written member, stopping at the first member a `modifies` entry names or at a member with no `redefines` link |

Each charge is the following exact `{ counter: amount }` vector. A semantic-size
counter records the high-water maximum, not a sum: admitting amount `n` changes
its consumed value to `max(consumed,n)`. `work_units` and `result_units` are
cumulative additions. Thus counter aggregation is independent of batching or an
implementation's internal algorithm.

Let `bits(x)` be the magnitude bit length of a mathematical integer (zero is
one), `digits(x)` its base-ten magnitude digit count (zero is one), `occ(v)` the
number of typed value occurrences including the outer value and every nested
scalar/composite occurrence (`none` counts one; a present option counts one plus
its payload; a record or tuple counts one plus its present field values, while an
`absent` or `null` slot counts zero; a union value counts one plus its payload
values, so a nullary union value counts one; a collection counts one plus every
occurrence, a bag occurrence once per multiplicity; a reference counts one), and `maxparts(r)` the maximum `bits` of a reduced
rational's numerator and positive denominator. For a coefficient `c` shifted
by `k` decimal places, `sbits(c,k)` is `bits(c)` when `k = 0` and
`bits(c) + bits(10^k)` otherwise, and `sdigits(c,k)` is `digits(c) + k`; both
are derived from the unshifted coefficient and never from the materialized
product. Every amount below is derived from the bit and digit lengths of the
charge's operands, which are materialized before the charge, and never from an
unmaterialized result.

| Charge point | Exact counter amount before adding one `work_unit` |
| --- | --- |
| `decimal.operands` | `integer_bits=max(bits(coeff_i))`, `decimal_digits=max(digits(coeff_i))`, `value_occurrences=operand_count` |
| `decimal.scale-expansion` | `scale_expansion=max decimal-place shift in this operation`; `integer_bits=max(sbits(c,k))` and `decimal_digits=max(sdigits(c,k))` over every coefficient `c` expanded by shift `k` |
| `decimal.arithmetic` | on the aligned operands, where aligned operand `i` has `A_i=sbits(c_i,k_i)` and `G_i=sdigits(c_i,k_i)` for its coefficient `c_i` and applied shift `k_i` (zero when not expanded): `integer_bits=max(A_1,A_2)+1`, `decimal_digits=max(G_1,G_2)+1` for addition and subtraction; `integer_bits=A_1+A_2`, `decimal_digits=G_1+G_2` for multiplication; `integer_bits=max(A_1,A_2)`, `decimal_digits=max(G_1,G_2)` for division; `integer_bits=bits(c)`, `decimal_digits=digits(c)` for negation and conversion |
| `decimal.rounding` | exactly the operand-derived `integer_bits` and `decimal_digits` of the preceding `decimal.arithmetic` row, computed by that row's formula from the aligned operands' `sbits(c_i,k_i)` and `sdigits(c_i,k_i)` (for negation and conversion, `bits(c)` and `digits(c)`), never from the intermediate or the rounded coefficient; they bound the rounded coefficient, which never exceeds the intermediate's numerator; charged only under the decimal applicability rule below |
| `decimal.result-retain` | computed analytically before the retained representation (`v × 10^T`, `T`) is materialized: for the result coefficient `c` that `decimal.arithmetic` or `decimal.rounding` materialized at scale `s` (for division and rounding `s = T`; when `s > T` with all-zero discarded digits, `c` is the materialized coefficient at `T` and `s` is taken as `T`), with upscale shift `k = T − s`: `scale_expansion=k`, `integer_bits=sbits(c,k)`, `decimal_digits=sdigits(c,k)`, `value_occurrences=1`, `result_units += 1`; the coefficient `c × 10^k` is materialized only after this charge is granted |
| `text.input-bytes` | `text_input_bytes=sum UTF-8 bytes of all operands`, `value_occurrences=operand_count` |
| `text.decode-scalars` | `text_scalars=sum decoded scalars of all operands` |
| `text.normalize-input` | no size-counter change |
| kth `text.normalize-output` | `normalized_scalars=k` across all result-side normalized operand sequences |
| `text.result-retain` | `result_units += 1` for a text or Boolean result |
| each `enum.identity-read` | `value_occurrences=number of enum operands read so far` |
| `enum.result-retain` | `result_units += 1` |
| each `unit.identity-read` | `value_occurrences=number of quantity operands read so far`, `integer_bits=maxparts` over their exact rational values |
| kth `unit.edge` | `unit_edges=k` across all source/target root paths |
| each `unit.rational-arithmetic` | `integer_bits` of the `rational-arithmetic.arithmetic` row for the scheduled operation applied to its operands, the current exact value `a/b` and the edge constant or other operand `c/d` (an integer taken as `n/1`); for integer power `x^n`, `integer_bits=max(1, abs(n) × maxparts(x))` |
| `unit.target-domain` | for the reduced final value `a/b` and a decimal target at scale `T`, `integer_bits=sbits(a,T)` and `decimal_digits=sdigits(a,T)`; for an integer target, only `integer_bits=bits(a)`; zero for an unbounded rational target |
| `unit.result-retain` | `value_occurrences=occ(result)`, `result_units += occ(result)` |
| `integer-division.operands` | `integer_bits=max(bits(a),bits(b))`, `value_occurrences=2` |
| `integer-division.arithmetic` | `integer_bits=max(bits(a),bits(b))`, which bounds both quotient and remainder |
| `integer-division.domain` | `value_occurrences=1`: the one member the operator exposes, the quotient for `div` and the remainder for `rem` |
| `integer-division.result-retain` | `result_units += 1` |
| `integer-modulus.operands` | `integer_bits=max(bits(a),bits(b))`, `value_occurrences=2` |
| `integer-modulus.arithmetic` | `integer_bits=max(bits(a),bits(b))`, which bounds the Euclidean quotient and remainder |
| `integer-modulus.domain` | `value_occurrences=1` |
| `integer-modulus.result-retain` | `result_units += 1` |
| `ieee.operands` | `integer_bits=selected width`, `value_occurrences=arity`; for conversions, the source width or the exact source size in the IEEE conversion paragraph |
| `ieee.exact-intermediate` | `integer_bits=selected width`, or for conversions the target width or IEEE-to-exact `maxparts` in the IEEE conversion paragraph; this is the fixed semantic allowance for one exact-real operation at binary32/binary64, not a claim that an irrational square root is materialized as a rational |
| `ieee.round` | `integer_bits=selected width`, or the target width for conversions |
| `ieee.result-retain` | `result_units += 1` for bits/flags or Boolean |
| `equality.plan-form` | `value_occurrences=max(occ(left),occ(right))`; `work_units += occ(left) + occ(right)` instead of one; charged before plan formation walks either operand |
| `equality.plan` | `value_occurrences=exact total planned pair events`; without changing consumed counters, atomically require remaining capacity for `pair_events + 2` work units and one result unit, then consume the plan's one work unit |
| each `equality.pair` | no size-counter change; `work_units += 1` |
| `equality.result-retain` | `result_units += 1` |
| each `function.call` | no size-counter change; `work_units += 1` |
| each `collection.element` | no size-counter change; `work_units += 1` |
| each `collection.visit` | no size-counter change; `work_units += 1` |
| each `collection.member-walk` | `value_occurrences=max(occ(c),occ(m))` for candidate `c` and member `m`; `work_units += occ(c) + occ(m)` instead of one; charged before that comparison's plan formation |
| each `collection.member-test` | `value_occurrences=p`, where `p` is the FR-149 planned pair count of that one comparison; `work_units += p` instead of one |
| `collection.bound` | `value_occurrences=n`, the FR-144 bound count of the formed collection |
| `collection.result-retain` | `value_occurrences=occ(result)`, `result_units += occ(result)`; `result_units += 1` for a scalar query result |
| `composite.result-retain` | `value_occurrences=occ(result)`, `result_units += occ(result)` |
| `integer-arithmetic.operands` | `integer_bits=max(bits(operand_i))`, `value_occurrences=operand_count` |
| `integer-arithmetic.arithmetic` | `integer_bits=bits(a)+bits(b)` for `*`; `integer_bits=max(bits(a),bits(b))+1` for `+` and `-`; `integer_bits=bits(a)` for unary `-` |
| `integer-arithmetic.result-retain` | `result_units += 1` |
| `rational-arithmetic.operands` | `integer_bits=max(maxparts(operand_i))`, an integer operand `n` taken as `n/1`, `value_occurrences=operand_count` |
| `rational-arithmetic.arithmetic` | `integer_bits=max(N,D)` for operands `a/b` and `c/d`, where `N=bits(a)+bits(c)`, `D=bits(b)+bits(d)` for `*`; `N=bits(a)+bits(d)`, `D=bits(b)+bits(c)` for `/`; `N=max(bits(a)+bits(d),bits(c)+bits(b))+1`, `D=bits(b)+bits(d)` for `+` and `-`; `integer_bits=max(bits(a),bits(b))` for unary `-` |
| `rational-arithmetic.normalize` | `integer_bits=max(bits(n),bits(d))` of the unreduced intermediate `n/d`, which is materialized by the arithmetic step after the `rational-arithmetic.arithmetic` charge and before this charge; the amount therefore sizes a charged, materialized value, as the operand rule above requires |
| `rational-arithmetic.result-retain` | `result_units += 1` |
| `ordering.operands` | `integer_bits=max` over both operands of `bits` for an integer, `maxparts` for a rational and `bits(coefficient)` for a decimal; `decimal_digits=max(digits(coefficient))` for decimals, each decimal measured in its retained representation `(coefficient, scale)` as retained and never normalized; `value_occurrences=2` |
| `ordering.arithmetic` | for integers, `integer_bits=max(bits(a),bits(b))`; for rationals `a/b` and `c/d`, `integer_bits=max(bits(a)+bits(d),bits(c)+bits(b))`; for decimals in their retained representations `(c1,s1)` and `(c2,s2)`, never normalized, aligned to `s=max(s1,s2)`, `scale_expansion=|s1-s2|`, `integer_bits=max(sbits(c1,s-s1),sbits(c2,s-s2))` and `decimal_digits=max(sdigits(c1,s-s1),sdigits(c2,s-s2))` |
| `ordering.result-retain` | `result_units += 1` |
| `boolean.result-retain` | `result_units += 1` |
| each `model.deref` | `value_occurrences=1` |
| each `model.navigate` | `value_occurrences=1` |
| kth `graph.expand` | `value_occurrences=k`, the number of distinct nodes discovered by this `reaches` evaluation including this one |
| each `graph.edge` | no size-counter change; `work_units += 1` |
| `graph.result-retain` | `result_units += 1` |
| `lookup.key` | `value_occurrences=1` |
| `lookup.result-retain` | `value_occurrences=occ(result)`, `result_units += occ(result)`: one for a `Reference<T>` or `none`, two for a present `Option<Reference<T>>` |
| each `population.visit` | no size-counter change; `work_units += 1` |
| `dispatch.select` | `value_occurrences=c`, where `c` is the number of linked candidates of the called effective operation; `work_units += c` instead of one |
| each `frame.write` | no size-counter change; `work_units += 1` |
| each `frame.redefines-link` | no size-counter change; `work_units += 1` |

For each non-repeated row, `work_units += 1`; repeated identity, edge, rational,
normalization-output, equality-pair, function-call, collection-element,
collection-visit, model-deref, model-navigate, graph-expand, graph-edge,
population-visit, frame-write and frame-redefines-link rows add one per event, and `equality.plan-form`,
`collection.member-walk`, `collection.member-test` and `dispatch.select` add
their stated amounts.
A composite retained value costs `occ(result)`. Checked amount derivation and counter arithmetic precede
allocation or semantic arithmetic. A counter amount greater than `u64::MAX`
returns incomplete at that point. No unnamed precision, shift, host-backend cost
or implementation-work counter may change an outcome.

Unit rational events are representation-independent. Traversing one affine
unit edge schedules exactly two events: multiply then add in the declared
direction, or subtract then divide in the reverse direction, including identity
multipliers and zero offsets. After all required conversions, quantity add,
subtract, multiply or divide schedules one event for its exact mathematical
result; integer power schedules one event for the exact powered result,
independent of the exponentiation algorithm. Dimension/compound-unit map
normalization schedules no rational event. Thus U10 has one identity read, one
edge, two rational events, target admission and result retention: six work
units. Equality preflight is only an availability check: the subsequent pair
and result-retention charges are the sole mutations for those reserved units,
so E21, whose operands each have `occ = 16`, consumes exactly
`32 + 1 + 17 + 1 = 51` work units and one result unit.

FR-149 selects the equality schedule for a top-level Boolean, `Integer`,
`Int[..]`, `Rational[..]`, `Decimal[..]` or `Reference<T>` comparison as a
one-pair plan: `equality.plan-form` with two work units, `equality.plan` with
`value_occurrences=1`, one `equality.pair` and `equality.result-retain`, which
is five work units and one result unit, and no `decimal.*` or `integer-*`
charge. Every comparison that uses the FR-149 equality schedule, top-level or
nested, charges `equality.plan-form` before plan formation walks either
operand; a top-level text, enumeration or quantity comparison uses its own
FR-141 or FR-142 schedule and charges no `equality.*` point. Plan formation then makes its
checks, including the `foreign_reference` universe check, without a further
charge, and precedes `equality.plan`; a refused check stops after
`equality.plan-form`. Of the FR-149 equality conversions, an FR-142 unit
conversion charges its `unit.*` schedule and a `Decimal`-to-`Decimal` conversion
charges the decimal conversion schedule below. An `Int[..]` or
`Rational[..;d,1]` source converted to `Decimal[c1,c2;s1,s2;m]` charges the
same four decimal points on the integer `n`: `decimal.operands` with
`integer_bits=bits(n)`, `decimal_digits=digits(n)` and `value_occurrences=1`;
`decimal.scale-expansion` with `scale_expansion=s1`, `integer_bits=sbits(n,s1)`
and `decimal_digits=sdigits(n,s1)`; `decimal.arithmetic` with those same amounts;
and `decimal.result-retain` for the retained `(n × 10^s1, s1)`. A `Decimal`
source `(c, s)` converted to `Rational[..]` charges `decimal.operands` on
`(c, s)`; `decimal.scale-expansion` with `scale_expansion=s` and
`integer_bits=bits(10^s)`; `decimal.arithmetic` with `integer_bits=max(bits(c),bits(10^s))`
and `decimal_digits=max(digits(c),s+1)`; and `decimal.result-retain` with
`integer_bits=maxparts` of the materialized reduced rational `c/10^s`,
`value_occurrences=1` and `result_units += 1`. Every amount before retention is
derived from operand sizes, and none of these conversions has a rounding step.
An explicit conversion of a `Rational[n1,n2;d1,d2]` source with `d2 > 1`, value
reduced `n/d`, to `Decimal[c1,c2;s1,s2;m]` is decimal division of `(n, 0)` by
`(d, 0)` into target scale `T = s2` with rounding mode `m`, whatever `d` is at
run time. It charges the decimal division schedule: `decimal.operands` with
`integer_bits=max(bits(n),bits(d))`, `decimal_digits=max(digits(n),digits(d))`
and `value_occurrences=2`; `decimal.scale-expansion` with `scale_expansion=T`,
`integer_bits=sbits(n,T)` and `decimal_digits=sdigits(n,T)`;
`decimal.arithmetic` with `integer_bits=max(sbits(n,T),bits(d))` and
`decimal_digits=max(sdigits(n,T),digits(d))` for the exact `N/D = (n × 10^T)/d`;
`decimal.rounding` exactly when that division has a rounding step and `m` is not
`exact`; and `decimal.result-retain` at scale `T`. Its divisor is never zero,
strict `exact` at a rounding step is refused after `decimal.arithmetic`, and
membership is decided before `decimal.result-retain`. Every other admitted equality conversion keeps the source
magnitude and has no charge point. Record, tuple and union value construction charges
`composite.result-retain` after its last field or argument, or at once for a
nullary union member, and its field expressions, payload arguments and calls
charge their own points. A `case` charges its scrutinee's points and then its
selected arm body's points; selecting the arm and binding its payload make no
charge (FR-146). Integer `+`, `-`, `*` and unary
`-` charge `integer-arithmetic.operands`, then `integer-arithmetic.arithmetic`,
then decide any FR-044 bounded-result membership without a charge, then
`integer-arithmetic.result-retain`. Boolean `and`, `or` and `not` charge
`boolean.result-retain` once their result is decided, and `implies` does the
same. `Rational[..]` `+`, `-`, `*`, `/` and unary `-`, and an `Integer` or
`Int[..]` `/` whose result is `Rational[..]`, charge
`rational-arithmetic.operands`, then `rational-arithmetic.arithmetic`, then
`rational-arithmetic.normalize`, then decide any FR-044 result-bound membership
without a charge, then `rational-arithmetic.result-retain`; a zero divisor met
by direct kernel evaluation is undefined after `rational-arithmetic.operands`.
A bounded integer or `Rational[..]` arithmetic result outside its result
domain, other than a `sum` seed or prefix, is refused, with no value and no
later charge, as
`refused { code: integer_out_of_domain, cause: outside-domain }` or
`refused { code: rational_out_of_domain, cause: outside-domain }`,
respectively (FR-044).
`Integer`, `Int[..]`, `Rational[..]` and `Decimal[..]` ordering `<`, `<=`, `>`
and `>=` charges `ordering.operands`, `ordering.arithmetic` and
`ordering.result-retain`: three work units and one result unit. Quantity
ordering keeps its FR-142 unit schedule. Every amount is derived from operand
sizes before any product or aligned coefficient is materialized. FR-144 collection construction and every FR-145 query and conversion
charge the collection family in the order those requirements define: a
constructor element or visited occurrence charges before it is evaluated or
bound, each membership comparison charges `collection.member-walk` before its
plan formation and `collection.member-test` before its pair comparisons, and
`collection.bound` precedes the bound check, which makes no charge. A membership
comparison charges no `equality.*` point, and an FR-149 equality count measures
only the equality expression over completed operands. `function.call` is the execution
fuel of FR-146: it is charged after the call's arguments are evaluated left to
right and before they are bound, so a denied call enters no callee.

Text applicability is profile-dependent. `nfc`, `nfd`, `nfkc` and `nfkd`
charge every text row in order. The non-normalizing `unicode-scalars` and
`binary-utf8` profiles charge only `text.input-bytes`, `text.decode-scalars`
and `text.result-retain`; they charge no `text.normalize-input` or
`text.normalize-output` point and leave `normalized_scalars` unchanged. The
length-bound refusal of an admission follows the charge that measures the
bounded length: after the last `text.normalize-output` for a normalizing profile, after
`text.decode-scalars` for `unicode-scalars` and after `text.input-bytes` for
`binary-utf8`; it always precedes `text.result-retain`, and an exhausted counter
at an earlier charge returns incomplete instead. An
unordered-enumeration ordering request or cross-declaration enum comparison is
refused by type checking before any `enum.*` charge.

Every unit operation follows one schedule. Incompatible dimensions,
affine-unit arithmetic and distinct units where the identical unit is required
are FR-142 type-time `ill_typed` refusals and make no charge. Evaluation charges
one `unit.identity-read` per operand in operand order. It then decides the
runtime conditions in order, first match wins, without a charge: a zero divisor,
then a zero base under a negative integer exponent; each is undefined. It then
charges one `unit.edge` for every edge the operation traverses, all before any
rational event: conversion traverses the full source-to-root path and then the
full root-to-target path with no common-ancestor shortcut, and a compound side
contributes no edge; multiplication, division, integer power, equality and
ordering traverse the left and then the right operand's root path; addition and
subtraction of the identical unit traverse no edge. It then schedules the two
rational events of each edge in that same order, followed by the operation's
own event; conversion, equality and ordering have no own event. It then charges
`unit.target-domain`, except for equality and ordering, and finally
`unit.result-retain`. Every quantity addition, subtraction, multiplication,
division and integer power, and every conversion to an exact rational target,
has an unbounded exact rational result, so its `unit.target-domain` charges no
size counter: that result's size is already charged by its final
`unit.rational-arithmetic` event, or by the identity read for a conversion
without edges. For a decimal target, FR-140's rounding step is decided
exactly from the reduced final value `a/b` without materializing the retained
coefficient `v × 10^T`; strict `exact` at a rounding step refuses before
`unit.target-domain`; that charge carries `integer_bits=sbits(a,T)` and
`decimal_digits=sdigits(a,T)`, which bound the retained coefficient; the
coefficient is materialized only after it; and FR-140 membership, which charges
nothing, refuses, other than a `sum` seed or running total, after it and before `unit.result-retain`. An integer target is
a decimal target at scale zero: its rounding step, strict `exact` refusal and
`unit.target-domain` sizing occur at the same positions, except that its
`unit.target-domain` carries only `integer_bits = bits(a)` and no
`decimal_digits` amount, and integer-domain
membership, which charges nothing, refuses (`integer_out_of_domain`,
`outside-domain`), other than a `sum` seed or running total, after `unit.target-domain` and
before `unit.result-retain`. Decimal and integer targets charge no `decimal.*`
or `integer-*` point.

A top-level quantity `=`, `!=`, `<`, `<=`, `>` or `>=` expression between two
operands of the identical unit uses that schedule with no operation event and
no `unit.target-domain`, then `unit.result-retain` with `occ(result) = 1`, and
compares exact root values; it charges no `equality.*` point. A quantity leaf
inside an FR-149 equality plan is instead compared by exact value in its
identical unit and charges only its planned `equality.pair`, like every other
scalar leaf, with no `unit.*` read, edge or event. The FR-142 order among the
static quantity refusals is informative only: every static cause returns the
same `refused { code: ill_typed }` with no charge, so the order has no
observable outcome difference.

IEEE applicability is also representation-independent. Equality, `totalOrder`
and bit identity charge only `ieee.operands` and `ieee.result-retain`; every
IEEE operation or comparison with mixed widths or an exact operand is a
type-time `ill_typed` refusal with no charge. An arithmetic operation charges
only those two points exactly when a NaN operand, an infinite operand, an
invalid operation or a division by zero determines its result; it constructs no
finite exact real and performs no rounding. Every other add, subtract, multiply,
divide and FMA, whose operands are all finite with zeros of either sign
included, plus square root of a finite operand that is a zero of either sign or
positive, additionally charges `ieee.exact-intermediate` and `ieee.round` in
that order. A negative nonzero finite square-root operand follows the
classified-invalid path. Strict `exact` refuses after `ieee.round` and before
`ieee.result-retain`, because inexactness at an IEEE width is decided by that
fixed-width rounding step; decimal and unit targets decide inexactness
analytically before their sizing charge. This applies even when the exact real
is irrational or the rounded result is exact. The fixed width amount is total
for every path and prevents an implementation's rational, symbolic-root or
hardware representation from changing accounting. Thus a comparison or
classified exceptional operation consumes two work units, while a finite
arithmetic operation consumes four.

Explicit IEEE conversions reuse the four IEEE points with arity one. A width
conversion charges `ieee.operands` with `integer_bits=source width`. For a NaN
source it then decides the payload refusal, which makes no charge, and for a NaN
or infinite source it then charges only `ieee.result-retain`. For a finite
source, zeros included, it charges `ieee.exact-intermediate` and `ieee.round`
with `integer_bits=target width`, then decides any strict `exact` refusal, then
charges `ieee.result-retain`. An IEEE-to-exact conversion charges
`ieee.operands` with `integer_bits=source width`; a NaN or infinity is then
undefined; a finite value, either zero included, then charges
`ieee.exact-intermediate` with `integer_bits=maxparts` of the exact result
rational, sized from the operand's decoded exponent and mantissa before the
rational is materialized, then decides `Rational[..]` target membership at
evaluation, exempt from FR-044's static result-bound proof, which makes no
charge and returns `refused { code: ieee_rational_out_of_domain, cause: outside-domain }` for a
non-member before `ieee.result-retain`, then `ieee.result-retain`. FR-148
defines no conversion from an IEEE value to `Decimal`, `Integer` or `Int[..]`; a
direct IEEE conversion to those types is refused `ill_typed` with no charge, and
an IEEE value reaches exact arithmetic only as `Rational[..]`. Conversion of a
`Rational[..]` quantity into a decimal or integer target remains governed by
FR-142 and is unaffected. An exact-to-IEEE conversion charges `ieee.operands`
with `integer_bits` measured on the source: `bits(n)` for an integer `n`,
`maxparts` for a rational, and `max(bits(coefficient), bits(10^scale))` of the
retained representation for a decimal; it then charges `ieee.exact-intermediate`
and `ieee.round` with `integer_bits=target width`, then decides any strict
`exact` refusal, then charges `ieee.result-retain`.

Decimal accounting is independent of implementation storage but not of
retained scale. Every decimal amount is measured on each operand's retained
representation: a literal's coefficient/scale as written, or a computed
result's (`v × 10^T`, `T`) at its target scale `T`; a normalized form is never
substituted. FR-140 membership makes no charge. Decimal addition,
subtraction, multiplication, division, unary negation and explicit conversion
each charge `decimal.operands`, `decimal.scale-expansion` and
`decimal.arithmetic` in that order unless an earlier outcome stops evaluation.
The `decimal.scale-expansion` shift is `abs(left scale - right scale)` for
addition and subtraction, `abs(T + divisor scale - dividend scale)` for division
into target scale `T`, and zero for multiplication, negation and conversion; a
zero shift is still charged, with `scale_expansion=0` and no expanded
coefficient. The `decimal.arithmetic` intermediate is the exact result
coefficient as `c/1` for addition, subtraction, multiplication, negation and
conversion, and the reduced FR-140 `N/D` for division. `decimal.rounding` is
charged exactly when FR-140 defines a rounding step (at least one nonzero
discarded digit) and the selected mode is not `exact`; all-zero discarded
digits are not a rounding step and make no `decimal.rounding` charge. A
divisor whose normalized coefficient is zero is undefined after
`decimal.operands` and before `decimal.scale-expansion`. Strict `exact` at a
rounding step is refused after `decimal.arithmetic` and before
`decimal.rounding`. A nonmember result, other than a `sum` seed or running
total, is refused after
`decimal.rounding`, or after `decimal.arithmetic` when no rounding step occurs,
and before `decimal.result-retain`. An undefined or refused outcome makes no
later charge.

Integer `div`/`rem` and `mod` charge their domain point on every evaluation,
including an unbounded mathematical-integer result. A zero divisor is undefined
after `integer-division.operands` or `integer-modulus.operands` and before the
corresponding arithmetic point, and makes no later charge. A bounded-consumer
membership refusal follows `integer-division.domain` or
`integer-modulus.domain` and precedes result retention. `mod` never charges an
`integer-division.*` point, and `div`/`rem` never charges an
`integer-modulus.*` point.

Model and graph evaluation follows the selected `quire.model.complete/v2`
rules. Static resolution, conformance, kind, direction, closure and foreign
checks decided at checking make no evaluation charge; they are charged under
`ModelNormalizationLimitsV1` below. Population binding admission makes no
evaluation charge; it is charged under `PopulationAdmissionLimitsV1` below.
Evaluation stops at its first undefined, refused or incomplete outcome.
`deref(r)` evaluates `r`, charges `model.deref`, then decides a dangling
reference without a charge. A field or relationship-end navigation `r.name`
evaluates `r`, charges `model.navigate` and takes the end's targets in
canonical reference-key order. A single-valued end then decides, without a
charge, every target's dangling check in that order and after them the end's
count bound, and returns the `Reference<T>` or `Option<Reference<T>>` with no
further charge. A multi-valued end decides each target's dangling check without
a charge and then charges one `collection.visit` for that target; after the
last target it charges `collection.bound`, decides the FR-144 bound check
without a charge, and charges `collection.result-retain`. A dangling target
therefore refuses before the count bound is decided.
`reaches(a, b, e)` evaluates `a` and `b`, then explores breadth-first. The start
`a` is discovered but not reached: `reaches` charges `graph.expand` for `a` and
enqueues it. For each dequeued node, in discovery order, and for each target `t`
of edge `e` in the edge value's own order (sequence order for a sequence edge),
it decides a snapshot mismatch or dangling target without a charge, charges one
`graph.edge`, and then, when `t` is `b`, the result is `true` and exploration
stops; otherwise, when `t` is not yet discovered, it charges `graph.expand` for
`t` and enqueues it. When the queue is empty the result is `false`. It then
charges `graph.result-retain`. Thus `reaches(a, a, e)` is `false` for an
isolated `a` and `true` when a self-loop or a cycle returns to `a`, as
FR-043-AC-2 requires.
`lookup<T>(p, r) absent m` evaluates `p` and then `r`, charges `lookup.key`,
decides the foreign-universe check and then key presence without a charge, and
charges `lookup.result-retain` only when it returns a value; `absent undefined`
and `absent refused` make no later charge. `allInstances<T>(p)` has its object
and subtype closure decided at binding admission; it walks every admitted
member of `p` in canonical reference-key order, charging one `population.visit`
before each member walked whether or not its most-specific type conforms to
`T`, and then selects the member, without a charge, exactly when that type
conforms to `T`. It then charges `collection.bound` with `value_occurrences=n`
for the `n` selected members, decides the declared maximum without a charge,
and charges `collection.result-retain`. A dispatched call evaluates the
receiver and then its arguments left to right, charges `dispatch.select`,
selects the linked method for the receiver's most-specific type without a
charge, decides the selected method's effective precondition (its own
evaluation charges apply), and then charges `function.call` before binding the
arguments.

## Collection sum schedule

`sum<N>(x in c: e)` (FR-145) charges, in order: one `collection.visit` and the
summand's own charges for the first occurrence, which seeds the running total
with no addition; then, for each later occurrence in visiting order, one
`collection.visit`, its summand's charges and one addition charged by the
family of `N`; then one `collection.result-retain` with `result_units += 1`. An
empty source charges only that `collection.result-retain`. A seed or running
total that is not a member of `N`'s domain ends the sum with FR-145's located
undefined outcome at that summand or addition, in place of the family's
membership refusal, and makes no later charge.

| Family of `N` | Charges of one addition |
| --- | --- |
| `Integer` or `Int[..]` | `integer-arithmetic.operands`, `integer-arithmetic.arithmetic`, the uncharged domain decision, then `integer-arithmetic.result-retain` |
| `Rational[..]` | the `rational-arithmetic` schedule |
| `Decimal[..]` | the decimal addition schedule, whose `decimal.rounding` step uses the `rounding` mode `N` pins |
| quantity | the unit addition schedule, whose decimal or integer target rounding step uses the `rounding` mode `N` pins |
| `Float32` or `Float64` | the IEEE addition schedule under the `rounding` mode `N` pins |

The checked package records that mode on the `quire.op.collection.sum.decimal`,
`sum.quantity`, `sum.float32` or `sum.float64` application (FR-322), and it must
equal the mode `N`'s type pins; a disagreement is refused before evaluation and
never resolved at a charge point. The domain decision after each addition and
seed charges nothing.

## Population admission limits

A request that binds a population binding carries `PopulationAdmissionLimitsV1`
with exact `u64` limits for `population_members` and `work_units`, in that
field order. Omission is a refused request; zero is a real limit and never
means unlimited. These counters are independent of `ScalarLimitsV1` and of
`ModelNormalizationLimitsV1`. `population_members` is a high-water semantic
size; `work_units` is cumulative. Binding admission first decides, without a
charge and in this order, the FR-153 model-selection check, object closure and
subtype closure; each is read from the population declaration and the effective
view. It then charges:

| Charge point | Order | Exact counter amount |
| --- | --- | --- |
| kth `binding.member` | each runtime member record of the population binding, in input order, before its key is formed | `population_members=k`; `work_units += 1` |
| each `binding.subset-value` | after the last `binding.member`: objects in canonical reference-key order, then each subsetting field reaching the object's most-specific type ascending by declaration key, then each value of the subsetting feature in its value order, before that value is tested | no size-counter change; `work_units += max(1, n)`, where `n` is the number of values of the subsetted feature of that object, which the test scans |

After each `binding.member` charge, admission decides without a further charge,
in this order, `foreign_reference`/`foreign-type` for a member type not covered
by the population declaration's member types, then
`invalid_runtime_input`/`abstract-instance` for an abstract most-specific type,
and then, against the members admitted before it, duplicate
collapse or `invalid_runtime_input`/`conflicting-identity`. After each
`binding.subset-value` charge it decides
`invalid_runtime_input`/`subsetting-violation` for that value. Admission stops at
its first refusal and admits no population. The first unavailable charge
returns `incomplete { limit_kind, limit, consumed, next_charge, charge_point }`,
where `limit_kind` is the first `PopulationAdmissionLimitsV1` field, in field
order, whose amount is unavailable, and admits no population. The
qualification seam below applies to these points with the `work_units` of this
limit object.

## Model normalization limits

Checking a package that selects `quire.model.complete/v2` carries
`ModelNormalizationLimitsV1` with exact `u64` limits for `declaration_records`,
`derivation_facts`, `effective_declarations`, `dispatch_candidates`,
`hashed_bytes`, `work_units` and `ancestor_steps`, in that field order. Omission is a refused
request; zero is a real limit and never means unlimited. These counters are
independent of `ScalarLimitsV1`. `declaration_records`, `derivation_facts`,
`effective_declarations` and `dispatch_candidates` are high-water semantic
sizes; `hashed_bytes` and `work_units` are cumulative. Every row adds one work
unit unless it states another addition.

Checking runs these stages in order: intake (`normalize.record`) and the
model feature rules; normalization phases 2, 3 and 4, each a stage;
canonicalization (phase 5); systems kind mapping, connection and allocation
checks; source resolution and structural assembly, including static navigation;
checked-fact admission; the entire conformance stage; and dispatch linking.
Checked-fact admission checks the actual authored clauses once and retains
their established FR-146 facts. It forms no normalization derivation fact and
adds no `conformance.axis` charge. Its existing expression-checking budgets
remain independent of this model meter. All conformance axes, including static
variance and fact-dependent refinement, run together after this admission.
An earlier assembly or clause refusal wins only for an independent defect:
assembly must not decide the model-owned narrowing/multiplicity/refinement
conditions of FR-151 early and replace their causes or suppress sibling axes.

One checking operation owns one model meter across all its selected models
and these stages. The meter is transferred from intake to checking without
reset, refund, cloning or repeated charges. Selections run in ascending
(`identity`, `digest_domain`, `digest`) byte order, a proper prefix first;
FR-321 rejects repeated identities before this order is consumed. Finish each
stage across all selections before advancing any selection to the next stage.
Failures of a completed stage are ordered by selection then local charge
order. High-water sizes count the operation's simultaneously retained records,
facts, effective declarations and candidates; cumulative counters never reset
between selections. Structural normalized data is internal input to checking,
not a validated effective view or a public partially checked package.

`ancestor_steps` is the exact u64 ceiling of the number of supertypes edges
followed by one walk, not its depth and not cumulative work. Each walk starts
at zero. Test availability immediately before each edge is followed, after
any owning axis/fact work charge already admitted. A permitted edge increments
this walk's count by one but introduces no additional work-unit charge: the
existing work pricing below is unchanged. Its named point is
`model.ancestor-edge`; denial returns
`incomplete { limit_kind: ancestor_steps, limit: A, consumed: k,
next_charge: 1, charge_point: model.ancestor-edge }` when k = A. A zero limit
denies the first followed edge; at n edges, n admits and n-1 denies edge n.
The six existing counters are checked first, in their existing field order,
at their owning charge; ancestor availability is checked only after that
charge succeeds. Thus a simultaneous work denial wins before the edge check.
Neither denial follows an edge, refunds admitted work or returns a semantic
conformance verdict. This row applies to normalization ancestor expansion,
conformance, systems conformance and dispatch ancestor walks; it does not
change family-step or population-admission limits.

| Charge point | Order | Exact counter amount |
| --- | --- | --- |
| kth `normalize.record` | each declaration record that intake builds from a domain package IR node (object type, field, relationship, operation, clause, population, part, port, interface, connection or allocation), ascending declaration key, before the node is read | `declaration_records=k` |
| kth `normalize.fact` | each derivation fact before it is formed: phase 2, then phase 3, then phase 4, each ascending by (owner declaration key, declaration key, inputs, rule identity), where a type fact has no owner and sorts before every owned fact; rule identity is compared as exact UTF-8 bytes, and identical facts retain their full multiplicity | `derivation_facts=k` |
| each `normalize.cycle-check` | immediately before the `normalize.fact` of each phase 3 type fact, before testing whether the extended path's new general type already lies on the path | no size-counter change; `work_units += L` instead of one, where `L` is the number of supertype edges in the extended path; a closing extension is charged even when its cycle's edge set was already reported, and each cycle is reported once, at its first closing charge, as `quire.model.complete/v2` states |
| each `normalize.redefinition-check` | phase 4, before its first `normalize.fact`: each redefining member ascending by declaration key, before testing that its target is a member inherited by its owning type and that no member of the same type checked before it redefines the same target | no size-counter change; `work_units += m + r` instead of one, where `m` is the number of effective members of the owning type and `r` the number of redefining members checked before it |
| each `normalize.conflict-check` | phase 4, after its last `normalize.fact`: each (effective type, redefined member) reached by `c >= 2` redefining members, ascending by (owner original declaration key, target original declaration key), before its redefining owners are compared pairwise; this preidentity order does not change the later effective member key | no size-counter change; `work_units += Σ (c − 1) × f(o)` instead of one, summed over the `c` redefining owners `o`, where `f(o)` is the number of type derivation facts (qualify and inherit) of `o` that each proper-descendant test of `o` against another owner scans, one work unit per fact |
| kth `normalize.declaration` | phase 5, before any identity preparation for that entry: all effective types ascending original declaration key, then all effective members ascending (computed owner effective type identity, original declaration key) | `effective_declarations=k` |
| each `normalize.metadata` | after the owning declaration charge, at first use of each semantic key-field occurrence defined below, before scanning its content for canonical-length metadata; the selection identity occurrence is prepared before its first universe/view hash | no size-counter change; `work_units += 1 + n` instead of one, where n is its admitted UTF-8 byte length |
| each `normalize.normal-form` | after its declaration charge, before forming that entry's canonical derivation descriptor or path index | no size-counter change; `work_units += max(1, f + p + t)` instead of one, where f is its logical fact count and p/t are its logical path/tail key-occurrence counts from the admitted phase-2/3/4 data |
| each `normalize.identity-work` | before each slot/metadata/dependency lookup, range or binding proof, node-price traversal, ordering projection/comparison/move or index admission in the closed schedule below, including reuse hits | no size-counter change; `work_units += a` instead of one, with a defined below for the named event |
| each `normalize.hash` | phase 5, required key/block dependencies children-before-parent at first use, then the owning declaration; after all entries, each universe ascending first root identity, then the complete declaration-sequence joins left-before-right postorder and the view header; each charge precedes node encoding AND hashing | `hashed_bytes += b`, where b is the exact sole-writer RFC-8785 length from the closed analytical template; one work unit, with no posthoc byte reservation |
| each `conformance.axis` | each FR-151 axis, redefining members then subsetting fields, each ascending by declaration key, axes in FR-151 order | no size-counter change; a type-conformance axis charges `work_units += f(x)` instead of one, where `x` is the type whose conformance is tested and `f(x)` the number of its type derivation facts (qualify and inherit) that the conformance walk scans, one work unit per fact, or one for a non-object type decided by the FR-149 equality-conversion table: the field value type axis and the subsetting type test use the redefining or subsetting feature's type, parameter type axis `i` uses the redefined parameter `i`'s type (so the parameter type axes together charge the sum of `f` over the parameters), and the result type axis uses the redefining result type, or one when either result is absent; for the operation effect axis, `work_units += max(1, t)` instead of one, where `t` sums, over each redefining frame entry in `modifies`, `creates`, `deletes` order: for a field write, `n × (1 + r)`, with `n` the number of redefined `modifies` entries it is compared against and `r` the number of `redefines` links walked from the written field; for a create or delete, `f(x)` for every redefined `creates` (respectively `deletes`) grant it is tested against, with `f(x)` the number of type derivation facts of the created or deleted type `x` that the conformance test scans; every entry is compared with every grant |
| each `systems.kind` | each interface, then each part, then each port, then each connection, then each allocation declaration, each group ascending by declaration key, before its FR-152 kind is mapped | no size-counter change |
| each `systems.connection-condition` | each Connection ascending by declaration key, then each of its three FR-152 conditions in table order, before that condition is checked | no size-counter change; the interface-type condition charges `work_units += f(x)` instead of one, where `x` is the flow-source port's interface type and `f(x)` the number of its type derivation facts scanned by the conformance walk (for `bidirectional`, the first end's interface type) |
| each `systems.allocation` | each Allocation ascending by declaration key, before its target is checked | no size-counter change |
| each `systems.resolve` | in source order, each qualified-name segment after the model alias and each runtime `.name` navigation site, before it is resolved statically | no size-counter change |
| each `dispatch.subtype` | per called effective operation in effective member order, each closed subtype ascending by effective type identity, before its applicable candidate set is formed | no size-counter change |
| kth `dispatch.candidate` | after that subtype's `dispatch.subtype`, each family member that has a body ascending by effective member key, before its applicability test | `dispatch_candidates=k`; `work_units += f` instead of one, where `f` is the number of type derivation facts (qualify and inherit) of the subtype, one work unit per fact scanned by the conformance walk |
| each `dispatch.dominance` | after a subtype's candidate tests, when `c >= 2` candidates are applicable, before the dominance pairs are enumerated | no size-counter change; `work_units += Σ (c − 1) × f(o)` instead of one, summed over the owners `o` of the `c` applicable candidates, where `f(o)` is the number of type derivation facts of `o` that each proper-descendant test of `o` against another candidate's owner scans, one work unit per fact |

Checking is exhaustive within an operation-wide stage and under these limits. Each row is
charged before the work it names runs, in the stated order, and every refusal
that work exposes is reported, in charge order, when its stage ends; a
refusal decided by an intake admission check before a stage's first charge is reported
first. A stage that reports a refusal ends checking: no later stage runs or
charges. Conformance therefore reports every failing axis of every record, and
dispatch linking reports every no-applicable or ambiguous subtype of every
called operation. A generalization cycle is exposed by the `normalize.cycle-check`
of the closing extension, whose `normalize.fact` is not charged; exploration
continues with the next extension, and a cycle whose edge set was already
reported is not reported again. Runtime evaluation, by contrast, stops at its
first failure. The first unavailable charge ends checking with
`incomplete { limit_kind, limit, consumed, next_charge, charge_point }`, where
`limit_kind` is the first `ModelNormalizationLimitsV1` field, in field order,
whose amount is unavailable; a checker surfaces it as `refused { code:
resource_exhausted, cause: insufficient-next-charge }` at stage
`model-normalization`, reports no refusal found in the unfinished stage and
produces no validated effective view. This suppression includes failures
already collected from earlier selections in the same unfinished stage.
Ancestor denial has the same public `resource_exhausted` /
`insufficient-next-charge` disposition and five-field incomplete payload;
retain the owning declaration's actual FCD origin, or an actual source-clause
location when the walk belongs to that clause, without inventing a source
span for generated origins. The separate type-environment stage-limit
contract is unchanged. The qualification seam below applies to these
points with the `work_units` of this limit object.

### Identity preparation, reuse and exact byte pricing

The replacement model commitment grammar is defined by
[model-complete.md](model-complete.md#closed-ordered-commitment-grammar).
The semantic stage order and all earlier phase charges are unchanged. No key,
block or effective-type digest may be computed in phases 1–4. All logical facts
and effective entries, including hidden entries and expanded run occurrences,
still count toward the high-water counters. Shared helper commitments are not
additional effective declarations or facts. An original without an effective
derivation rule is retained without a new key hash merely for retention.

#### Semantic handles and metadata occurrences

Accounting uses the already admitted selection row ordinal and original
inventory key rank, not allocation addresses, string interning or claimed
digest equality. Ranks are ordered by the exact DeclarationKey comparison
already established by intake. They are local scheduling handles only and
never identity inputs. No phase-5 routine discovers them by an uncharged text
scan. Admitted immutable lengths/counts are available for charge derivation;
deriving a new lookup/proof/descriptor is the named work, not charge arithmetic.

A variable-string metadata occurrence is exactly `(selection row, original
inventory rank, key field)` for `package` or `node`, or `(selection row,
selection identity field)` for `DomainPackageRef.identity`. These are different
occurrences even when their UTF-8 text is equal. Fixed field names, domains,
rules and versions have literal lengths; hex and decimal scalars use their
fixed/count-derived lengths, not variable-string scans. Outside the separately
admitted Price traversal, before every metadata access charge one identity-work
lookup with a=1 using this numeric handle and
closed field tag. On a miss charge normalize.metadata with 1+n BEFORE any byte
is scanned; retain the raw length and S(s). On a hit read only that immutable
metadata, with no scan and no second normalize.metadata. No content-based
metadata table or uncharged equality test is allowed. Two different key ranks
with the same package text of n bytes therefore incur TWO 1+n scans; two metadata
accesses to one rank/field incur one scan and two lookup events. A key Require
hit does not access its fields again and hence creates no metadata access.
Interning the strings
physically cannot change either trace. A new selection repeats its own charges.

#### Closed template event interpreter

The following interpreter defines the finite logical event stream. Every
identity-work event charges its stated a BEFORE its named reads, comparisons,
formation or validation. Its trace is a function of admitted semantics only;
actual search probes, buffering or retained-slot optimizations add no public
events and remove none. Physical resource bounds remain separate obligations.

`Read(x)` charges a=1 for the root occurrence of x, including a scalar or
null. For an object it then recursively Reads each member value in the field
order below; this child's root event admits that member lookup and value read.
For an array its root event admits length read, then it Reads each element in
index order. An object root admits its closed shape; constants and scalar
discriminators/counts are checked at their scalar event. Member names are
closed literals, not additional value occurrences. No omitted owner/null,
original, count, kind, rule, version or header slot is free.

| Closed value | Object member order; recursive child interpretation |
| --- | --- |
| Key preimage | key (DeclarationKey), version |
| DeclarationKey | digest_domain, node, package; node/package use their metadata occurrence after the scalar read |
| Block preimage | count, left, right, version; left/right are dependency references of the prescribed half length |
| Range | count, first, last; nonnull references require their prescribed key/block dependencies, first before last |
| Literal segment | kind, path (Range), rule (Rule), tail (array of key references) |
| Prefix segment | kind, path (Range), rule (Rule) |
| Rule | identity |
| Derivation | count, segments (array of Literal/Prefix values) |
| Effective declaration preimage | derivation, original (key reference), owner_effective_type (null or EffectiveId), version |
| EffectiveId | digest, domain; digest performs one admitted-owner/root lookup with a=1, never recursively rehashes the declaration |
| SequenceChild | count, digest; nonnull digest performs one admitted declaration/join lookup with a=1 |
| Join preimage | count, left (SequenceChild), right (SequenceChild), version |
| DomainPackageRef | digest, digest_domain, identity; identity uses the selection metadata occurrence |
| Universe preimage | model_selection (DomainPackageRef), root_types (array of EffectiveIds), version |
| View preimage | declarations (SequenceChild), model_selection (DomainPackageRef), rules (Rule), version |

At each key/block reference, after its scalar Read, `Require` charges one
descriptor lookup with a=1, admitting only the already formed source descriptor
handle/count read, not a content-based reuse search. It next charges a binding-validation event:
a=1 for a key's exact inventory-rank binding, a=max(1,L) for a block's exact
ordered L-key sequence binding. This admits the reuse lookup/equality/proof formation
against the selection's immutable input, including on a hit. A block reuse
key is its node kind, count and exact ordered inventory-rank sequence; a key
reuse key is its inventory rank. An admitted shared path/index may establish
sequence equality without a physical L-key scan, but neither a digest nor
an uncharged content-search can establish it. This proof and table admission
are part of the binding event, independent of the table's physical layout.
The binding event also admits miss descriptor formation from that exact source,
before the node's Read. On a hit Require returns the retained digest/price/binding and stops: no child
walk, price traversal, metadata scan or hash repeats. On a miss Read the node's
preimage, recursively requiring left then right as encountered, then Price
and Hash below; successful binding/digest retention is included in Hash.
Failure does not insert a successful record. No unneeded block is prepared.

After Reading each Range, charge a=max(1,L) once for that Range occurrence,
even an empty range or a range whose blocks hit. This admits prescribed lengths,
exact overlap, path coverage and its provenance proof, not a digest-only check.
Normal form is admitted once per declaration by normalize.normal-form BEFORE
formation or semantic replay. Its max(1,f+p+t) amount admits all logical fact,
input, rule and ordinal reads, total semantic phase/input/rule ordering with
multiplicity, greedy prefix decisions and construction/verification of that
entry's canonical descriptor and source-path index. It does not admit hashes,
later Read/Require events or another entry's work. The semantic input is the
ordered phase-2/3/4 stream, not an arbitrarily ordered expanded list to sort for
free. Index/proof acceleration never refunds f+p+t or range/binding amounts.

After a complete hash-node Read and all encountered dependencies/checks, Price
charges a=max(1,v), where v is EVERY object/array/scalar occurrence of that
node's closed compressed preimage, including root, nulls, constants and
embedded objects, but excluding dictionary dependencies outside the preimage.
v and lengths are retained by the admitted formation/Read events; computing
them cannot require a new uncharged walk. Price traverses the SAME recursive
field/array order once using retained slots. Its single v charge covers ALL
those pricing reads, including metadata hits and length arithmetic. There
are NO additional Read, metadata lookup, Require or range events during Price,
even if a physical implementation rereads fields. It may not rescan variable
text. This is the explicit retained-slot pricing policy, not an optional one.
Then Hash charges normalize.hash with exact b and one work unit before the
sole writer encodes/hashes the node. Encoder-internal reads are covered by
Hash, not a second Read stream. Its returned length must equal b. Every
declaration has its own Read/Price/Hash; no effective-entry hit suppresses it.
Headers and joins use the same interpreter, with already admitted identity
lookups rather than recursively hashing their leaves again.

#### Prerequisite ordering and index events

All orders below use one logical bottom-up stable merge schedule. Input
projection charges a=1 before reading/retaining each record's complete fixed
comparison tuple; original keys compare by admitted rank, domains/rules by
their closed literal byte order, digests by their 32 bytes. No variable text
is compared here. Project records in the prescribed input sequence. Merge
adjacent singleton runs left-to-right, then adjacent runs of widths 2,4,...;
an unmatched final run is carried forward. Charge a=1 before each comparison
of the two run heads' FULL tuples, selecting the left on equality, and a=1
before each emitted logical record move, including all remainder/carry moves.
At zero or one record there are no comparison/merge-move events, only its
projection when present. A physical in-place sort still charges those logical
moves. No physical algorithm can change the comparison or denial trace.
After the sort, charge a=1 per output record, in output order, before installing
its ordering/index slot. No sort, prerequisite index or proof prepass is free.

The schedule within each phase-5 selection is exactly:

1. Project effective types from phase-4 inventory in original-key order;
   their order key is original rank. Apply the merge/index schedule. Project
   members in `(owner original rank, original rank)` order for a retained
   staging index, charging its projection a=1 then a separate a=1 per record
   before its slot is installed (no sort
   because this is the already ordered phase-4 stream). These are ordering
   prerequisites, expressly admitted BEFORE the first normalize.declaration;
   they compute no identity, metadata or derivation descriptor.
2. In type order, charge normalize.declaration, then normalize.normal-form,
   then Read/Price/Hash that declaration and its required dependencies. After
   every type, charge a=1 before installing its owner-rank-to-EffectiveId slot.
3. Project members from the staging index, in that input order, charging a=1
   per owner-ID lookup before the projection event. Sort/index by `(owner
   effective digest, original rank, owner original rank)`. The final rank is
   only a deterministic collision tie; it never resolves a semantic ambiguity.
   In this order charge each member's normalize.declaration, normal-form and
   Read/Price/Hash. This is the unchanged effective member-key meaning.
4. Charge a=max(1,T+G) before constructing/verifying the universe component/root
   index over T admitted object types and G declared supertype edges. Its
   logical source traversal is types in original rank order and each type's
   supertype targets in original rank order, retaining each component under its
   least original type rank. It covers each type/edge visit and root test;
   prior cycle refusals already prevent this stage. Visit components in least
   rank order, project their roots in original rank order, then sort/index by
   `(effective digest, original rank)`. Project the resulting components in
   least rank order and sort/index by `(first root effective digest, least
   original rank)`; an empty root list sorts first. Price/hash universe headers
   by Read in that resulting order. Component/root index formation is not
   inferred for free from identities or a caller-supplied root list.
5. Project ALL declarations in the actual declaration-charge order (types
   then ordered members). Sort/index by `(effective digest, owner original rank
   with null first, original rank)`. Then build the canonical sequence tree in
   left-before-right postorder: charge a=1 before forming each join descriptor,
   followed by its Read/Price/Hash. Form the view descriptor after charge a=1,
   then Read/Price/Hash it. Empty/singleton sequences create no join event.
6. Project the reachable key/block dictionary records in their first successful
   Hash order, children before dependents as above. Sort/index by `(domain byte order,
   digest, first-success ordinal)`, then visit adjacent records left-to-right.
   Charge a=1 per adjacent pair before testing domain/digest equality. On equal
   domain/digest charge a=max(1,v_left+v_right+n_left+n_right) BEFORE comparing
   exact retained preimages, where n sums variable-string bytes; identical
   bindings share one transport record, incompatible bindings refuse. Apply
   the same adjacent-pair and equality-proof events to the sorted declaration
   inventory, never dropping its effective-entry multiplicity. Dictionary
   transport slot installation charges a=1 per retained record. Every exact
   binding comparison includes its retained slots and any necessary text
   equality; no hash-only collision shortcut or uncharged rescanning is allowed.

Descriptor formation for each universe header is charged a=1 before Read;
member-owner, root and SequenceChild lookups read only earlier admitted
identity slots. All component, member and transport ties remain semantic
rank-based, not physical layout. Dictionary collision verification covers
each transported key/block binding; declaration bindings are verified separately
without merging entries. The complete declaration inventory permits deterministic
join reconstruction; no extra join dictionary or root-only export is implied.
The reader must
independently perform this semantic reconstruction and the same logical
verification events under its own operation admission, not borrow producer
reuse or accept a caller's precomputed inventory/index. Wire decoding/export
and any additional reader budget remain owned by the matched carrier contract;
this definition supplies no free transport proof or speculative carrier API.

Exact analytical pricing emits no bytes. Let S(s) be 2 plus the UTF-8 byte
length of s, plus one for each quote, backslash or short-escaped C0 control
(backspace, tab, LF, form-feed, CR), plus five for each other C0 control. This
is the RFC-8785 quoted-string length, without Unicode normalization. Metadata
scanning to obtain it occurs only after normalize.metadata is admitted. For
an empty array A=2; otherwise A=1+n+sum(element lengths), where n is its
element count. For an empty object O=2; otherwise
O=2+sum(S(member name)+1+member value length)+(n−1). Null length is 4,
a hex digest string length is 66, and a decimal count string length is
2+digits(count). Apply these equations ONLY to the closed templates in the
model definition using admitted lengths/counts and fixed discriminators.
Checked arithmetic must succeed before a narrowed charge or allocation.

These equations price the existing sole canonical writer; they are not an
alternative serializer or an assumed structural-price API. The admitted
normalize.hash amount is exact, not an upper reservation. Encoding occurs
once only after that charge succeeds; its returned byte count must equal the
analytical amount before its digest can be admitted. A disagreement is an
internal failure, never success, recharging or an expanded-hash fallback.
No price pass encodes the node into a sink to discover b. No prefix checkpoint,
fragment digest or second canonical writer is used.

The owning normalize.declaration must succeed before dependencies are
prepared for its entry. A denied metadata/normal-form/identity-work event
does none of its named scan, formation, lookup, comparison or pricing work.
A denied hash does zero encoding/hash work for that node and zero dependent
work; earlier admitted preparation remains charged. No partial node/view is
publicly admitted. Universe/view selection strings are metadata-priced before
their first hash, with the same work meter. Selection-local reusable metadata
is charged on first use and each later lookup remains charged. Earlier-stage
refusals suppress every phase-5 identity event. Later conformance refusals
cannot replace earlier identity charge denial. Strict-reader replay has its
own operation admission; producer reuse is not a free reader proof.

## Incomplete record

The normative conformance record is exactly
`incomplete { limit_kind, limit, consumed, next_charge, charge_point }`.
Implementations must not expose any additional member in that record, including
a denial cause, fault-injection marker, retry hint or internal partial state.
`charge_point` is the named point whose charge was unavailable. `limit_kind` is
the first `ScalarLimitsV1` counter, in field order, whose amount is unavailable:
a semantic-size counter whose amount exceeds its limit, or a cumulative counter
whose consumed value plus its addition exceeds its limit. `limit` (`u64`) is
that counter's limit, `consumed` (`u64`) is its consumed value before the
denied charge, and `next_charge` (a nonnegative mathematical integer that may
exceed `u64::MAX`) is its semantic-size amount or cumulative addition. For the
`equality.plan` availability check, `next_charge` is the unavailable
reservation: `pair_events + 2` for `work_units` or one for `result_units`;
its canonical vector is TC-194 E23. A
denied charge changes no counter. An injected denial is not a counter
exhaustion: it is reported as defined under
[Qualification seam](#qualification-seam), as if the `work_units` limit were
`w`, so `limit = consumed = w` regardless of the configured limits. Because
decimal scales are `u32` and integer-division operands are materialized values,
no decimal or integer-division amount exceeds `u64::MAX`; a `next_charge`
greater than `u64::MAX` arises from unit integer power (TC-187 U13).

## Qualification seam

NFR-071 fault injection may deny the exact next charge at any named point. An
injected denial at point `P` returns exactly
`incomplete { limit_kind: work_units, limit: w, consumed: w, next_charge: n,
charge_point: P }`, where `w` is the work units consumed before `P` and `n` is
the `work_units` addition `P` requires (one, the `equality.plan`
reservation, or the stated amount of `equality.plan-form`,
`collection.member-walk`, `collection.member-test` or `dispatch.select`, or
the stated `work_units` addition of a population admission or model
normalization row). A canonical exact-bound run permits every charge through result
retention; its one-less partner denies the final named charge. Implementations
may use a more efficient internal algorithm, but cannot omit, reorder or change
normative charges or expose internal partial state.
