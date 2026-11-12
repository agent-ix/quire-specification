---
id: FR-174
title: "Project global protocols soundly"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-060
    type: references
  - target: ix://agent-ix/quire-specification/AD-007
    type: references
  - target: ix://agent-ix/quire-specification/FR-288
    type: references
---

# FR-174: Project global protocols soundly

## Description

When local projection is requested, the analyzer SHALL derive each selected
role's behavior only when the global graph and visibility premises justify a
sound projection.

## Inputs

Checked global graph, selected roles, channel/choice premises, projection profile
and bounds.

## Outputs

Role-local graphs plus soundness record, counterexample, inconclusive result or
typed refusal.

## Behavior

Projection preserves observable sends/receives, causal/control order, queues,
dynamic lifecycle and obligations for the selected role. Unobservable choice,
incompatible merges or insufficient bounds return a minimal global-path
counterexample or non-conclusive result; no branch is dropped.

## Projection correctness

Projection maps each global send to one local send and its matching local
receive, erases only steps unobservable to the selected role, and retains the
induced causal order. Branches may merge for a role only when their next visible
actions have the same direction, peer, message type and compatible continuation,
or prior evidence lets that role distinguish the branch. Parallel composition
preserves channel-key order and all admitted interleavings. Correctness requires
both local-composition trace inclusion in the global semantics and preservation
of every globally required visible action.

## Derived-artifact identity

A role-local graph, like a realizability witness (FR-175), is a derived
artifact: analysis derives it from selected inputs and no author states its
identity. Its identity kind is *derived* and its key is structural, as
[FR-249](../../objects/protocol/FR-249-derived-projection-result.md) keys a
derived projection result, with no hash:

- the FR-288 qualified definition key of the global protocol definition;
- the selected role;
- the projection profile;
- the bounds.

Two projected artifacts are the same artifact exactly when all four members
compare equal. Display name, path, span, producing run, timestamp and artifact
bytes never participate, and no digest stands in for a member. An absent member
is a refusal, never a default. A change to any member is a different artifact.
Where the artifact is carried inside a canonical protocol result, FR-062 and
FR-249 identify only the projected E02 result around it, not the artifact.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-174-AC-1 | A projectable graph yields local graphs whose composed traces are included in the admitted global traces. | Test (TC-205) |
| FR-174-AC-2 | A hidden choice or incompatible local merge yields a source-correspondent counterexample. | Test (TC-205) |
| FR-174-AC-3 | Unsupported dynamic/queue constructs or exhausted bounds return non-conclusive/unsupported without a substitute graph. | Test (TC-205) |
| FR-174-AC-4 | Every global communication projects to exactly one matching send/receive pair with preserved occurrence correspondence. | Test (TC-205) |
| FR-174-AC-5 | A local branch merge is accepted only under compatible next actions or explicit distinguishing evidence. | Test (TC-205) |
| FR-174-AC-6 | Changing the FR-288 definition key, the selected role, the projection profile or the bounds yields a different derived-artifact identity, while changing only display name, path, span or producing run does not; an absent key member is refused, and no digest of the artifact stands in for a member. | Test (TC-205) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
