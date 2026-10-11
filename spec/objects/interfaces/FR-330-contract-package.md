---
id: FR-330
title: "Target-neutral contract package"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/interface_012
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-195
    type: references
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-035
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-038
    type: references
---

# FR-330: Target-neutral contract package

## Description

The lowerer SHALL emit the closed ContractPackage value defined here for each
checked native meaning lowered for backend consumption. The value is an
interchange contract, not an untyped map, display string, implementation-local
structure, authored source, or semantic authority.

## Properties

The ContractPackage is a closed object with exactly six required members:

- `contract_version`: exactly `quire.contract-package/v1`.
- `package`: the closed object `{source, lowering_profile}`. `source` is the
  closed object `{package_id, lock}`. `source.package_id` is the exact
  `PackageId` of the admitted originating `quire.checked-package/v2` value,
  including its `quire.package.semantic/v2` domain and SHA-256 algorithm.
  `source.lock` is that value's exact FR-322 lock, with the same members,
  selected identities, and ordering. It is provenance, not a second package
  or authority; the producer SHALL NOT reconstruct, normalize, or silently
  amend it. `lowering_profile` is the exact FR-322 `DefinitionRef` selected by
  the caller for this lowering operation. It is distinct from, and does not
  duplicate or amend, the source-language `profile_selections` in
  `source.lock`. It SHALL be explicit; no default profile is inferred. It
  cannot reinterpret a semantic profile selected by the lock: if the
  requested lowering profile requires a different semantic selection, each
  affected request is `invalid_input` and emits no graph node. Neither profile
  takes precedence over or silently replaces the other.
- `semantic_graph`: the closed object `{graph_version, nodes}` with exactly
  one `nodes` array. `graph_version` is exactly
  `quire.checked-semantic-graph/v2`. The node array uses FR-322's closed
  node-tag vocabulary, typed-node-key order, stable references,
  dependency-reference and recursion-group rules. Each node occurs once.
  `types`, `expressions`, `clauses`, and `protocol_references` are not separate
  graph properties or alternate reference forms.
- `source_map`: the total source correspondence defined below.
- `feature_accounting`: exactly one lowering record for every requested item,
  in the caller's request order, using the seven-member typed disposition
  vocabulary consumed by Contract IR FR-038.
- `identity`: the canonical semantic identity defined below.

The selected profile's semantic settings are not a seventh top-level property
or a second caller-controlled map. Their authoritative representation is the
exact source-profile selections in `package.source.lock`, together with typed
setting values carried by the semantic graph under FR-322. A value that can
change meaning SHALL be represented there. Display labels, ambient settings,
backend installation, and caller resource limits are not semantic settings and
are not added to the ContractPackage. `package.lowering_profile` records the
separate, explicit FR-195 lowering choice; its compatibility check cannot
change the lock's selected source meaning.

Each `feature_accounting` record is a closed object with exactly the common
members `node_id` and `disposition`; `node_id` is the requested exact
`CheckedNodeId`, and `disposition` is one of the seven values below. Each
variant has exactly the additional payload members listed for it and no
others:

- `lowered`: no additional payload; `node_id` is the stable root-node key in
  `semantic_graph`, and all reachable compatible dependencies are present by
  stable key.
- `unsupported`: the requested `node_id`, the first unsupported reachable
  `unsupported_node_id` in ascending typed-key order, and its `node_tag`.
- `requires_bound`: the requested `node_id` and the first unbounded
  `unbounded_type` in ascending typed-key order.
- `invalid_input`: no additional payload; `node_id` may be absent from the
  originating checked package.
- `failed`: the requested `node_id`, exact FR-038 `limit_kind`, caller's
  `limit`, and `consumed` count at the failed charge. In this ContractPackage
  JSON wire representation, both
  counters are canonical unsigned decimal strings (`0` or a nonzero digit
  followed by digits, with no sign or leading zero); each parses exactly to
  the corresponding FR-038 `u64` value. This preserves the full typed range in
  the RFC 8785 identity without IEEE-754 rounding.
- `invalid_body`: the requested `node_id`, offending `body_node_id`, and exact
  FR-322 `CheckedPackageRefusal`.
- `body_incomplete`: the requested `node_id`, stopped `body_node_id`, and
  exact FR-322 `CheckedPackageIncomplete`.

The two defensive payloads have closed serialized shapes. The `invalid_body`
record carries `refusal`, a closed object with required string `code` and
optional members `path`, `cause`, `locus`, `contract_version`, and
`document_pointer`. The field names and types are those of Contract IR
FR-038's `CheckedPackageRefusal`: `code` uses the exact FR-322 code spelling;
`path` is its RFC 6901 pointer into the checked-package document; `cause` uses
the FR-272 spelling paired by FR-322; `locus` is its exact closed
`CheckedNodeId` object `{domain, digest}`; `contract_version` is the string
read for `unknown_contract_version`; and
`document_pointer` is its RFC 6901
pointer into a supplied domain-package document. Each optional JSON member is
present exactly when the corresponding FR-322 typed field is present and is
omitted otherwise, never written as `null`, empty text, or a fabricated
location. The reader requires `contract_version` for
`unknown_contract_version` and forbids it for every other `code`; absence of
that required member uses `missing-member`, and presence for another code uses
`unknown-member`.

The `body_incomplete` record carries `incomplete`, a closed object with
required `limit_kind`, `limit`, and `consumed`, plus optional `path`, matching
the typed fields of Contract IR FR-038's `CheckedPackageIncomplete`.
`limit_kind` uses exactly one of the lowercase values `bytes`, `nodes`, `edges`,
`occurrences`,
`diagnostics`, or `work`. `limit` and `consumed` are canonical unsigned
decimal strings with the same grammar as the `failed` record and parse exactly
to their typed FR-038 `u64` values; `consumed` is the counter value at the
failed charge, as defined by FR-038. `path` is the FR-038 RFC 6901 pointer into
the checked-package document. FR-038 requires it for every limit kind except
`bytes`; the reader requires it for non-byte kinds and forbids it for `bytes`,
using `missing-member` and `unknown-member`, respectively. The nested records
do not use FR-322's outer `ReadResult` envelope or add a second diagnostic
vocabulary. The reader validates each counter's spelling and `u64` range but
does not infer whether `consumed` could have resulted from a charge against
`limit`; that relationship is producer accounting truth owned by FR-038 and
is checked against the originating typed outcome when emitted.

For example, this illustrative serialized shape of a defensive body limit
result uses placeholder node digests and an illustrative checked-package
pointer; the domain and field types are normative:

```json
{
  "node_id": {
    "domain": "quire.checked-semantic-node/v1",
    "digest": "0000000000000000000000000000000000000000000000000000000000000000"
  },
  "disposition": "body_incomplete",
  "body_node_id": {
    "domain": "quire.checked-semantic-node/v1",
    "digest": "1111111111111111111111111111111111111111111111111111111111111111"
  },
  "incomplete": {
    "limit_kind": "work",
    "limit": "8",
    "consumed": "8",
    "path": "/semantic_graph/nodes/3/body"
  }
}
```

The example shows the present `path` form. An absent typed path is omitted,
never encoded as `null` or an empty string.

The record's variant determines its required payload; payloads for other
variants are forbidden. `invalid_body` and `body_incomplete` are defensive
records: a package admitted by FR-322 should not produce them, but the closed
consumer contract admits them so a body refusal or limit stop is not guessed
past or erased. A non-`lowered` record emits no substitute semantic node for
that item. Sibling records remain independent. The two defensive records
reuse FR-322's typed refusal/incomplete forms and all seven variants match
Contract IR FR-038; no second disposition or diagnostic vocabulary is created.

`source_map` is a closed value containing ordered `nodes`, `requested_items`,
and `refusals` collections:

- Each `nodes` entry is `{node_id, occurrences}`. `node_id` is one output
  semantic node's stable key, and `occurrences` is an ordered array of exact
  FR-322 `SourceMapEntry` values `{node_id, role, ordinal, regions}` from the
  originating checked package. There is exactly one entry per output semantic
  node.
- Each `requested_items` entry is `{requested_item, occurrences}`. It names
  the exact requested `CheckedNodeId` and the ordered FR-322 source-map entries
  for the source occurrences associated with that request. There is exactly
  one entry per requested item.
- Each `refusals` entry is `{requested_item, disposition, occurrences}`. It
  points to exactly one non-`lowered` accounting record by the exact requested
  id and disposition and preserves every source occurrence available in the
  checked package, even when no output node is emitted.

An absent requested key has an explicit requested-item entry with an empty
`occurrences` array and exact requested-id subject; it MUST NOT be given a
guessed source span or silently omitted. Occurrence keys, source-region
identity, byte intervals, and ordering are exactly those in FR-322. A
synthesized node maps to the source occurrences that derive it. The three
collections have one entry per subject in semantic node-key order or original
request order as applicable; occurrence keys and regions retain FR-322 order.

`identity` is the closed value `{domain, algorithm, digest}` with domain
exactly `quire.contract-ir.semantic/v1`, algorithm `sha256`, and digest exactly
64 lowercase hexadecimal characters. Its digest is SHA-256 of the UTF-8 RFC
8785 canonical JSON encoding of this closed projection, with no envelope:

```json
{
  "contract_version": "quire.contract-package/v1",
  "package": {
    "source": {"package_id": "<exact PackageId>", "lock": "<exact FR-322 lock>"},
    "lowering_profile": "<exact DefinitionRef>"
  },
  "semantic_graph": {
    "graph_version": "quire.checked-semantic-graph/v2",
    "nodes": "<exact key-ordered node array>"
  },
  "feature_accounting": "<exact ordered per-item records>"
}
```

The quoted placeholders above denote values themselves, not strings in the
preimage. `identity` is excluded to avoid recursion. The projection contains
the parsed JSON values of the named serialized wire members, including
decimal-string counters exactly as strings; it does not convert them to JSON
numbers before hashing. The originating package identity and lock, lowering
profile, every graph node, and every terminal
accounting result are included. `source_map` is strictly validated but excluded
from the semantic digest because occurrence coordinates establish provenance,
not semantic meaning. A change only to valid source occurrence/region data
therefore leaves identity unchanged; corruption or incompleteness still refuses
admission. Caller limits are not
copied as separate package settings; where a `failed` accounting record names
the exact failed `limit_kind`, limit and consumed count, those result fields
are part of that record and therefore the projection. A limit change that
changes a record changes the identity; a limit change that leaves every
serialized member unchanged does not. Execution traces, timestamps, display
text, diagnostic rendering, backend state, JSON whitespace, and the identity
value itself are excluded. Object member order follows RFC 8785; semantic
array order follows FR-322 and the request-order rules above. The domain label
is outside the preimage and is compared exactly as FR-201 requires; it is not
prefixed to the hashed bytes.
The digest uses FR-201's `quire.contract-ir.semantic/v1` domain, not the
checked package's `quire.package.semantic/v2` identity.
After validating the typed projection, a declared digest that differs from
the recomputed digest refuses `stale_dependency`/`content-mismatch`; the FR-272
diagnostic carries the declared digest as `expected` and the recomputed digest
as `actual`.

The ContractPackage reader is the single owner of outer wire admission. It
first applies only the generic wire checks reused from FR-322: strict JSON
parsing including duplicate-member rejection, then canonical-byte validation.
It then validates
FR-330's own exact `quire.contract-package/v1` version (it MUST NOT apply
FR-322's `quire.checked-package/v2` version predicate to this output), and
checks the exact top-level and nested closed shapes,
typed node keys and references, package provenance, profile selection,
source-map totality, feature-accounting/request bijection, and recomputed
identity before exposing an admitted typed value to a consumer. The order
after generic wire checks is: outer version, all top-level and nested closed
shapes and typed values, package and profile validity, source-map totality, and
accounting/request consistency, then identity recomputation, dependency inputs
in lock order, then graph references. Thus a
malformed nested payload is refused before identity is recomputed, while a
well-shaped changed value with a stale identity is refused at identity
validation. It returns the existing typed `admitted`, `refused`, or `incomplete` result
envelope shape, with FR-330 validation failures mapped to the existing FR-322
code/cause pairings and FR-272 catalogued diagnostics at `package_read`; limit
stops retain the exact incomplete kind, bound and consumed count. These are
FR-330 reader results, not a claim that the output value is an FR-322 checked
package. A consumer SHALL receive only an admitted
typed ContractPackage and SHALL NOT reparse the wire or reinterpret a refusal.
Unknown version, missing, duplicate, extra, wrong-kind, out-of-domain or
noncanonical data refuses before consumption. Strict nested-member parsing
detects a duplicate JSON member before canonical-byte comparison or identity
validation; it uses FR-322's `invalid_package` code and `duplicate-member`
cause at that member's ContractPackage pointer. A nested payload's missing,
duplicate, unknown, wrong-kind, or out-of-domain member uses FR-322's
`invalid_package` code with its corresponding `missing-member`,
`duplicate-member`, `unknown-member`, `wrong-value-kind`, or `invalid-value`
cause and the pointer into the ContractPackage document. A null optional
member is `wrong-value-kind`. A payload member forbidden by its selected
disposition is `unknown-member`. Limits remain caller inputs to reading and
lowering; they are not package data or identity inputs.

The reader input also includes one admitted FR-322 dependency package for every
entry of `package.source.lock.dependency_selections`, keyed by that entry's
library `identity`, as FR-322's `dependency_selections` rule requires. These
admitted values are inputs, not ContractPackage members, and do not enter its
identity; their selected `package_id` values remain in `package.source.lock`.
After identity validation in the order above, the reader checks dependency
inputs in lock order before checking graph references. It requires
each selected input and confirms both its library `identity` and admitted
`package_id` equal the selection. The admitted input has already had its own
`package_id` verified by FR-322. A missing input, including a value supplied
under the selected key with a different library identity, refuses
`missing_import`/`missing-selection` at the corresponding ContractPackage
pointer
`/package/source/lock/dependency_selections/{i}`. A supplied admitted package
with the selected identity but a different `package_id` refuses
`stale_dependency`/`content-mismatch` at that same pointer. Its FR-272
diagnostic carries the selected `package_id` as `expected` and the supplied
`package_id` as `actual`, as required for a typed content-mismatch; the library
identity is the selected lock entry at that pointer.
An extra input keyed by an identity absent from the lock is ignored and cannot
satisfy or redirect a selected reference. Only after these checks does the
reader resolve each `dependency_reference`, and it resolves it only against
the admitted package keyed by that reference's selected library identity. The
lock alone is not evidence that a selected package was supplied or admitted.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-330-AC-1 | Every represented node has identical native meaning under its explicit selected lowering profile. | Test (TC-935) |
| FR-330-AC-2 | An unrepresentable item has no substitute node and retains exactly one typed terminal refusal with every available source locus (or its exact requested identity when no source key exists), while independent sibling records remain unchanged. | Test (TC-933, TC-934) |
| FR-330-AC-3 | When two packages have the same included semantic projection, their canonical identities are equal; when any included projection member differs, their identities differ. | Test (TC-936) |
| FR-330-AC-4 | A package has exactly one semantic node collection ordered by the FR-322 typed node key; each node occurs once, local references resolve to compatible nodes, dependency references retain FR-322 selection semantics, protocol references name protocol-tagged nodes, and recursion is represented only by permitted keyed references. | Test (TC-931, TC-937) |
| FR-330-AC-5 | The embedded source `package_id` and lock exactly equal those of the originating admitted checked package; the explicit lowering profile is separate from lock profile selections, and any semantic-profile disagreement produces `invalid_input` before node emission without replacing the locked meaning. | Test (TC-932, TC-935) |
| FR-330-AC-6 | `feature_accounting` has one ordered record for each requested item and exactly one disposition from the seven-member vocabulary; non-`lowered` records have no substitute node, and `invalid_body.refusal` and `body_incomplete.incomplete` use the exact closed nested shapes and omission rules above. | Test (TC-933) |
| FR-330-AC-7 | `source_map` has an entry for every semantic node, requested item, and refusal; an absent `CheckedNodeId` remains an explicitly mapped `invalid_input` with no fabricated region, while every available occurrence and region matches FR-322 exactly. | Test (TC-934) |
| FR-330-AC-8 | The identity recomputes from the specified RFC 8785 projection under `quire.contract-ir.semantic/v1`; changing package provenance, lowering profile, graph, or an accounting result changes it, while changing only valid source locations, caller limits that leave serialized members unchanged, rendering, or JSON layout does not. Source-map corruption still refuses admission. | Test (TC-936) |
| FR-330-AC-9 | The sole reader admits a valid canonical value only with every lock-selected admitted dependency package supplied and matching its selection; it ignores extra unselected inputs, refuses a missing or wrong-identity dependency as `missing_import`/`missing-selection` and a same-identity admitted package with a different `package_id` as `stale_dependency`/`content-mismatch` at the selected lock pointer, all before dependent-reference checks. It refuses malformed nested payloads as `invalid_package` with FR-322's `missing-member`, `duplicate-member`, `unknown-member`, `wrong-value-kind`, or `invalid-value` cause as applicable, and refuses each wrong version, invalid reference, incomplete source map, accounting mismatch, or identity mismatch with a typed FR-330 reader result and FR-272 diagnostic mappings before any consumer sees a value. | Test (TC-937) |

## Dependencies

- [interface_012](./interface_012-contract-package.md) owns the operation
  boundary and its caller-supplied limits.
- [FR-322](FR-322-checked-package-artifact.md) owns the admitted input's
  closed node, profile, lock, source occurrence, and source-region forms reused
  here.
- [FR-195](../../functional/ir-proof/FR-195-lower-complete-contract-ir.md)
  owns exact lowering and the five ordinary dispositions plus the two
  defensive walk outcomes. Contract IR
  [FR-035](ix://agent-ix/quire-contract-ir/FR-035) owns the emitted package's
  six-property outer shape; Contract IR [FR-038](ix://agent-ix/quire-contract-ir/FR-038)
  consumes its seven-member disposition vocabulary, including defensive
  records. Neither defines a competing QSL wire schema.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) owns exact
  identity-domain separation; this requirement selects its existing
  `quire.contract-ir.semantic/v1` domain for the ContractPackage projection.
