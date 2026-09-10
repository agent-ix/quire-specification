---
id: TC-116
title: "Separate open, closed and incomplete temporal inputs"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-091
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-093
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-094
    type: verifies
---
## Description

Planned lifecycle controls for activation, pending future, complete closure and
incomplete input.

## Test Procedure

Reuse identical visible observations as an open prefix, an authoritatively
complete closed execution and an EOF-closed file missing one required source.
Exercise an already decisive witness and counterexample, an undecided future,
an untriggered scope and a late record contradicting earlier completeness.
Mutate EOF into execution closure and restamp the earlier result after the late
record.

## Expected Results

Decisive open prefixes may settle; undecided ones remain pending. Complete
closure applies the selected boundary rule, while the incomplete file reports
incomplete. Inactive activation remains separate from truth. A late
contradiction names the prior premise/result and preserves historical bytes.

