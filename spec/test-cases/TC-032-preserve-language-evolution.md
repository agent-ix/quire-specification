---
id: TC-032
title: "Preserve selected meanings across language evolution"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-032
    type: verifies
---
## Description

Planned acceptance controls for [FR-032](../functional/FR-032-preserve-language-evolution.md).
No executable test or completed coverage is claimed by this artifact.

## Test Procedure

Retain one historical source and definition. Introduce independently a new operator, declaration and family, including a keyword colliding with a previously legal identifier. Read old and new selections with old/new support sets. Migrate one source explicitly and compare original artifacts before and after.

Use the [edition/state candidates](../../proposals/quire-v1/definitions/README.md)
as exact new-definition inputs, distinct from the historical profile. Change a
selected rule-file byte while retaining its declared digest: selection must
refuse. Then issue an explicitly revised definition and dependency closure:
it has a new selection and cannot relabel the prior result. Merely knowing the
new artifact bytes, parsing its source or supporting its core subset cannot
establish full graph-profile support. Assert the actual requested feature scope.

Select the preserved protocol definition 1-draft.1 with its original rule bytes,
then select 1-draft.2 with B's revised settlement/closure relation. Substituting
the revised FR-061 bytes under the earlier definition's declared digest refuses.
Explicitly migrating to 1-draft.2 creates a new linked subject and leaves the
earlier source, definition and result identities unchanged. Digest-checking an
envelope alone does not establish support for the revised result relation.

The earlier core candidate's prose mentioned indexing despite selecting a grammar
without it. Retain those original bytes as an unadopted inconsistent candidate;
do not convert the mention into an admitted source form. Core/query/graph and
temporal revision 1-draft.2 remove the claim or refresh its dependency closure;
protocol 1-draft.3 consumes that closure. Substituting new bytes under an earlier
dependency digest refuses. The historical grammar, finite-sequence operations,
temporal truth rules and protocol settlement relation remain unchanged.

Select the preserved temporal 1-draft.2 and protocol 1-draft.3 definitions, then
explicitly migrate to temporal 1-draft.3 and protocol 1-draft.4. Substitute the
new FR-090/091/093/094/095 or FR-061/protocol-contract bytes under the old digest,
and substitute the old temporal-facet digest under the new protocol selection:
each mismatched closure refuses. Equal source/formula bytes cannot hide a
changed definition selection. The unchanged state 1-draft.2 closure remains
valid in both new roots.

Under the new selections, exercise a closed decision scope in an open workflow,
an open trigger scope without an observed trigger, a missing runtime sample
under a valid binding and a malformed clock binding. Expect, respectively,
independent closure axes, activation-unknown, incomplete and refused. Retain
exact decisive support and supersession through the result correspondence.
These controls select the new rules explicitly; they do not relabel an old
candidate's outcomes. A missing E02 projection capability cannot refuse native
source admission, but the independently requested projection remains unsupported.

## Expected Results

The historical source retains its interpretation. New required meaning either has explicit support or refuses. Migration produces a new subject and source correspondence without mutating old bytes or restamping old evidence. Exact keyword/grammar vectors are supplied by L1 before implementation.
