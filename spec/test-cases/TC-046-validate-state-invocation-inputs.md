---
id: TC-046
title: "Validate state invocation inputs before clause evaluation"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-046
    type: verifies
---
## Description

Accepted portable controls for [FR-046](../functional/foundation/FR-046-validate-state-invocation-inputs.md).
The matrix remains Planned; no executable test or observed coverage is claimed.

## Test Procedure

Reuse the explicit pre-version=1/post-version=2 operation and authored preserve/update/delete frames, without modifying historical fixtures. Independently change each binding identity, substitute a frame, misstate create/delete inventories, introduce forbidden field changes, omit observation/completeness, and delete required self under an otherwise permissive frame.

Also bind an invariant to its named initialization/handler observation, then
substitute an arbitrary current snapshot with matching object values.

Also read the pre and post snapshots with an `invocation` member holding the
invocation document's four identity labels, with the labels of another
invocation document, and with no `invocation` member.

## Expected Results

The substituted invariant snapshot refuses the anchor identity, regardless of
coincident content. A snapshot whose `invocation` labels differ from the
document's refuses `wrong_snapshot`/`wrong-invocation` before evaluation; the
matching member and the absent member are read unchanged.

Only the exact validated request evaluates. Complete adverse inputs refuse for their actual binding/frame condition; unavailable evidence is incomplete rather than an invented model change. No deleted post object is synthesized from pre identity.
