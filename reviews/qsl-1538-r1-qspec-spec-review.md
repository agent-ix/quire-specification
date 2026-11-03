---
id: SR-20261103145405
title: "QSL-1538 r1 spec review of quire-specification PR #610 (FR-465 delivery count interval)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; PR #610 head f7934212; spec/objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md; spec/functional/protocol/FR-426-apply-protocol-steps.md; spec/test-cases/TC-947-checked-package-protocol-declaration-bodies.md; proposals/checked-package-v2/fixtures/protocol-node-{vectors,mutation-manifest}.json"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-465
    type: references
  - target: ix://agent-ix/quire-specification/FR-426
    type: references
  - target: ix://agent-ix/quire-specification/FR-054
    type: references
  - target: ix://agent-ix/quire-specification/FR-216
    type: references
---

# QSL-1538 r1 spec review of PR #610

## Summary

Ticket: QSL-1538. The PR adds a fifth `delivery` text to the FR-465 channel
layout: the canonical count interval `[lo,hi]`. It adds FR-465-AC-10 and
TC-947 vectors CH13a, CH13b and CH14a, excludes intervals from FR-426's
`duplicate`/`lose` guard, and removes a hard-coded vector count from the
manifest test.

The served requirement is QSL FR-509 (AC-2, AC-4, AC-5), read from QSL
`origin/main`. The text grammar matches FR-509's emission rule: `[lo,hi]`
with no space, `u64` bounds, and `lo <= hi`. It also matches the companion
Contract IR PR #855 (`ChannelDelivery::parse`: digits only, no leading zero,
fits `u64`, `lo <= hi`). The edge cases hold: `[0,0]`, `[2,2]` and
`[0,18446744073709551615]` are admitted, and `[1,1]` and
`exactly-once-premise` are different bodies. FR-465's statement that an
interval enables neither `duplicate` nor `lose` matches FR-509 ("adds no
separate `lose` or `duplicate` nondeterminism") and IR #855's
`channel_classes`.

`quire validate` (quire 0.36.2, engine 0.50.2) on the three touched spec
files exits 0 with no findings, so no baseline comparison was needed.

## Verdict

**CHANGES REQUESTED.** The vocabulary change is correct and agrees with
FR-509 and IR #855. But FR-465 now says one send produces `d` deliveries with
`lo <= d <= hi`, and FR-426 and FR-170 still give every send exactly one
entry. With `duplicate`/`lose` disabled for intervals, QSpec's step relation
treats every interval channel as delivering exactly once. That contradiction
is introduced by this PR, so it is resolved here, not left as a follow-up.
FR-054 and FR-216 still list the delivery policy as exactly four words.

Out of PR scope, for the QSL owner: QSL FR-509's Dependencies cite "QSpec
FR-464/FR-466", and its relationship targets FR-464. The channel delivery
vocabulary is in FR-465 (channel layout row 5). FR-466 holds control bodies
and has no channel layout.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-465 now states that an interval is a per-send cardinality ("one send produces a number of deliveries `d` with `lo <= d <= hi`"). FR-426's `send` row still appends one entry per FR-170, and its send record has one `outcome`. This PR makes the `duplicate`/`lose` row exclude intervals. So in QSpec's step relation, every interval channel delivers exactly once per send. `[0,0]` still delivers once, `[2,2]` delivers once, and `[0,3]` behaves like `[1,1]`. That contradicts FR-465 and makes FR-509-AC-5's executions (d = 0, d = 2, and d = 3 stopping `full-queue`) unspecifiable. Fix it in this PR: give FR-426's `send` row (and FR-170's `send`) the interval case, with one step per `d` in `lo..=hi`, each of the `d` entries taking a capacity place and subject to the overflow policy. State what the send record carries for `d != 1`. | spec/functional/protocol/FR-426-apply-protocol-steps.md:97-99,113-124; spec/objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md:146-154; spec/functional/protocol/FR-170-evaluate-channel-queue-semantics.md:42-48 |
| FND-002 | medium | FR-054 still says "The delivery policy is one of `unknown`, `at-most-once`, `at-least-once` and `exactly-once-premise`", and FR-054-AC-3 refuses "an unknown delivery policy". FR-216's Delivery policy row says "Exactly one of" the four words. An interval channel admitted by FR-465 is therefore refused by FR-054's own text. Neither gives the interval's duplication or loss permission. Add the interval as the fifth form in both, with its permission rule (no separate duplication or loss), and cite FR-465. | spec/functional/protocol/FR-054-bind-channel-premises.md:45-50,66; spec/objects/protocol/FR-216-channel-premise.md:26,40 |
| FND-003 | low | FR-465-AC-4 still ends "and another delivery spelling refuses `malformed_wire`". Read against its first clause (only `exactly-once-premise` and `unknown` are named), that literally refuses `[0,3]`, which AC-10 admits. Reword it, for example "a delivery text that is neither a policy word nor a canonical interval refuses `malformed_wire`". | spec/objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md:311 |
