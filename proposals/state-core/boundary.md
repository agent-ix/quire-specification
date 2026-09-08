# Proposed shared model, identity and source boundary

Status: proposed. These contracts are fixture expectations for review, not a
registered wire schema. Do not implement a permissive production reader from
these examples or freeze another consumer's method/result contract around them.

## Authority and import contract

Domain producers own declarations and export stable type/field/operation IDs.
The existing semantic IR `contractVersion: 1.1.0` is the proposed input boundary;
the language typechecks against its resolved graph. A standalone producer may
supply that contract without embedding an authoring application. Local display
names are aliases, never declaration identity. The language must not infer type
definitions from application Rust strings, Markdown prose or runtime examples.

Resolve imports by package identity, exact version, exact bytes digest, exports
and transitive lock closure. Do not choose the first duplicate, newest version,
or a reachable but undeclared package. Reject missing, ambiguous, stale,
foreign, ill-typed and unsupported definitions with both reference and available
declaration loci. Stable declaration IDs survive display-name changes; their
meaning and dependency digest may change and invalidate a compiled package.

The inspected input IR distinguishes required/optional presence from nullable
values, and expresses lower/upper multiplicity, ordered/unique flags, units,
relationships, operations and opaque clauses. Preserve each distinction or
refuse the mapping. An absent numeric maximum or sequence upper bound is not
a finite execution bound. No maximum may be inferred from machine width or
the observed sample. Exact domain limits are reviewed model/profile data.

## Independent identity dimensions

| Identity | Proposed representation and binding |
|---|---|
| Authored requirement/criterion | Package namespace, requirement ID, positive authored revision, clause ID; retained when a derived file changes |
| Declaration | Stable semantic URI and containing package lock entry; field and operation IDs distinct from display names |
| Source unit | Source ID, revision namespace/value, exact UTF-8 digest, local path, byte region and coordinate convention |
| Language | Language tag and edition, independent from implementation version |
| Semantic profile | Explicit profile identifier/version and required feature set |
| Wire | Exact wire schema/version; an unknown version refuses |
| Canonicalization | Named domain and canonical byte algorithm; no cross-profile digest substitution |
| Linked package | Static source/profile/model dependency closure and declaration/obligation graph; excludes runtime populations |
| Executable projection | Existing bound-package identity plus explicit frontend/source/model derivation lineage; optional per-feature projection |
| Runtime object | Universe ID, stable type ID and object ID; immutable observations are separately identified |
| Snapshot/invocation | Population digest, current/pre/post role, universe, completeness and operation/invocation correspondence |
| Artifact/run | Actual byte digest and producer version; distinct run ID and environment binding owned by the execution layer |

A Git commit is not the positive numeric `SourceRevision` used by the existing
contract IR. Preserve an explicit mapping from source namespace/value to the
IR source-document/revision pair. Do not hash a Git string down to a revision
integer or silently assign revision 1 to every source. The same applies to
requirement revisions. Missing mappings refuse the executable projection.

## Source reference proposal

Exactly one editable source per clause: inline bytes or a digest-bound local
file region. An external path is not enough. No network fetching is implicit.
Preserve the complete source document digest and clause-region digest. Source
spans are zero-based half-open UTF-8 byte offsets; lines and columns are one-based,
with columns counting Unicode scalar values. Reject split UTF-8 boundaries,
reversed/out-of-range offsets, source identity mismatch and coordinate mismatch.
CRLF counts as one line break but two original bytes. Tabs count as one scalar,
not a configurable display width. EOF zero-width spans are valid diagnostics.

Existing extraction locations cover the fence, while a native parser diagnoses
the body. The adapter must verify the extracted body against the original
digest-bound document and construct an explicit body-to-document byte map. It
must retain extraction availability and diagnostics. A normalized or indented
body needs a segment map; arithmetic offset addition is valid only after exact
byte correspondence has been established. The extractor remains opaque to the
expression. Namespaced `ix:native` currently travels as an unchecked language;
that is not a parser/typechecker pass.

## Proposed digest rules and strict reading

The proposed fixture byte digest is `sha256:` plus lowercase SHA-256 of the
exact named file bytes. It is a transport integrity binding, not a semantic
canonical digest. No dummy digest represents verified content.

The future linked identity needs its own named canonical profile covering
edition, semantic profile, required features, resolved declarations/obligations
and exact static dependency closure. Source provenance and implementation
lineage remain explicit even when a source-free expression digest is unchanged.
Do not change existing `quire.contract.bound-identity/v1` or existing canonical
profile bytes to accommodate a new source language. A migration reports old
and new identities, mapping version and preserved/refused obligations; old
evidence stays addressed to its original subject.

Production readers must reject duplicate JSON keys before creating maps,
unknown versions, unknown required features, unexpected closed-record members,
digest mismatch and malformed source maps. Optional extensions need a named
container and explicit nonsemantic treatment; they cannot alter meaning while
claiming compatibility. Canonical encoding and an independently implemented
strict consumer remain interchange acceptance work; raw-file digest checks do not
complete them.

## Diagnostic boundary

Diagnostics carry phase, stable code, source reference, primary byte span,
related declaration loci, and the original obligation identity. Proposed codes:
`invalid_syntax`, `unsupported_construct`, `unknown_profile`, `unknown_wire`,
`unknown_required_feature`, `missing_import`, `ambiguous_declaration`,
`stale_dependency`, `foreign_reference`, `ill_typed`, `unbounded_domain`,
`undefined_expression`, `invalid_source_map`, `duplicate_object`,
`dangling_reference`, `wrong_snapshot`, `incomplete_population`,
`unavailable_observation`, `resource_exhausted` and `cancelled`.

Invalid syntax/input and unsupported capability refuse a requested operation;
missing observations and exhausted resources leave it incomplete. Neither
produces a logical counterexample. Healthy/violating values exist only after
input validation and completed evaluation. This is the language producer's
semantic boundary; shared execution/truth/activation/participation/adequacy
envelopes are supplied by the verification owner.
