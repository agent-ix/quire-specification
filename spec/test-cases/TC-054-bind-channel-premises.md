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

Evaluate duplicate receipt under at-least-once delivery and two FIFO channels with independent keys. Remove or conflict each sender, receiver, message, order and delivery premise; change one premise in the valid request.

## Expected Results

Duplicates and ordering follow only the declared channel/key scope. Missing/conflicting premises refuse, and the one-axis change alters request identity and result provenance.

