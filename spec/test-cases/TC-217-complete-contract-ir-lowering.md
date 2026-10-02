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
unsupported nodes.

## Test Procedure

Use the subject and inputs named by FR-195 and FR-322. Validate the
`proposals/checked-package-v2` schema and positive fixtures; require the schema
to refuse an unrecognized `contract_version`. Then exercise unknown-version,
missing/duplicate/unknown member, a recorded `package_id` that differs from its
identity preimage, cross-domain digest, unknown required capability,
non-canonical-byte and each selected byte, node, edge, locus, diagnostic and
work boundary through the real local Rust reader; each one-over-limit input must
report the limit, its value and its setting. Validate every object in the V2
`node-identity-vectors.json` against `node-identity-preimage.schema.json`,
recompute its RFC 8785 JCS SHA-256 and require the recorded node ID. Mutate each
enum/member/dimension/unit preimage by using an owner absent from the lock, a
stale node ID, an unreduced or signed-denominator rational, a
duplicate/zero/unsorted term, a noncanonical unordered-member order, and a
non-root/root scale-offset violation; each applicable mutation must refuse as
`invalid_semantic_graph`. Mutate an application node's `dependencies` list out
of ascending-digest order; this mutation must also refuse as
`invalid_semantic_graph` (FR-322-AC-10). In a real package, retain an
enum/member/unit preimage while independently changing the enum literal case,
`semantic_type`, declaration/dimension dependency or unit target dependency,
mirror the changed identity projection and recompute package ID; the reader must
still refuse the contradictory cross-field join. Then execute the positive case,
every admitted graph family, each declared boundary, and every explicit
refusal/non-conclusive mutation through lowering. Compare complete typed
read/lower results and artifact accounting with the vector criterion.

## Expected Results

Every positive and exact-boundary result matches FR-322 and FR-195. Each
invalid, stale, cross-domain, unsupported or exhausted mutation returns its
exact typed refusal or incomplete disposition with source/provenance, exposes no
partial admitted package, emits no approximation, and preserves independent
sibling lowerer results.
