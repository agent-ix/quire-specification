# Complete-V1 conformance corpus v1

This proposal defines the wire shape owned by QSpec FR-336. `schema.json` is
the closed Draft 2020-12 structural schema identified by
`quire.conformance-corpus/v1`; `fixtures/complete-v1.json` is a small
schema-valid corpus fixture for the native-language formatter.

The corpus has four top-level members:

- `contract_version` selects this exact contract.
- `vectors` contains each vector's `id`, `capability_id`, `class`,
  `subject`, `applicability`, `input` and `expected` members. `subject`
  names a consumer type and its operation. `input.files` names source,
  model, operation-input, `call`, `object-context`, `population`,
  `replay-request` or `bytes` files by path relative to the corpus root. The
  corpus-owned `$defs.InputValue` and `$defs.CallInput` define test harness
  input values and calls; a subject's requirement maps those values to its
  existing typed API and does not add fields to a product request wire.
  `accounting` limits use canonical unsigned decimal strings so the corpus's
  canonical JSON does not round large `u64` values; the adapter parses and
  range-checks them before constructing the typed call.
  `$defs.InputValue` is a closed tagged union of Boolean, canonical Integer,
  canonical Reference, and named Population values. The `population` file is
  the consuming API's object-context document; QSL binds it to FR-106's
  `quire.state.snapshot/v1` document and resolves Population names only after
  admission. A `replay-request` adapter uses `$defs.ReplayRequest`, a closed
  request adapter whose 15 members map one-to-one to `ReplayRequestWire`.
  FR-323 source, target and result members are reused; other nested types have
  closed structural encodings with `snake_case` Rust fields, tagged variants,
  decimal-string integers and no unknown members.
  `$defs.ReplayProvisionPath` rows carry the four QSL FR-001 identity labels
  under `identity` and a `bytes_path` to a sibling `bytes` file. The consumer
  reads raw file bytes into the ordered, unkeyed provision. These are harness adapters, not
  product wires.
  `expected.disposition` is either `value` or `refusal`. `value` carries the
  operation's typed JSON result; `refusal` carries the typed code and cause,
  and may carry operation-defined details. A vector class of `incomplete`
  still expects the operation's typed refusal when that is the expected
  result. A `metamorphic` vector also names its `mutation_of` vector.
- `results` records prior subject/run/vector outcomes and artifact references.
  A consumer executing the corpus leaves this authored member unchanged.
- `coverage` contains `primary` capability entries, `criteria_rollups` and
  `consumer_rollups`. A primary entry's `applicability` declares the consumer
  types and semantic profiles required for that inventory capability.

Structural validation is not the entire admission rule. Before executing
any vector, a reader also checks that object keys are unique, identifiers are
in-domain and unique where they identify records, each vector names one
primary capability, its subject is included by its vector applicability,
rollups refer to listed capabilities, and the JSON document uses its
canonical encoding. A valid path whose file is absent is reported as that
vector's execution failure, as FR-350 requires; it is not a corpus preflight
refusal. These checks do not rewrite `results` or `coverage`.

The fixture `fixtures/replay-request.json` is a schema-valid example of the
closed replay adapter shape. It is not an executable replay vector and makes
no qualification claim. The main corpus fixture remains intentionally a small
wire example rather than a complete qualification claim. FR-336-AC-1 still requires corpus owners to cover every
inventory capability with positive and adverse or boundary vectors where
applicable. The QSL runner selects a scope from the primary rows, then runs
only vectors whose own applicability includes the selected QSL consumer, as
specified by QSL FR-350/351.
