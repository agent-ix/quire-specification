---
id: TC-187
title: "Quantity and unit conversion"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-142
    type: verifies
---

# TC-187: Quantity and unit conversion

## Description

Evaluate compatible exact/lossy conversions and incompatible or affine-unit operations.

## Test Procedure

Select language `ix:native`, edition `1-draft`, and the
`quire.value.complete/v1` definition through
[`complete-value-lock.json`](../../proposals/quire-v1/definitions/complete-value-lock.json),
including `quire.value.compound-unit/v1`. Use distinct I04 semantic-node keys for length `L`, time `T`
and temperature `Theta`. Define canonical roots metre `m`, second `s` and kelvin `K`; centimetre `cm`
targeting `m` with scale `1/100`; millisecond `ms` targeting `s` with scale `1/1000`; inch `in` targeting `m` with scale `127/5000`;
degree Celsius `degC` targeting `K` with scale `1` and canonical offset `5463/20`; and
degree Fahrenheit `degF` targeting `K` with scale `5/9` and offset
`45967/180`. Also define mass `M` with root `kg`; distinct derived dimension
nodes `Torque` and `Energy`, both with terms `{ L: 2, M: 1, T: -2 }` and roots
`N_m` and `J`; derived dimension `Area` with terms `{ L: 2 }` and root `m2`;
`u1` targeting `K` with scale `1` and offset `10`; `u2` targeting `u1` with
scale `1` and offset `-10`; `u3` targeting `degC` with scale `1` and offset
`0`; `rev` targeting `m` with scale `-1` and offset `0`; and `cm2` targeting `m2` with scale `1/10000` and offset `0`. In the vectors
below, `L(i,d,e,o,w,r)` abbreviates `ScalarLimitsV1 { integer_bits: i,
decimal_digits: d, scale_expansion: 0, text_input_bytes: 0, text_scalars: 0,
normalized_scalars: 0, unit_edges: e, value_occurrences: o, work_units: w,
result_units: r }`. Execute:

| Vector | Operation | Expected value/disposition |
| --- | --- | --- |
| U01 | Convert `250 cm` to `m`; add to `1 m` | `5/2 m`; sum `7/2 m`, exact and dimension `L` |
| U02 | Normalize `(L*T)/T` and `L^0` | respectively `L` and the empty dimension; zero exponents are absent from each retained sorted map |
| U03 | Add `1 m` to `1 s` where `s` has time dimension `T` | `refused { code: ill_typed }` for incompatible dimensions before arithmetic |
| U04 | Convert `0 degC` and `32 degF` to `K` | both equal exact `5463/20 K` |
| U05 | Add, subtract, multiply, divide or raise a `degC`/`degF` value to a power | `refused { code: ill_typed }` because nonzero-offset points have no implicit difference-unit arithmetic |
| U06 | Add to the `L` graph `unit zero : L = 0 * m;` | `invalid_package`/`invalid-value` naming `zero` and its scale, before any conversion value is exposed (FR-142-AC-15) |
| U07 | Convert `1 in` to decimal metres at scale two | `exact`: refused; `nearest-even`: `(3,2) m` plus `{ exact_numerator: 127, exact_denominator: 5000, rounded_coefficient: 3, rounded_scale: 2, mode: nearest-even }` |
| U08 | Convert `-2 m`, `2 m`, `-3 m`, `3 m` into target Decimal coefficient domain `[-2,2]`, scale zero | `-2` and `2` succeed; one-below/one-over refuse atomically |
| U09 | Use a base-dimension name/shape equal to `L` but a different I04 semantic-node key | `refused { code: ill_typed }` as incompatible despite structural resemblance |
| U09b | Declare a distinct unit key under `L` with scale one and convert it to `m` | conversion succeeds exactly while source and target unit identities remain distinct |
| U10 | Convert `100 cm` to `m` with `ScalarLimitsV1 { integer_bits: 8, decimal_digits: 0, scale_expansion: 0, text_input_bytes: 0, text_scalars: 0, normalized_scalars: 0, unit_edges: 1, value_occurrences: 1, work_units: 6, result_units: 1 }`; then deny `unit.result-retain` | the exact tuple returns `1 m`, with the largest amount `max(bits(100) + bits(1), bits(1) + bits(100)) = 8` at the multiply event; the denied named final charge returns incomplete without a partial quantity |
| U11a | Add `unit m_alt : L = 1;` beside `unit m : L = 1;` | `invalid_package`/`conflicting-definition` naming `L`, `m` and `m_alt` (FR-142-AC-15) |
| U11b | Declare `unit m : L = 1 * cm;` and `unit cm : L = 1/100 * m;` (no unit is targetless) | `invalid_package`/`definition-cycle` retaining the edges `m -> cm`, `cm -> m`, starting at `m`, the least unit by source position (name order would start at `cm`); there is no missing-root cause for a nonempty dimension (FR-142-AC-15) |
| U11c | Add `unit x : L = 1 * nothing;` | `missing_declaration`/`missing-name` naming `x` and `nothing` (FR-142-AC-15) |
| U11d | Declare `unit cm : L = 1/100 * s;` with `s` a unit of `T` | `invalid_package`/`incompatible-definition` naming `cm`, `s` and the two dimension nodes (FR-142-AC-15) |
| U11e | Declare `unit m : L = 2;`; then `unit m : L = 1 + 1;` | each `invalid_package`/`invalid-value` naming `m` with its scale and offset (FR-142-AC-15) |
| U11f | Add `unit u_self : L = 1 * u_self;` | `invalid_package`/`definition-cycle` retaining the edge `u_self -> u_self` (FR-142-AC-15) |
| U11h | `unit z : L = 0 * nothing;` | `invalid_package`/`invalid-value` (row 1 before row 2 within one unit) (FR-142-AC-15) |
| U11i | In dimension `L`, `unit a : L = 1 * nothing;` declared before `unit b : L = 0 * m;` | `missing_declaration`/`missing-name` naming `a` (the earliest unit is processed first) (FR-142-AC-15) |
| U11j | In dimension `L`, `unit m_alt : L = 1;` and `unit u_self : L = 1 * u_self;` beside `unit m : L = 1;` | `invalid_package`/`conflicting-definition` (row 4 before row 6) (FR-142-AC-15) |
| U11k | Dimension `L` declared before `T`: in `L`, `unit u_self : L = 1 * u_self;` with no other unit; in `T`, `unit z : T = 0;` | `invalid_package`/`definition-cycle` for `L`; `T` is not examined (FR-142-AC-15) |
| U11g1 | `dimension L;` `dimension Area = L ^ 2;` `unit m2 : Area = 1;` | `missing_declaration`/`missing-name` at `L`'s turn, retaining `L` and the unit `m2` (Area's map has `L^2`) (FR-142-AC-16) |
| U11g2 | `dimension L;` `dimension Area = L ^ 2;` `dimension Vol = Area * L;` `unit m3 : Vol = 1;` | `missing_declaration`/`missing-name` retaining `L` and `m3` (Vol's map is `L^3`); `Area`, a derived dimension without a unit, is not refused (FR-142-AC-16) |
| U11g3 | `dimension L;` `dimension N = L / L;` `unit one : N = 1;` | admitted: `N` is the dimensionless dimension with the empty map, `L` is carried by no unit-bearing dimension (FR-142-AC-16) |
| U11g4 | `dimension L;` `dimension Area = L ^ 2;` `dimension Vol = Area * Area;` `unit m4 : Vol = 1;` | `missing_declaration`/`missing-name` retaining `L` and `m4` (Vol's map is `L^4`) (FR-142-AC-16) |
| U11l | The U11b cycle with `unit cm : L = 1/100 * m;` declared before `unit m : L = 1 * cm;` | `invalid_package`/`definition-cycle` retaining `cm -> m`, `m -> cm` from the least unit by source position, `cm` (FR-142-AC-15) |
| U11m | `dimension Area = L ^ 2;` `dimension L;` `unit a : Area = 1 * b;` `unit b : Area = 1 * a;`; then `dimension L;` `dimension Area = L ^ 2;` and the same units | `invalid_package`/`definition-cycle` at `Area` retaining `a -> b`, `b -> a` (`Area` is processed first, `L` is not examined); then `missing_declaration`/`missing-name` at `L` (`L` is processed first) (FR-142-AC-16) |
| U11n | `dimension A = B;` `dimension B = A ^ 2;` `unit a : A = 1;` | `invalid_package`/`definition-cycle` at `A`'s turn, retaining the term edges `A -> B`, `B -> A` from `A`, declared first; `a` is not examined (FR-142-AC-17) |
| U11o | `dimension L;` `dimension D = D * L;` `unit m : L = 1;` | `invalid_package`/`definition-cycle` at `D`, retaining `D -> D`; `L` has a unit, so it is not refused (it is processed first and admitted) (FR-142-AC-17) |
| U11p | `dimension L;` `dimension D = L * K;` `unit m : L = 1;` | `L` is admitted at its turn; `D` then refuses `missing_declaration`/`missing-name` retaining `D` and the term `K` (FR-142-AC-17) |
| U11q | Document 1 declares `dimension T;` and `unit z : T = 0;`; document 2 declares `dimension L;` and `unit u_self : L = 1 * u_self;`; the request lists document 1 before document 2; then the request lists document 2 before document 1 | the first run: `invalid_package`/`invalid-value` at `z` (`T` is processed first); the second run: `invalid_package`/`definition-cycle` retaining `u_self -> u_self` (`L` is processed first) (FR-142-AC-17) |
| U11r | `dimension L;` imported from package P (with `unit m : L = 1;` there) and local `unit cm : L = 1/100 * m;` | admitted: the imported `m` is a terminal root in the same node; the imported dimension `L` takes its turn before `cm` (rule of the turn list), where `cm` is the only local unit and passes rows 1 to 6 (FR-142-AC-18) |
| U11s | Imported `dimension L;` with `unit m : L = 1;` in P; local `unit a : L = 1 * b;` and `unit b : L = 1 * a;` | `invalid_package`/`definition-cycle` at node `L`, retaining `a -> b`, `b -> a`; P's `m` is not re-checked (FR-142-AC-18) |
| U11t | The U11r imports; local `unit m2 : L = 1;` | `invalid_package`/`conflicting-definition` retaining the imported root `m` and `m2` (FR-142-AC-18) |
| U11u | The U11r imports; local `unit z : L = 0 * m;` | `invalid_package`/`invalid-value` at `z` (FR-142-AC-18) |
| U11w | `dimension A = B * K;` `dimension B = A;` | `missing_declaration`/`missing-name` retaining `A` and `K` (the unknown term is checked before the cycle) (FR-142-AC-17) |
| U11x | `dimension L;` `unit m : L = 1;` `dimension E = D;` `dimension D = L * K;`; then the same with `unit e : E = 0;` added | `missing_declaration`/`missing-name` retaining `D` and `K` at `D`'s turn (`E`'s turn does not look through `D`); then `invalid_package`/`invalid-value` at `e` (`E`'s turn comes first and its unit rows run) (FR-142-AC-17) |
| U11y | Imported `dimension L;` with no unit in P; local `dimension Area = L ^ 2;` and `unit m2 : Area = 1;` | `missing_declaration`/`missing-name` retaining `L` and `m2`, at `L`'s turn placed before `Area`'s declaration (FR-142-AC-16) |
| U11z1 | Imported package P declares `dimension L;` and `dimension Area = L ^ 2;` with no unit; local `unit m2 : P.Area = 1;` | `P.Area` takes its turn (reached by the unit's type) and passes; `P.L` takes its turn next (reached through `Area`'s terms): `missing_declaration`/`missing-name` retaining `P.L` and `m2` (FR-142-AC-16) |
| U11z2 | `dimension X = P.L * Q.T;` `unit x : X = 1;` and, declared later, `unit t : Q.T = 0 * Q.s;`, with `P.L` unitless in P, `Q.T` a dimension of Q with unit `Q.s` | the walk reaches `P.L` then `Q.T` at `X`'s declaration, before the later unit: `P.L`'s turn comes first, `missing_declaration`/`missing-name` retaining `P.L` and `x`; the zero scale of `t` is not reported (FR-142-AC-18) |
| U11z3 | `dimension E = A;` `unit e : E = 0;` `dimension A = B;` `dimension B = A;` | `invalid_package`/`invalid-value` at `e` (`E`'s turn is first; `E` is not a member of the cycle, so it is not refused for it, has no map, and its unit rows run); with `unit e` removed, `invalid_package`/`definition-cycle` at `A`, retaining `A -> B`, `B -> A` (FR-142-AC-17) |
| U12 | Multiply `2 m * 3 s`, divide `6 m / 2 s`, square `2 m`, and divide `5 m / 5 m` | exact values `6`, `3`, `4`, `1` under canonical compound units `m*s`, `m*s^-1`, `m^2`, and the empty dimensionless unit respectively |
| U13 | Raise `2 m` to integer power `18446744073709551616` with `ScalarLimitsV1 { integer_bits: 64, decimal_digits: 0, scale_expansion: 0, text_input_bytes: 0, text_scalars: 0, normalized_scalars: 0, unit_edges: 0, value_occurrences: 1, work_units: 5, result_units: 1 }` | `incomplete { limit_kind: integer_bits, limit: 64, consumed: 2, next_charge: 36893488147419103232, charge_point: unit.rational-arithmetic }` before the power is computed, since the amount is `abs(n) × maxparts(2) = 2^64 × 2`; `next_charge` exceeds `u64::MAX` and is reported exactly |
| U14 | Under `L(0,0,0,0,0,0)`, add `1 m + 1 s`, add `1 degC + 1 s`, add `1 degC + 1 degC`, divide `1 degC / 0 degC`, and add `1 cm + 1 m` | each returns `refused { code: ill_typed }` with no charge; the static order among incompatible dimensions, affine arithmetic and distinct units is informative only, while `1 degC / 0 degC` is refused statically as affine arithmetic rather than reaching the runtime zero divisor |
| U15 | Convert `1 in` to `cm` under `L(15,0,2,1,9,1)`, then under `L(7,0,1,1,9,1)` | exact `127/50 cm` after one read, two edges, four rational events (amounts 14, 15, 15 and 14, the add of offset `0/1` to `127/5000` being `max(bits(127) + bits(1), bits(0) + bits(5000)) + 1 = 15`), target admission and retention; then `incomplete { limit_kind: unit_edges, limit: 1, consumed: 1, next_charge: 2, charge_point: unit.edge }`, because both edges are charged before the first rational event |
| U16 | Evaluate U07 under `L(15,5,1,1,6,1)`; repeat `exact` and then `nearest-even` under `L(15,5,1,1,4,1)` | `exact` refuses after one read, one edge and two rational events (four work units), before `unit.target-domain`; `nearest-even` returns U07's value after `unit.target-domain` with `integer_bits = sbits(127,2) = 14` and `decimal_digits = sdigits(127,2) = 5`; under the second tuple `exact` is still refused, not incomplete, while `nearest-even` returns `incomplete { limit_kind: work_units, limit: 4, consumed: 4, next_charge: 1, charge_point: unit.target-domain }` |
| U17 | Convert `3 m` into U08's target under `L(2,1,0,1,2,0)`, then under `L(2,1,0,1,1,0)` | refused by membership after `unit.target-domain` and before `unit.result-retain`; then `incomplete { limit_kind: work_units, limit: 1, consumed: 1, next_charge: 1, charge_point: unit.target-domain }` |
| U18 | Convert `1 m` into target `Decimal[0,1;0,4294967295;exact]` under `L(18446744073709551615,64,0,1,2,1)` | `incomplete { limit_kind: decimal_digits, limit: 64, consumed: 0, next_charge: 4294967296, charge_point: unit.target-domain }`; the amount is `sdigits(1,4294967295) = 4294967296`, and the retained coefficient `10^4294967295` is never materialized |
| U19 | Add `2 m + 3 m` under `L(4,0,0,2,5,1)`, then under `L(4,0,0,2,4,1)` | `5 m` after the add event with `max(bits(2) + bits(1), bits(3) + bits(1)) + 1 = 4`; then `incomplete { limit_kind: work_units, limit: 4, consumed: 4, next_charge: 1, charge_point: unit.result-retain }`: the rational target charges `unit.target-domain` with one work unit and no size amount |
| U20 | Under `L(0,0,0,0,0,0)`, add `1 u1 + 2 u1` and multiply `1 u3 * 1 m`; add `1 u2 + 2 u2` under `L(4,0,0,2,5,1)` | `u1` and `u3` are affine by composed offset and return `refused { code: ill_typed }` with no charge; `u2`'s offsets compose to zero, so it is not affine and the sum is exact `3 u2` with no edge |
| U21 | Raise `0 m` to integer power `0` under `L(1,0,0,1,4,1)`; raise `0 m` to `-1` under `L(1,0,0,1,1,0)`, then under `L(1,0,0,1,0,0)` | exact dimensionless `1` after read, power event, target admission and retention; undefined after one `unit.identity-read`; then `incomplete { limit_kind: work_units, limit: 0, consumed: 0, next_charge: 1, charge_point: unit.identity-read }` |
| U22 | Convert `1 N_m` to `J` under `L(0,0,0,0,0,0)`; convert compound `4 m^2` to `m2` under `L(3,0,0,1,3,1)`; convert compound `4 m^2` to `cm2` under `L(17,0,1,1,6,1)`, then under `L(16,0,1,1,6,1)` | `refused { code: ill_typed }` with no charge, because the dimension nodes differ despite equal base-dimension maps; exact `4 m2` after one read, target admission and retention with no edge; exact `40000 cm2` after one read, one reverse edge, its two rational events, target admission and retention; then `incomplete { limit_kind: integer_bits, limit: 16, consumed: 5, next_charge: 17, charge_point: unit.rational-arithmetic }` at the divide event, whose amount is `bits(4) + bits(10000) = 17` after the subtract event's 5 |
| U23 | Order `1 degC < 2 degC` under `L(15,0,2,2,9,1)`, then under `L(15,0,2,2,8,1)`; order `1 rev < 2 rev` and test `1 degC == 1 degC`; under `L(0,0,0,0,0,0)`, order `0 degC < 32 degF` and test `0 degC == 32 degF` | `true` after two reads, two edges (left then right), four rational events (the offset add `1 + 5463/20` has amount `max(bits(1) + bits(20), bits(5463) + bits(1)) + 1 = 15`) and retention, with no `unit.target-domain` or `equality.*` charge; then `incomplete { limit_kind: work_units, limit: 8, consumed: 8, next_charge: 1, charge_point: unit.result-retain }`; `false`, because root values are `-1` and `-2`; `true`; both distinct-unit comparisons return `refused { code: ill_typed }` with no charge |
| U24 | Add `1 cm + 2 cm` under `L(4,0,0,2,5,1)` | exact `3 cm` with no `unit.edge`: two reads, one addition event in `cm`, target admission and retention |
| U25 | Convert `1 N_m` to compound `kg*m^2*s^-2` under `L(1,0,0,1,3,1)`, then convert that result to `J` under the same tuple | both explicit steps are admitted by equal base-dimension maps and each makes one read, target admission and retention with no edge; the result is exact `1 J` |
| U26 | Convert `1 u2` to `u1` under `L(6,0,3,1,12,1)`, then under `L(6,0,2,1,12,1)` | exact `-9 u1` after one read, three edges (`u2` to `u1`, `u1` to `K`, `K` to `u1`), six rational events, target admission and retention; then `incomplete { limit_kind: unit_edges, limit: 2, consumed: 2, next_charge: 3, charge_point: unit.edge }`, because no common-ancestor shortcut applies |
| U27 | Divide `1 cm / 0 cm` under `L(1,0,0,2,2,0)`, then under `L(1,0,0,2,1,0)` | undefined after both `unit.identity-read`s and before any `unit.edge`; then `incomplete { limit_kind: work_units, limit: 1, consumed: 1, next_charge: 1, charge_point: unit.identity-read }` |
| U28 | Multiply `1 cm * 1 in` under `L(20,0,2,2,11,1)`, then under `L(13,0,2,2,11,1)`, then under `L(19,0,2,2,11,1)` | exact `127/500000 m^2` after two reads, the `cm` edge then the `in` edge, the `cm` events then the `in` events, the multiplication event (amounts 8, 9, 14, 15 and `max(bits(1) + bits(127), bits(100) + bits(5000)) = 20`), target admission and retention; then `incomplete { limit_kind: integer_bits, limit: 13, consumed: 9, next_charge: 14, charge_point: unit.rational-arithmetic }` at the right operand's first event; then `incomplete { limit_kind: integer_bits, limit: 19, consumed: 15, next_charge: 20, charge_point: unit.rational-arithmetic }` at the multiplication event |
| U29 | Convert `1 in` into an integer target with domain `[-2,2]` under `exact` and then `nearest-even`, each under `L(15,0,1,1,4,1)`; convert `3 m` into that integer target under `L(2,0,0,1,2,0)`, then under `L(2,0,0,1,1,0)` | `exact` returns `refused` after one read, one edge and two rational events, before `unit.target-domain`; `nearest-even` rounds to `0` and returns `incomplete { limit_kind: work_units, limit: 4, consumed: 4, next_charge: 1, charge_point: unit.target-domain }`; `3 m` is refused by integer-domain membership after `unit.target-domain` and before `unit.result-retain`; then `incomplete { limit_kind: work_units, limit: 1, consumed: 1, next_charge: 1, charge_point: unit.target-domain }` |
| U30 | Check `f(d: Quantity<ms>)` called as `f(3 ms)` and as `f(3 s)`, then `d: Quantity<T>`, `d: Quantity<nothing>` and a bare `d: ms` (FR-142-AC-11) | `f(3 ms)` admitted; `f(3 s)` refused `ill_typed`/`type-mismatch` at the argument; each of the three unit-namespace misses refused `missing_declaration`/`missing-name` at the name; no charge on any check |
| U31 | Check `f(d: Quantity<ms>): Quantity<ms>` returning `d`, then returning `convert<Quantity<s>>(d)` (FR-142-AC-12) | the first checks and names the node of `ms` as the type of `d` and the result; the second refused `ill_typed`/`type-mismatch` at the returned expression |
| U32 | Declare a unit `ms` and a record `ms` and use `ms` in a type position and as `Quantity<ms>`; declare two units named `ms` in one linked package and use `Quantity<ms>`; declare `record Quantity` (FR-142-AC-13) | the first checks, the type position meaning the record and `Quantity<ms>` the unit; the second refused `ambiguous_declaration`/`ambiguous-name` at `Quantity<ms>`; the third refused `invalid_syntax` at the name |
| U33 | Lower `f(3 ms)` and a domain-package operation parameter `typeRef` naming `ms` (FR-142-AC-14) | a `literal` of `value_kind` `quantity` typed by the node of `ms`, and a member type that is that node; the exact wire vectors are TC-233 QL-01 to QL-04 and TC-280 MU-01 to MU-06 |
| U34 | A unit of compilation declaring `dimension Time; unit s : Time = 1; unit ms : Time = 1/1000 * s;` and `model M = "acme/rpc" digest "…";`, whose admitted domain package declares base dimension `Rate`, root unit `us` of `Rate` and a unit `ms` of `Rate` (`target` `us`, `scale` `1000`); check `f(d: Quantity<ms>)`, `g(d: Quantity<M::ms>)` and `unit tick : Rate = 1/2 * M::us;`; then the same package with a base dimension `Time` added; then a second admitted package `acme/flow` also declaring a base dimension `Rate`, with the source declaring no `Rate`; then `acme/rpc` with a base dimension `Length` and no unit added, beside a source `dimension Area = Length ^ 2; unit m2 : Area = 1;` (FR-142-AC-19) | the first check admits all three: `d` of `f` is typed by the source `ms` node, `d` of `g` by the package unit node `ix://acme/rpc/unit/ms`, which carries no declaration and whose preimage has `qualified_declaration` `["ms"]` under the `acme/rpc` `ModelOwner`, a key distinct from the source `ms` node's, and `tick` by the package dimension `Rate`, with no `ambiguous_declaration`; the second refuses `conflicting_declaration`/`duplicate-declaration` at the source `dimension Time` span, naming the source declaration and `ix://acme/rpc/dimension/Time` at `/lock/model_selections/0`; the third refuses `conflicting_declaration`/`duplicate-declaration` at `/lock/model_selections/1`, naming both `Rate` declarations; the fourth refuses `missing_declaration`/`missing-name` by the base-dimension rule at `/lock/model_selections/0`, naming `ix://acme/rpc/dimension/Length` and `m2` |

For U01, U04 and U07, compare the complete exact affine composition through
the canonical unit, result unit identity, normalized dimension map, original
unit/value provenance and any FR-140 `DecimalLoss`.

Generate finite acyclic same-dimension unit graphs with one canonical root,
nonzero reduced rational scales, rational offsets and bounded quantity values.
The oracle composes exact affine maps through the root, applies FR-140 only at
an explicit decimal target, and normalizes compound dimensions/units. Shrink
edge count, rational magnitude, exponent count and value magnitude while
preserving the selected valid or invalid topology. Mutate every topology rule,
dimension key, zero scale, affine-operation restriction and applicable named
unit charge independently.

## Expected Results

Every vector returns the exact quantity or typed refused/incomplete disposition
in the table. No conversion is inferred from equal names or scales. A failure
publishes neither canonical nor target partial values, and exhaustion leaves
independent sibling conversions unchanged.
