---
id: TC-280
title: "Model-owned member types"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322-AC-80
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-82
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-98
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-99
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-102
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-103
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-104
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-105
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-106
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-107
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-118
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-149-AC-23
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-149-AC-24
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-116
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-144-AC-27
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-144-AC-28
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-154
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-340
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-341
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-342
    type: verifies
---

# TC-280: Model-owned member types

## Description

Show that a reader derives the type of a `field` or `operation` member whose
declaring node is a model declaration node from the lock-selected domain
package, as FR-322's "Model-owned members" states: it admits the selected
document as evidence, reads the declaring node's `ModelOwner` from its
`owner` member, looks up the declaration that owner names, resolves the member name among
the declaring type's exposed effective members, own and inherited, and derives
the member type from the resolved record.

## Preconditions

- `proposals/checked-package-v2/domain-package-acme-orders.json` is the
  selected evidence: the FCD semantic IR 2.0.0 domain package `acme/orders`
  `1.0.0` (FR-154), authored by hand with FCD-minted identities (types
  `ix://acme/orders/<Id>`, members `<type>/<member>`, relationships
  `ix://acme/orders/relationship/<Name>-<verb>-<Target>`). Its identity and version `1.0.0`
  are the row every published selection of `acme/orders` carries.
  Each operation redefinition is the redefining operation's
  `Operation.redefines` member (FR-154): `Sub/size` and `SubSub/size` name
  `Order/size`, and `Sub/scaled` names `Order/scaled`.
- `proposals/checked-package-v2/model-member-type-vectors.json` carries the
  `domain_package` member, the projection of the document's declarations
  that FR-154 intake reads, derived from the document (a test derives it and
  compares) (`typeRef`, `interfaceFeatures`,
  value types with the `Int[lo, hi]` they are bound as (TC-196, TC-197) and with
  the `Text[min, max; profile]` they are bound as, presence, multiplicity, supertypes
  and redefinitions), the model declaration node vectors M1 to M12, the member
  cases, each recorded with its criterion, the `operation_redefinition_cases`
  (MA-16 to MA-18, each a patch of the document with its expected result; MA-19 is the
  consumer-scope case that no patch of the document expresses),
  the wire cases and the selection cases. The selection cases patch the document, not the projection.
  The value type `OrderTotal` is the alias FR-095 mints for `Order`'s
  constrained field `total` (`<Name><Field>`), with its constraints under
  `constraint/Order-total-min` and `-max`.
- The wire cases are hand authored: each is a single model declaration
  node, a shape QSL emits only inside a package, as emitter output would be. The model declaration nodes of the positive
  fixtures are QSL emitter output (TC-233).
- `proposals/checked-package-v2/model-owner-form-vectors.json` carries a
  second projection, `acme/runs`, in the same shape and with the lists
  `processes`, `state_machines` and `persistence_interfaces` beside
  `object_types` (each declaration has `node`, `supertypes`, `fields` and
  `operations`). It declares the object type `Counter` (field `value`,
  operation `bump`), the systems interface `Plug`, the process `Flow` (field
  `n`, operation `go(step: Integer): Integer`), the process `SubFlow`
  specializing `Flow` (field `m`), the state machine `Gate` (field `open`,
  operations `toggle` and `check(): Boolean`) and the persistence interface
  `Store` (operation `save`), and the `populations` list, whose entries are FCD populations
  (`identity`, `displayName`, `kind`, `members`, `extent`, `origin`): the
  populations `counters` and `archive`, whose `identity` is the IR node
  identity. Its model
  declaration node vectors `P1` to `P6`
  name the declarations in that order (`Counter`, `Flow`, `SubFlow`, `Gate`, `Store`,
  `Plug`; `P2` is `Flow`), and `P7` and `P8` the populations `counters` and
  `archive`. The tests build each wire node of its owner cases from the case. The
  owner, frame field, anchor and clause signature cases below are read
  against this projection, and the receiver frame cases against the published
  packages (FR-340-AC-19, FR-340-AC-20).
- For the `acme/orders` projection the lock selects the identity `acme/orders` only; for the `acme/runs` projection it selects the identity `acme/runs` only.

## Test Procedure

Validate each model declaration node vector against the closed schema.
Run FR-322's owner presence rule, the
owner join and step 2 over
each wire case; each wire case's node carries the `owner` its row names. For each member
case, look up the declaration the declaring node's owner names in the
selected domain package,
resolve the member and derive its type or refusal, and compare with the
recorded expectation. An operation case also carries its call's receiver type
and argument types, and a reference type names its target by the
model declaration node id. For each selection case, run FR-322's selection
admission checks over the supplied documents and compare with the recorded
result. For each frame field case, look up the declaration the declaring node's
owner names and
resolve the name among its exposed fields (FR-340 "Field entries"). For each
anchor case, resolve the operation among the context's exposed operations
and require it to be declared by the context itself (FR-342). For each clause
signature case, derive the list `self`, then the result for a postcondition
of an operation with one, then the operation's parameters (FR-341), and
compare it with the case's parameter types.

For the `acme/runs` projection, run the owner join over each owner case's
wire node (a process, a state machine or a persistence interface owned by the
named declaration), resolve each frame field case among the exposed fields of
its declaring process or state machine and refuse a persistence interface or
systems interface as a frame field declaration, resolve each anchor case's
operation among the operations its context declares itself and refuse a
context that is none of an object type, a process and a state machine, and
derive each clause signature case with `self` as `Reference<C>` over the
process or state machine.

## Vectors

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| MT-01 | `field` `total` on M1 (`Order`), typed value type `OrderTotal`, `Integer` bound to `[0, 9]` | Resolves `Order/total`; type `Int[0, 9]` | FR-322-AC-31 |
| MT-02 | `field` `total` on M5 (`Sub`, supertype `Order`) | Resolves the inherited `Order/total`; type `Int[0, 9]` | FR-322-AC-30 |
| MT-03 | `field` `note`, optional `Boolean` `[1, 1]` | `Option<Boolean>` | FR-322-AC-31 |
| MT-04 | `field` `lines`, `Integer` `[0, 3]`, unordered, unique | `Set<Integer>[0, 3]` | FR-322-AC-31 |
| MT-05 | `field` `history`, `Integer` `[0, 5]`, ordered, not unique | `Sequence<Integer>[0, 5]` | FR-322-AC-31 |
| MT-06 | `field` `invoice`, typed object type `Invoice` | `Reference<Invoice>` over M3 | FR-322-AC-31 |
| MT-07 | `operation` `size` on M1, receiver `Reference<Order>` | Resolves `Order/size`; type `Integer` | FR-322-AC-31 |
| MT-08 | `operation` `size` on M5, which `Sub/size` redefines | Resolves `Sub/size`; type `Integer` | FR-322-AC-30, FR-154-AC-16 |
| MT-09 | `operation` `scaled` on M5, redefined with a renamed parameter, called with an `Integer` argument | Resolves `Sub/scaled`; type `Integer` | FR-322-AC-30 |
| MT-10 | `operation` `size` on M7 (`SubSub`, under `Sub`), which both `Sub/size` and `SubSub/size` redefine | Resolves the most derived `SubSub/size`; type `Integer` | FR-322-AC-30 |
| MT-11 | `operation` `scaled` on M1 with an `Integer` argument | Resolves `Order/scaled`; type `Integer` | FR-322-AC-31 |
| MT-12 | `field` `floor`, native `Integer` carrying a `min` constraint | `Integer` | FR-322-AC-31 |
| MT-13 | `field` `tags`, `Integer` `[0, unbounded]`, unordered, unique | `Set<Integer>` with no bounds | FR-322-AC-31 |
| MT-14 | `field` `pick`, `Integer` `[0, 1]`, unordered, unique | `Set<Integer>[0, 1]` | FR-322-AC-31 |
| MT-16 | `field` `code`, native `String` | `String` over `scalar_type`/`string`, distinct from `Text` | FR-322-AC-31 |
| MT-17 | `field` `ref`, native `UUID` | `UUID` over `scalar_type`/`uuid` | FR-322-AC-31 |
| MT-18 | `field` `at`, native `Timestamp` | `Timestamp` over `scalar_type`/`timestamp` | FR-322-AC-31 |
| MT-19 | `field` `gain`, native `Float64`; `field` `coarse`, native `Float32`; `field` `levels`, native `Float64` `[0, 3]`, unordered, unique | `Float64` over `scalar_type`/`float64`; `Float32` over `scalar_type`/`float32`; `Set<Float64>[0, 3]` | FR-322-AC-31 |
| MA-01 | `field` `missing` on M1 | `ill_typed`/`operator-ineligible` | FR-322-AC-30 |
| MA-02 | `field` `size`, an operation name | `ill_typed`/`operator-ineligible` | FR-322-AC-30 |
| MA-03 | `operation` `total`, a field name | `ill_typed`/`operator-ineligible` | FR-322-AC-30 |
| MA-04 | `field` `total` on M4, the relationship `Order-order-Order` | `ill_typed`/`operator-ineligible` | FR-322-AC-30 |
| MA-05 | `field` `total` on M2, `Order` owned by the unselected `acme/billing` | `missing_declaration`/`missing-selection` | FR-322-AC-29 |
| MA-06 | `field` `label`, typed native `Text` with no bounds | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| MA-07 | `field` `backlog`, `[1, unbounded]` | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| MA-08 | `operation` `reset`, with no result | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| MA-09 | `field` `code` on M6 (`Both`), declared by both supertypes `Tagged` and `Coded` | `ambiguous_declaration`/`ambiguous-name` | FR-322-AC-30 |
| MA-10 | `operation` `scaled` on M1 with no argument | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| MA-11 | `operation` `scaled` on M1 with a `Boolean` argument | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| MA-12 | `operation` `rate` on M8 (`Flow`, a `systems_interface`) | `ill_typed`/`operator-ineligible` | FR-322-AC-30 |
| MA-13 | `operation` `size` on M1 with receiver `Reference<Invoice>` | `ill_typed`/`operator-ineligible` | FR-322-AC-31 |
| MT-15 | `field` `sku` on M1, typed value type `Code`, `Text` bound to `[0, 64; nfc]` | Resolves `Order/sku`; type `Text[0, 64; nfc]` | FR-322-AC-44 |
| MA-14 | `field` `alias`, typed value type `Inverted`, `Text` bound to `[5, 2; nfc]` | `ill_typed`/`operator-ineligible` | FR-322-AC-44 |
| MA-15 | `field` `slug`, typed value type `Folded`, `Text` bound to `[0, 64; casefold]` | `ill_typed`/`operator-ineligible` | FR-322-AC-44 |
| MA-16 | `operation` `size` on M5, over the document with `Sub/size`'s `Operation.redefines` removed | `ambiguous_declaration`/`ambiguous-name`: `Order/size` and `Sub/size` are both exposed | FR-322-AC-30, FR-154-AC-16 |
| MA-17 | `operation` `size` on M5, over the document whose `Sub/size` carries no `Operation.redefines` and whose `Sub` lists the pair `Sub/size`, `Order/size` only in an extension payload, once with `required` false (MA-17a) and once with `required` true (MA-17b), the consumer admitting the extension | `ambiguous_declaration`/`ambiguous-name` both times: an extension declares no redefinition to intake | FR-154-AC-16 |
| MA-18 | The document with `Sub/size`'s `Operation.redefines` set to `ix://acme/orders/Order/missing` | Selection refused `missing_declaration`/`missing-name` at `Sub/size` and `Operation.redefines` | FR-154-AC-16 |
| MA-19 | The document whose `Sub` lists the pair `Sub/size`, `Order/size` in an extension payload with `required` true, supplied to a consumer that does not admit that extension | Document refused before intake `unknown_required_feature`/`unsupported-feature`, naming the extension identity and the consuming scope, at `Sub` and its `extensions` member | FR-154-AC-16 |
| MU-01 | `field` `window` on T1 (`Timer` of the `acme/timing` domain package, which no checked-package fixture selects), `typeRef` `ix://quire/unit/ms`, checked units `ms` and `s` | Resolves `Timer/window`; type `Quantity<ms>` | FR-322-AC-80 |
| MU-02 | `field` `waits` on T1, the same `typeRef`, `[0, unbounded]`, unordered, unique | `Set<Quantity<ms>>` | FR-322-AC-80 |
| MU-03 | `operation` `wait` on T1 with a `Quantity<ms>` argument | Resolves `Timer/wait`; result `Integer` | FR-322-AC-80 |
| MU-04 | `field` `ghost` on T1, `typeRef` `ix://quire/unit/ghost` | `missing_declaration`/`missing-name` | FR-322-AC-80 |
| MU-05 | `operation` `wait` on T1 with a `Quantity<s>` argument | `ill_typed`/`operator-ineligible` | FR-322-AC-80 |
| MU-06 | `operation` `wait` on T1 with an `Integer` argument | `ill_typed`/`operator-ineligible` | FR-322-AC-80 |
| MU-07 | `field` `budget` on T2 (`Call` of the admitted domain package `acme/rpc`, which declares base dimension `Rate`, root unit `us` of `Rate` and its own unit `ms` of `Rate`, `target` `us`, `scale` `1000`), `typeRef` `ix://quire/unit/acme/rpc/ms`, beside MU-01's source units `ms` and `s` of the source dimension `Time` | Resolves `Call/budget`; type `Quantity<M::ms>` (with `model M = "acme/rpc"`) over the package unit's `scalar_type`/`unit` node, which carries no `declaration`, has `owner` `{kind: "model", identity: "acme/rpc", node: "ix://acme/rpc/unit/ms"}`, a `qualified_declaration` `["ms"]`, and a node id distinct from the source `ms` node's, and depends exactly on the package `Rate` and `us` nodes; no `ambiguous_declaration` | FR-322-AC-99 |
| MU-08 | `field` `limit` on T2, `typeRef` `ix://quire/unit/ms` | Resolves `Call/limit`; type `Quantity<ms>` over the source unit's node, distinct from MU-07's | FR-322-AC-99 |
| MU-09 | `field` `stale` on T2, `typeRef` `ix://quire/unit/rpc/ms` | `missing_declaration`/`missing-name` | FR-322-AC-99 |
| MU-10 | `field` `x` on T2, `typeRef` `ix://quire/native/Clock` with FR-154's clock field shape, resolved as a `field` member; then a `field` application over `x` used as a value; then `x` as the clock operand of the atomic clock constraint `self.x >= 1 ms` in a guard | The member resolves to the clock type with no checked-package node; the value application refuses `invalid_timed_model`/`clock-read-outside-constraint`; the clock-constraint operand raises no such refusal | FR-322-AC-99 |
| MT-20 | `field` `blob`, native `Bytes`; `field` `blob2`, native `Bytes` carrying a `maxLength` constraint | Each: `Bytes` over one `scalar_type`/`bytes` node, distinct from `String` and `Text` | FR-322-AC-102 |
| MT-21 | `field` `meta`, native `JsonObject` | `JsonObject` over `scalar_type`/`json_object` | FR-322-AC-102 |
| MT-22 | `field` `blobs`, native `Bytes`, `[0, unbounded]`, unordered, unique | `Set<Bytes>` | FR-322-AC-102 |
| MT-23 | `field` `metas`, native `JsonObject`, `[0, 3]`, ordered, not unique | `Sequence<JsonObject>[0, 3]` | FR-322-AC-102 |
| MT-24 | `field` `metas`, native `JsonObject`, `[0, unbounded]`, unordered, unique; `field` `keys`, native `Bytes`, `[0, 4]`, unordered, not unique | `Set<JsonObject>`; `Bag<Bytes>[0, 4]`, each admitted with a canonical key (FR-144) | FR-322-AC-102, FR-144-AC-27 |
| MA-20 | `quire.op.bytes.concat` over a `bytes` operand and a `json_object` operand; `quire.op.integer.lt` over two `bytes` operands | `ill_typed`/`operator-ineligible` each | FR-322-AC-103 |
| MA-21 | `bytes` literal `value` `"00FF"`, `"0"`, `"0x00"`, `"00 ff"`; `json_object` literal `value` `"{\"b\":1,\"a\":1}"`, `"{\"a\": 1}"`, `"{\"a\":1.0}"`, `"{\"a\":9007199254740993}"`, `""`, `"{"`, `"{\"a\":1,\"a\":2}"` | `invalid_package`/`invalid-value` at the literal's `value`, each | FR-322-AC-103 |
| MA-22 | `bytes` literal `value` `"00FF"` whose `type` is the `uuid` node | `ill_typed`/`operator-ineligible`, not `invalid-value` | FR-322-AC-103 |
| MU-11 | The source declares `dimension Time; unit s : Time = 1; unit ms : Time = 1/1000 * s;`; `field` `at` on T2, native `Duration`; then `field` `at2` on T2, `typeRef` `ix://quire/unit/s`; then `field` `d` on T2, native `Duration`, compared with a `Quantity<ms>` field `m` | `at` has type `Quantity<s>` over the node of `s`, the node of `at2`; the comparison of `d` and `m` refuses `ill_typed` (FR-142's identical-unit rule) until `m` is converted to `s`, and `1500 ms` converted to `s` equals the `Duration` `3/2` | FR-322-AC-104 |
| MU-12 | `field` `at` on a type of the admitted domain package `acme/rpc` declaring base dimension `Time` and root unit `s`, native `Duration`, the source declaring no `s`; then the source also declaring `dimension Time; unit s : Time = 1`; then the package declaring `s` typed by a dimension `Length`; then neither declaring `s` | `Quantity<s>` over the package unit's node (`ix://quire/unit/acme/rpc/s`); the same, the package's `s`, with no refusal and a distinct type from the source `s`; `ill_typed`/`operator-ineligible`; `missing_declaration`/`missing-name` | FR-322-AC-104, FR-322-AC-116 |
| MU-14 | the unit of compilation declaring `dimension Time; unit s : Time = 1; dimension Speed = R::Time ^ -1; unit tenth : R::Time = 1/10 * R::s;` with `acme/rpc` (alias `R`) and `acme/flow` (alias `F`) each declaring base dimension `Time` and root unit `s`; read the unit graph | Three `scalar_type`/`dimension` nodes named `Time` with pairwise distinct node ids (one under a `SourceOwner`, two under the `ModelOwner`s `acme/rpc` and `acme/flow`, the package nodes carrying no `declaration` and a `qualified_declaration` `["Time"]`); each package `s` depends on its own `Time`; `Speed` is self-typed with the `acme/rpc` `Time` key at exponent `-1` as its one term; `tenth` is typed by the `acme/rpc` `Time` and targets the `acme/rpc` `s`; no `ambiguous_declaration`; the package is admitted | FR-322-AC-105 |
| MU-15 | `field` `a` on T2 typed `ix://quire/unit/acme/rpc/s` and `field` `b` typed `ix://quire/unit/acme/flow/s`, beside the source `s` of MU-14 | `Call/a` has type `Quantity<R::s>` over the `acme/rpc` unit node and `Call/b` has type `Quantity<F::s>` over the `acme/flow` unit node, two distinct `scalar_type`/`unit` nodes typed by two distinct dimension nodes; no `ambiguous_declaration` | FR-322-AC-105 |
| MU-16 | The document of FR-322-AC-106: `Call` with clock field `x` and optional clock field `w`, `Timer` with clock field `x` and data field `n` held by field `t`, operation `reset` with a `Clock` parameter `c`, operation `clk` with a `Clock` result; `field` members `x`, `t`, `w` resolved, then a `field` application over `x`, over `t.x` and over `w` used as a value, then the `Timer` record node read | The members resolve; `w` is `Option<Clock>`; each value-position application `invalid_timed_model`/`clock-read-outside-constraint`; the `Timer` node lists `n` only and no node stands for `x`, `t.x`, `w`, `c` or the result of `clk` | FR-322-AC-106 |
| MU-17 | T2's package with alias `D` (`target` `ix://quire/unit/acme/rpc/ms`), a sequence type with that `items`, a map type with that `values`, and a field `d` typed `D` | `d` is `Quantity<M::ms>`; the sequence `Sequence<Quantity<M::ms>>`; the map `Map<Text, Quantity<M::ms>>`; one `scalar_type`/`unit` node for `ms` and no node for `D`, the sequence or the map | FR-322-AC-107 |
| MU-18 | The document of FR-322-AC-109: `Call` with `s` (`Clock`, `{0, 3, true, false}`), `b` (`Clock`, `{0, 3, false, false}`), `ts` (`Timer`, `{0, 3, true, false}`), a sequence type `CS` over `Clock` and a map type `CM` over `Clock`; `s`, `b`, `ts` read as members, then a `field` application over each in a value position, then a clock constraint over `ts`'s elements; the same document with `Timer` held at `{1, 1}` and at `{0, 3, false, true}`; the same document with the maximum of `s` changed to `4` | `s`, `b`, `CS`, `CM` have no node; `ts` is the `collection_bounds` node over the sequence node over the `Timer` node listing `n` only; each value-position application `invalid_timed_model`/`clock-read-outside-constraint` and the constraint none; the `Timer` node is the same node in all three runs; the maximum change gives a different package identity | FR-322-AC-109, FR-322-AC-110 |
| MU-19 | `field` `a` on a type of `acme/rpc` (base dimension `Time`, root `s`, `min` of scale 60 to `s`), native `Duration`; `field` `b` on a type of the unit of compilation, native `Duration`, the source declaring `dimension Time; unit s : Time = 1`; then `a` compared with `b` in an equality | `a` is `Quantity<s>` over the package unit's node and `b` over the source unit's node; the comparison refuses `ill_typed` (two distinct units, FR-142) | FR-322-AC-116 |
| MU-20 | `field` `a` on a type of a package declaring `Time` with root `sec` and `s` of scale 60 to `sec`; the same with `s` of scale 1 and offset 5 to its root; the same with `s` reaching the root through `t` (scale 60 from `s` to `t`, scale 1/60 from `t` to the root); the same with the root `s` itself | `ill_typed`/`operator-ineligible`; `ill_typed`/`operator-ineligible`; admitted as `Quantity<s>`; admitted as `Quantity<s>` | FR-322-AC-104, FR-322-AC-116 |
| MU-21 | `field` `a` on a type of a package declaring no `s`, the source declaring `dimension Time; unit s : Time = 1`; then a package declaring `Time` and `s` while the source declares its own `Time` and `s`, both fields native `Duration` | `Quantity<s>` over the source unit's node; `Quantity<s>` over the package's own `s`, typed by the package's own `Time` | FR-322-AC-116 |
| MU-22 | the unit of compilation of MU-14 with `dimension R::Time ~ F::Time by 1 R::s = 1000 F::ms;` read as the unit graph; then the same with `dimension F::Time ~ R::Time by 1 F::s = 1 R::s;`; then with `acme/clock` (alias `G`, base `Time`, root `ms`) added, `dimension R::Time ~ G::Time by 1 R::s = 1000 G::ms;`, the same written `dimension G::Time ~ R::Time by 1000 G::ms = 1 R::s;`, and `dimension F::Time ~ G::Time by 1 F::s = 1000 G::ms;` | one more node, `scalar_type`/`dimension_equivalence`: self-typed, no `declaration`, with the `SourceOwner`, the two package `Time` keys in ascending node-id order and `root_scale` `1/1`, `dependencies` exactly those keys, one occurrence of role `anchor` and ordinal 0 ; the second spelling gives the same node; the unequal-root declarations give `root_scale` `1000/1` over `[acme/rpc, acme/clock]` in both spellings, and `1000/1` over `[acme/flow, acme/clock]`; the three `Time` nodes keep their ids; the strict reader admits the package and derives one class of the three | FR-322-AC-118 |
| MU-23 | the equivalence node of MU-22 with its ids descending, with both ids equal, with a unit key as an id, repeated as a second node, over two derived dimensions of unequal maps or of a `root_scale` the base roots do not fix, with one dependency, with a `semantic_type` of another node, with an owner outside the lock, with `root_scale` unreduced (`2000/2`), and the third node of the three-package cycle with `root_scale` `1001/1` | `invalid_semantic_graph` at the node, each | FR-322-AC-118 |
| MU-24 | the equivalence node of MU-22 with `root_scale` missing, with `root_scale` `numerator` `0`, with `numerator` `-1`, with `denominator` `0`, with only one id in `dimension_node_ids`, and with a `declaration` member added | `malformed_wire` at the node, each (the closed V2 schema refuses it before the graph) | FR-322-AC-118, FR-322-AC-139 |
| KS-01 | Equality of two `Bytes` values `00ff` and `00ff`, then `00ff` and `00fe`, `00ff` and `00ff00`, `` (empty) and `00`; `quire.op.bytes.length` of `00ff00` and of the empty sequence; `quire.op.bytes.concat` of `00ff` and `01` | `true`, `false`, `false`, `false`; `3` and `0`; `00ff01` | FR-149-AC-23, FR-322-AC-103 |
| KS-02 | Equality of `{"a":1,"b":[true,null]}` with itself, with `{"a":1,"b":[null,true]}`, with `{"a":2,"b":[true,null]}`, with `{"a":1}`, with `{"a":1,"b":[true,null],"c":{}}`; `{}` with `{}`; `[]` with `{}`; `3` with `3`; `null` with `{}` | `true`, `false`, `false`, `false`, `false`; `true`; `false`; `true`; `false` | FR-149-AC-23 |
| KS-03 | `Duration` `3/2` against `Duration` `3/2`, against `Duration` `2`, against the quantity `3/2 s`, and against `1500 ms` unconverted and then converted to `s` | `true`, `false`, `true`; the unconverted comparison `ill_typed` (FR-142), the converted `true` | FR-149-AC-23, FR-322-AC-104 |
| KS-04 | `Bytes` against `JsonObject`, `Bytes` against `String`, `JsonObject` against `Boolean`, `UUID` against `Bytes` in an equality | `ill_typed`/`type-mismatch` before an equality charge, no Boolean | FR-149-AC-24 |
| KS-05 | The four ordering operators over `Bytes`, `+` over `Bytes`, and `+` over `JsonObject` | `ill_typed`/`operator-ineligible` before any charge | FR-149-AC-24 |
| KS-06 | A `Set<Bytes>` holding `2000`, `00`, `10`, `0000`, then the empty sequence; a `Set<JsonObject>` holding `{}`, `null`, `[]`, `3`, `"x"`; one holding `"z"` and `"é"`; one holding `{"b":1,"a":2}` and `{"a":2,"b":1}` | visited `` (empty), `00`, `0000`, `10`, `2000`; `"x"`, `3`, `[]`, `null`, `{}`; `"z"` then `"é"`; one member | FR-144-AC-27, FR-149-AC-24 |
| KS-07 | `Set<Bytes>` supplied `{"type":"bytes","value":"00ff"}` twice, and `"0"`, `"00FF"`, `"0g"` as values; `Set<JsonObject>` supplied `{"type":"json_object","value":"{\"b\":1,\"a\":2}"}` and `"[]"` twice | `duplicate-member`; `invalid-value` each; `invalid-value`; `duplicate-member` | FR-144-AC-28, FR-181-AC-26 |
| RE-ends-1 | `relationship_end` `order` on M4, the relationship `Order-order-Order` | Resolves the end `order` | FR-322-AC-56 |
| RE-ends-2 | `relationship_end` `next` on M4 | Resolves the end `next` (`targetEnd.role`) | FR-322-AC-56 |
| RE-ends-3 | `relationship_end` `invoiced` on M10, the relationship `Order-invoiced-Invoice` with no `targetEnd.role` | Resolves the end `invoiced` | FR-322-AC-56 |
| RE-ends-4 | `relationship_end` `next` on M10 | `missing_declaration`/`missing-name` at `member.name`: M10 has no inverse end | FR-322-AC-56 |
| RE-ends-5 | `relationship_end` `invoice` on M10 | `missing_declaration`/`missing-name` | FR-322-AC-56 |
| RE-ends-6 | `relationship_end` `order` on M1, an object type | `ill_typed`/`operator-ineligible` at `member.declaration` | FR-322-AC-56 |
| RE-ends-7 | `relationship_end` `follows` on M11, the `source-to-target` relationship `Order-follows-Order` | Resolves the end `follows` | FR-322-AC-56 |
| RE-ends-8 | `relationship_end` `followedBy` on M11 (the named inverse, traversed target to source) | `ill_typed`/`operator-ineligible` at `member.name`: the direction is `source-to-target` (FR-152) | FR-322-AC-56 |
| RE-ends-9 | `relationship_end` `twin` on M12, whose source and target roles are both `twin` | `ambiguous_declaration`/`ambiguous-name` at `member.name` | FR-322-AC-56 |
| MW-01 | A `model`/`object_type` wire node with M1's id, M1's owner (`acme/orders`, `ix://acme/orders/Order`), its own id as `semantic_type` and an empty body | Resolves `Order` | FR-322-AC-29 |
| MW-03 | MW-01 with a `declaration` | Fails the V2 schema; `malformed_wire` | FR-322-AC-29 |
| MW-06 | A `model`/`object_type` wire node carrying M4's id and M4's owner (`acme/orders`, `ix://acme/orders/relationship/Order-order-Order`) | `missing_declaration`/`missing-selection` at the owner join: `Order-order-Order` is a relationship, and the join runs over every owner-bearing node first; the outcome equals MW-07's | FR-322-AC-29, FR-322-AC-55 |
| MW-07 | A `model`/`object_type` wire node carrying M4's owner | `missing_declaration`/`missing-selection` at the owner join: `Order-order-Order` is a relationship | FR-322-AC-29, FR-322-AC-55 |
| MW-08 | A `model`/`object_type` wire node with M2's id and M2's owner (`acme/billing`, `ix://acme/billing/Order`) | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-29 |
| MW-09 | MW-01 with no `owner` | Fails the V2 schema; `malformed_wire` | FR-322-AC-29 |
| MW-10 | A `model`/`object_type` wire node with owner (`acme/orders`, `ix://acme/orders/Nope`), that no member reaches | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-55 |
| MW-11 | A clause function wire node (`clause` binding `precondition`) owned by (`acme/orders`, `ix://acme/orders/Order/size`) | Passes the owner join | FR-322-AC-55 |
| MW-12 | A clause function wire node (`clause` binding `invariant`) owned by (`acme/orders`, `ix://acme/orders/Order`) | Passes the owner join | FR-322-AC-55 |
| MW-13 | MW-11 owned by `ix://acme/orders/Order/missing` | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-55 |
| MW-14 | MW-11 owned by `ix://acme/orders/Order/total`, a field | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-55 |
| MW-16 | A clause function (`clause` binding `precondition`) owned by the object type `ix://acme/orders/Order` | `missing_declaration`/`missing-selection` at the owner join: only an invariant is owned by an object type | FR-322-AC-55 |
| MW-17 | A clause function (`clause` binding `body`) owned by `ix://acme/orders/Order/size` | Passes the owner join | FR-322-AC-55 |
| MW-18 | A clause function (`clause` binding `invariant`) owned by the operation member `ix://acme/orders/Order/size` | `missing_declaration`/`missing-selection` at the owner join: an invariant is owned by an object type | FR-322-AC-55 |
| MW-19 | A clause function whose `clause` binding is `postcondition`, owned by `ix://acme/orders/Order/size` | `missing_declaration`/`missing-selection` at the owner join: a postcondition is FR-341's `state_clause` node, not a clause function kind | FR-322-AC-55 |
| MW-20 | MW-17 with two `clause` bindings | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-55 |
| MW-21 | MW-17 with no `clause` binding | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-55 |
| MW-22 | A clause function (`clause` binding `invariant`) owned by the systems interface `ix://acme/orders/Flow` | `missing_declaration`/`missing-selection` at the owner join: an invariant is owned by an object type with no `interfaceFeatures` | FR-322-AC-55 |
| MS-01 | The document supplied under the lock row's identity, identity and version equal | Passes the selection admission checks | FR-322-AC-32 |
| MS-02 | No document supplied | `missing_import`/`missing-selection` | FR-322-AC-32 |
| MS-03 | A document with `OrderTotal`'s upper bound changed to 8, supplied under the lock row's identity, identity and version equal | Passes the selection admission checks: the reader compares identity and version only | FR-322-AC-32 |
| MS-05 | The supplied document's `version` differs from the lock row's | `invalid_model_binding`/`wrong-model-selection` | FR-322-AC-32 |
| MS-06 | The selection names identity `acme/invoices` | `invalid_model_binding`/`wrong-model-selection` | FR-322-AC-32 |
| MS-07 | The selected document with `dimensions` `[Time (base), Rate (terms [{Time, "-1"}])]`, `units` `[s (root of Time), ms (target s, scale 1/1000)]` and a clock field `Order/x` typed `ix://quire/native/Clock` with the clock field shape, selected; then the same document with `dimensions` and `units` removed, selected | The first read holds the dimension nodes `Time` then `Rate`, the unit nodes `s` then `ms` and the clock field `Order/x`, in authored order; the second read holds no dimension and no unit node and is otherwise equal | FR-322-AC-98 |
| MS-08 | MS-07's first document with unit `ms`'s `scale` set to `0`, selected | `invalid_model_binding`/`malformed-declaration` at stage `package_read`, naming `ix://acme/orders/unit/ms` and `Unit.scale` | FR-322-AC-98 |
| FF-01 | Frame field entry `(M1, "total")` | Resolves `Order/total` | FR-340-AC-13 |
| FF-02 | Frame field entry `(M5, "total")` | Resolves the inherited `Order/total` | FR-340-AC-13 |
| FF-03 | Frame field entry `(M1, "missing")` | `missing_declaration`/`missing-name` | FR-340-AC-13 |
| FF-04 | Frame field entry `(M1, "size")`, an operation name | `missing_declaration`/`missing-name` | FR-340-AC-13 |
| FF-05 | Frame field entry `(M6, "code")` | `ambiguous_declaration`/`ambiguous-name` | FR-340-AC-13 |
| FF-06 | Frame field entry `(M2, "total")` | `missing_declaration`/`missing-selection` | FR-340-AC-13 |
| AN-01 | Anchor `(M1, "scaled")` | Resolves `Order/scaled` | FR-342-AC-4 |
| AN-02 | Anchor `(M5, "size")` | Resolves `Sub/size` | FR-342-AC-4 |
| AN-03 | Anchor `(M5, "reset")`, which `Sub` only inherits | `invalid_model_binding`/`malformed-declaration` | FR-342-AC-4 |
| AN-04 | Anchor `(M1, "missing")` | `missing_declaration`/`missing-name` | FR-342-AC-4 |
| AN-05 | Anchor `(M1, "total")`, a field name | `missing_declaration`/`missing-name` | FR-342-AC-4 |
| AN-06 | Anchor `(M2, "size")` | `missing_declaration`/`missing-selection` | FR-342-AC-4 |
| AN-07 | Anchor `(M8, "rate")`, a `systems_interface` context | `invalid_model_binding`/`malformed-declaration` | FR-342-AC-4 |
| CS-01 | Precondition of `Order.scaled` binding `[Reference<Order>, Integer]` | Admitted | FR-341-AC-5 |
| CS-02 | Postcondition of `Order.scaled` binding `[Reference<Order>, Integer, Integer]` | Admitted | FR-341-AC-5 |
| CS-03 | Postcondition of `Order.reset` binding `[Reference<Order>]` | Admitted: no result | FR-341-AC-5 |
| CS-04 | Precondition of `Order.scaled` binding `[Reference<Order>]` | `ill_typed`/`operator-ineligible` | FR-341-AC-5 |
| CS-05 | Postcondition of `Order.scaled` binding `[Reference<Order>, Integer]` | `ill_typed`/`operator-ineligible` | FR-341-AC-5 |
| CS-06 | Precondition of `Order.scaled` binding `[Reference<Sub>, Integer]` | `ill_typed`/`operator-ineligible` | FR-341-AC-5 |
| CS-07 | Precondition of `Order.scaled` binding `[Reference<Order>, Boolean]` | `ill_typed`/`operator-ineligible` | FR-341-AC-5 |
| CS-08 | Invariant of `Order` binding `[Reference<Order>]` | Admitted | FR-341-AC-5 |
| CS-09 | Invariant of `Order` binding `[Reference<Order>, Integer]` | `ill_typed`/`operator-ineligible` | FR-341-AC-5 |
| OF-01 | A `model`/`process` wire node owned by `Flow` | Recovers `Flow` | FR-322-AC-72 |
| OF-02 | A `model`/`state_machine` wire node owned by `Gate` | Recovers `Gate` | FR-322-AC-72 |
| OF-03 | A `model`/`persistence_interface` wire node owned by `Store` | Recovers `Store` | FR-322-AC-72 |
| OF-04 | A `model`/`process` wire node owned by `Counter` | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-72 |
| OF-05 | A `model`/`state_machine` wire node owned by `Flow` | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-72 |
| OF-06 | A `model`/`object_type` wire node owned by `Flow` | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-72 |
| OF-07 | A `model`/`process` wire node owned by `Nope` | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-72 |
| OF-08 | A `model`/`persistence_interface` wire node owned by `Gate` | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-72 |
| PR-01 | A `relation`/`population` wire node owned by `counters` (`P7`) | Recovers `counters` | FR-322-AC-82 |
| PR-02 | A `relation`/`population` wire node owned by `archive` (`P8`) | Recovers `archive`; its node id differs from `P7` | FR-322-AC-82 |
| PR-03 | A `relation`/`population` wire node owned by `Counter`, an object type | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-82 |
| PR-04 | A `model`/`object_type` wire node owned by `counters`, a population | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-82 |
| PR-05 | A `relation`/`population` wire node owned by `Nope`, which the document does not declare | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-82 |
| PR-06 | A `relation`/`relationship` wire node owned by `counters`, a population | `missing_declaration`/`missing-selection` at the owner join | FR-322-AC-82 |
| FP-01 | Frame field entry `(P2, "n")`, `P2` being `Flow` | Resolves `Flow/n` | FR-340-AC-19 |
| FP-02 | Frame field entry `(P3, "n")`, `P3` being `SubFlow` | Resolves `Flow/n` | FR-340-AC-19 |
| FP-03 | Frame field entry `(P4, "open")`, `P4` being `Gate` | Resolves `Gate/open` | FR-340-AC-19 |
| FP-04 | Frame field entry `(P2, "go")`, `P2` being `Flow` | `missing_declaration`/`missing-name` | FR-340-AC-19 |
| FP-05 | Frame field entry `(P2, "missing")`, `P2` being `Flow` | `missing_declaration`/`missing-name` | FR-340-AC-19 |
| FP-06 | Frame field entry `(P5, "x")`, `P5` being `Store` | `invalid_model_binding`/`malformed-declaration` | FR-340-AC-19 |
| FP-07 | Frame field entry `(P6, "x")`, `P6` being `Plug` | `invalid_model_binding`/`malformed-declaration` | FR-340-AC-19 |
| AP-01 | Anchor `(P1, "bump")`, `P1` being `Counter` | Resolves `Counter/bump` | FR-342-AC-4 |
| AP-02 | Anchor `(P2, "go")`, `P2` being `Flow` | Resolves `Flow/go` | FR-342-AC-7 |
| AP-03 | Anchor `(P3, "go")`, `P3` being `SubFlow` | `invalid_model_binding`/`malformed-declaration` | FR-342-AC-7 |
| AP-04 | Anchor `(P2, "missing")`, `P2` being `Flow` | `missing_declaration`/`missing-name` | FR-342-AC-7 |
| AP-05 | Anchor `(P2, "n")`, `P2` being `Flow` | `missing_declaration`/`missing-name` | FR-342-AC-7 |
| AP-06 | Anchor `(P4, "toggle")`, `P4` being `Gate` | Resolves `Gate/toggle` | FR-342-AC-8 |
| AP-07 | Anchor `(P4, "open")`, `P4` being `Gate` | `missing_declaration`/`missing-name` | FR-342-AC-8 |
| AP-08 | Anchor `(P5, "save")`, `P5` being `Store` | `invalid_model_binding`/`malformed-declaration` | FR-342-AC-9 |
| AP-09 | Anchor `(P6, "rate")`, `P6` being `Plug` | `invalid_model_binding`/`malformed-declaration` | FR-342-AC-9 |
| CP-01 | Invariant of `Flow` binding `[Reference<Flow>]` | Admitted | FR-341-AC-11 |
| CP-02 | Precondition of `Flow.go` binding `[Reference<Flow>, Integer]` | Admitted | FR-341-AC-11 |
| CP-03 | Postcondition of `Flow.go` binding `[Reference<Flow>, Integer, Integer]` | Admitted | FR-341-AC-11 |
| CP-04 | Postcondition of `Flow.go` binding `[Reference<Flow>, Integer]` | `ill_typed`/`operator-ineligible` | FR-341-AC-12 |
| CP-05 | Precondition of `Flow.go` binding `[Reference<Counter>, Integer]` | `ill_typed`/`operator-ineligible` | FR-341-AC-12 |
| CP-06 | Invariant of `Gate` binding `[Reference<Gate>]` | Admitted | FR-341-AC-11 |
| CP-07 | Postcondition of `Gate.check` binding `[Reference<Gate>, Boolean]` | Admitted | FR-341-AC-11 |
| CP-08 | Precondition of `Gate.toggle` binding `[Reference<Gate>]` | Admitted | FR-341-AC-11 |
| CP-09 | Precondition of `Gate.toggle` binding `[Reference<Flow>]` | `ill_typed`/`operator-ineligible` | FR-341-AC-12 |
| CP-10 | Invariant of `Flow` binding `[Reference<Flow>, Integer]` | `ill_typed`/`operator-ineligible` | FR-341-AC-12 |

## Expected Results

M1 to M12 and P1 to P6 validate. Every wire,
member, selection, frame field, anchor, clause signature and owner case gives
exactly its recorded result.

## Operation parameter name cases

`fixtures/adverse.json` `parameter_name_cases` patch the `Order/scaled`
operation of the selected document and are read under FR-154's parameter-name
row (FR-154-AC-14); each refusal is reported at the selection row with its member path, naming
the shared parameter identity for a repeated identity and the operation's
identity for a name refusal; a repeated identity is checked first and
suppresses the name check of both colliding parameters. Ids: `parameter-names-base-admitted`,
`parameter-names-distinct-admitted`, `parameter-names-case-distinct-admitted`,
`parameter-name-absent`, `parameter-name-not-a-string`, `parameter-name-null`,
`parameter-name-empty`, `parameter-name-leading-digit`,
`parameter-name-with-separator`, `parameter-name-differs-from-identity`,
`parameter-name-repeated`, `parameter-name-repeated-after-malformed-name`,
`parameter-name-malformed-second-parameter`.

## Relationship declaration cases

`fixtures/adverse.json` `relationship_document_cases` patch the selected
document and are read under FR-154's relationship declaration refusals
(FR-154-AC-10); each refusal is reported at the selection row. Ids: `relationship-document-admitted`, `relationship-without-target-role-admitted`, `relationship-with-generated-origin-admitted`, `relationship-identity-owner-nested`, `relationship-identity-foreign-package`, `relationship-identity-wrong-slot`, `relationship-identity-missing`, `relationship-two-identities-missing`, `relationship-direction-missing`, `relationship-direction-null`, `relationship-direction-unsupported`, `relationship-category-missing`, `relationship-category-unsupported`, `relationship-composite-wrong-type`, `relationship-origin-missing`, `relationship-origin-both-branches`, `relationship-source-role-missing`, `relationship-target-role-null`, `relationship-target-role-empty`, `relationship-target-role-non-string`, `relationship-source-end-names-another-owner`, `relationship-target-end-type-unresolved`, `relationship-target-end-type-names-a-relationship`, `relationship-target-end-type-names-a-value-type`, `relationship-end-multiplicity-malformed`, `relationship-end-multiplicity-reversed`, `relationship-identity-repeated-under-two-owners`, `relationship-malformed-role-beats-unresolved-end-type`, `relationship-unresolved-end-type-beats-reversed-multiplicity`, `relationship-duplicate-identity-beats-reversed-multiplicity`.
