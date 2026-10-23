---
id: TC-210
title: "Finite simulation reproducibility"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-181
    type: verifies
---

# TC-210: Finite simulation reproducibility

## Description

Enumerate and seed-sample declared schedules, replay them, and exhaust the exploration bound.

## Test Procedure

Use the subject and inputs named by FR-181. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-181; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.

## Sampler

Seeded sampling selects the `quire.simulation.sampler/v1` sampler by its
identity.

## Refusal vectors

The explicit refusal/non-conclusive mutations include:

- an application whose operation identity is outside the closed operation
  catalog, which refuses `invalid_package`/`unknown-operation` at its
  `source_map` locus;
- a write outside the operation's declared frame, which refuses
  `frame_violation`/`unauthorized-change`;
- a model-described external effect, which returns only as typed trace data.

## Non-coalescing and cancellation vectors

- Two states whose typed semantic state differs only in an IEEE value's NaN
  payload or a signed zero's sign have distinct state keys and remain
  distinct states.
- A successor that violates a declared invariant is recorded as a successor
  rather than pruned from exploration.
- A caller cancellation mid-exploration returns `incomplete` with cause
  `cancelled`/`caller-cancelled` and carries the unexplored frontier.

## Successor and initial-state order vectors

- A parent with one transition identity `t` and two post-states `s1` and
  `s2` whose state-key bytes order `s1` before `s2` visits `s1` before
  `s2`, whichever order the successor relation yields them in; an equal
  third post-state with the key of `s1` is not a further successor
  (FR-181-AC-16).
- A subject supplied with initial states `[b, a]`, where `a`'s state-key bytes
  order before `b`'s, explores and indexes them as `[a, b]`, exactly as when
  supplied as `[a, b]`; supplied as `[a, b, a]` it has two initial states,
  `a` at index 0 and `b` at index 1 (FR-181-AC-17).
- Over that two-initial-state subject with no requirement naming the start,
  sampled traces 0, 1, 2 and 3 start at `a`, `b`, `a` and `b`; no draw selects
  the start, so each trace's first draw is the step-0 draw of its own trace
  index (FR-181-AC-18).

## Typed canonical form vectors

- Encode each value FR-181-AC-19 lists and compare the JCS bytes exactly
  (FR-181-AC-19).
- Encode decimal `1.50`, the quantity `1.50` in `Units::metre` and a
  `Set<Timestamp>` of `2` and `10` as set elements and as a record field, and
  compare the bytes FR-181-AC-20 states (FR-181-AC-20).
- Encode two states whose population member holds the same reference
  identities, inserted into the set in two different orders, and compare the
  state keys; read the uppercase UUID, the `+1` and `01` Timestamp payloads,
  the Timestamp payloads 2^127 and -2^127 - 1, and 2^127 - 1 against the
  form (FR-181-AC-21).
- Encode each string, rational, float and rational-quantity value
  FR-181-AC-22 lists and compare the JCS bytes exactly, and read the
  zero-denominator rational and the 7-digit, 9-digit and uppercase `bits`
  strings against the form (FR-181-AC-22).
- Encode the `Set<String>`, `Set<Rational>`, `Set<Float64>` and
  `Set<Quantity<Units::metre>>` values FR-181-AC-23 lists and compare the
  bytes and the element order exactly (FR-181-AC-23).
- Compare the state keys of the state pairs FR-181-AC-24 lists: `+0` against
  `-0`, one float set inserted in two orders, composed against decomposed
  `é`, and `2/4` against `1/2` (FR-181-AC-24).

## Sampler index-sequence vector

For seed `424242`, trace `0`, choice `0` and enabled-successor-set size
`n = 5`, a real implementation of the counter-mode generator draws, at
`draw = 0` for every step: index `1` at step `0`, index `3` at step `1`,
index `3` at step `2`, index `4` at step `3` and index `4` at step `4`. The
step-0 preimage is
`{"choice":"0","draw":"0","seed":"424242","step":"0","trace":"0"}`; its
SHA-256 is
`d5160380d7495443315376de306a5d3613f3e010df74a5db92853829205995f2`.
