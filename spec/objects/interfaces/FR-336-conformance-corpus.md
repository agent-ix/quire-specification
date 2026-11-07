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
  - target: ix://agent-ix/quire-specification/FR-323
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
| `reference` | `kind`, `universe`, `type`, `object_identity` | Reuses FR-100's canonical typed Reference form: `universe` and `type` are node ids `{package, path, ordinal}` (FR-322); `object_identity` is nonempty UTF-8. |
| `population` | `kind`, `name` | Harness-only reference to the `population` member's declaration identity in this vector's `population` file. It is resolved by that consumer's adapter after population admission; the file does not carry or mint a `PopulationId`. |

Every tag is a closed object; no other tag or member is admitted. The JSON
Schema publishes these forms as `$defs.InputValue`.

### Call input file

A `call` file is a closed JSON object matching `$defs.CallInput` in the
published schema:

| Field | Type | Constraint |
| --- | --- | --- |
| `function` | String | Existing function identity spelling assigned by the consumer requirement. |
| `arguments` | Array | Each item is one `$defs.InputValue`. Items are positional: item at zero-based index `i` supplies the function's declared parameter at index `i`. |
| `accounting` | Object | Required; any subset of the ten `quire.value.accounting/v1` counter names, each a canonical nonnegative decimal string. The adapter parses it as `u64` before constructing the call; an omitted counter uses its operation's existing default. |

No other call-file field is admitted. The consumer resolves the declared
parameter names from the selected function signature, pairs each item with the
parameter at the same zero-based index, and maps each pair and limit onto the
existing typed Call API; it does not add a product call wire. If an item has
no corresponding declared parameter, the consumer reports an input-adapter
refusal rather than dropping or reassigning it. A shorter array remains a
prefix call and the typed operation settles the unbound declared parameters.

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

A `replay-request` file is a closed JSON adapter document matching
`$defs.ReplayRequest`. It is corpus-owned, not a product wire. Its top-level
members map one-to-one to the corresponding `ReplayRequestWire` fields in the
consuming requirement. Member values use FR-323 `NativeRuntimeEnvelope`
shapes for replay source, target and result; all other nested typed values use
the corpus-owned structural encoding below. A structural object has exactly
one member per Rust field, uses that field's `snake_case` spelling and refuses
unknown members. Integers and limit values are canonical decimal strings.
Every Rust enum is a closed object tagged by `kind`, with the Rust variant's
`snake_case` name and exactly that variant's fields. Arrays retain vector
order; object maps retain their string keys.

#### Top-level field mapping

| JSON member | Rust field and value mapping |
| --- | --- |
| `profile_selections` | `ReplayRequestWire.profile_selections`: ordered `ProfileSelection` objects `{profile, value}`. |
| `package` | `ReplayRequestWire.package`: the whole proved checked-package document (`urn:quire:checked-package:v2`, `contract_version` the constant `quire.checked-package/v2`); the wire's package name is `package.name`, its sources are `package.lock.sources` and its refinement internals are `package.lock.refinement_internals`. |
| `dependencies` | `ReplayRequestWire.dependencies`: ordered `DependencyEntryWire` objects, the schema's `$defs.DependencyEntry`: the closed top-level `{identity, sources}` entries plus an optional `refinement_internals` (a non-empty list of the checked-package `RefinementInternal` rows, the dependency lock's value; absent is the empty list), each source an `{authority, identity}` `SourceRef`. |
| `replay_target` | `ReplayRequestWire.replay_target`: FR-323 target members under `members`, plus request fields `arguments`, `occurrence_key` and `over`. |
| `source` | `ReplayRequestWire.source`: the FR-323 `ReplaySource`, one `witness` or `input` arm. |
| `obligation_identity` | `ReplayRequestWire.obligation_identity`: the structural obligation identity `{node_id, model, ordinal}` ([FR-201](../foundation/FR-201-canonical-identity-domain.md)); the required `model` member is `null` for no owning model or the non-empty owning-model identity string. |
| `backend` | `ReplayRequestWire.backend`: FR-323 backend identity string, unchanged. |
| `state_environment` | `ReplayRequestWire.state_environment`: ordered `[name, value]` string pairs, preserving the stored tuple order. |
| `accounting_limits` | `ReplayRequestWire.accounting_limits`: complete FR-255 `ScalarLimitsV1`; every numeric member is an unsigned decimal string. |
| `stage_limits` | `ReplayRequestWire.stage_limits`: string-keyed map of FR-255 setting names to unsigned decimal strings. |
| `supplied_limits` | `ReplayRequestWire.supplied_limits`: `null` or complete FR-323 `limits.supplied` object `{admission_work_units, conversion_work_units, numeric_magnitude_bits}`. |
| `object_environment` | `ReplayRequestWire.object_environment`: `null` or FR-106 reference `{identity}`; identity is the closed two-member `{authority, identity}`, the FR-001 labels. |
| `declared_domains` | `ReplayRequestWire.declared_domains`: ordered `DeclaredDomain` objects as specified below. |
| `byte_provision` | `ReplayRequestWire.byte_provision`: ordered `$defs.ReplayProvisionPath` rows `{authority, identity, bytes_path}`; all three members are required. The adapter reads each referenced sibling file as raw bytes and constructs `(authority, identity, bytes)`. |

The corpus adapter SHALL map `obligation_identity.model: null` to the existing
typed obligation identity's absent owning model (`None`). It SHALL map a
non-empty string to the present owning-model identity (`Some`), preserving
that string exactly. The member remains required: omission, an empty string
or any other JSON type is refused. In particular, the adapter SHALL NOT infer
a model from the package, selected function, dependency or another identity.
This is a corpus encoding of the existing optional owning-model component;
it does not change the FR-071 replay request or its product wire.

#### FR-323 replay source and target

`source` is the closed FR-323 `ReplaySource` subset:
`{"kind":"witness","transcript":STRING}` or
`{"kind":"input","assignments":[...]}`. A witness transcript is the
admitted, trimmed assertion-block transcript; decoded values are not stored
beside it. An input assignment is keyed by parameter node id, never by
position: `{"parameter": NODE_ID, "value": VALUE}`. `NODE_ID` is a node id
`{package, path, ordinal}` (FR-322). Every node id of a corpus document (a
replay `parameter` key, an `occurrence_key` or `obligation_identity`
`node_id`, a `DomainKey` node, a `reference`'s `universe` and `type`) has the
checked-package `NodeId` shape: the corpus schema's `$defs.NodeId` is
`urn:quire:checked-package:v2#/$defs/NodeId`, so a path segment is a string or
an index segment `{"node": i}` and never a nested node id. Every such id is
document-held (FR-322 "Operand index segments"): an operand slot
`{"operand": k}`, which the shared shape admits, is misplaced there and
refuses `malformed_wire` at its `operand` member. A corpus document
holds no semantic graph, so each index segment of a corpus node id resolves
against the `semantic_graph.nodes` of the checked package the document is
about, written as a document-held id of that package (FR-322 "Operand index
segments"): for a replay request, the checked package of its top-level
`package` member (`ReplayRequestWire.package`), never a `dependencies` entry;
for any other corpus vector, the checked package its `input` compiles to. A
node id's own `package` member names the node's owner and selects no graph: an
imported node's id resolves against the importing package's graph, where the
imported node sits. `VALUE` uses the structural `WitnessValue` tagged union
below because replay assignments may contain complete-V1 values beyond the
corpus call `InputValue` forms. No other replay-source arm is admitted by
this adapter.

`replay_target.members` is exactly one FR-323 target member: `function` with
its FR-323 `QualifiedName`,
`frame_operation` with `{object, operation}`, or `state_clause` with a
qualified name. Absent target members are omitted, never null. The containing
`replay_target` object has exactly `members`, `arguments`, `occurrence_key`
and `over`. `arguments` is boolean; `occurrence_key` is null or the FR-322
`{node_id, role, ordinal}` key with decimal-string ordinal; `over` is null or
`{population, key}`. FR-323's target validity rules still apply, including
the function-only occurrence key and the frame/state-clause absence rules.

The expected replay value is the FR-323 result envelope, a closed
`{"results":[RESULT]}` with exactly one result for this request's one
obligation. `RESULT` uses FR-323's typed result disposition and the matching
FR-072 replay-result arm. It is compared as a typed result, never rendered
debug text. It remains an independently authored expected value and is never
copied from actual replay.

#### Structural nested types

| Rust type | Closed structural fields |
| --- | --- |
| `ProfileSelection` | `{profile, value}`; both strings. |
| `DependencyEntryWire` | `{identity, sources}` plus the optional `refinement_internals`; identity string and source entries use the FR-323 shapes. |
| `ScalarLimits` | `integer_bits`, `decimal_digits`, `scale_expansion`, `text_input_bytes`, `text_scalars`, `normalized_scalars`, `unit_edges`, `value_occurrences`, `work_units`, `result_units`; each is an unsigned decimal string. |
| `RequestTargetWire` | `{members, arguments, occurrence_key, over}` as described above. |
| `ReplayOverBinding` | `{population, key}`, both strings. |
| `DeclaredDomain` | `{domain, kind, bound}`; `domain` is tagged `node` `{node, path}` or `population` `{member_type, ordinal}`, `kind` is null or tagged `DomainKind`, and `bound` uses `FiniteBound`. |
| `DomainKind` | tagged variants `collection`, `population`, `integer`, `recursive`, `loop`, `infinite_trace` or `quantity`; each has only its `kind` tag. |
| `FiniteBound` | tagged `cardinality` `{maximum}`, `integer_range` `{lower, upper}`, `depth` `{maximum}` or `variants` `{members}`; integral values are decimal strings and arrays preserve order. |
| `WitnessValue` | closed tagged variants `boolean(value)`, `integer(decimal)`, `exact_integer(decimal)`, `enum(declaration, member)`, `text(value)`, `uuid(value)`, `timestamp(decimal)`, `string(value)`, `rational(numerator, denominator)`, `decimal(coefficient, scale)`, `float32(bits)`, `float64(bits)`, `quantity(magnitude, unit)`, `reference(universe, object_type, identity)`, `option(value)`, `record(declaration, fields)`, `tuple(declaration, components)`, `union(declaration, member, components)`, `sequence(elements)`, `ordered_set(elements)`, `set(elements)`, and `bag(elements)`. `option.value` is null or another `WitnessValue`; record fields are ordered `{name, slot}` objects whose slot is tagged `present(value)`, `absent` or `null`; quantity magnitude is tagged `rational(numerator, denominator)` or `decimal(coefficient, scale)`. |
| `StateEnvironment` | ordered `[name, value]` pairs of strings, matching its stored tuples. |

Arbitrary byte
vectors use lowercase hexadecimal; signed integers use canonical signed
decimal strings and unsigned integers use canonical unsigned decimal strings.
All object and enum forms are closed. `$defs.ReplayRequest` and its referenced
structural definitions publish these rules in the JSON Schema.

Each adapter `byte_provision` row is exactly
`{authority, identity, bytes_path}`. `authority` and `identity` are the exact
source identity pair used for the provided bytes, as in FR-322 `lock.sources`;
neither field is inferred from the package, path, or another source entry. Both
are required and non-empty. Omitting `authority` is a schema refusal; it has no
default or fallback. `bytes_path` is relative to the
vector directory and SHALL resolve to exactly one `kind: bytes` path listed in the same vector. It SHALL be
nonempty, relative and contain no parent traversal. The adapter reads raw
bytes and supplies those bytes for the row's `{authority, identity}` key. The path is not
added to the typed product request.

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
| FR-336-AC-6 | The published schema admits exactly the four tagged input-value forms and the closed call-file object with required `function`, positional `arguments` containing only tagged input values, and `accounting`; it refuses an unknown tag, unknown member, named `{parameter, value}` argument object, malformed decimal or identity, and a malformed accounting decimal. It also admits a replay-provision path row with exactly the required non-empty `authority`, `identity` and `bytes_path` members and refuses a missing or extra member. The consumer adapter refuses a canonical accounting decimal greater than `u64::MAX` before constructing the call. | Test (TC-969) |
| FR-336-AC-7 | A `replay-request` file matches the closed `$defs.ReplayRequest`; all 14 top-level fields, `package` (the proved FR-322 checked package, required) among them, map to their `ReplayRequestWire` fields, a request without `package` or with a `package` that is not a checked package is refused, every node id the request names resolves in that package's graph, each nested value follows its FR-323 or structural type mapping, integers use canonical decimal strings, unknown object members and tags are refused, and setting names outside FR-255 are refused by the consumer decoder. The Witness/Input source arms, FR-323 replay target members and one-result envelope retain their stated shapes. | Test (TC-970) |
| FR-336-AC-8 | For a replay-request input, each `byte_provision[*].bytes_path` resolves relative to the vector directory to exactly a path listed as `kind: bytes` in that vector; an absent, unlisted, absolute or parent-traversing path is refused before request construction. The consumer reads the referenced file as raw bytes and supplies it under the row's `{authority, identity}`. | Test (TC-970) |
| FR-336-AC-9 | **RETIRED (source-byte integrity removed).** Withdrawn with no replacement: a `$defs.ReplayRequest` `dependencies[*].sources[*]` entry is `{authority, identity}`. | Test (TC-969), retired |
| FR-336-AC-10 | The published schema keeps `object_environment.identity` the closed two-member `{authority, identity}` and refuses any other member. | Test (TC-969) |
| FR-336-AC-11 | A replay-provision path uses the explicit non-empty `{authority, identity}` source key paired with its `bytes_path`. The present pair is admitted; omission of either member or an empty value for either member is refused by the schema at the row, with no default or inference from the package, path or another source. | Test (TC-969; RP-01..RP-05) |
| FR-336-AC-12 | The required `obligation_identity.model` member admits `null` and maps it to no owning model (`None`), or admits a non-empty string and maps it unchanged to the present owning model (`Some`). A missing member, an empty string, another JSON type or an unknown obligation-identity member is refused; no model identity is inferred or fabricated. | Test |

## Dependencies

- [interface_019](./interface_019-conformance-corpus.md) owns the operation boundary.
- FR-311 owns the behavior that produces or consumes this value.
