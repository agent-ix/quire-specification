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
including structured revision and the referenced artifact's wire identity and
version. Semantic values require artifact, language, semanticProfile,
requiredFeatures and canonicalIdentity. Explicit null is permitted only where
specified; a missing member never means null, empty, current or latest.

This resolves an ambiguity in the earlier FS02 draft: **requiredFeatures must
be present, with [] for the empty set**. It is a duplicate-free set of exact
Unicode scalar strings. Producers order it by UTF-8 bytes; a reader accepts
other orders and compares the decoded set. Duplicate spellings, including
escaped spellings that decode to the same string, refuse before comparison.
No case folding, normalization or global array sorting is permitted.

A profile-bearing semantic subject needs a non-null profile definition digest.
A null language permits a subject whose semantics belong to another explicitly
selected contract, not an inferred native language. A byte-only reference can
use the artifact envelope directly; null profile is not permission to claim
semantic equivalence or evaluate the artifact.

Structural validation does not establish authority, wire support, required
features, dependency closure, canonical identity, or source correspondence.
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
   features against the consumer's declared support. An unknown canonical domain
   or unqualified derivation refuses canonical interpretation. No network lookup,
   fall-through decoder, string-to-number revision cast or profile default occurs.
4. Resolve locally supplied artifacts and compare their exact byte digests.
   Check immutable identity/revision bindings, model manifest/lock/closure, and
   any explicit numeric source revision mapping independently. A retrieval path
   never confers authority or joins a different artifact kind.
5. Validate source regions/maps and canonical derivations, then compare the
   intended reference role using its local collection semantics. Only a fully
   checked match may support the next pipeline stage. A match alone is not a
   logical result or adequate verification evidence.

Keep distinct diagnostics for unsupported-version, unknown-required-feature,
invalid-wire, identity-mismatch, identity-content-conflict, stale,
malformed-source-map, unsupported-canonicalization and ambiguous-equivalence.
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
discontiguous original ranges, and reject foreign identity/revision/path/bytes.
Zero-width diagnostics select the following segment at a boundary and the last
mapped end at EOF; an empty map selects the original region start. Coordinates
derive from original bytes under the established scalar-column convention.

The adapter must additionally prove that the selected original region belongs
to the authored clause/fence and bind the approved extraction policy. A byte
map proves correspondence, not fence ownership. Quire extraction remains opaque;
C owns its existing-repository decoder/adapter and independent regressions.
Schema success alone does not prove any cross-span or source-byte condition.

## Preserve each canonical identity domain

| Selected domain | Established behavior to retain |
|---|---|
| Exact artifact bytes | `sha256:` plus lowercase SHA-256 of the complete supplied bytes, including whitespace and final newline. No semantic canonicalization. |
| `quire.contract.canonical-json/v1` | Existing IR source-free projections and Unicode-scalar object-key ordering; semantic arrays retain their specified order. Minimal exact integers; no floats/null in this canonical model. Digest is SHA-256 of `quire-contract-ir`, NUL, profile, NUL, object kind, NUL, then canonical envelope bytes. |
| `quire.contract.bound-identity/v1` | Existing binder's complete sorted clause bindings, package canonical digest, declaration/expression digests, and profile headers. Digest is raw SHA-256 of that canonical bound-identity envelope; do not add the other domain's prefix. |
| B's provisional `sha256-jcs:` domain | Preserve B's selected canonicalized structured value and prefix. Do not run arbitrary IR/source bytes through it and claim their existing identity. Its adoption into a shared semantic reference remains B-reviewed. |

The canonicalIdentity fields select domain, version, algorithm and the exact
domain-produced digest. For the two IR domains above the candidate selects
domain `quire.contract.canonical-json` or `quire.contract.bound-identity`,
version `v1`, algorithm `sha256`, and the existing bare lowercase hex digest.
This split names the established full profile; it does not rename it. Other
domains require an explicit reviewed mapping of their digest syntax and inputs.

The consumer must also possess the canonicalizer's qualified input/derivation,
including object kind where the domain requires it. A canonicalIdentity alone
does not prove a source artifact corresponds to a typed expression. The current
native parser therefore continues to emit no canonical semantic identity. Do
not hash an unlinked syntax tree under the existing typed-expression domain.

JCS orders object keys by UTF-16 code units and uses IEEE-754/ECMAScript number
serialization. The existing IR instead preserves exact integer values and uses
Unicode-scalar key order. These can differ for supplementary characters and
large integers; shared spelling of "JSON" or "SHA-256" does not imply equality.
See [RFC 8785 sections 3.1 and 3.2](https://www.rfc-editor.org/rfc/rfc8785.html#section-3).
No RFC code or example corpus is copied into this packet.

Historical canonical vectors stay byte-identical. Corpus `.sha256` sidecars are
file-integrity digests unless that corpus explicitly says otherwise; they must
not be relabeled as domain-separated semantic digests. Existing IR schema 1.0
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
environment, model-manifest, model-lock or dependency-closure.

An accepted mapping between common kinds may establish the same underlying byte
artifact only when kind, authority, identity, structured revision, digest and
wire all remain equal. Changed reference encoding yields a new wrapper artifact;
it does not rewrite the stored old record or adopt an authored identity. The
mapping must be selected by exact identity/revision/digest and explicitly
accepted for the requested equivalence. Without it, reject the version join.

This mapping preserves byte-artifact correspondence only. It does not preserve
logical obligations, generated lineage, source activation or evidence adequacy
by implication. Semantic/profile migrations need separately reviewed per-
obligation mappings, preserved/refused claims and derivations. Missing,
conflicting or multiply applicable mappings produce refusal or ambiguity;
transitive composition and reverse mappings are not inferred automatically.

## Qualification and owners

The structural schema plus raw JSON refusal fixtures exercise selected record
shapes; canonical vectors preserve existing domain bytes/digests; source-map
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
