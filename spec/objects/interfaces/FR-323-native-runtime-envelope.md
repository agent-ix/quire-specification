---
id: FR-323
title: "Native runtime request and result envelope"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_005
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-180
    type: references
---

# FR-323: Native runtime request and result envelope

## Description

When a checked package is executed or replayed, the producer SHALL use the immutable NativeRuntimeEnvelope contract below rather than an untyped map, display string or implementation-local structure.

## Properties

- `contract_version` — exactly `quire.native-runtime/v1`.
- `request_id` — identity over package, selection, inputs, limits and options.
- `package` — exact FR-322 checked-package reference.
- `selection` — ordered clause/function identifiers.
- `state_environment` — typed anchors, values, object populations, frames and invocation captures.
- `limits` — the `quire.value.accounting/v1` limit objects: `ScalarLimitsV1`,
  plus `PopulationAdmissionLimitsV1` when the request binds a population.
  Work maps to `work_units`, output to `result_units`, and allocation to the
  high-water size counters. Depth counts toward the high-water size
  counters. A zero limit admits zero units.
- `results` — one ordered typed disposition per selected item.
- `replay` — optional counterexample identity and expected originating verdict.

## Identity and validation

Request identity includes every semantic input and limit. Result identity additionally includes implementation/toolchain pin and canonical per-item results.
Unknown contract versions, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-323-AC-1 | Every selected item yields exactly one value, false/violation, refusal, undefined, incomplete or failed disposition; `undefined` is its own disposition carrying its reason and locus, and `failed` is exactly the `runtime_invariant`/`established-invariant-broken` failed evaluation. | Analysis |
| FR-323-AC-2 | Replay uses the same evaluator and inputs rather than a generated-oracle shortcut. | Analysis |
| FR-323-AC-3 | Missing anchors, exhausted limits and false predicates remain distinct. | Analysis |

## Dependencies

- [interface_005](./interface_005-native-runtime.md) owns the operation boundary.
- FR-180 owns the behavior that produces or consumes this value.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
