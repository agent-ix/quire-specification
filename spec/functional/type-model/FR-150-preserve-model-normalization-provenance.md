---
id: FR-150
title: "Preserve original and effective model declarations"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
  - target: ix://agent-ix/quire-specification/FR-035
    type: references
  - target: ix://agent-ix/quire-specification/FR-321
    type: references
---

# FR-150: Preserve original and effective model declarations

## Description

When normalizing the declarations of an admitted domain package, the model
checker SHALL retain each original declaration key, effective declaration
identity and ordered derivation provenance.

## Inputs

The original declarations that model intake (FR-154) builds from one domain
package, its ModelSelection (FR-321) and the selected
`quire.model.complete/v2` definition with its rule manifest
`quire.model.complete.rules/v2`.

## Outputs

An immutable retained-original inventory and a structural effective view,
carried as one `quire.model.effective-view/v2` identity, or a typed refusal
or incomplete result.
The structural view is checking input, not a validated checked package.

## Behavior

Each original declaration is keyed by `(domain package identity, IR node
identity, sha256-jcs)`. Supertypes, subsets and redefines are members of the
IR nodes, so every edge normalization reads is part of the admitted package.
QSL derives effective declarations. It never mints, renames or merges a
declaration key, and never derives an edge the package does not declare.
Normalization never replaces the original key or actual FCD origin. Every derived
member records its source declarations and rule. Equal normalized shapes from
distinct original declarations always remain distinct effective declarations;
no identity-equivalence declaration exists.

The model checker SHALL retain every phase-1 original declaration and its
actual FCD origin in the immutable selection inventory, including originals
whose kinds have no phase-2 qualify rule. It SHALL derive effective entries
only for the selected rules' kinds. A retained scalar, clause or population
without such a rule SHALL NOT receive an invented qualify fact, effective
declaration or additional declaration/hash charge. Effective entries and fact
inputs reference keys from the exact retained inventory. This inventory is
existing selection data, not an added effective-identity preimage member.

The effective member key is `(owner effective type identity, original
declaration key)`. Paths that reach the same original declaration collapse to
one effective member retaining a derivation fact for every path. A source name
that resolves to distinct original declarations is refused
`ambiguous_declaration`/`ambiguous-name` at source resolution and, for a
model-owned `field` or `operation` member of a checked package, when a V2
reader resolves that member's name (FR-322 "Model-owned members" step 3);
the effective view keeps both members.

Each effective declaration identity is `quire.model.effective-declaration/v2`
over `{version, owner_effective_type, original, derivation}`; the effective view
is `quire.model.effective-view/v2` over the ModelSelection, rule manifest and
every effective declaration sorted by identity; and the object universe of a
connected supertype component is `quire.model.object-universe/v2`. The exact
preimages, orders and vectors are defined by the selected
`quire.model.complete/v2` definition and its closed ordered commitment grammar.
The coordinated schema/vector targets are
[`model-effective-declaration.schema.json`](../../../proposals/checked-package-v2/model-effective-declaration.schema.json)
and
[`model-effective-declaration-vectors.json`](../../../proposals/checked-package-v2/model-effective-declaration-vectors.json).
These domains differ from `sha256-jcs` and from the I04 node domain, so no
derived identity equals a domain package digest or an I04 node key. The
effective view is carried in the checked package as a `correspondence` node
with semantic form `model_correspondence`.

The checker SHALL retain the exact expanded semantic meaning of every fact,
including hidden declarations, all input keys, rule identities, ordinals and
distinct inheritance paths. It SHALL encode each declaration using the selected
definition's unique literal/prefix normal form and canonical overlapping range
commitments. It SHALL eagerly compute every effective identity. It SHALL
compute the complete digest-ordered declaration-sequence commitment and view
identity, not a requested-entry subset. Key/block/join helper nodes SHALL NOT
create extra effective declarations or derivation facts.

The complete retained provenance transport SHALL include every declaration's
compressed preimage and its reachable exact key/block records, with original
inventory and actual FCD origins still available. Its strict reader SHALL
verify admitted selection, semantic normalization, canonical expansion and
overlap, every eager identity and complete inventory before admission. A
root-only view or a proof for selected members SHALL NOT replace that check.
Structural checked-node correspondence and model effective-declaration
identities remain different domains; this change does not redefine I04 NodeKeys.

This is a pre-release replacement draft. The old closed schema/vector bytes
do not satisfy it. QSL SHALL atomically adopt the matching selected closure,
rule manifest, accounting, domain-package edge protocol, strict reader and
caller-minted identity domains as its implementation cutover. No mixed-cohort
default, compatibility reader, digest relabelling or old-digest transport is
admitted. Optional operation-edge document schema support and released
selected-document reader retention are delivered intake capabilities, not
complete replacement adoption. Before code cutover the actual producer's
operation edges and origins SHALL survive selected-document admission,
normalization, full provenance/correspondence re-export and independent
reconstruction under the same selected reader contract. The owning stages
retain kind/proper-ancestor, cycle/conflict and signature/contract checks,
and absent, changed, malformed, dangling and ambiguity controls. An older
locked reader is not adoption of the released intake. Structural checked-node
correspondence cannot discharge this full-view round-trip. Bounded sole-writer
full-provenance export, matched wire/schema/vectors/domains and separately
admitted reader costs remain prerequisites, not successful qualification.

The system owner of that atomic adoption is QSL as the consumer/implementation
of this contract. The selected closure, transport, sole writer, strict reader
and accounting must cut over together in QSL; this assigns ownership to the
system, not to an individual. QSpec specifies the contract and does not claim
that QSL has adopted or qualified it.

## Normalization derivation

The model checker applies the manifest's rules in this fixed phase order:
(1) `quire.model.normalize.intake/v1` and `quire.model.features/v1`, domain
package admission, declaration building and feature checks (FR-154);
(2) `quire.model.normalize.qualify/v1`, qualified-identity formation for
object types, interfaces, fields, relationships, operations, parts, ports
(owned by their part's owning type), connections and allocations (owned by each
end's end-owner type);
(3) `quire.model.normalize.inherit/v1`, inheritance expansion along declared
supertypes; (4) `quire.model.normalize.subset/v1` and
`quire.model.normalize.redefine/v1`, declared subsetting and redefinition; and
(5) `quire.model.normalize.canonicalize/v2`, identity computation. Each derived
fact records its declaration-key inputs, rule identity and
ordinal. A later phase may reference but cannot erase an earlier derivation. No
rule declares an override relation, and no semantic precedence exists between
rules. Before normal form, facts compare by phase, then lexicographic inputs,
then exact rule-identity UTF-8 bytes, retaining identical-fact multiplicity.
This serialization tie-break never resolves a conflict or changes a rule.

For the existing canonical-expansion acceptance, a field `s` of subtype `D`
with both `subsets=[t]` and `redefines=t` on proper ancestor `B`'s field `t`
retains both `[s,t]` facts, redefine before subset at consecutive phase-4
ordinals. Input/storage permutations produce the same expansion and digest;
swapped rules, a dropped occurrence, altered multiplicity or refusal solely
for this valid overlap cannot satisfy that acceptance.

Two phase 4 redefinitions of the same member reaching one effective type
conflict unless one redefining owner is a proper descendant of every other;
subsetting never conflicts with redefinition because it derives no replacement.
A conflict is refused `invalid_model_binding`/`derivation-conflict` with both
derivation paths and exposes no effective member for either. A redefinition
whose target is not an inherited member is refused
`invalid_model_binding`/`redefinition-target`. A supertype cycle is refused
`invalid_model_binding`/`specialization-cycle`. A ModelSelection whose digest
domain is not `sha256-jcs` is refused `stale_dependency`/`digest-domain-mismatch`,
and a supplied document whose recomputed canonical digest differs from the
selected digest is refused `stale_dependency`/`content-mismatch`. A model `variant` type referenced by
source is refused `unsupported_construct`/`declaration-form`. Canonical ordering
affects identity bytes only; it never changes a declaration key or resolves
semantic ambiguity.

Normalization and later conformance share the one operation-owned meter under
`ModelNormalizationLimitsV1` of the selected
`quire.value.accounting/v2` definition at its named record, fact, cycle-check,
redefinition-check, conflict-check, declaration, metadata, normal-form,
identity-work and preimage-hash charge points, the last by exact analytically
priced sole-writer JCS byte length before encoding or hashing. Phase-4 conflict
ordering uses original owner/target keys before identities exist; the effective
member key's meaning does not change. No preparatory hash occurs in phases 1–4.
Shared reuse is selection-local under the operation meter and follows the
exact closed-template Read/Require/Price/Hash and prerequisite ordering/index
schedule; lookup/pricing/validation work is not free. A phase-3 inheritance
ladder SHALL be admitted as a canonical prefix run: its `derivation_facts`
charge SHALL count every expanded logical fact and preserve its exact ordinal,
but shared path indexing, cycle classification, range overlap and binding
proofs SHALL charge each distinct input edge once and each reuse lookup a
constant one work unit, independent of the expanded path length. For a
single-inheritance chain with E edges, phase-3 fact runs, cycle checks,
normal-form segment/link work and range/binding proofs SHALL total no more
than `12 × (E + 1)` work units. The exact closed identity preimage bytes,
canonical writer output, digest domains and published identity vectors SHALL
remain unchanged by this accounting rule; every effective identity and the
complete view remain eagerly computed.

Metadata reuse is by selection-local admitted field occurrence, not equal
text or physical interning. Equal package text at different original ranks is
scanned/charged separately; repeat access to the same rank/field keeps its
lookup charge without rescanning. Every owner/root/universe/dictionary sort
uses the prescribed input projection, stable comparison, logical move and
index admission events. Pricing's retained-slot reads are covered by its
v-priced traversal, not optional extra field charges. For exact-bound/denial
acceptance, independent layouts and search/sort choices SHALL produce the same
complete event sequence and first unavailable charge, including hits,
equal-input facts and ordering prerequisites; zero/one-less injections
suppress the named and dependent work. Denial performs none of its named
work, encodes/hashes no denied node and runs no dependent work. No second
encoder, speculative pricing API, posthoc byte reservation or
expanded-preimage fallback is permitted. Each normalization phase checks
exhaustively and reports every refusal it exposes in charge order; a phase that
reports a refusal ends checking. Exhaustion ends checking as
`resource_exhausted`/`insufficient-next-charge` with no effective view.

## Acceptance Criteria

TC-195 remains the existing verifying procedure through its `verifies`
relationship; method-only cells below do not erase that relationship or any
criterion trace. Its superseded selection/vector bytes are not replacement
qualification evidence. The closed replacement and adverse cases above require
the matched integration cohort and independent qualification.

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-150-AC-1 | An inherited/redefined member links its normalized identity to every contributing original declaration and rule. | Test |
| FR-150-AC-2 | Two equal effective shapes from distinct original identities remain distinguishable. | Test |
| FR-150-AC-3 | A wrong digest domain, a stale package digest or a conflicting phase 4 derivation refuses normalization with its named cause. | Test |
| FR-150-AC-4 | Replaying the ordered derivation from the same domain package reproduces the `quire.model.effective-view/v2` identity, independent of IR node order. | Test |
| FR-150-AC-5 | Conflicting derivations report both rule paths and expose no chosen effective member. | Test |
| FR-150-AC-6 | Diamond paths reaching one original declaration collapse to one effective member retaining every path, and each effective identity equals its published vector. | Test |
| FR-150-AC-7 | Declaration keys are fixed by the domain package identity and IR node identity: two versions of one package with equal nodes yield equal effective declaration identities and different effective views. | Test |
| FR-150-AC-8 | A normalization run at its exact `ModelNormalizationLimitsV1` bound completes, and the one-less run is incomplete at the named charge point. | Test |
| FR-150-AC-9 | An admitted scalar without a phase-2 qualify rule remains retrievable by its byte-identical original key and origin, with no scalar qualify fact, effective entry or extra effective-declaration/hash charge; all effective/fact-origin keys belong to the retained intake inventory. | Test |
| FR-150-AC-10 | Transfer from structural normalization to checked-fact admission and conformance preserves the same model counters; a denied next charge follows no work and exposes no validated effective view or unfinished-stage failure, including failures already collected from another selected model. | Test |
| FR-150-AC-11 | A walk following n supertype edges admits at ancestor_steps=n; at n-1 it returns exactly incomplete(ancestor_steps,n-1,n-1,1,model.ancestor-edge), and zero denies its first edge. An unavailable owning work charge wins before a simultaneous edge denial; previously admitted axis work is never refunded. | Test |
| FR-150-AC-12 | Literal/prefix expansion reproduces every original ordered fact, input key, rule and zero-based ordinal, including diamond paths and hidden redefinitions; a changed tail, missing prefix, incorrect overlap/count, noncanonical split or alternate literal/run factoring cannot be admitted as the canonical view. | Test |
| FR-150-AC-13 | Every effective identity and the complete view/universe identities are computed eagerly; a transport missing any effective preimage or reachable key/block record, or supplying only a root/requested membership proofs, cannot be admitted as a complete view. | Test |
| FR-150-AC-14 | Each admitted template's analytical price equals the sole canonical writer's returned length, including escaped admitted keys; exact-bound/one-less/zero denial at metadata, normal-form, identity-work or hash performs none of that event's named work and no dependent work, with zero node encoding/hashing on hash denial. Shared hits do not rehash a node but retain their prescribed lookup charges. | Test |
| FR-150-AC-15 | The replacement selected closure, schema/vectors, edge protocol, strict reader, accounting and identity domains agree on one contract; an old or mixed selection and an old digest relabelled into the replacement domain are not admitted through a compatibility path. | Inspection |
| FR-150-AC-16 | For an admitted single-inheritance chain of E edges, all E+1 effective type identities and the complete view are computed with every logical inheritance fact retained; shared-prefix-specific charges are at most `12 × (E + 1)`, exact v2 preimage bytes and published vectors are unchanged, and the 10,000-edge/512-KiB case completes under `hashed_bytes ≤ 268435456`, `work_units ≤ 16777216`, `derivation_facts = 50015001`, and peak resident memory ≤ 21474836480 bytes. | Test |
| FR-150-AC-17 | When the deterministic FR-150-AC-16 case runs in the release profile on reference host Terra (Intel Core i9-10900K, 14 allocated threads, WSL2, 94 GiB), monotonic wall time from the start of package input admission through completion and retention of the complete normalized result is at most 300 seconds. Fixture generation and compilation are outside the timed interval. The run records the actual `rustc --version` and `cargo --version`; this criterion makes no qualification claim without measured run evidence. | Test |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
