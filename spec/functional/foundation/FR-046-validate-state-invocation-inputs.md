---
id: FR-046
title: "Validate state invocation inputs before clause evaluation"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-005
    type: implements
  - target: ix://agent-ix/quire-specification/FR-208
    type: references
---
## Description

When accepting a state assessment request, the validator SHALL establish the exact anchor, observations, context and applicable invocation/frame correspondence required by its selected clause before evaluating it.

## Inputs

Operation/invocation/context/universe identities, immutable parameter/result values, exact pre/post snapshots or a pre-call observation, completeness statements and selected authored frame.

## Outputs

A validated anchored request or a typed binding/frame refusal or unavailable-input result.

## Behavior

A precondition is observed pre-call. Its input is a **pre-call observation**: the operation, the pre snapshot the call starts from, the receiver `self` in that snapshot and the call's immutable parameter values. A precondition reads exactly that observation; it reads no post snapshot, result or delta, and no frame check is a condition of its verdict. A request supplies the pre-call observation on its own, with no post snapshot, or as the pre snapshot, receiver and parameters of one invocation; the precondition's verdict is the same either way, whatever the invocation's post snapshot, result and delta hold. A snapshot's role is its `observation` member (`pre`, `post` or `current`). A pre or post snapshot has the optional `invocation` member FR-012 names (the four identity labels of its invocation document); a present member that differs from the invocation document's labels is `refused { code: wrong_snapshot, cause: wrong-invocation }` before evaluation, and an absent member leaves the snapshot unbound. Before evaluation, a pre-call observation whose snapshot's `observation` is not `pre` is `refused { code: wrong_snapshot, cause: wrong-observation }`, and one whose parameters omit a declared parameter of the operation is `refused { code: invalid_runtime_input, cause: missing-member }`, naming the omitted parameter. Postconditions use pre and post, and result is post-only. Parameters come from one declared call. Complete pre/post populations determine actual create/delete/change inventories; caller-supplied lists must match and stay within the selected frame. Missing observation is not empty population or a frame violation. A complete post snapshot lacking required self is a context-binding refusal even when its deletion was permitted. Callers cannot select a more permissive frame or substitute another operation's observation.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-046-AC-1 | Correctly bound pre/post inputs and parameters from one invocation supply the expected distinct self reads and immutable parameter values. | Test (TC-046) |
| FR-046-AC-2 | Foreign operation/invocation/universe, wrong parameter/result type or a substituted frame refuses before evaluation. | Test (TC-046) |
| FR-046-AC-3 | For complete observations, incorrect creation/deletion inventories and changes beyond the selected frame refuse with distinct typed causes. | Test (TC-046) |
| FR-046-AC-4 | Missing pre/post/completeness input that the selected clause reads remains unavailable/incomplete, without fabricating a delta, violation or Boolean; a precondition request with no post snapshot is complete. | Test (TC-046) |
| FR-046-AC-5 | Allowed deletion of required post self still refuses the post-context binding; an independently qualified pre reference remains a pre reference. | Test (TC-046) |
| FR-046-AC-6 | An invariant binds its named initialization/handler observation; a bare context type or unrelated current snapshot refuses as a substitute. | Test (TC-046) |
| FR-046-AC-7 | A precondition over a pre-call observation (pre snapshot, receiver and parameters, no post snapshot) evaluates to the same verdict as over an invocation with that pre snapshot, receiver and parameters, including an invocation whose post snapshot changes fields outside the selected frame; a pre-call observation whose snapshot's `observation` is `post` refuses `wrong_snapshot`/`wrong-observation`, and one whose parameters omit a declared parameter refuses `invalid_runtime_input`/`missing-member` naming it, each before evaluation. | Test (TC-401) |
| FR-046-AC-8 | A pre or post snapshot whose `invocation` member differs from the four identity labels of the invocation document it is read with refuses `wrong_snapshot`/`wrong-invocation` before evaluation; a matching member and an absent member are read unchanged. | Test (TC-046) |

## Dependencies

- [Detailed draft contract](../../../proposals/quire-v1/state-contract.md).
- [Shared grammar](../../../proposals/quire-v1/shared-grammar.md).
- [Planned acceptance matrix](../../composed-foundation/tests.md).

Accepted by FS03. Historical definitions and their acceptance remain separate;
this artifact does not establish implementation coverage.
