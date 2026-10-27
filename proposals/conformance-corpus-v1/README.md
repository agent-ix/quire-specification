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
  model, operation-input, `call`, `object-context`, `replay-request` or
  `bytes` files by path relative to the corpus root. A subject's requirement
  assigns meanings to its consumer-input kinds; this corpus contract does not
  add members to that subject's existing request wire.
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

The fixture is intentionally a small wire example rather than a complete
qualification claim. FR-336-AC-1 still requires corpus owners to cover every
inventory capability with positive and adverse or boundary vectors where
applicable. The QSL runner selects a scope from the primary rows, then runs
only vectors whose own applicability includes the selected QSL consumer, as
specified by QSL FR-350/351.
