---
id: TC-112
title: "Distinguish closure boundaries and bounded past history"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-091
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-092
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-293
    type: verifies
---
## Description

Planned distinguishing vectors for false-extension versus finite-window closure
and for authoritative origin versus missing history.

## Test Procedure

Evaluate `always[0,1] p` at position zero on a one-position closed trace where
`p` is true. Evaluate `once`, `historically`, `since` and `triggered` across
zero-width, origin-crossing and nonzero-lower-bound windows. Repeat identical
visible suffixes once with an authoritative origin and once with an unknown
earlier cutoff. Mutate the origin flag into a cutoff and replace missing-history
disposition with false padding.
Read exact `execution-origin`, exact `history-cutoff`, unknown, aliased and
absent boundary kinds at the same clock position. Restore authenticated state
whose earliest retained position is the cutoff and attempt to relabel it as an
execution origin.

## Expected Results

False-extension makes `always[0,1] p` false while complete finite-window
quantification makes it true. Past operators match their independent mirror
oracles only with complete history/origin. The cutoff case reports missing
history and every mutation that converts it to a Boolean is detected.
Only the two exact boundary labels are admitted. Equal positions with different
kinds remain unequal, and restoration can prove retained cutoff coverage but
cannot authorize false extension before it.
