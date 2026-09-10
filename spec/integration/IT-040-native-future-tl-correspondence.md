---
id: IT-040
title: "Qualify supported native future lowering against TL"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-095
    type: verifies
---
## Objective

Verify the supported integration boundary from one checked native temporal
subject through typed predicate projection and TL lowering to the selected TL
future evaluator without losing profile, clock, capture or source meaning.

## Target Integration

The system under test is the native temporal bridge specified by
[FR-095](../functional/FR-095-preserve-native-tl-correspondence.md). Its external
boundaries are the native frontend subject, the contract-IR typed predicate
projection, `tl-syntax` formula/profile documents and `tl-mltl` evaluation.

## Preconditions

The coherent native grammar/profile baseline and IR #63/#64 bridge contracts
have passed their required reviews. The exact candidate revisions of every
producer/consumer are recorded. The reference baseline inspected for this draft
is native shared seed `e687474d47b6f7ea511d26f1a160cb85562e1f65`,
`tl-syntax` `7fb0fe32f6ba4ef2606d120ddbe5858965d4fb5a` and `tl-mltl`
`a8ad227473d58c2d2ed42411b80d1e09b9c4cdf0`; inspection is not qualification.

## Inputs

Generated bounded future native clauses within the admitted event-position and
fixed-sample false-extension domains; exact total-Boolean predicate projections;
closed traces and open prefixes; clock/capture/source bindings; independent
native truth expectations; and one-axis identity mutations.

## Test Procedure

1. Admit and lower every supported native vector.
   - IT-040-SC-01: every emitted TL subject names the exact native, bridge and TL identities and all required correspondence dimensions; the bridge record separately retains assessment execution, decision-scope and surrounding-execution progress/closure, settlement basis, exact decision support, completeness and supersession.
2. Evaluate each vector independently in the native oracle and selected TL evaluator.
   - IT-040-SC-02: truth/pending and boundary outcomes agree for every vector, including the `until` lower-bound convention and false extension.
3. Apply each one-axis identity/value/result mutation.
   - IT-040-SC-03: every mutation refuses or breaks correspondence on the changed axis without producing a substitute healthy result, including when TL truth and formula bytes are unchanged.

## Expected Results

The supported domain has exact differential agreement and complete source,
profile, valuation, clock and capture correspondence. No current implementation
claim exists until these planned steps execute against reviewed candidates.

## Metadata

- Priority: High
- Target Integration: native frontend → contract IR → tl-syntax/tl-mltl
- Automation: Planned Rust integration test

## Dependencies

- [Contract-IR issue #63](https://github.com/agent-ix/quire-contract-ir/issues/63)
- [Contract-IR issue #64](https://github.com/agent-ix/quire-contract-ir/issues/64)
- [FR-090](../functional/FR-090-select-temporal-profile-and-clock.md)
