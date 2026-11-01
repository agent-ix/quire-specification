# Backend provider envelope `quire.backend-provider/v1`

This directory is the normative wire contract of the
[FR-331](../../spec/objects/interfaces/FR-331-backend-provider-envelope.md)
backend provider envelope, the body of every `quire.plugin-wire/v1` frame
([FR-462](../../spec/functional/tooling/FR-462-speak-the-plugin-wire.md)) and
the value a compile-time provider exchanges with the host. It is a
schema-and-vector boundary. Each reader converts from this wire into its own
types (QSL ADR-013 T-7): no shared Rust crate carries these types, and no
consumer holds a copy of this schema.

## Contents

- `schema.json` is the closed Draft 2020-12 schema
  `urn:quire:backend-provider:v1`. Every member's description names its
  owning requirement. It references the checked-package schema
  `urn:quire:checked-package:v2` for `NodeId`, `Occurrence`,
  `Rational` and the decimal-string forms, so those shapes have one
  definition.
- `vectors.json` holds positive vectors, each with an `owners` map from a JSON
  pointer to the FR that owns that member, and adverse vectors, each an RFC
  6902 patch of one positive vector that the schema refuses, with the FR that
  owns the refusal. The Kani vectors come first: a `kani` manifest, a bounded
  `kani` request item, and results covering `proved`, `refuted` (transcript
  and canonical assignments), `tested`, `inconclusive`
  (`kani_vacuous_proof`, `replay-parity`), `unsupported`
  (`unsupported_projection`/`tool-unavailable`), `declined` (each refusal
  category), `incomplete` (`limit-reached`, `bounded-search-incomplete`, `timed-out`, `cancelled`) and
  `failed` (`tool-failure`, `tool-unavailable` after a passing probe, and
  `runtime_invariant` for an IR mapping invariant break).
  Temporal, state-graph, hyperproperty and probabilistic vectors follow.
- `refinement-failure-vectors.json` holds the wire cases of the FR-379
  `RefinementFailure` definition (the seven model kinds and the protocol
  relation kinds `RefusalNotPermitted` and `TerminalSuccessMismatch`) and the
  product and replay cases of those two kinds (TC-318, TC-431 BP-55).
- `task-priority-vectors.json` holds the optional-priority identity and
  policy-order vectors for FR-419/FR-364. The Rust qualification test checks
  each carrier against the wire schema, checks priority against the declared
  task value, and recomputes the authored EDF/fixed-priority vector orders.
- `tests/backend_provider_v1.rs` validates every vector against the schema
  and checks the package-bytes rule (TC-431).
- `tests/task_priority_vectors.rs` runs the FR-419/FR-364 priority vectors;
  these QSpec checks do not replace the independent QSL strict-reader tests.

## Stages

One envelope has three stages, each a closed object carrying
`contract_version` and `capability_vocabulary`:

| Stage | Members | Plugin-wire frame |
| --- | --- | --- |
| manifest | `manifest` | `hello` body |
| request | `manifest`, `request` | `request` body |
| outcome | `dispositions`, `artifacts`, `results`, `counterexamples`, `accounting` | `result` body |

A request's `manifest` is the registry snapshot its candidates are checked
against (FR-290).

Every descriptor carries `tool` (FR-331 "Backend tool"), a `{name, version}`
object: the executable a backend that fronts an external tool invokes, or the
crate that implements a pure in-process engine. A registry snapshot carries each
descriptor as its provider gave it.

## Package bytes

`request.package` is the RFC 8785 byte string of the
`quire.checked-package/v2` document, spelled as RFC 4648 section 5 base64url
with no padding and zero unused trailing bits, so one byte string has one
spelling and the frame keeps one RFC 8785 encoding. A reader decodes
`package` and admits it with the v2 reader.

A request-stage vector omits `package`; the test fills it
from `../checked-package-v2/fixtures/positive-all-families.json`, so the vector
carries no second copy of that fixture.

## Schema and reader

The schema fixes structure. The reader also enforces what a schema cannot
express: `request_index` values run 0 to n-1 with exactly one disposition and
one accounting record each; candidates, symmetry classes, provision entries
and dependencies are in their stated orders; a result's `kind` equals its
item's; a `counterexample` or `transcript` position names an existing entry;
and the RFC 8785 canonical form of the frame. The schema admits an item with
an absent `kind`, a `kind` outside the vocabulary, an absent `extent`, or an
absent or unknown probabilistic evidence kind, because FR-290 settles each
such item `invalid-request` on its own. A present `extent` is closed, so an
unknown classification or bound kind is refused, as is an item without
`candidates` (FR-331-AC-4).

A `hello` the manifest stage refuses only for FR-290 registration causes is
an FR-290 registration refusal keyed by backend identity, not an FR-462
`malformed-body` failure. FR-290 owns the list of those causes.

Leaves owned by another requirement stay in that owner's shape: FR-272
diagnostic payloads (`detail`), FR-407 bounds, FR-412 objectives, FR-413
scheduler choices, FR-418 component reasons, FR-419 closed-form evidence (the
only open `evidence`; under `well-definedness-unchecked` it is the FR-393
witness arm) and FR-425 protocol-transition identities.
