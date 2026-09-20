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
- `package` — exact FR-322 checked-package reference, plus the `RawSourceRef`
  digest of every source and definition document a replay recompiles; a
  non-replay request carries the reference alone.
- `byte_provision` — present on a replay request only: the digest-addressed
  bytes of every source, definition and domain-package input the replay
  recompiles, each keyed by its declared digest domain and digest (QSL
  ADR-013 QC-1). The executor obtains every recompilation input from this
  provision and never from a path, environment variable or search location.
- `selection` — ordered clause/function identifiers.
- `state_environment` — typed anchors, values, object populations, frames and invocation captures.
- `limits` — the `quire.value.accounting/v1` limit objects: `ScalarLimitsV1`,
  plus `PopulationAdmissionLimitsV1` when the request binds a population.
  Work maps to `work_units`, output to `result_units`, and allocation to the
  high-water size counters. Depth counts toward the high-water size
  counters. A zero limit admits zero units.
- `results` — one ordered typed disposition per selected item.
- `replay` — present only on a replay request, and absent otherwise. It
  carries: the originating counterexample identity and expected originating
  verdict; the `ReplaySource` (`Witness` or `Input`, FR-197); the selected
  function's `QualifiedName`; arguments keyed by parameter node id; the
  semantic profile selections; the failing node's occurrence key
  (`node_id`, `role`, `ordinal`, FR-322 `source_map` key); the proof bounds
  and declared domains carried from the obligation identity; the trace
  position; and the `backend` member (QSL ADR-013 O-19). No replay member is
  inferred from the package or from the non-replay properties above (QC-8).

## Outcome-to-verdict map

QSpec fixes the replay outcome-to-verdict map once per QSL ADR-013 O-16
category (FND-009; QC-8); no request instance selects or overrides a
category's verdict.

- When a replay outcome is settled, the producer SHALL apply the verdict
  fixed for the outcome's QSL ADR-013 O-16 category.
- The producer SHALL settle no agreement for an `Undefined` outcome or for
  a refused `FamilyOutcome` (its `Refused` case, QSL ADR-011 #209 FND-008
  S6a stage), regardless of the `replay` property's expected originating
  verdict.

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
| FR-323-AC-4 | The outcome-to-verdict map assigns one verdict per QSL ADR-013 O-16 category, fixed independently of any request; an `Undefined` outcome and a refused `FamilyOutcome` never settle agreement. | Test (TC-273) |
| FR-323-AC-5 | A replay request's `byte_provision` supplies every source, definition and domain-package input the replay recompiles, each keyed by its declared digest domain and digest; the `package` reference's `RawSourceRef` digests are exactly the source and definition digests the executor recomputes and compares, and the executor reads no recompilation input by path, environment variable or search location. | Test (TC-277) |
| FR-323-AC-6 | A replay request's `replay` property carries the `ReplaySource`, the selected function's `QualifiedName`, arguments keyed by parameter node id, the semantic profile selections, the failing node's occurrence key, the proof bounds and declared domains, the trace position and the `backend` member; a non-replay request carries no `replay` property. | Test (TC-278) |

## Dependencies

- [interface_005](./interface_005-native-runtime.md) owns the operation boundary.
- FR-180 owns the behavior that produces or consumes this value.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
