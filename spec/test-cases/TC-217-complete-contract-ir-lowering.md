---
id: TC-217
title: "Complete contract IR lowering"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-195
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-217: Complete contract IR lowering

## Description

Read the I04 `quire.checked-package/v2` positive, boundary, refusal, mutation
and canonical-byte/digest vectors through the real Contract IR V2 public reader;
then lower supported complete-V1 nodes and mutate source/type/anchor/bounds plus
unsupported nodes. Exercise the frozen V1 contract only through its unchanged
compatibility reader and the explicit V1-to-V2 re-link migration.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-195 and FR-322.
First validate the unchanged `proposals/checked-package-v1` schema and positive
fixture, then the `proposals/checked-package-v2` schema and positive fixtures.
Require each schema to reject the other version and require V1 to reject a
V2-only `nominal_identity_preimage` member; no test may reinterpret one closed
wire as the other.
Pin the exact bytes of every published V1 schema/vector file independently of
the current tree. Relabel a V1-shaped graph consistently as V2, recompute its
V2 package identity while leaving a nominal V2 node without its required
preimage, and require V2 refusal rather than inferred evidence.
Then exercise unknown-version, missing/duplicate/unknown member, stale raw
source/definition/model digest, cross-domain digest, unknown required
capability, non-canonical-byte and each selected resource boundary through the
real local Rust reader. Validate every object in
the V2 `node-identity-vectors.json` against
`node-identity-preimage.schema.json`,
recompute its RFC 8785 JCS SHA-256 and require the recorded node ID. Mutate each
enum/member/dimension/unit preimage by using an owner absent from
the lock, a stale owner revision/digest join, a stale node ID, an unreduced or
signed-denominator rational, a duplicate/zero/unsorted term, a noncanonical
unordered-member order, and a non-root/root scale-offset violation; each
applicable mutation must refuse as `invalid_semantic_graph`. In a real package,
retain an enum/member/unit preimage while independently changing the enum
literal case, `semantic_type`, declaration/dimension dependency or unit target
dependency, mirror the changed identity projection and recompute package ID;
the reader must still refuse the contradictory cross-field join. Then execute the positive case, every admitted graph
family, each declared boundary, and every explicit refusal/non-conclusive
mutation through lowering. Compare complete typed read/lower results and
artifact accounting with the vector criterion.

For migration, supply the exact authored source documents, definition artifacts
and compiled-model documents named by the V1 lock, rerun the V2 linker/checker,
and compare the returned source/target package keys plus every ordered node
mapping against `migration-correspondence-vectors.json`. Remove or stale each
required reconstruction input in turn and require refusal before a V2 package
is exposed. A byte-only conversion, version relabel or attempted synthesis of
nominal owner/name/order/dimension/unit evidence must likewise refuse. Each
refusal is the closed `MigrationOutcome` `refused` form carrying the vector's
exact code and subject; flipping any row basis against its node digests refuses.

## Expected Results

Every positive and exact-boundary result matches FR-322 and FR-195. Each
invalid, stale, cross-domain, unsupported or exhausted mutation returns its
exact typed refusal or incomplete disposition with source/provenance, exposes no
partial admitted package, emits no approximation, and preserves independent
sibling lowerer results.
