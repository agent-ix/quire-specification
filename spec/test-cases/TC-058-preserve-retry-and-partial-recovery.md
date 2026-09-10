---
id: TC-058
title: "Preserve retries, duplicates and partial recovery"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-058
    type: verifies
---
## Description

Verify bounded retry identity and exact aggregate recovery over a declared related-effect population.

## Test Procedure

Present failed then successful refund attempts, a timed-out attempt and a duplicate effect. Sum two partial refunds under closed membership, then remove a member, make membership unknown and exceed the retry maximum.

## Expected Results

Attempt/effect outcomes remain distinct. Only the complete exact population can satisfy aggregate recovery; missing/unknown members and excess attempts cannot produce success.

