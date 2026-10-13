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
  - target: ix://agent-ix/quire-spec-language/FR-096
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
  amend it. `lowering_profile` is the explicit I12 `LoweringProfileRef`
  supplied as the resolved profile's `reference` to `lower`; it has the same
  closed `{authority, identity}` value form as FR-322 `DefinitionRef`. This
  output records that reference only, separately from the authoritative typed
  requirements described below. It is distinct from, and does not duplicate or
  amend, the source-language `profile_selections` in `source.lock`. It SHALL be
  explicit; no default profile is inferred. Neither value replaces the other.
- `semantic_graph`: the closed object `{graph_version, nodes}` with exactly
  one `nodes` array. `graph_version` is exactly
  `quire.checked-semantic-graph/v2`. The node array uses FR-322's closed
  node-tag vocabulary, typed-node-key order, stable references,
  dependency-reference and recursion-group rules. Each node occurs once.
  `types`, `expressions`, `clauses`, and `protocol_references` are not graph
  properties or separate reference forms. A protocol reference is FR-322's
  `protocol`/`protocol_reference` node in this array, with its body target
  bound by exact local typed node key to a `protocol` node. Its body,
  dependency and semantic-type joins retain FR-322's validation and identity
  rules; no separate protocol-reference collection or implicit target lookup
  is introduced.
- `source_map`: the total source correspondence defined below.
- `feature_accounting`: exactly one lowering record for every requested item,
  in the caller's request order, using the seven-member typed disposition
  vocabulary consumed by Contract IR FR-038.
- `identity`: the canonical semantic identity defined below.

The selected source profile's semantic settings are not a seventh top-level
property or a second caller-controlled map. Their authoritative representation
is the exact source-profile selections in `package.source.lock`, together with
typed setting values carried by the semantic graph under FR-322. A value that
can change meaning SHALL be represented there. Display labels, ambient
settings, backend installation, caller resource limits, and the lowering
profile's capability settings are not added to the ContractPackage.
`package.lowering_profile` records only the explicit I12 profile reference; it
does not amend the lock's selected source meaning. Compatibility is checked
against the resolved profile's typed requirements, never the reference alone.

### Typed source-selection compatibility

The I12 lowering input is the resolved Contract IR FR-035
`CompleteLoweringProfileV2`, whose `required_source_selections` field is a
`Vec<CheckedSelection>` of exact FR-322 `Selection` rows. The owner of those
requirements is the producer of that resolved profile, not the checked-package
producer, the ContractPackage reader, or a backend display label. A caller SHALL
supply the requirements declared by the selected resolved profile without
replacing them with requirements derived from its reference. No registry lookup,
parsed identifier, inferred default, or untyped settings map supplies them.

For each required row, the lowerer SHALL require a row in the admitted input's
`lock.profile_selections` with equal `role`, `definition.authority`, and
`definition.identity`. Equality is exact enum/string equality, including case;
there is no alias, version, prefix, subtype, or digest comparison. The predicate
is conjunction of exact row membership, not array equality or lookup by role
alone. Extra lock rows, including other definitions of the same role, do not
invalidate a match. Repeated identical requirements are idempotent; requirement
order and lock order do not change the predicate. Different requirements sharing
one role require each exact row. An empty requirement vector imposes no
source-selection restriction; it does not infer requirements from `reference`.
This comparison does not re-admit or repair the already admitted lock.

For each requested item, the lowerer SHALL use that request's existing FR-038
work meter and `CompleteLoweringProfileV2.work_limit`. Its counter starts once
at zero for that request. The FR-038 per-request work unit is charged first;
if it exceeds the ceiling the record is `failed` for `work`, with the requested
`node_id`, caller's limit and consumed counter at that failed charge. Thus zero
work yields `failed` with consumed `1` before key lookup or compatibility.
An absent requested key still returns `invalid_input` after that first charge,
without running compatibility, as FR-038 requires.

For an existing requested key, the lowerer SHALL check compatibility before
visiting its semantic-node closure. The charged procedure visits requirement
rows in their supplied order, charging one work unit before each requirement-row
visit, then scans lock profile-selection rows in their original order, charging
one work unit before each candidate-row comparison. The comparison tests the
whole typed row defined above and stops that scan at the first equal row. A
requirement whose scan finds no equal row stops compatibility as a mismatch;
otherwise the procedure continues to the next requirement. Empty requirements
have no compatibility charges. Repeated requirements are checked and charged
again; there is no uncharged normalization, sorting, index construction or
cross-request memoization. Requirement and lock order do not change the
unlimited-budget membership predicate but can change the number of candidate
visits and therefore a resource-limited outcome.

The lowerer SHALL charge before the visited work proceeds and stop immediately
at the first charge exceeding the existing ceiling, returning the existing
`failed`/`work` record with that exact limit and consumed count and no node.
A successful comparison continues node, body-term and successor-edge charges
on this same meter, without resetting or replacing it, adding a second budget,
or dropping the request/comparison charges. Each sibling request retains its
own existing FR-038 meter and request order; compatibility is charged per
request, not as an unmetered global pre-pass.

When a completed check finds a required row absent, including when only a
different authority or identity occurs for its role, the lowerer SHALL return
the existing `invalid_input` disposition for that request with its exact
requested `node_id` and no additional payload, before unsupported-tag, bounds
or body checks. With sufficient work every present request receives that same
mismatch result. The result retains its available source occurrences; an absent
key retains an empty occurrence array. This path emits no semantic node or
backend artifact. The FR-038 package byte ceiling remains the existing global
exception: it fails every record if even the refusal package cannot be encoded.
A completed mismatch is not a reader `invalid_package` diagnostic and gains no
invented pointer/cause.

The lowerer SHALL retain the originating `package.source.package_id` and exact `source.lock`,
including every row and its order, without sorting, filtering, adding or
substituting selections. The resolved requirements are not serialized into
`package.lowering_profile` or another ContractPackage member. A strict reader
cannot re-check compatibility from the emitted reference alone; this is the
lowerer's producer obligation, verified against its typed operation input.

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
- `failed`: the requested `node_id`, exact Contract IR FR-038 `limit_kind`,
  caller's `limit`, and `consumed` count at the failed charge. In this ContractPackage
  JSON wire representation, both counters are canonical unsigned decimal
  strings (`0`, or an ASCII digit
  `1`–`9` followed by zero or more ASCII digits `0`–`9`, with no sign or leading
  zero); each parses exactly to the corresponding Contract IR FR-038 `u64`
  value. This preserves the full typed range in the RFC 8785 identity without
  IEEE-754 rounding.
- `invalid_body`: the requested `node_id`, offending `body_node_id`, and exact
  Contract IR FR-038 `CheckedPackageRefusal` carried by FR-322, with path
  semantics constrained by QSL FR-096.
- `body_incomplete`: the requested `node_id`, stopped `body_node_id`, and
  exact Contract IR FR-038 `CheckedPackageIncomplete` carried by FR-322. Its
  closed `limit_kind` vocabulary comes from Contract IR FR-038; QSL FR-096
  constrains the producer's measured-counter and path semantics.

The two defensive payloads have closed serialized shapes. The `invalid_body`
record carries `refusal`, a closed object with required string `code` and
optional members `path`, `cause`, `locus`, `contract_version`, and
`document_pointer`. Their field names and types are those of Contract IR
FR-038's `CheckedPackageRefusal`, carried by FR-322; QSL FR-096 constrains the
QSL producer's path semantics:
`code` uses the exact FR-322 code spelling;
`path` is its RFC 6901 pointer into the checked-package document; `cause` uses
the FR-272 spelling paired by FR-322; `locus` is its exact closed
`CheckedNodeId` object `{domain, digest}`; `contract_version` is the string
read for `unknown_contract_version`; and
`document_pointer` is its RFC 6901 pointer into a supplied domain-package
document. Each optional JSON member is present exactly when the corresponding
typed field is present and is omitted otherwise; a present pointer may be the
empty string, the RFC 6901 pointer to the document root. A pointer is never
replaced with `null` or a fabricated location. Before checking conditional
members, the reader validates the required discriminator fields:
`refusal.code` must be a known FR-322 code and
`incomplete.limit_kind` must be one of its six Contract IR FR-038 values. A
wrong type is
`wrong-value-kind`; an unknown string is `invalid-value`, each at the
discriminator's pointer. `path`, `cause`, `locus`, and `document_pointer` are
optional for every recognized refusal code; when present, each must have its
specified type. A `cause` string must be a known FR-272 cause and its
`(code, cause)` pair must be admitted by FR-322; an unknown cause or an
unpaired cause is `invalid-value` at the cause member's pointer. A cause of a
wrong JSON kind is `wrong-value-kind` at that pointer.
The reader does not infer whether these members were present in the originating
typed refusal; exact presence remains the producer's Contract IR FR-038
conformance obligation. A `null` for an allowed optional member is
`wrong-value-kind`. The
reader requires `contract_version` for
`unknown_contract_version` and forbids it for every other `code`; absence of
that required member uses `missing-member`, and presence for another code uses
`unknown-member`.

The `body_incomplete` record carries `incomplete`, a closed object with
required `limit_kind`, `limit`, and `consumed`, plus optional `path`, matching
the typed fields of Contract IR FR-038's `CheckedPackageIncomplete`; QSL
FR-096 constrains the producer's measured-counter and path semantics. The
typed FR-322 `ReadResult` has no serialized
counter wire form; this ContractPackage projection is the sole JSON encoding
of these nested values. It encodes the exact IR `u64` values as canonical
decimal strings so RFC 8785 identity does not round values above the exact
integer range of JSON numbers.
`limit_kind` uses exactly one of the lowercase values `bytes`, `nodes`, `edges`,
`occurrences`,
`diagnostics`, or `work`. `limit` and `consumed` are canonical unsigned
decimal strings with the same grammar as the `failed` record and parse exactly
to their typed Contract IR FR-038 `u64` values; `consumed` is the counter value
at the failed charge, as defined by Contract IR FR-038 and QSL FR-096. `path` is the
Contract IR FR-038 RFC 6901 pointer into the checked-package document.
Contract IR FR-038 requires it for every limit kind except `bytes`; the reader
requires it for non-byte kinds and forbids it for `bytes`,
using `missing-member` and `unknown-member`, respectively. The nested records
do not use FR-322's outer `ReadResult` envelope or add a second diagnostic
vocabulary. The reader validates each counter's spelling and `u64` range but
does not infer whether `consumed` could have resulted from a charge against
`limit`; that relationship is producer accounting truth owned by Contract IR
FR-038 and is checked against the originating typed outcome by the producer,
as TC-933 asserts.

For example, this illustrates the serialized shape of a defensive body-limit
result. Node-ID domains, digests, and pointer are illustrative; member names
and field types are normative.

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

The example shows a present non-root `path`. An absent typed path is omitted,
never encoded as `null`; the empty string remains valid when the typed path
identifies the document root.

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
    "lowering_profile": "<exact LoweringProfileRef>"
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
checked package's `quire.package.semantic/v2` identity. A mismatch between
this ContractPackage's declared digest and its recomputed projection is an
`invalid_package`/`invalid-value` at `/identity/digest`. FR-272's
`content-mismatch` cases compare the identity retained by a request or
selection with its supplied or recomputed subject identity, including a
declared `package_id` compared with one recomputed from compiled source. This
self-digest is recomputed by the reader from the wire projection, so a mismatch
is a malformed value rather than that compile-time content mismatch. It is not
FR-322's `stale_dependency`/`content-mismatch`, which applies to a supplied
selected dependency whose recomputed package identity differs from the lock
selection.

The ContractPackage reader is the single owner of outer wire admission. It
first applies only the generic wire checks reused from FR-322: strict JSON
parsing including duplicate-member rejection, then canonical-byte validation.
It then validates
FR-330's own exact `quire.contract-package/v1` version (it MUST NOT apply
FR-322's `quire.checked-package/v2` version predicate to this output), and
checks the exact top-level and nested closed shapes, typed node keys and
references, package provenance, the closed lowering-profile reference shape,
source-map totality, feature-accounting/request bijection, and recomputed
identity before exposing an admitted typed value to a consumer. The order
after generic wire checks is: outer version, all top-level and nested closed
shapes and typed values, package provenance and lowering-profile reference
shape, source-map totality, accounting/request consistency, then identity
recomputation, dependency inputs
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
cause at that member's ContractPackage pointer. A missing, duplicate, unknown,
wrong-kind, or out-of-domain member uses FR-322's `invalid_package` code with
its corresponding `missing-member`, `duplicate-member`, `unknown-member`,
`wrong-value-kind`, or `invalid-value` cause. The pointer for `missing-member`
and `unknown-member` is the containing closed object whose member set was
violated, including the ContractPackage root (the empty pointer `""`). The
pointer for `duplicate-member`, `wrong-value-kind`, and `invalid-value` is the
offending member itself. For example, an extra root `settings` member is
reported at `""`, while an extra `settings` member in
`package.lowering_profile` is reported at `/package/lowering_profile`. A
`null` value for an allowed optional member is `wrong-value-kind` at that
member's pointer. A payload member forbidden by its selected disposition is
`unknown-member` at the containing record's pointer. Accounting/request
bijection failures use `invalid_package`/`invalid-value` at `/feature_accounting`
(the array value). Source-map totality failures use
`invalid_package`/`invalid-value` at `/source_map` (the containing closed
object, whose three collections jointly establish totality). This difference
is intentional. A malformed or non-corresponding source-map member uses
`invalid_package`/`invalid-value` at the offending member. A source region whose
`start` is not less than `end` is reported at its `start` member. Other
malformed members follow the code/cause rules above.
FR-322-AC-35's "at the mutated entry" describes schema
validation; FR-330 defines the distinct typed reader-refusal pointer here.
Limits remain caller inputs to reading and
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
| FR-330-AC-1 | Every represented node has identical native meaning to its originating checked-package node under the source profiles selected by the exact FR-322 lock. | Test (TC-935) |
| FR-330-AC-2 | An unrepresentable item has no substitute node and retains exactly one typed terminal refusal with every available source locus (or its exact requested identity when no source key exists), while independent sibling records remain unchanged. | Test (TC-933, TC-934) |
| FR-330-AC-3 | When two packages have the same included semantic projection, their canonical identities are equal; when any included projection member differs, their identities differ. | Test (TC-936) |
| FR-330-AC-4 | A package has exactly one semantic node collection ordered by the FR-322 typed node key; each node occurs once, local references resolve to compatible nodes, dependency references retain FR-322 selection semantics, a `protocol`/`protocol_reference` carrier binds by its body target to a local `protocol` node under FR-322-AC-66, and recursion is represented only by permitted keyed references. | Test |
| FR-330-AC-5 | The embedded source `package_id` and lock exactly equal those of the originating admitted checked package, and the explicit lowering-profile reference is retained in its closed `{authority, identity}` form separately from the lock's source-profile selections. | Test (TC-932, TC-935) |
| FR-330-AC-6 | `feature_accounting` has one ordered record for each requested item and exactly one disposition from the seven-member vocabulary; non-`lowered` records have no substitute node, and `invalid_body.refusal` and `body_incomplete.incomplete` use the exact closed nested shapes and omission rules above. | Test (TC-933) |
| FR-330-AC-7 | `source_map` has an entry for every semantic node, requested item, and refusal; an absent `CheckedNodeId` remains an explicitly mapped `invalid_input` with no fabricated region, while every available occurrence and region matches FR-322 exactly. | Test (TC-934) |
| FR-330-AC-8 | The identity recomputes from the specified RFC 8785 projection under `quire.contract-ir.semantic/v1`; changing package provenance, lowering profile, graph, or an accounting result changes it, while changing only valid source locations, caller limits that leave serialized members unchanged, rendering, or JSON layout does not. Source-map corruption still refuses admission. | Test (TC-936) |
| FR-330-AC-9 | The sole reader admits a valid canonical value only with every lock-selected admitted dependency package supplied and matching its selection; it ignores extra unselected inputs, refuses a missing or wrong-identity dependency as `missing_import`/`missing-selection` and a same-identity admitted package with a different `package_id` as `stale_dependency`/`content-mismatch` at the selected lock pointer, all before dependent-reference checks. It refuses a mismatched own identity as `invalid_package`/`invalid-value` at `/identity/digest`; malformed nested payloads use `invalid_package` with FR-322's `missing-member`, `duplicate-member`, `unknown-member`, `wrong-value-kind`, or `invalid-value` cause as applicable. Wrong versions, invalid references, incomplete source maps, and accounting mismatches also refuse with a typed FR-330 reader result and FR-272 diagnostic mapping before any consumer sees a value. | Test (TC-937) |
| FR-330-AC-10 | The resolved profile carries authoritative typed `required_source_selections`; exact role/authority/identity matches admit lowering, empty requirements impose no restriction, and unrelated lock rows or reordered/repeated requirements do not change compatibility. Per-request compatibility uses the existing first-charge meter and stated row-visit charges without reset; zero-work and an unpayable comparison charge yield `failed` before compatibility refusal. With sufficient work a missing or different required row yields one request-ordered `invalid_input` per item before node traversal, with no extra payload, no nodes and exact available source occurrences; both success and refusal retain the exact originating lock and package id. | Test |

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
- [QSL FR-096](ix://agent-ix/quire-spec-language/FR-096) constrains the QSL
  producer's stage-limit counter and path semantics carried in defensive
  outcomes; it does not define a competing ContractPackage wire enum.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) owns exact
  identity-domain separation; this requirement selects its existing
  `quire.contract-ir.semantic/v1` domain for the ContractPackage projection.
