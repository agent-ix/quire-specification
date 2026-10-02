---
id: TC-379
title: "Split-shipment causal interleavings"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-052
    type: verifies
---

# TC-379: Split-shipment causal interleavings

## Description

Check that the behaviours of a `parallel` project onto exactly the linear
extensions of its causal edges, as FR-052 states, and that no other order is
added.

## Test Procedure

Build the split-shipment protocol of FR-052-AC-1: `parallel Ship { branch s1
sequence S1 { event P1 …; event D1 …; } branch s2 sequence S2 { event P2 …;
event D2 …; } } join all [s1, s2];` with every constraint true and one record
per event type. Explore exhaustively and project each behaviour onto its
`attempt`, `event`, `send` and `receive` steps.

## Vectors

| Vector | Variant | Expected projections |
| --- | --- | --- |
| SS-01 | as built | Exactly six: `P1 D1 P2 D2`, `P1 P2 D1 D2`, `P1 P2 D2 D1`, `P2 P1 D1 D2`, `P2 P1 D2 D1`, `P2 D2 P1 D1`; `D1` never precedes `P1` and `D2` never precedes `P2` |
| SS-02 | `P1` replaced by an attempt that sets `shipped1 := true`, and `D2` by an attempt whose precondition is `shipped1` | Exactly the five extensions of SS-01 in which `P1` precedes `D2`; `P2 D2 P1 D1` is not a behaviour |
| SS-03 | the branches written in the opposite source order, and the event records given equal timestamps | The same six projections as SS-01 |
| SS-04 | `s1` sends a message after `P1` that `s2` receives before `P2` | Exactly the extensions in which `P1` precedes the send, the send precedes the receive and the receive precedes `P2` |

## Expected Results

The projection set equals the expected set in every vector; canonical
transition order changes the order of exploration and the canonical
counterexample, never the set.
