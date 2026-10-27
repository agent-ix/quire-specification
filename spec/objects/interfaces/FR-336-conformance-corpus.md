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
  - target: ix://agent-ix/quire-spec-language/FR-350
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
  object-context, population, replay-request or bytes files by `kind` and path relative to
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
`object-context`, `population`, `replay-request` and `bytes`. A consumer
interprets only the file kinds assigned to its subject by its own requirement.
The following test-harness files are owned by corpus-v1. They are adapters to
existing typed operations, not product request formats. A consumer's
requirement states the field mapping to its existing typed API.

### Tagged input value

The closed tagged input-value union reuses the tagged Boolean, Integer and
Reference values used in typed corpus results, and adds a harness-only named
Population reference:

| Tag | Closed fields | Meaning |
| --- | --- | --- |
| `boolean` | `kind`, `value` | `value` is a JSON boolean. |
| `integer` | `kind`, `decimal` | `decimal` is FR-038's canonical signed decimal integer string. |
| `reference` | `kind`, `universe`, `type`, `object_identity` | Reuses FR-100's canonical typed Reference form: the first two are 64 lowercase hexadecimal digits; `object_identity` is nonempty UTF-8. |
| `population` | `kind`, `name` | Harness-only reference to the `population` member's declaration identity in this vector's `population` file. It is resolved by that consumer's adapter after population admission; the file does not carry or mint a `PopulationId`. |

Every tag is a closed object; no other tag or member is admitted. The JSON
Schema publishes these forms as `$defs.InputValue`.

### Call input file

A `call` file is a closed JSON object matching `$defs.CallInput` in the
published schema:

| Field | Type | Constraint |
| --- | --- | --- |
| `function` | String | Existing function identity spelling assigned by the consumer requirement. |
| `arguments` | Array | Each row is the closed object `{parameter, value}`; `parameter` is the declared parameter name and `value` is one tagged input value. |
| `accounting` | Object | Required; any subset of the ten `quire.value.accounting/v1` counter names, each a canonical nonnegative decimal string. The adapter parses it as `u64` before constructing the call; an omitted counter uses its operation's existing default. |

No other call-file field is admitted. The consumer maps each row and limit
onto the existing typed Call API; it does not add a product call wire.

### Population file

A `population` file is a `quire.state.snapshot/v1` document as defined by
QSL FR-106. Its `populations` array and each population's `objects` array
retain their authored order. An object fixture's name is its existing `key`;
its `fields` use FR-106's closed tagged value grammar. A call input value
tagged `population` names exactly one `populations[*].population` declaration
identity in that file. The consumer admits the snapshot and resolves that
name to the admitted binding's opaque `PopulationId`. The harness file and
tagged name never mint, decode or substitute a PopulationId. This is the
corpus harness's Population encoding; native-run/1 has no Population argument
wire encoding (QSL FR-100).

### Replay request file

A `replay-request` file is a closed JSON adapter document whose request
members map one-to-one, by member name and value, to the existing replay
request type selected by the consuming requirement. The harness JSON
`byte_provision` is an array of closed rows `{digest_domain, digest,
bytes_path}`. `digest_domain` is a string or `null` exactly as the typed
request uses; `digest` is 64 lowercase hexadecimal digits. `bytes_path` is
relative to the directory containing that vector's `replay-request` file and
SHALL resolve to exactly one `kind: bytes` path also listed in the same
vector. It SHALL be relative, nonempty and contain no parent traversal. The
adapter reads that file as raw bytes and supplies those bytes for the row's
digest key. The row maps to the existing typed provision tuple
`(digest_domain, digest, bytes)`; it adds no path field to the typed product
request. `$defs.ReplayProvisionPath` publishes the row shape. The consumer
requirement owns the remaining field-to-type mapping and retains existing
digest validation and refusal behavior.

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
| FR-336-AC-5 | The published schema admits each `input.files[*].kind` in the closed vocabulary `source`, `model`, `input`, `call`, `object-context`, `population`, `replay-request` and `bytes`, and refuses another kind. | Test |
| FR-336-AC-6 | The published schema admits exactly the four tagged input-value forms and the closed call-file object with required `function`, `arguments` and `accounting`; it refuses an unknown tag, unknown member, malformed decimal or identity, and a malformed accounting decimal. It also admits a replay-provision path row with exactly `digest_domain`, `digest` and `bytes_path` and refuses extra members or malformed digest text. The consumer adapter refuses a canonical accounting decimal greater than `u64::MAX` before constructing the call. | Test |
| FR-336-AC-7 | For a replay-request input, each `byte_provision[*].bytes_path` resolves relative to the vector directory to exactly a path listed as `kind: bytes` in that vector; an absent, unlisted, absolute or parent-traversing path is refused before request construction. The consumer reads the referenced file as raw bytes and validates it under the named digest record. | Test |

## Dependencies

- [interface_019](./interface_019-conformance-corpus.md) owns the operation boundary.
- FR-311 owns the behavior that produces or consumes this value.
