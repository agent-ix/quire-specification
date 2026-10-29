# FS05: strict references, canonical domains and migration

Status: accepted by FS05 as `agent-a-fs05/1-draft`. This selects the standard's
strict reference, canonical-domain and explicit-migration behavior. It does not
register public interchange, qualify every consumer, or rewrite earlier review
wrappers; those wrappers retain their original bytes and meanings.
`schemas/shared-reference-1-draft.schema.json` is the single structural schema
for this candidate. No method, plan, result or typed-model payload is defined here.

## Exact wire selection and closed records

The envelope has exactly `wireVersion`, `kind`, and `value`. Its version is
`ix.shared-reference/1-draft`; kind is artifact, semantic, source-map or migration.
The selected kind determines the value schema. Schema identifiers are local
contract selectors, not instructions to fetch a remote schema or executable.

Artifact values use `ix.artifact-ref/2-draft` from FS02. Every member is required,
including the referenced artifact's wire identity and
version. Semantic values require artifact, language, semanticProfile and
requiredFeatures. Explicit null is permitted only where
specified; a missing member never means null, empty, current or latest.

This resolves an ambiguity in the earlier FS02 draft: **requiredFeatures must
be present, with [] for the empty set**. It is a duplicate-free set of exact
Unicode scalar strings. Producers order it by UTF-8 bytes; a reader accepts
other orders and compares the decoded set. Duplicate spellings, including
escaped spellings that decode to the same string, refuse before comparison.
No case folding, normalization or global array sorting is permitted.

A profile-bearing semantic subject needs a non-null profile identity and version.
A null language permits a subject whose semantics belong to another explicitly
selected contract, not an inferred native language. A byte-only reference can
use the artifact envelope directly; null profile is not permission to claim
semantic equivalence or evaluate the artifact.

Structural validation does not establish authority, wire support, required
features, dependency closure or source correspondence.
Those are separate checked obligations with explicitly selected registries and
bindings. A structurally valid unfamiliar wire still refuses interpretation.

## Reader sequence and failure categories

1. Bound input work before parsing. Require UTF-8, one complete JSON document,
   valid scalar strings, and no BOM, duplicate decoded member names, trailing
   document, non-JSON numbers or unpaired surrogate escapes. Check duplicates
   before creating a host map. Do not lose integers through floating-point
   conversion. Exhaustion is incomplete and returns no partially accepted record.
2. Read the exact envelope version and kind without guessing another decoder.
   Unknown versions/kinds refuse. Validate the selected closed schema; unexpected
   members, omitted required members and wrong primitive kinds refuse.
3. Validate selected authority, wire/language/profile versions and all required
   features against the consumer's declared support. No network lookup,
   fall-through decoder, string-to-number cast or profile default occurs.
4. Resolve locally supplied artifacts. Check immutable identity bindings
   independently. A retrieval path
   never confers authority or joins a different artifact kind.
5. Validate source regions/maps and canonical derivations, then compare the
   intended reference role using its local collection semantics. Only a fully
   checked match may support the next pipeline stage. A match alone is not a
   logical result or adequate verification evidence.

Keep distinct diagnostics for unsupported-version, unknown-required-feature,
invalid-wire, identity-mismatch,
malformed-source-map and ambiguous-equivalence.
Source-map budgets and cancellation remain incomplete rather than malformed
input. Missing runtime observations are incomplete; missing static authority
refuses linkage. B maps these producer distinctions into its portable envelope.

## Source-map value

The map value declares `mapVersion: ix.source-map/1-draft`, exact original/body
source ArtifactRefs, an original byte region, explicit layout flags, and ordered
segments. A span has integer startByte/endByte, zero based and half open. The
wire permits nonnegative integers up to 2^53-1 so these offsets remain exact in
the existing JavaScript consumers; the native implementation's lower source
and segment ceilings still apply as resource limits. This bound is not a domain
integer bound and does not change existing contract-IR u64 source offsets.

Each segment binds equal original/body byte slices with UTF-8-aligned boundaries.
Segments cover every body byte exactly once, in order, without original overlap,
and remain inside the selected original region. Positive-length segments are
required; an empty body uses no segments. The only optional deletions are actual
leading indentation, CR from a CRLF, and one final newline, each explicitly
enabled by its layout flag. No source token insertion, rewriting or interior
whitespace deletion is inferred from a matching expression meaning.

The existing native SourceMap verifier defines the candidate producer behavior:
verify exact source bytes and all segments before exposing a map, preserve
discontiguous original ranges, and reject foreign identity/path/bytes.
Zero-width diagnostics select the following segment at a boundary and the last
mapped end at EOF; an empty map selects the original region start. Coordinates
derive from original bytes under the established scalar-column convention.

The adapter must additionally prove that the selected original region belongs
to the authored clause/fence and bind the approved extraction policy. A byte
map proves correspondence, not fence ownership. Quire extraction remains opaque;
C owns its existing-repository decoder/adapter and independent regressions.
Schema success alone does not prove any cross-span or source-byte condition.

Existing IR schema 1.0
to 1.1 migration intentionally changes package canonical bytes because the
schema version participates in them. It preserves only its documented content.

## Collection rules and reuse

Required features are a duplicate-free set. Static dependency/binding sets use
their owner's declared identity key and refuse conflicting duplicate identities.
Existing IR parameter, operand, call-argument and collection order remains
significant. Quoin trace_ids preserve order and bytes; legacy omitted trace IDs
remain readable but unbound. Source-map segments retain body order. Diagnostic
sorting does not change which defects occurred. No single deep-equality or
canonicalization helper supplies all these meanings.

Same clause bytes under a different language/profile definition, feature set,
model/closure, binding/frame, snapshot/invocation, producer/adapter, environment,
plan/task/run or required technique do not support automatic evidence reuse.
B owns exact result selection and deterministic ambiguity handling. Equal
timestamps or input order cannot supply a missing unique selection rule.

## Explicit migration

The candidate migration envelope records an exact old ArtifactRef, new
ArtifactRef, mapping ArtifactRef, preserved aspects and a reason. Old artifact
references retain their selected version; only the explicitly named 1-draft and
2-draft forms are admitted here. The earlier closed kind set cannot represent
environment, model-manifest or dependency-closure.

An accepted mapping between common kinds may establish the same underlying byte
artifact only when kind, authority, identity and
wire all remain equal. Changed reference encoding yields a new wrapper artifact;
it does not rewrite the stored old record or adopt an authored identity. The
mapping must be selected by exact identity and explicitly
accepted for the requested equivalence. Without it, reject the version join.

This mapping preserves byte-artifact correspondence only. It does not preserve
logical obligations, generated lineage, source activation or evidence adequacy
by implication. Semantic/profile migrations need separately reviewed per-
obligation mappings, preserved/refused claims and derivations. Missing,
conflicting or multiply applicable mappings produce refusal or ambiguity;
transitive composition and reverse mappings are not inferred automatically.

## Qualification and owners

The structural schema plus raw JSON refusal fixtures exercise selected record
shapes; source-map
cases specify additional semantic checks. Producer checks must report which
layer ran. A schema validator is not an independent consumer or evaluator.
Independent consumer evidence must cover the applicable refusal conditions
before a consumer claims conformance. FS05 acceptance does not convert unrun
consumer-specific matrices into executed evidence.

A owns this standard/native producer contract, C existing-repository consumers,
and B method/plan/result selection. Existing Quire formatVersion 1, contract-IR
projection/canonical/bound versions, Quoin conformance JSONL v1, and TL's distinct
version paths remain under their owners. This acceptance does not renumber them,
change a canonical algorithm, close consumer-owned work, or authorize public
promotion.
