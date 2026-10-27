---
id: FR-336
title: "Independent complete-V1 conformance corpus"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/interface_019
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-311
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-018
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-005
    type: references
---

# FR-336: Independent complete-V1 conformance corpus

## Description

When the complete-V1 conformance corpus is published or executed, the producer SHALL use the immutable ConformanceCorpus contract below rather than an untyped map, display string or implementation-local structure.

The closed Draft 2020-12 schema and its example fixture are published in
[conformance-corpus-v1](../../../proposals/conformance-corpus-v1/README.md).
The schema's `$id` is `urn:quire:conformance-corpus:v1`.

## Corpus members

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| contract_version | String | 1..1 | exactly `quire.conformance-corpus/v1` |
| vectors | CorpusVector | 1..many | ordered; every vector names one primary capability |
| results | QualificationResult | 0..many | prior outcomes only; consumers preserve these rows |
| coverage | CorpusCoverage | 1..1 | primary capability inventory and rollups |

- `contract_version` — exactly `quire.conformance-corpus/v1`.
- `vectors` — an ordered array of vectors. Each vector has `id`,
  `capability_id`, `class`, `subject`, `applicability`, `input` and
  `expected`. `input.files` lists source, model, operation-input, call,
  object-context, replay-request or bytes files by `kind` and path relative to
  the corpus root. Consumer requirements assign meanings to these file kinds;
  they do not add members to the corpus vector or to their existing request
  wires. `expected.disposition` is
  either `value` or `refusal` and is the vector's independently authored
  oracle: `value` carries the operation's
  typed JSON result; `refusal` carries `code` and `cause`, with optional
  operation-defined `details`.
- `class` — exactly one of `positive`, `lower-boundary`, `upper-boundary`,
  `just-outside`, `refusal`, `incomplete` or `metamorphic`. A metamorphic
  vector names the vector it mutates in `mutation_of`.
- `subject` — the `consumer_type` and operation the vector exercises.
  The consumer type is one of `language`, `runtime`, `monitor`,
  `analyzer/backend`, `tool` or `consuming-system`.
- `applicability` — each `coverage.primary` capability entry lists its
  required `consumer_types` and `semantic_profiles`; each vector lists the
  `consumer_types` to which it applies. Its `subject.consumer_type` SHALL
  appear in the vector's applicability.
- `results` — prior subject/run/vector outcomes (`passed`, `failed`,
  `unsupported` or `unexecuted`) and artifact references. A consumer
  executing the corpus SHALL NOT rewrite this member.
- `coverage` — `primary` contains exactly one capability entry for each
  inventory capability; `criteria_rollups` and `consumer_rollups` provide
  the criterion and consumer views. Each vector SHALL name a capability
  from `coverage.primary`.

The `kind` vocabulary of `input.files` is `source`, `model`, `input`, `call`,
`object-context`, `replay-request` and `bytes`. A consumer interprets only the
file kinds assigned to its subject by its own requirement; this interface
does not define a second encoding for those subject-owned inputs.

## Identity and validation

Vector identity covers its complete input bytes and expected oracle; corpus
identity covers the ordered vector set. The schema rejects missing required
members, unknown members and out-of-domain values. Before executing any
vector, a reader SHALL also validate unique member names, unique vector and
primary capability IDs, vector-to-capability and rollup references, and
canonical JSON encoding. Input paths are relative to the corpus root. A
syntactically valid input path whose file cannot be read is a per-vector
execution failure, not a corpus preflight refusal.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-336-AC-1 | Every inventory capability has positive and adverse/boundary vectors where applicable. | Analysis |
| FR-336-AC-2 | Removing or mutating a required vector makes qualification incomplete. | Analysis |
| FR-336-AC-3 | A producer cannot qualify itself solely with an oracle sharing its implementation. | Analysis |
| FR-336-AC-4 | The published schema admits the complete-v1 fixture and refuses a missing required member, an unknown member, and out-of-domain class, consumer-type or expected-disposition values. | Test |
| FR-336-AC-5 | The published schema admits each `input.files[*].kind` in the closed vocabulary `source`, `model`, `input`, `call`, `object-context`, `replay-request` and `bytes`, and refuses another kind. | Test |

## Dependencies

- [interface_019](./interface_019-conformance-corpus.md) owns the operation boundary.
- FR-311 owns the behavior that produces or consumes this value.
