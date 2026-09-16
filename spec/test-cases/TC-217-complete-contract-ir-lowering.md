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

Read the frozen I04 `quire.checked-package/v1` positive, boundary, refusal,
mutation and canonical-byte/digest vectors through the real Contract IR public
reader; then lower supported complete-V1 nodes and mutate
source/type/anchor/bounds plus unsupported nodes.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-195 and FR-322.
First exercise unknown-version, missing/duplicate/unknown member, stale raw
source/definition/model digest, cross-domain digest, unknown required
capability, non-canonical-byte and each selected resource boundary through the
real local Rust reader. Then execute the positive case, every admitted graph
family, each declared boundary, and every explicit refusal/non-conclusive
mutation through lowering. Compare complete typed read/lower results and
artifact accounting with the vector criterion.

## Expected Results

Every positive and exact-boundary result matches FR-322 and FR-195. Each
invalid, stale, cross-domain, unsupported or exhausted mutation returns its
exact typed refusal or incomplete disposition with source/provenance, exposes no
partial admitted package, emits no approximation, and preserves independent
sibling lowerer results.
