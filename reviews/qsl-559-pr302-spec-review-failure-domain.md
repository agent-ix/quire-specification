---
id: SR-13588
title: "Spec review (failure-domain) of quire-specification PR #302: wait causes and binder environments"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@cf0af5ff8eb2a11406f410cb1bdcb49ad358ea22; PR #302, git diff origin/main...cf0af5f: spec/functional/protocol/FR-425 (binder environments), FR-426 (Waiting), FR-429 (closed cause list); read against FR-427, FR-428 and FR-466"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-425
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-426
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-429
    type: reviews
---
# Spec review (failure-domain) of quire-specification PR #302

## Summary

Tickets: QSL-559, QSL-562. This pass looked for rest points the closed cause
list does not cover, and for edge cases where the binder-environment rule and
FR-466's scope give different answers.

**Covered:** every channel case. That includes FIFO-key queues, `unordered`
channels, a `lost` send, and `displaced` versus `empty-queue` across repeated
sends.

Both findings with medium severity come from ADR-027 PD-4 and PS-5. QSpec
copies those rules word for word, so fixing them needs a matching QSL change.

## Verdict

Changes requested: two medium findings and one low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-429 now says "The list of causes is closed", but some reachable waits fit no cause. (a) A compensation thread at its first compensation attempt after the commit node has run (FR-427: no further `cattempt`, and `cend` needs the retry point). The root then waits at `finish` on that `running` thread, with a finish record available. (b) A thread at an event node `by r` when `r` has no live instance and cannot spawn one. Neither is `constraint`, `join-waiting` or `gate`, so the deadlock's blocked list cannot be built. Add a cause for each of these, or bound "closed" to the rest points the table names. ADR-027 PD-4 has the same gap. | spec/functional/protocol/FR-429-report-protocol-deadlocks.md:111; spec/functional/protocol/FR-427-check-protocol-compensation.md:78-83 |
| FND-002 | medium | The `all`-join rule adds entries "that each joined branch thread, or a thread below it, bound after the fork". That includes entries bound below the branch under an inner join that is not `all`, and FR-466's `Out` leaves those out (the citation in the same bullet). The reach rule normally drops them. Inside a `repeat`, though, the back edge widens the joiner's reach to the whole repeat body, which contains the inner reader, so `observations` keeps an entry FR-466 says is not readable there. Coalescing then depends on whether an implementation follows the literal rule or FR-466. Limit the copy to entries in the joined branches' `Out` (inner `all` joins only). ADR-027 PS-5 has the same wording. | spec/functional/protocol/FR-425-define-protocol-subjects-and-state-keys.md:142-145; spec/objects/interfaces/FR-466-checked-package-protocol-control-bodies.md:192 |
| FND-003 | low | FR-426's **Waiting** paragraph now lists a `block` send at capacity and a fence. It still leaves out a `send`, `event` or `finish` whose constraint no record satisfies, which FR-429 reports as `constraint`. The paragraph reads as a complete list. | spec/functional/protocol/FR-426-apply-protocol-steps.md:115-118 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | The one-cause-per-thread order gives no position to the new causes at a compensation attempt. `commit-closed` and a compensation-attempt node owned by a replicated role (the `no-role-instance` row) are absent from the order list. A compensation attempt is also blocked by ST-1's conditions (`precondition`, `no-post-state`, `constraint`), and its first attempt can be both commit-closed and have a false precondition, so FR-429 does not determine which single cause is reported. Add a `cattempt` bullet, for example `no-role-instance`, `commit-closed`, `precondition`, `no-post-state`, `constraint`. Also say whether "an `event` owned by a replicated role" includes `send` and `receive` nodes `by r`. | spec/functional/protocol/FR-429-report-protocol-deadlocks.md:146-155 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-429 adds `finish-waiting`, `commit-closed` and `no-role-instance`, with FR-429-AC-13 and AC-14 and TD-08h and TD-08i. The closed list now covers both waits this finding named. ADR-027 PD-4 and QSL FR-211's `WaitCause` do not have these causes, so QSL needs a matching amendment. FND-004 records the precedence gap. |
| FND-002 | fixed | The `all`-join addition is now limited to the joined branches' FR-466 `Out` (FR-425:144-152), and the reach extension uses the same set. FR-425-AC-11 and PK-15 test it inside a `repeat`, but their fixture is invalid (SR-13586 FND-007). ADR-027 PS-5 still says "or a thread below it" and needs the same narrowing. |
| FND-003 | fixed | FR-426's **Waiting** paragraph now covers an event, send or finish whose constraint no record satisfies, as well as `finish-waiting`, a replicated role with no instance, and a commit-closed attempt. |
