---
id: SR-9133
title: "Failure-domain review of the domain package map type key and bound"
type: SpecReview
analysis: failure-domain
scope: "quire-specification spec/package-map-key-bound diff origin/main...cd6c4059: FR-154 Map types, FR-322 map row and timed-model nodes, FR-415 Clock places and Element places, map-type-vectors.json package_map_types, tests/package_map_types.rs"
review_set: subset
review_date: "2026-11-08"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-154, type: references }
  - { target: ix://agent-ix/quire-specification/FR-322, type: references }
  - { target: ix://agent-ix/quire-specification/FR-415, type: references }
---

## Summary

REVISE. The key-selected place rule and its two refusals cover the identity
hazard: a decimal slot cannot name a key. The bound has an unhandled edge. A
`bound` with a non-zero `lower` and no `upper` is admitted by FR-154, typed with
the lower bound dropped by the new FR-322 row, and refused outright by FR-322's
general multiplicity rule. The test renders it a third way. Two smaller gaps:
"exactly three members" has no error path, and FR-322 still calls a map
place's selector tuple its "place identity".

## Review set

The topological edges were walked from the boundaries of `bound` (absent
`upper`, `lower` 0 and above 0, `lower` equal to and above `upper`). Identity
was checked from map places nested in and around ordered collections. Members
were checked across type kinds.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `bound` `{lower: l}` with `l > 0` and no `upper` has three incompatible meanings, and no vector covers it. (1) FR-154:325 and row 133 admit it. (2) The new FR-322 map row types it as the bare `composite_type`/`map` node, the lower "checked by construction at run time (FR-144)" and "carried by no node". That drops `l` from the checked package, so no reader of the package can enforce it. (3) FR-322:2069-2072 says "an `unbounded` upper bound with a lower bound above `0` … give[s] the member no type, and the application refuses `ill_typed`/`operator-ineligible`", which TC-280 MA-07 tests for `[1, unbounded]`. `package_map_types.rs:176` renders it `Map<K, V>[l, unbounded]`, a form neither spec gives. Fix: treat it as a field multiplicity is treated. Either refuse it (FR-154: `invalid_model_binding`/`unpreserved-model-meaning` at `TypeDefinition.bound`, or let FR-322:2069 refuse it) and add a refused PM case `{lower: 1}`, or define a lower-only `collection_bounds` form that carries `l` and amend FR-322:2069 and the multiplicity table to match. Either way, remove the "checked at run time … carried by no node" clause. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1942; spec/objects/interfaces/FR-322-checked-package-artifact.md:2069; spec/functional/type-model/FR-154-admit-domain-package-model.md:325; spec/test-cases/TC-280-model-owned-member-types.md:176; tests/package_map_types.rs:173 |
| FND-002 | medium | FR-154 says a map type "declares exactly three type members", but no row refuses a fourth (for example `items`, `inner`, `target`, `positions` or `fields` on a `map`). The reverse is unstated too: `keys` or `bound` on a `sequence`, `alias` or `record`. The option row (FR-154:131) refuses extra members at their path. FR-322-AC-241 says a bound on a `sequence` definition is not read, which is the opposite disposition. The order statement at FR-154:353-355 also leaves out `values` failures, so a map with a bad `values` and an absent `bound` has no defined first refusal. Fix: add a map row like the option row (an extra member refuses `invalid_model_binding`/`malformed-declaration` at that member's path), say whether `keys`/`bound` on another kind is refused or ignored, add `values` form and resolution to the report order, and add one PM case for an extra member. | spec/functional/type-model/FR-154-admit-domain-package-model.md:322; spec/functional/type-model/FR-154-admit-domain-package-model.md:131; spec/functional/type-model/FR-154-admit-domain-package-model.md:353; spec/objects/interfaces/FR-322-checked-package-artifact.md:4552 |
| FND-003 | medium | FR-322:4093 still says the presence operand's "(owner, `clock_place` node id, slots)" is "the place identity (QSL FR-230 'Place identity')". Its slot terms include "a key for a `Map`" (FR-322:4092). FR-415:284-292 and QSL FR-230 now say a place under a `Map` value has no place id. So the text gives a map place a place identity and denies it one, and a reader can derive a place id from the presence operand of `Call.m.["a"].x`. Fix: at FR-322:4093, call the operand the place's selector terms, and add that for a place under a `Map` value they are no place id (FR-415 "Clock places"). | spec/objects/interfaces/FR-322-checked-package-artifact.md:4093; spec/functional/temporal/FR-415-declare-clocks-and-timed-models.md:284 |
| FND-004 | low | `keys` naming a package node that is not a type at all (namespace, event type, state machine, process, persistence interface) has two readings. FR-154:342 "every other key type" makes it `ill_typed`/`operator-ineligible`, while the parallel option row refuses a non-value-type `inner` as `invalid_model_binding`/`malformed-declaration`. `package_map_types.rs::resolve` treats any node as a type and so follows the first reading. Fix: state the code for a non-type `keys` (matching the option row's `malformed-declaration` is the consistent choice), and add a PM case naming a namespace. | spec/functional/type-model/FR-154-admit-domain-package-model.md:342; spec/functional/type-model/FR-154-admit-domain-package-model.md:131; tests/package_map_types.rs:60 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | `{lower: l > 0}` with no `upper` now refuses `invalid_model_binding`/`unpreserved-model-meaning` at `TypeDefinition.bound` (FR-154 row and AC-49, PM-28), which is consistent with FR-322's no-type rule for `[l, unbounded]`. The "carried by no node" clause is deleted. |
| FND-002 | fixed | A map that declares another kind's member (`items`, `inner`, `target` and so on), a `keys` or `bound` on another kind, and an absent `values` each refuse `malformed-declaration` at that member (PM-29, PM-30, PM-34). The report order now includes extra members and `values`. FR-322-AC-241 cites the refusal. |
| FND-003 | fixed | FR-322's presence paragraph now says the owner, node and slot terms are the place identity outside a `Map`, and that a place under a `Map` value is selected by its key term and has no place id. |
| FND-004 | fixed | A `keys` naming a node that is not a type refuses `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.keys`, matching the option row (PM-31). |
