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
  `urn:quire:checked-package:v2` for `PackageId`, `NodeId`, `Occurrence`,
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
  category), `incomplete` (`limit-reached`, `timed-out`, `cancelled`) and
  `failed` (`tool-failure`, and `tool-unavailable` after a passing probe).
  Temporal, state-graph, hyperproperty and probabilistic vectors follow.
- `tests/backend_provider_v1.rs` validates every vector against the schema
  and checks the package-bytes rule (TC-431).

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

## Package bytes

`request.package` is the RFC 8785 byte string of the
`quire.checked-package/v2` document, spelled as RFC 4648 section 5 base64url
with no padding and zero unused trailing bits, so one byte string has one
spelling and the frame keeps one RFC 8785 encoding. `request.package_id` is
that package's `package_id`. A reader decodes `package`, admits it with the v2
reader and refuses a recomputed `package_id` that differs from
`request.package_id` with `stale_dependency`/`content-mismatch`.

A request-stage vector omits `package` and `package_id`; the test fills them
from `../checked-package-v2/fixtures/positive-all-families.json`, so the vector
carries no second copy of that fixture.

## Schema and reader

The schema fixes structure. The reader also enforces what a schema cannot
express: `request_index` values run 0 to n-1 with exactly one disposition and
one accounting record each; candidates, symmetry classes, provision entries
and dependencies are in their stated orders; a result's `kind` equals its
item's; a `counterexample` or `transcript` position names an existing entry;
and the RFC 8785 canonical form of the frame. The schema admits an item whose
`kind`, `extent` or probabilistic evidence kind is absent or outside its
vocabulary, because FR-290 settles each such item `invalid-request` on its
own; it refuses an item without `candidates` (FR-331-AC-4).

Leaves owned by another requirement stay in that owner's shape: FR-272
diagnostic payloads (`detail`), FR-407 bounds, FR-412 objectives, FR-413
scheduler choices, FR-418 component reasons, FR-419 closed-form evidence and
FR-425 protocol-transition identities.
