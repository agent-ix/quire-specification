---
id: SR-13586
title: "Spec review (base) of quire-specification PR #302: protocol checking rules aligned with QSL ADR-027"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@cf0af5ff8eb2a11406f410cb1bdcb49ad358ea22; PR #302, git diff origin/main...cf0af5f: spec/functional/protocol/FR-170-evaluate-channel-queue-semantics.md, spec/functional/protocol/FR-425-define-protocol-subjects-and-state-keys.md, spec/functional/protocol/FR-426-apply-protocol-steps.md, spec/functional/protocol/FR-429-report-protocol-deadlocks.md, spec/test-cases/TC-375-protocol-state-key-and-coalescing.md, spec/test-cases/TC-376-two-branch-join-worked-example.md, spec/test-cases/TC-377-join-policies-at-thresholds.md, spec/test-cases/TC-378-channels-under-delivery-policies.md, spec/test-cases/TC-383-protocol-terminal-member-and-wait-causes.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-170
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-425
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-426
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-429
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-375
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-376
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-377
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-378
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-383
    type: reviews
---
# Spec review (base) of quire-specification PR #302

## Summary

Tickets: QSL-559, QSL-562. This PR is the QSpec side of QSL PR #1109
(quire-spec-language main 693c168). It touches 9 files, +159/-23. The
checklist pass compared every new normative statement with ADR-027 PS-2, PS-5,
PS-7, PS-8, ST-3, ST-7, SE-5 and PD-4, and with QSL FR-205, FR-206 and FR-211.

**Matches ADR-027 with no semantics added or lost:**

- the wait causes and their order (Gate first; receive: Constraint,
  HeadBlocked, Displaced, EmptyQueue; send: Constraint, else FullQueue);
- `displaced` as a property of the replay prefix that does not enter the key;
- the send record `{channel, payload, outcome, displaced}`;
- the fence row, which matches FR-436's `tso` table;
- nested cancellation, including "settled and removed", as QSL FR-206 states;
- the ordinal hold rule and its sibling definition for forks and compensation
  threads;
- the binder-environment rule: values, reach, the all-join reach extension,
  and the empty environment of a compensation thread;
- the qualified channel name.

The old cause list has no stale users: a grep over spec/ and proposals/ finds
no schema or vector that still uses it, and TC-378 CH-04 was moved to
`displaced`. No changelog, status or ticket wording was added.

Gates: `quire validate --scope . 'spec/**/*.md'` fails 134 documents on both
origin/main and the PR head. The set of failing files is identical, and none
of the 9 touched files has a diagnostic.

## Verdict

Changes requested. The findings below are in coverage and fixtures: one trace
gap, two fixture gaps, one AC that cannot be observed, and two nits. The
companion integrity (SR-13587) and failure-domain (SR-13588) reviews hold the
semantic findings, including the one high.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-170-AC-6 names `Test (TC-201, TC-378)`, but TC-201 has no vector that checks a send record or an overflow outcome. TC-378, which does hold one (CH-10), declares `verifies` FR-426 only, not FR-170. No TC that verifies FR-170 checks AC-6. Either add `verifies FR-170` to TC-378 and drop TC-201 from the cell, or add a TC-201 vector. | spec/functional/protocol/FR-170-evaluate-channel-queue-semantics.md:62; spec/test-cases/TC-201-channel-and-queue-premises.md; spec/test-cases/TC-378-channels-under-delivery-policies.md:1-8 |
| FND-002 | medium | FR-429-AC-10's `loss head` and `loss new` cases do not say which delivery policy they use, and neither do AC-11, TD-08d (first two variants) or TD-08e. Delivery is mandatory (FR-054-AC-3), and under `unknown` or `at-most-once`, `lose` stays enabled at `[Q]` or `[P]`. Those states are then not terminal, the stated queue contents do not hold, and AC-11's reported counterexample can end at `R1` with `displaced` instead of at `R2` with `empty-queue`. QSL FR-211-AC-5 fixes `delivery exactly-once-premise` for these protocols. State it here too. | spec/functional/protocol/FR-429-report-protocol-deadlocks.md:168-169; spec/test-cases/TC-383-protocol-terminal-member-and-wait-causes.md:39-40 |
| FND-003 | medium | TD-08f is not a fixture: it names no protocol, no capacity and no delivery policy. A deadlock-freedom item reports one counterexample, so to get two prefixes that end in one state the test must build both and replay them (FR-433 step 5), and the vector says neither. A test could pass without exercising the claim, so this does not meet the bar the other TD rows meet. A concrete fixture: capacity 1, `overflow loss head`, `delivery at-most-once`, branch `s` `repeat maximum 2 { send S of P }`, branch `r` `receive R1 of S; receive R2 of S`. Prefix A (send, send that displaces, `R1`) ends at `R2` with `empty-queue`. Prefix B (send, `R1`, send, `lose`) ends in the same key with `displaced`. Replay both. | spec/test-cases/TC-383-protocol-terminal-member-and-wait-causes.md:41; spec/functional/protocol/FR-429-report-protocol-deadlocks.md:169 |
| FND-004 | medium | FR-426-AC-14 says that a `block` send and a `fence` whose rule is false "wait and are reported by their causes (FR-429)". No deadlock can contain such a fence: under `tso`, `flush` is enabled while the buffer is non-empty (FR-436 "Terminal states"), and under `sc` and `ra` every fence is enabled. So the blocked list never shows a fence cause, and FR-429 names none (SR-13587 FND-002). The TC side is thin as well: CH-12 is a one-line generic row that checks no cause and is filed under channels, and TC-377, the other TC the cell names, has no row for AC-14. Restate the fence clause as "has no enabled step" only, and give CH-12 concrete protocols. | spec/functional/protocol/FR-426-apply-protocol-steps.md:206; spec/test-cases/TC-378-channels-under-delivery-policies.md:42 |
| FND-005 | low | No TC-375 vector covers FR-425-AC-9's second sentence: a joining thread keeps an entry through a later fork that copies it, and a thread resting at or before the binding node holds no entry. PK-12 covers only the first sentence. | spec/functional/protocol/FR-425-define-protocol-subjects-and-state-keys.md:234; spec/test-cases/TC-375-protocol-state-key-and-coalescing.md:41 |
| FND-006 | low | CH-08t's first clause still says the send "records the loss". In the send-record vocabulary that send has outcome `accepted` with `displaced` `m1`, as CH-08h says. | spec/test-cases/TC-378-channels-under-delivery-policies.md:39 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | medium | No implementation can build the fixture for FR-425-AC-11 and PK-15. It places "a reader of `x` inside `Inner`'s other branch", but FR-466's binder scope says that "sibling branches read nothing of each other" (FR-466-AC-10), so S3 refuses the protocol with `missing_declaration`/`missing-name` before anything is explored, under both the `any` and the `all` join. Put the reader after the binding in `x`'s own branch, for example `sequence { attempt X binding x; check C { x.f } }`. That reader is still inside the `repeat` body, so the back edge still widens the joiner's reach to it, which is the case the AC is meant to test. | spec/functional/protocol/FR-425-define-protocol-subjects-and-state-keys.md:243; spec/test-cases/TC-375-protocol-state-key-and-coalescing.md:44 |
| FND-008 | low | No test can observe the first clause of FR-429-AC-15 ("has cause `gate` where the blocked list is computed") through the public boundary. The blocked list exists only on a deadlock counterexample, and the second clause of the same AC says no deadlock contains such a thread. TD-08j has to "pass a state to the blocked-list computation", which is an internal function. Either keep only the observable clause (FL-09 already covers it) or state the cause as a rule with no AC of its own. | spec/functional/protocol/FR-429-report-protocol-deadlocks.md:191; spec/test-cases/TC-383-protocol-terminal-member-and-wait-causes.md:45 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-170-AC-6 now traces to TC-378 only, and TC-378 declares `verifies` FR-170 (0825a54). |
| FND-002 | fixed | FR-429-AC-10 and AC-11 and TD-08b, TD-08d and TD-08e now state `delivery exactly-once-premise`, and give `at-most-once` only where the AC relies on `lose`. |
| FND-003 | fixed | TD-08f and FR-429-AC-11 now give the concrete two-branch fixture and replay both prefixes through FR-433. |
| FND-004 | fixed | FR-426-AC-14 now states only cases a test can observe. CH-12 is a concrete `block` deadlock, and FL-09 covers a fence that waits while `flush` is enabled. |
| FND-005 | fixed | PK-14 covers the second sentence of FR-425-AC-9. |
| FND-006 | fixed | CH-08t now reads outcome `accepted` with `displaced` the entry of `m1`. |
