---
id: IT-041
title: "Qualify unsupported temporal bridge boundaries"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-092
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-094
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-095
    type: verifies
---
## Objective

Verify that current TL consumers fail closed for native past-time,
timestamped-event, finite-window and incomplete-observation requests rather
than emitting a syntactically valid artifact with changed meaning.

## Target Integration

The system under test is the native temporal bridge at the native/contract-IR/TL
boundary. Agent F's reviewed observation interface supplies synthetic valid and
invalid progress/history premises; this test does not redefine their authority.

## Preconditions

The native temporal requirements and the consumed observation/result interfaces
have passed their coherent reviews. Current TL profile capabilities are queried
from exact candidate artifacts rather than inferred from repository or ticket
names.

## Inputs

Each bounded past operator; timestamped finite-window clauses; the
false-extension/finite-window discriminator; open, closed and incomplete inputs;
missing history; silent deadline progress; and exact unsupported expectations.

## Test Procedure

1. Submit each unsupported semantic request to the bridge.
   - IT-041-SC-01: the bridge returns a typed unsupported result naming every unmatched profile/operator/clock/boundary dimension and emits no substitute TL formula.
2. Submit open, closed-complete and closed-incomplete variants with valid and foreign progress/history bindings.
   - IT-041-SC-02: each lifecycle premise produces its specified distinct disposition and foreign premises cannot settle a result.
3. Add a newly reviewed mock capability for one formerly unsupported dimension, then alter one semantic rule.
   - IT-041-SC-03: support appears only for the exact matching capability; the altered rule remains unsupported.

## Expected Results

Every unsupported or incomplete request remains explicit and source-bound. A
capability name, parsable formula or available future evaluator never supplies
missing past, event-time, finite-window, history or completeness semantics.

## Metadata

- Priority: High
- Target Integration: native temporal bridge refusal boundary
- Automation: Planned Rust integration test

## Dependencies

- [FR-092](../functional/FR-092-evaluate-bounded-past.md)
- [FR-094](../functional/FR-094-interpret-progress-history-and-closure.md)
- [FR-095](../functional/FR-095-preserve-native-tl-correspondence.md)

