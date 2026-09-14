---
id: TC-058
title: "Preserve retries, duplicates and partial recovery"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-058
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-210
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-235
    type: verifies
---
## Description

Verify bounded retry identity and exact aggregate recovery over a declared related-effect population.

## Test Procedure

Present a failed compensation-attempt key, replay it, then present a different
successful retry key, a timed-out attempt and a distinct duplicate-effect key.
Reuse an attempt/effect key with another registration, operation, predecessor or
requirement; substitute a forward-operation-attempt kind; permute keys and
reverse timestamp/arrival order. Sum two partial refunds under closed membership,
then remove a member, make membership unknown and exceed the retry maximum.
For one finite maximum, count the initial attempt plus admitted failed,
timed-out and successful keys; repeat one key and its receipt; offer the same
over-bound key twice and a second over-bound key. Remove observation/progress
evidence and independently remove, foreign-bind, conflict and duplicate the
authored predecessor/successor relation.

## Expected Results

Attempt/effect outcomes and subject kinds remain distinct; replay mints no
attempt, incompatible rebinding refuses, and only the authored retry relation
orders attempts. Neither the retry maximum nor key/timestamp/arrival order
identifies or orders an attempt; retry counting is asserted separately under its
own policy rule. Only the complete exact population can satisfy aggregate recovery;
missing/unknown members and excess attempts cannot produce success.
Only distinct admitted attempt keys count. Replay counts once; refused
over-bound candidates are not retained and never change the prior count.
Missing observation/progress remains incomplete, while malformed retry
relationships refuse without inferred order.
