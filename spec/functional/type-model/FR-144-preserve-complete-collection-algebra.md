---
id: FR-144
title: "Preserve complete collection-kind semantics"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-008
    type: references
  - target: ix://agent-ix/quire-specification/AD-005
    type: references
  - target: ix://agent-ix/quire-specification/FR-149
    type: references
  - target: ix://agent-ix/quire-specification/FR-204
    type: references
  - target: ix://agent-ix/quire-specification/FR-143
    type: references
  - target: ix://agent-ix/quire-specification/FR-181
    type: references
  - target: ix://agent-ix/quire-specification/FR-148
    type: references
---

# FR-144: Preserve complete collection-kind semantics

## Description

When constructing or comparing a collection, the evaluator SHALL preserve the
selected sequence, set, bag or ordered-set algebra.

## Inputs

Element type, collection kind, an optional declared cardinality bound and
occurrences. An absent bound means unbounded.

## Outputs

A typed bounded collection/equality result or refusal.

## Behavior

Sequences preserve order and duplicates. Sets preserve membership without
duplicates and expose no semantic iteration order. Bags preserve multiplicity
without semantic order. Ordered sets preserve first-occurrence order and
uniqueness. Equality follows those rules and all materialization checks the
declared maximum before allocation.

## Collection-kind table

| Kind | Occurrence model | Equality | Canonical representation |
| --- | --- | --- | --- |
| `Sequence<T>` | ordered occurrences; duplicates retained | equal length and pairwise element equality | occurrence order |
| `Set<T>` | unique members; no semantic order | identical member set | ascending type-owned total key |
| `Bag<T>` | member-to-positive-multiplicity map; no semantic order | identical members and multiplicities | ascending key plus multiplicity |
| `OrderedSet<T>` | first-occurrence order plus uniqueness | identical ordered member sequence | semantic occurrence order |

Every admitted set, bag and ordered-set element type has a total canonical key
(below), so set and bag canonical ordering always exists and never uses
insertion or hash order. Bounds are inclusive (`min..max`), checked before
materialization, and count occurrences for sequences/bags and members for
sets/ordered sets.

The canonical representation is an order, not a byte encoding. The canonical
order of a set or bag is exactly the ascending typed total key order below;
this requirement defines no byte encoding, sort-key bytes or hash. The one
QSpec-owned collection serialization is the FR-181 typed canonical form, which
orders set and bag elements by their encoded bytes, not by this key order; any
other consumer that serializes a collection value uses its own owning
serialization contract.

## Bound and type identity

The cardinality bound, when authored, is part of the collection type. An
absent bound means unbounded: no minimum and no maximum apply, and
construction and traversal never refuse for cardinality. `K<T>[a,b]` and
`K2<T2>[c,d]` are the same type exactly when `K = K2`, `T` and `T2` are the same
type, and either both bounds are present with `a = c` and `b = d`, or both are
absent; element-type identity includes every nested bound or its absence. A
bound present on one side and absent on the other is a different type. FR-149
admits no collection equality conversion, so `=` or `!=` between collection
types that differ only in bound, including only in bound presence, is
`refused { code: ill_typed, cause: type-mismatch }`. Such values are compared by
first binding an FR-145 same-kind `convert<K<T>[a,b]>(c)` (or, to an unbounded
target, `convert<K<T>>(c)`) through `let`, whose loss is empty and whose target
bound, when present, is checked at run time. A bound is not only a construction
check, and no implicit widening or narrowing exists between a present and an
absent bound, or between two different present bounds.

## Collection literal typing

A collection literal (`sequence[...]`, `set[...]`, `bag[...]` or
`orderedSet[...]`) has no standalone type. It is checked against the unique
expected type of its position: a declared field, parameter or result type, a
`convert` target, the element type of an enclosing literal, or the static type
of the other operand of `=` or `!=`. The expected type must be a collection of
the literal's kind, `K<T>[a,b]` or the unbounded `K<T>`; the literal then has
exactly that type, and
each element expression is checked against the expected type `T`. A literal
without a unique expected type, such as both operands of `=` or the right side
of an unannotated `let`, is `refused { code: ill_typed, cause: ambiguous-literal }`.
A literal of another kind is `refused { code: ill_typed, cause: type-mismatch }`.
A present bound is checked by construction at run time, not by type checking;
an absent bound admits every constructed count.

## Element admission and canonical keys

Uniqueness and multiplicity in a `Set`, `Bag` and `OrderedSet` are decided by
the **collection membership relation**: FR-149's equality matrix with one
change, that a `Float32` or `Float64` leaf at any depth inside a member is
compared by FR-148 `bitIdentical` (the same width and identical bits). So
`+0` and `-0` are two members, two NaNs are one member exactly when their bits
are identical, and a record, tuple, option or union holding a float is one
member with another exactly when every field or payload is the same member. The
relation is selected by the collection kind, never by an operator: `=` and `!=`
over a type containing a float compare a float leaf by `numericEqual`, and `=`
between two sets, bags or ordered sets of floats matches their members by this
relation (FR-149 "IEEE relations").
Construction, FR-145 coalescing and `contains` all use this relation. An event
type (FR-149), or a type containing one at any depth, admits no membership
relation, so a `Set`, `Bag` or `OrderedSet` of it is
`refused { code: ill_typed, cause: operator-ineligible }`. `Sequence<T>` admits
every complete-V1 element type.

An element type that is clock-bearing (FR-415 "Clock places") admits no FR-149
`=`, because equality would read its clocks. A `Set`, `Bag` or `OrderedSet` of a
clock-bearing element type is admitted all the same: uniqueness and
multiplicity are decided by the collection membership relation over the
elements' data parts (the values with every clock component removed), which
must admit that relation, and the canonical
key of an element is the key of its data part. `Set<Clock>` therefore holds at
most one element and `Bag<Clock>` one element for each occurrence. Collection
equality, `contains` and `convert` that compare elements as values refuse
`invalid_timed_model`/`clock-read-outside-constraint` over such a collection.
A `Map` key type SHALL be a scalar type with a total canonical key below (a
scalar key type, FR-154 "Map types"): a key type that is clock-bearing, a
record, tuple or union, an `Option`, a collection, a `Map`, a `Reference`, or a
type with no row below (the natives `Rational` and `Decimal`) is
`refused { code: ill_typed, cause: operator-ineligible }`. A native `Text` map
key with no bounds and no profile (FR-154 "Map types") has the `String` row's
key: its exact Unicode scalar sequence, with no normalization.

Every type that admits the membership relation has a total canonical key,
exactly as follows. Keys are compared as written, and two values have equal
keys exactly when they are the same member under the relation.

| Type | Canonical key |
| --- | --- |
| `Boolean` | `false` before `true` |
| `Integer`, `Int[..]`, `Rational[..]`, `Decimal[..]` | exact mathematical value, ascending |
| `Float32`, `Float64` | the FR-148 `totalOrder` key of the bit pattern `b`: `not(b)` when the sign bit is one, otherwise `b xor sign_mask`, ascending as an unsigned integer; `-NaN`s precede `-infinity`, then the negative finite values, `-0`, `+0`, the positive finite values, `+infinity` and the `+NaN`s, and each NaN payload has its own place |
| Quantity of one unit | exact value in that unit, ascending |
| `String` | the exact Unicode scalar sequence, ordered lexicographically by scalar value (which is UTF-8 byte order), a proper prefix first; no normalization or case folding, so a composed and a decomposed spelling are two keys |
| `Text[..]` | the FR-141 compared form under the type's profile (the normalized scalar sequence, or the UTF-8 bytes for `binary-utf8`), ordered lexicographically by scalar value or byte |
| enumeration | declaration position for an `ordered enum`; otherwise the case identifier's ASCII bytes, lexicographically, never its declaration position; for an enumeration declared by a `model`/`variant_type` node, the UTF-8 bytes of the case name, lexicographically, which may be any non-empty string (FR-141) |
| `UUID`, `Timestamp` | the canonical payload's ASCII bytes, unsigned bytewise, a proper prefix first (FR-149) |
| `Bytes` | the octet sequence, compared octet by octet as unsigned values, a proper prefix first, so the empty sequence is the least key; this is the order of the canonical lowercase hexadecimal payload's ASCII bytes (FR-149), because two digits encode each octet and `0`-`9` precede `a`-`f` |
| `JsonObject` | the UTF-8 bytes of the value's canonical RFC 8785 (JCS) text, unsigned bytewise, a proper prefix first (FR-149); so `"x"` precedes `3`, `3` precedes `[]`, `[]` precedes `null` and `null` precedes `{}`, and a member-order or spelling variant of one value has the one key |
| `Reference<T>` | the FR-204 identity triple (universe identity, object-type declaration identity, object identity), compared component by component; under `quire.model.complete/v1` the universe and type are structural identities compared componentwise (strings as unsigned UTF-8 bytes, arrays element by element with a proper prefix first), and the object bytes are the exact producer UTF-8 object identity compared unsigned bytewise with a proper prefix first |
| `Option<T>` | `none` before a present value, then the key of the payload |
| record, tuple | for two records of distinct declarations, the ascending byte order of their 32-byte declaration node keys, compared before any slot; otherwise lexicographic over fields in declaration order or positions in order; a slot orders `absent` before `null` before a present value, and present values by key |
| union | the active member's identifier ASCII bytes, lexicographically, never its declaration position; then, for two values of one member, lexicographic over the payload values' keys in position order |
| `Sequence<T>`, `OrderedSet<T>` | lexicographic over element keys; a proper prefix comes first |
| `Set<T>`, `Bag<T>` | lexicographic over the ascending element-key list, with a bag occurrence listed once per multiplicity |

Records of distinct declarations meet in one collection when a record subtype
value is held at an ancestor-typed element (FR-208): `Set<Money>` can hold a
`Money` and a `TaxedMoney`. The declaration comparison is a total order that is
unrelated to ancestry, which no collection order needs. Two records have equal
keys exactly when they have one declaration and equal fields, which is when they
are FR-149 equal, so a `Money` and a `TaxedMoney` with equal shared fields are
two members of a `Set<Money>`.

A recursive record or union type has a key because every finite value of it
does. A union has a membership relation exactly when every payload type of
every member has one, so a union with a `Float32` or `Float64` payload is an
admitted element type whose key orders its payload floats by the row above, while
its `=` compares that payload by `numericEqual` (FR-143, FR-149). Because declaration position is
never used for a union, reordering its members changes no visiting order.
References from different universes never meet in one collection, because
their FR-149 comparison refuses construction. A universe is a model-level object
universe shared by a whole generalization component, never a population, so
references to a supertype and its subtypes do meet in one collection.

A canonical key is not an ordering operator and is not semantic. The
identifier-byte key of an unordered enumeration and the identity-byte key of a
reference expose no order that the type denies: `<`, `<=`, `>` and `>=` on an
unordered enumeration or a reference remain
`refused { code: ill_typed, cause: operator-ineligible }`, and FR-204 still
declares no semantic order between object identities. A key only fixes
FR-144/FR-145 visiting order, which decides charge order and which member stops
a short-circuiting query. Because declaration position is never used for an
unordered enumeration, reordering its cases changes no visiting order.

## Construction and traversal

Collection element expressions are evaluated in source position order under
FR-143's first-stopped-element rule. The evaluator then forms the kind's
occurrences, decides duplicates with the FR-149 relation and, when the type
declares a bound, checks it. A duplicate decision that FR-149 refuses, such as a
`foreign_reference` pair, refuses construction with that outcome. For a bounded
type, a count outside `min..max` returns
`refused { code: cardinality_out_of_bound, cause: below-minimum }` or
`refused { code: cardinality_out_of_bound, cause: above-maximum }` before
materialization, with the collection type, the bound and the formed count. An
unbounded type admits every formed count and makes no `cardinality_out_of_bound`
refusal.

## Supplied collection values

A snapshot or runtime input may supply a field, member or parameter value of
type `Sequence<T>`, `Set<T>`, `Bag<T>` or `OrderedSet<T>`. Its wire form is
the FR-181 typed canonical form of that kind, `{"type": <tag>, "elements": [...]}`
with `<tag>` one of `sequence`, `set`, `bag` and `ordered-set`. Each entry of
`elements` is itself the FR-181 typed canonical form of the element type `T`,
nested, with no second grammar. A bag is written as repeated elements, one array
entry per occurrence, and never as element-count pairs.

Reading a typed canonical form against a declared type, for the collection
object and for every element, refuses with these causes:

| Condition | Refusal |
| --- | --- |
| the value is not a JSON object | `invalid_runtime_input`/`wrong-value-kind` |
| the value is an object with no `type` member | `invalid_runtime_input`/`missing-member` |
| `type` is not the FR-181 tag of the declared type | `invalid_runtime_input`/`wrong-value-kind` |
| a member FR-181 requires for that tag is absent (for a collection, `elements`) | `invalid_runtime_input`/`missing-member` |
| a member FR-181 does not define for that tag is present | `invalid_runtime_input`/`unknown-member` |
| a member has the wrong JSON kind (for a collection, `elements` not an array) | `invalid_runtime_input`/`wrong-value-kind` |
| a member is outside its FR-181 grammar or the declared type's domain, such as a non-decimal integer string | `invalid_runtime_input`/`invalid-value` |

Every element type that has a canonical key has an FR-181 form, and a supplied
collection of it is read by that form: `String` by `string`, `Rational[..]` by
`rational`, `Float32` and `Float64` by `float32` and `float64`, `Quantity<u>` by
`rational` (or `decimal`) with its `unit`, `UUID` by `uuid`, `Timestamp` by
`timestamp`, `Bytes` by `bytes` and `JsonObject` by `json_object`. A payload outside its form, such as a zero rational denominator, a
`bits` string of the wrong length or with an uppercase digit, a `uuid` or
`timestamp` out of its domain, a `bytes` `value` with an odd digit count, an uppercase
digit or a non-hexadecimal character, or a `json_object` `value` that is not the
canonical JCS text of one JSON value (FR-322 "Scalar literal values"), is `invalid_runtime_input`/`invalid-value`. A
`Quantity<u>` element whose `unit` names another unit is
`invalid_runtime_input`/`wrong-value-kind`, one with no `unit` is `missing-member`,
and a `unit` member on an element of any other type is `unknown-member`. A
population is not a value (FR-181) and has no FR-181 form, so a supplied
collection whose element type is, or contains at any depth, a population is
`refused { code: unknown_required_feature, cause: unsupported-feature }` naming
the element type as the feature and the supplied collection as the scope, before
any element is read.

Admission is a validation of data, not a construction expression, and it runs
before any evaluation or charge in this order:

1. The collection object is read as above. The `elements` length is then
   checked against the declared inclusive bound, as an occurrence count for a
   sequence or bag and as a member count for a set or ordered set, before any
   element is read. A length outside `min..max` is
   `refused { code: cardinality_out_of_bound, cause: below-minimum | above-maximum }`
   with the collection type, the bound, the supplied count and the input path
   of the collection. A type with no declared bound makes no such refusal.
2. Every element is read, in array order, as the FR-181 typed canonical form of
   `T` by the table above; the first refusal stops admission and no partial
   collection exists. All elements are read before any duplicate is decided.
3. For a set or ordered set, each element after the first is compared with the
   earlier elements by the collection membership relation (FR-149 `=`, with a
   float compared by `bitIdentical`), never by encoded bytes, and an element equal
   to an earlier one is
   `refused { code: invalid_runtime_input, cause: duplicate-member }` at the
   input path of the later element, adding the input path of the earlier equal
   element. A supplied set or ordered set is never silently deduplicated;
   deduplication belongs only to the construction expressions above. A
   comparison FR-149 refuses, such as a `foreign_reference` pair, refuses with
   that outcome. A bag and a sequence admit equal elements; a bag groups
   FR-149-equal elements into one member whose multiplicity is their number.

Input paths are relative to the supplied collection value. A reader accepts an
element in any FR-181 encoding of its value, including a non-canonical one
below, and normalizes it.

A reader accepts a `rational` element in any spelling with a nonzero
denominator and reads its FR-205 normalized value, so `2/4` and `1/2` are one
member; it reads a `float32` or `float64` element by its `bits`, so two elements
are one member exactly when their `bits` are equal, and it never merges `+0`
with `-0` or two NaN payloads. A string element is its exact scalar sequence.

Element order is not part of the value of a set or bag: two supplied sets, or
two supplied bags, whose elements differ only in array order are one value. The
array order of an ordered set is its first-occurrence order and the array order
of a sequence is its order; both are retained.

A producer writes each element of a set or bag in the element's canonical
encoding, so that FR-149-equal values encode identically, and only then orders
the elements ascending by encoded bytes (FR-181). The canonical set or bag wire
is therefore unique for a value, and the occurrences of one bag member are
byte-identical and adjacent. FR-181 "Canonical element encoding" defines the
canonical encoding of each kind.

## Collection charges

Every collection construction, membership comparison and result retention is
metered by the `quire.value.accounting/v1` collection family. A collection
constructor expression such as `set[e1, ..., en]` charges, in this order:

1. For each `i` from 1 to `n`: `collection.element` (one work unit), then the
   evaluation of `ei` with its own charges.
2. Occurrence formation. A sequence makes no membership comparison. For a set,
   bag or ordered set, occurrence 1 is retained, and each occurrence `i >= 2`,
   in source order, is compared with the distinct members retained before it,
   in retention order, stopping at the first equal member. Each comparison of
   candidate `c` with member `m` charges `collection.member-walk`
   (`work_units += occ(c) + occ(m)`), then decides FR-149 plan formation,
   including the `foreign_reference` universe check, then charges
   `collection.member-test` (`work_units += p`, the planned pair count).
3. `collection.bound` with `value_occurrences` equal to the formed bound count,
   then the bound check, which makes no charge. An unbounded type charges
   `collection.bound` identically; its bound check never refuses.
4. `collection.result-retain` with `result_units += occ(result)`.

Occurrence `i` is retained as a new member when no comparison is equal;
otherwise it adds one occurrence to the equal bag member or is discarded by a
set or ordered set. A float leaf compared by `bitIdentical` charges one `equality.pair` like any
other leaf pair, so the schedule above is unchanged for a float member. FR-145 result formation uses steps 2 to 4 over its produced
occurrences. The first unavailable charge returns incomplete, and no partial
collection is exposed. Every amount depends only on the completed occurrences
and their source or visiting order, never on a hash, sort or other internal
algorithm. The equality counts of FR-149 and TC-194 measure only the `=`
expression over completed operands; building those operands charges this
schedule separately.

Where an operation must visit the members of a `Set` or `Bag` one at a time,
it visits them in ascending canonical-key order, and a bag's equal-key
occurrences are consecutive. This order is not semantic. It fixes which charge
is reached first and which element stops a short-circuiting query, so the
outcome never depends on insertion or hash order.

Complete V1 defines no collection ordering (`<`, `<=`, `>`, `>=`) and no
union, intersection, difference, sort, minimum or maximum operator. Such a
comparison is `refused { code: ill_typed, cause: operator-ineligible }`. A call
to such a name resolves only to a checked user or library pure function under
FR-146 and FR-307; otherwise it is
`refused { code: missing_declaration, cause: missing-name }`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-144-AC-1 | Permuting a set or bag does not change equality, while permuting a sequence or ordered set can. | Test (TC-189) |
| FR-144-AC-2 | Duplicate insertion is preserved by sequence/bag and eliminated by set/ordered set according to their defined order rule. | Test (TC-189) |
| FR-144-AC-3 | Missing or exceeded cardinality bounds refuse with `cardinality_out_of_bound` and its bound cause, without partial materialization. | Test (TC-189) |
| FR-144-AC-4 | Set/bag insertion order never affects equality or canonical order, and canonical order is the typed total key order. | Test (TC-189) |
| FR-144-AC-5 | Bound counting follows occurrence versus unique-member semantics for each collection kind. | Test (TC-189) |
| FR-144-AC-6 | Every `=`-admitting type, `Reference` included, has exactly the tabled total canonical key, and a set, bag or ordered-set element type holding a float is admitted, keyed by the FR-148 `totalOrder` key, while an event-type element is `ill_typed`. | Test (TC-189) |
| FR-144-AC-7 | Set and bag visiting uses ascending canonical-key order independent of insertion order, and a key never admits an ordering comparison. | Test (TC-189) |
| FR-144-AC-8 | Collection construction charges `collection.element`, `collection.member-walk`, `collection.member-test`, `collection.bound` and `collection.result-retain` in the stated order and amounts, and each denied charge returns incomplete with no partial collection. | Test (TC-189) |
| FR-144-AC-9 | The cardinality bound, or its absence, is part of collection type identity, so equality between a present bound and an absent bound, or between two different present bounds, is `ill_typed`; an unordered enumeration key uses identifier bytes and never declaration position. | Test (TC-189) |
| FR-144-AC-10 | A collection literal takes exactly its unique expected type, and a literal without one is `ill_typed` with cause `ambiguous-literal`. | Test (TC-189) |
| FR-144-AC-11 | Reference keys order by universe, most-specific type and object bytes exactly as the published model reference-key vectors state. | Test (TC-198) |
| FR-144-AC-12 | A collection type with no declared bound admits every occurrence count at construction and traversal and never returns `cardinality_out_of_bound`. | Test (TC-246) |
| FR-144-AC-13 | A collection type with an absent bound and one with a present bound, same kind and same element type, are different types; equality between them is `refused { code: ill_typed, cause: type-mismatch }`, and moving a value between them requires an explicit FR-145 `convert` whose present-bound direction is checked at run time. | Test (TC-246) |
| FR-144-AC-14 | For `union Shape { Rect(Integer, Integer), Circle(Integer), Empty }`, `set[Shape::Rect(2, 3), Shape::Empty, Shape::Circle(5), Shape::Circle(1), Shape::Rect(2, 3)]` holds four members visited in the order `Circle(1)`, `Circle(5)`, `Empty`, `Rect(2, 3)`, the same order after the members are declared in another order; a `Bag<Shape>` of the same elements holds `Rect(2, 3)` with multiplicity two; and a `Set<U>` whose union `U` has a `Float64` payload is admitted, while `u1 = u2` over two values of `U` is `ill_typed` with cause `operator-ineligible`. | Test (TC-397) |
| FR-144-AC-15 | A `Set<Money>`, `Bag<Money>` or `OrderedSet<Money>` holding a `Money` and a `TaxedMoney` of equal shared fields holds two distinct members. A `Set<Money>` or `Bag<Money>` visits them in ascending byte order of their declaration node keys in either source order, and construction does not refuse for want of a canonical key. Two `TaxedMoney` values with equal fields are one member, and two that differ in `tax_minor` are two. | Test (TC-189) |
| FR-144-AC-16 | A supplied `Set`, `Bag` or `OrderedSet` value is read from its FR-181 tagged `elements` form (`set`, `bag` or `ordered-set`) whose entries are FR-181 typed forms of the element type, a bag as repeated elements; an absent `type` or `elements` member is `invalid_runtime_input`/`missing-member`, an extra member is `unknown-member`, a tag of another kind or a non-array `elements` is `wrong-value-kind`, and a member outside its grammar is `invalid-value`. | Test (TC-189) |
| FR-144-AC-17 | A supplied set or ordered set holding an element FR-149-equal to an earlier element is `invalid_runtime_input`/`duplicate-member` at the later element's input path, naming the earlier element's path, and is never deduplicated; a supplied bag or sequence holding equal elements is admitted. | Test (TC-189) |
| FR-144-AC-18 | A supplied collection whose `elements` length is below the minimum or above the maximum of its declared bound is `cardinality_out_of_bound` with `below-minimum` or `above-maximum`, naming the type, bound, supplied count and input path, before any element is validated; a bag counts occurrences and a set or ordered set counts supplied entries. | Test (TC-189) |
| FR-144-AC-19 | Two supplied sets, or two supplied bags, whose elements differ only in array order are equal; two supplied ordered sets or sequences that differ in array order are unequal. | Test (TC-189) |
| FR-144-AC-20 | A supplied collection's elements are all read before any duplicate is decided: a `Set<Integer>[0,3]` supplied with elements `1`, `1` and an invalid integer is `invalid-value` at the invalid element's path, not `duplicate-member`. | Test (TC-189) |
| FR-144-AC-21 | A producer writes each set or bag element in its canonical encoding before ordering by encoded bytes, and a reader accepts non-canonical encodings, normalizes them and decides duplicates by FR-149 equality: a `Bag<Decimal>` supplied `1.0` and `1.00` is one member of multiplicity 2 written as two identical canonical elements, and a `Set<Decimal>` supplied `1.0` and `1.00` is `duplicate-member`. | Test (TC-189) |
| FR-144-AC-22 | A supplied `Set<UUID>` of `{"type":"uuid","value":"00112233-4455-6677-8899-aabbccddeeff"}` and `Set<Timestamp>` of `{"type":"timestamp","nanoseconds":"0"}` and `{"type":"timestamp","nanoseconds":"1"}` are admitted; an element `{"type":"uuid","value":"00112233-4455-6677-8899-AABBCCDDEEFF"}`, `{"type":"timestamp","nanoseconds":"01"}` or `{"type":"timestamp","nanoseconds":"170141183460469231731687303715884105728"}` (2^127, outside the signed 128-bit domain) is `invalid_runtime_input`/`invalid-value`, while `{"type":"timestamp","nanoseconds":"170141183460469231731687303715884105727"}` is admitted. | Test (TC-189) |
| FR-144-AC-23 | A supplied `Set<String>` of `{"type":"string","value":"a"}` and `{"type":"string","value":""}` is admitted with the empty string ordered first; of `é` (U+00E9) and U+0065 U+0301 it holds two members; of `a` twice it is `duplicate-member` at the later element; a `Bag<String>` of `a` twice holds `a` with multiplicity 2; an element `{"type":"text","value":"a"}` is `wrong-value-kind` and `{"type":"string","value":1}` is `wrong-value-kind`; and a `String` collection built by `set[...]` is unchanged. | Test (TC-189) |
| FR-144-AC-24 | A supplied `Set<Rational>` of `2/4` and `1/2` (`{"type":"rational","numerator":"2","denominator":"4"}` and `{"type":"rational","numerator":"1","denominator":"2"}`) is `duplicate-member` at the later element and a `Bag<Rational>` of the same two holds `1/2` with multiplicity 2; a rational with denominator `0` is `invalid-value`; a supplied `Set<Quantity<Units::metre>>` of `{"type":"rational","numerator":"1","denominator":"2","unit":"Units::metre"}` and the `decimal` form with coefficient `5`, scale `0` and the same unit is admitted as two members, an element with `"unit":"Units::second"` is `wrong-value-kind`, an element with no `unit` is `missing-member`, and the key order of `1/3`, `1/2` and `-1` in a `Set<Rational>` is `-1`, `1/3`, `1/2`. | Test (TC-189) |
| FR-144-AC-25 | A supplied `Set<Float64>` of the `bits` `0000000000000000` and `8000000000000000` holds two members; of `7ff8000000000001` twice it is `duplicate-member` at the later element; of `7ff8000000000001` and `7ff8000000000002` it holds two members; a `Bag<Float32>` of `3f800000` twice holds one member of multiplicity 2; a `Set<Float64>` of the `bits` `fff0000000000000`, `bff0000000000000`, `8000000000000000`, `0000000000000000`, `3ff0000000000000` and `7ff0000000000000` is visited in that order, which is the `totalOrder` key order and not the order of the encoded bytes; a `bits` string with an uppercase digit or of 15 digits is `invalid-value`; and a `float32` element in a `Set<Float64>` is `wrong-value-kind`. | Test (TC-189) |
| FR-144-AC-26 | `Set<Timer>` over `record Timer { x: Clock; n: Int }` is admitted and holds one element for each distinct `n`; `Set<Clock>` holds at most one element; `Bag<Clock>` holds one element for each occurrence; `OrderedSet<Timer>` keeps the first occurrence of each `n`; the canonical key of a `Timer` element is the key of its `n`; a `Set` of an event type still refuses `ill_typed`/`operator-ineligible`; `=` between two `Set<Timer>` values refuses `invalid_timed_model`/`clock-read-outside-constraint`; `Map<Timer, Int>` refuses `ill_typed`/`operator-ineligible`, as do `Map<Cfg, Int>` over `record Cfg { a: Boolean }` and `Map<Option<Text>, Int>`. | Test (TC-367) |
| FR-144-AC-27 | A `Set<Bytes>` or `Bag<Bytes>` holding `2000`, `00`, `10` and `0000` (hexadecimal payloads) visits `00`, `0000`, `10`, `2000`, with `00` before `0000` as a proper prefix and the empty sequence, when held, before all of them; a `Set<JsonObject>` holding `{}`, `null`, `[]`, `3` and `"x"` visits `"x"`, `3`, `[]`, `null`, `{}`; one holding `"z"` and `"é"` visits `"z"` first, because the UTF-8 bytes of `é` begin `c3`; and `{"b":1,"a":2}` and `{"a":2,"b":1}` are one member whose key is the text `{"a":2,"b":1}`. A `Bytes` and a `JsonObject` never meet in one collection. | Test (TC-189) |
| FR-144-AC-28 | A supplied `Set<Bytes>` of `{"type":"bytes","value":"00ff"}` and `{"type":"bytes","value":""}` is admitted with the empty sequence first, and of `00ff` twice is `invalid_runtime_input`/`duplicate-member` at the later element; a `Bag<Bytes>` of `00ff` twice holds `00ff` with multiplicity 2; `{"type":"bytes","value":"0"}`, `"00FF"`, `"0g"` and `"0x00"` are `invalid-value`, and `{"type":"string","value":"00ff"}` is `wrong-value-kind`. A supplied `Set<JsonObject>` of `{"type":"json_object","value":"{\"a\":1,\"b\":[true,null]}"}` and `{"type":"json_object","value":"[]"}` is admitted, of `{"b":1,"a":2}` (not canonical JCS text) is `invalid-value`, and of the same canonical text twice is `duplicate-member`. | Test (TC-189) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
