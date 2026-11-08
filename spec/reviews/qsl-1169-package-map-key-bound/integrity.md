---
id: SR-9132
title: "Integrity review of the domain package map type key and bound"
type: SpecReview
analysis: integrity
scope: "quire-specification spec/package-map-key-bound diff origin/main...cd6c4059: FR-154 Map types, FR-144, FR-322, FR-415, TC-234, TC-280, TC-367, map-type-vectors.json package_map_types, README, tests/package_map_types.rs, tests/checked_package_v2.rs"
review_set: subset
review_date: "2026-11-08"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-154, type: references }
  - { target: ix://agent-ix/quire-specification/FR-144, type: references }
  - { target: ix://agent-ix/quire-specification/FR-322, type: references }
  - { target: ix://agent-ix/quire-specification/TC-234, type: references }
  - { target: ix://agent-ix/quire-specification/TC-280, type: references }
---

## Summary

REVISE. The key-eligibility list is stated the same way in FR-154, FR-144,
FR-415 and the FR-322 timed-model text, and it matches QSL FR-230 "Clock
collections". QSL's list does not name `Reference`, but its "every other key
type" clause refuses one too, so the added `Reference` row does not contradict
QSL. One high contradiction remains. The headline key, bare native `Text`
(and the native `Rational` and `Decimal` FR-154 also admits), has no row and
no canonical key in FR-322 or FR-144. Two medium findings: the `reads`
rendering in the PM vectors differs from FR-322's rendering rule, and three
unchanged test-case rows still describe map types that the new rows refuse.

## Review set

Diff consumers were found by grepping `Map<`, `map type`, `` `values` ``,
`keyed`, `place id` and `native/Text` across `spec/`, `proposals/` and
`tests/`. QSL FR-230 on `origin/main` was read as context data.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-154's scalar key types include the natives `Text`, `Rational` and `Decimal`, and every example key is bare `ix://quire/native/Text`. FR-322 says those three natives have no row: a member of one "has no type" and refuses `ill_typed`/`operator-ineligible` (FR-322-AC-31, AC-183). The new FR-322 map row types `K` as "the element type this table gives `keys`", so a bare `Text` key gets no type and no node. That contradicts FR-322-AC-396's "`composite_type`/`map` node of the `Text` node". FR-144:134 requires a key to be "a scalar type with a total canonical key below", but the FR-144 key table has rows only for `Text[..]`, `Rational[..]` and `Decimal[..]`, none for the bare native types. Bare `Text` compares only under a profile (FR-141), so `m["é"]` against `m["é"]` has no defined answer. Fix: pick one. (a) State in the FR-322 map row that a native `Text` key is the `scalar_type`/`text` node (as MT-01 already uses), add an FR-144 key row for bare `Text` (for example, `String`'s scalar order with no normalization), and say whether native `Rational`/`Decimal` keys are admitted with the exact-value key. Or (b) drop the native `Text`, `Rational` and `Decimal` from FR-154's list and require `Text[min, max; profile]`, which changes every `keys` `Text` example and fixture and the QSL FR-230-AC-19 alignment. | spec/functional/type-model/FR-154-admit-domain-package-model.md:336; spec/objects/interfaces/FR-322-checked-package-artifact.md:1942; spec/objects/interfaces/FR-322-checked-package-artifact.md:2181; spec/objects/interfaces/FR-322-checked-package-artifact.md:4684; spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md:134; spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md:146 |
| FND-002 | medium | One type renders two ways. FR-322:4079 says a `map` level renders `Map<K, V>` with "a native key by its name" (`Map<Text, ix://quire/native/Clock>`), and `timed-slot-selector-vectors.json` and `checked_package_v2.rs::selected_element` follow that. The `package_map_types` note says `reads` "is the FR-322 member type of a field typed M", yet PM-01..PM-03 and PM-07 read `Map<ix://quire/native/Text, …>`. PM-04 reads `Map<Int[0, 9], …>` and PM-06 `Map<Quantity<ms>, …>`, while the FR-322:4079 rule renders a package key by its identity. FR-154-AC-48 spells the same type `Map<Text, Timer>[0, 2]`. Fix: define one rendering in FR-322 (native key by name, package key by identity, unit key as written) and regenerate the PM `reads` strings and `package_map_types.rs::rendered` to match. Otherwise, relabel `reads` as a vector-local spelling that is not the FR-322 member type. | spec/objects/interfaces/FR-322-checked-package-artifact.md:4079; proposals/checked-package-v2/map-type-vectors.json:14; proposals/checked-package-v2/map-type-vectors.json:36; tests/package_map_types.rs:108 |
| FND-003 | medium | Unchanged test-case rows contradict the new required `keys`/`bound`. TC-234 I32 declares "a map type `MO` whose `values` is `OptInt`" and expects "admitted … every position above raises no refusal", but under FR-154:133 `MO` now refuses `malformed-declaration` at `TypeDefinition.keys`. TC-280 MU-17 ("a map type with that `values`") and MU-18 ("a map type `CM` over `Clock`") restate FR-322-AC-107/AC-109, which this diff updated to carry `keys` and `bound`, but the rows were not updated. Fix: add `keys` `ix://quire/native/Text` and `bound` `{lower: 0}` to `MO` in TC-234 I32 and to the map types of TC-280 MU-17 and MU-18. | spec/test-cases/TC-234-domain-package-model-intake.md:74; spec/test-cases/TC-280-model-owned-member-types.md:214; spec/test-cases/TC-280-model-owned-member-types.md:215; spec/functional/type-model/FR-154-admit-domain-package-model.md:133 |
| FND-004 | low | "A value type of the package bound as `Int[lo, hi]`, `Text[min, max; profile]` or `Rational[..]`" has two readings for a Rational value type with a `unit`, which FR-322:1934 types as the carried quantity `Quantity<u, Rational[..]>`, not `Rational[..]`. The test's `scalar_key` admits every `kind` `value`, including a `Decimal[...]` value type and a bare-`Text` value type that FR-322 refuses, so it would pass a wrong rule. Fix: name the carried-quantity case in FR-154 (admitted, keyed by FR-144 "Quantity of one unit", or refused). Narrow `scalar_key` to the three bound forms, and add one refused PM case for a `Decimal[...]` or unbounded value type. | spec/functional/type-model/FR-154-admit-domain-package-model.md:339; spec/objects/interfaces/FR-322-checked-package-artifact.md:1934; tests/package_map_types.rs:98 |

## Consistency checks that pass

- Ineligible key list: FR-154:344-350, FR-144:134-137, FR-415:312-317,
  FR-322:3949-3953, FR-154-AC-49, TC-234 I36 and PM-08..PM-18 name the same
  set (clock-bearing, record, tuple, union, option, collection, map,
  reference/object) with `ill_typed`/`operator-ineligible` at
  `TypeDefinition.keys`.
- QSL FR-230 "Clock collections" and "Place identity" agree with FR-415:284-292
  and FR-415-AC-27 on key selection, the `ref` refusal
  (`ill_typed`/`type-mismatch`) and the snapshot refusal
  (`invalid_runtime_input`/`invalid-value`).
- The three published timed documents and `model-member-type-vectors.json`
  carry `keys` `Text` and `bound` `{lower: 0}` for `ClockMap`, and FR-322-AC-241
  reads its row as `map`, `0`, `unbounded` from `bound`.
- FR-154-AC-27, FR-322-AC-107 and AC-109, and TC-234 I19 and I21 were updated
  to carry `keys` and `bound`.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Option a. A native `Text` key is typed by the `scalar_type`/`text` node and keyed by its exact scalar sequence, like `String` (FR-154 "Map types", FR-144, FR-322 map row). It is the key type only, so a bare-`Text` member still has no type. The natives `Rational` and `Decimal`, `Decimal[...]` value types and unbound value types are refused as keys (PM-32, PM-33). |
| FND-002 | fixed | `reads` is now the FR-322 row `{rendered, min, max}`, where `rendered` gives a native key by its name and any other key by its `keys` reference, and FR-322 states that rule. `package_map_types.rs` renders it the same way. |
| FND-003 | fixed | TC-234 I32 `MO`, TC-280 MU-17 and MU-18 now declare `keys` and `bound`. The `MO` patch in `model-option-type-vectors.json` does too. |
| FND-004 | fixed | A bounded Rational value type is a key with or without a `unit`. The test's `scalar_key` admits only the `integer`, `string` and `rational` value types, and PM-32 refuses a `Decimal[...]` key. |
