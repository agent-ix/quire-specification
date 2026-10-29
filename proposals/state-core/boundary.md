# Proposed shared model, identity and source boundary

Status: proposed. These contracts are fixture expectations for review, not a
registered wire schema. Do not implement a permissive production reader from
these examples or freeze another consumer's method/result contract around them.

## Authority and import contract

Local display names are aliases, never declaration identity. The language must
not infer type definitions from application Rust strings or runtime examples.

Resolve imports by identity alone, then by
exports. Do not choose the first duplicate or a
reachable but undeclared package. Reject
missing, ambiguous, foreign, ill-typed and unsupported definitions with
both reference and available declaration loci. Stable declaration IDs survive
display-name changes; their meaning and dependencies may change and
invalidate a checked package.

The domain package IR distinguishes required/optional presence from nullable
values, and expresses lower/upper multiplicity, ordered/unique flags, units,
relationships, operations and opaque clauses. Preserve each distinction or
refuse the mapping. An absent numeric maximum or sequence upper bound is not
a finite execution bound. No maximum may be inferred from machine width or
the observed sample. Exact domain limits are reviewed model/profile data.

## Independent identity dimensions

| Identity | Proposed representation and binding |
|---|---|
| Authored requirement/criterion | Package namespace, requirement ID, positive authored revision, clause ID; retained when a derived file changes |
| Declaration | Stable semantic URI and containing package; field and operation IDs distinct from display names |
| Source unit | Source ID, local path, byte region and coordinate convention |
| Language | Language tag and edition, independent from implementation version |
| Semantic profile | Explicit profile identifier/version and required feature set |
| Wire | Exact wire schema/version; an unknown version refuses |
| Canonicalization | Named domain and canonical byte algorithm; no cross-profile identity substitution |
| Linked package | Static source/profile/model dependency closure and declaration/obligation graph; excludes runtime populations |
| Executable projection | Existing bound-package identity plus explicit frontend/source/model derivation lineage; optional per-feature projection |
| Runtime object | Universe ID, stable type ID and object ID; immutable observations are separately identified |
| Snapshot/invocation | Population identity, current/pre/post role, universe, completeness and operation/invocation correspondence |
| Artifact/run | Producer version; distinct run ID and environment binding owned by the execution layer |

A source unit is identified by its source ID. A
requirement's positive authored revision maps explicitly to the IR requirement
revision; a missing mapping refuses the executable projection.

## Source reference proposal

Exactly one editable source per clause: inline bytes or a local
file region. An external path is not enough. No network fetching is implicit.
Preserve the complete source document and clause region. Source
spans are zero-based half-open UTF-8 byte offsets; lines and columns are one-based,
with columns counting Unicode scalar values. Reject split UTF-8 boundaries,
reversed/out-of-range offsets, source identity mismatch and coordinate mismatch.
CRLF counts as one line break but two original bytes. Tabs count as one scalar,
not a configurable display width. EOF zero-width spans are valid diagnostics.

Existing extraction locations cover the fence, while a native parser diagnoses
the body. The adapter must verify the extracted body against the original
document and construct an explicit body-to-document byte map. It
must retain extraction availability and diagnostics. A normalized or indented
body needs a segment map; arithmetic offset addition is valid only after exact
byte correspondence has been established. The extractor remains opaque to the
expression. Namespaced `ix:native` currently travels as an unchecked language;
that is not a parser/typechecker pass.

## Linked identity and strict reading

The future linked identity needs its own named canonical profile covering
edition, semantic profile, required features, resolved declarations/obligations
and exact static dependency closure. Source provenance and implementation
lineage remain explicit even when the source-free expression is unchanged.
Do not change existing `quire.contract.bound-identity/v1` or existing canonical
profile bytes to accommodate a new source language. A migration reports old
and new identities, mapping version and preserved/refused obligations; old
evidence stays addressed to its original subject.

Production readers must reject duplicate JSON keys before creating maps,
unknown versions, unknown required features, unexpected closed-record members
and malformed source maps. Optional extensions need a named
container and explicit nonsemantic treatment; they cannot alter meaning while
claiming compatibility. Canonical encoding and an independently implemented
strict consumer remain interchange acceptance work.

## Diagnostic boundary

Diagnostics carry phase, stable code, source reference, primary byte span,
related declaration loci, and the original obligation identity. Proposed codes:
`invalid_syntax`, `unsupported_construct`, `unknown_profile`, `unknown_wire`,
`unknown_required_feature`, `missing_import`, `ambiguous_declaration`,
`foreign_reference`, `ill_typed`, `unbounded_domain`,
`undefined_expression`, `invalid_source_map`, `duplicate_object`,
`dangling_reference`, `wrong_snapshot`, `frame_violation`,
`population_delta_mismatch`, `incomplete_population`,
`unavailable_observation`, `resource_exhausted` and `cancelled`.

Invalid syntax/input and unsupported capability refuse a requested operation;
missing observations and exhausted resources leave it incomplete. Neither
produces a logical counterexample. Healthy/violating values exist only after
input validation and completed evaluation. This is the language producer's
semantic boundary; shared execution/truth/activation/participation/adequacy
envelopes are supplied by the verification owner.

Operation fixtures bind an identified frame contract separately from the actual
pre/post populations. A caller cannot permit its own extra changes by replacing
the frame at run time. The selected frame identity is a derivation/input binding
for evidence freshness; changing it cannot reuse a previous result silently.

Validate known structural, identity and invocation errors before predicate
evaluation. If a required observation is unavailable, report incompleteness
without comparing unknown object deltas or fabricating frame violations. For
complete snapshots, verify reference closure and creation/deletion inventories,
then the selected frame and context availability. Evaluate the predicate only
after these checks succeed. If several known defects exist, retain deterministic
diagnostics ordered by phase, source locus and code; no defect yields a Boolean
result. The mutation cases isolate individual defects where practical.

## Local operation review artifacts

The amended operation fixture uses a local reference wrapper with `path` and
`artifactRef`. The artifact reference has exactly the common dimensions under
review with the verification owner: `authority`, `identity` and
`profile`. `profile` identifies the fixture encoding, independently of the
language semantic profile. The path locates the named file.

Each invocation binds one immutable pre/post observation per available role and
one exact frame contract. Populations and frame data live once in separate files;
expected outcomes live outside invocation inputs. Case-scoped invocation and
observation IDs remove accidental identity reuse across independent examples.
An authority/identity already bound to one profile cannot be rebound to another.
Changing a selected frame or population reference selects a different invocation
artifact, so downstream evidence must bind the new invocation artifact.

These are concrete `1-draft` review fixtures, not a registered FS05 schema or an
accepted generic artifact resolver. Model/operation binding is still unavailable.
The producer audit tests identity conflict refusal separately
from the manually expected predicate/frame/observation outcomes. Independent
consumer qualification remains C's work; the portable result envelope remains B's.
