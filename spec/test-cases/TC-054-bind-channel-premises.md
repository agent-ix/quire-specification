---
id: TC-054
title: "Bind channel delivery and order premises"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-054
    type: verifies
---
## Description

Verify that channel semantics are explicit, scoped and part of request identity.

## Test Procedure

Evaluate delivery intervals `[1,1]` and `[1,2]`, `unordered`, and two FIFO
channels whose key expressions can produce the same exact typed value. Change
the FIFO expression while preserving table positions. Remove or conflict each
sender, receiver, message, ordering and delivery premise; offer an unknown
ordering member, a negative/reversed interval, and a FIFO binder, message type,
expression or result type that fails admission. Change each independent premise
in turn.

## Expected Results

Duplicate permission follows only the inclusive delivery interval; no separate
duplication value exists. FIFO order follows only the exact channel/evaluated-key
pair, while `unordered` has no key scope. Every malformed, missing, conflicting
or unsupported premise refuses, and each admitted one-axis change alters request
identity and result provenance.
