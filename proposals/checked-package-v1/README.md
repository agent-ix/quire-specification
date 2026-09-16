# CheckedPackage V1 public interchange

This directory is the normative public transport contract for I04 immutable
checked packages. It is intentionally a schema-and-vector boundary, not a
native Rust package API: QSL remains free to use internal parser/CST/package
structures, but any external consumer receives only the versioned contract.

## Contract contents

- `schema.json` defines the closed `quire.checked-package/v1` wire shape.
- `fixtures/` contains independently authored positive, exact-boundary,
  refusal, mutation and canonical-byte/digest vectors.
- The checked semantic graph is a closed `quire.checked-semantic-graph/v1`
  tagged union. It is source-language-neutral transport, not Contract IR and
  not an alternative language semantic authority.

## Identity rules

The package key is the selected contract version and a
`quire.package.semantic/v1` digest over a non-circular JCS preimage. The
preimage is exactly the `quire.checked-package-id/v1` object containing ordered
edition, profile, definition, model, feature and dependency selections plus the
checked semantic graph's typed-key ordered identity projection. It excludes
package identity itself, source-occurrence/source-map data, raw-byte artifacts,
presentation, runtime inputs, installed backends, diagnostic wording and reader
limits.

Raw source documents, native definition documents, compiled-model documents,
checked semantic graph/package identities and generated Contract IR identities
remain separately typed. Equal hash bytes never authorize substitution across
those domains.

## Reader boundary

A reader selects exactly this version, rejects duplicate and unknown members,
checks canonical bytes and all identity joins before exposing a graph, and
reports malformed/stale/cross-domain/unknown-capability input as typed refusal.
Its outcome is the closed sum type admitted/refused/incomplete. It enforces
caller-provided byte/depth/node/edge/occurrence/diagnostic/work limits;
exhaustion is incomplete, not an empty graph or a successful package.

The QSpec qualification crate validates schema/vector correspondence only. The
Contract IR consumer owns the production Rust strict reader and lowering path.
