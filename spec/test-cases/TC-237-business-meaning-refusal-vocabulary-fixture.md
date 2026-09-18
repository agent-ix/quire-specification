---
id: TC-237
title: "Business-meaning refusal-vocabulary fixture"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-208
    type: verifies
---

# TC-237: Business-meaning refusal-vocabulary fixture

## Description

Schema-validate the K2 business-meanings fixture and check that it names at
least one vector for every distinct admission or refusal situation
FR-208-AC-4 through AC-12 describes. This is a fixture and vocabulary
conformance check, not an evaluation of FR-208 itself: it runs no
business-meaning binder and asserts no computed result. FR-208's own
acceptance criteria stay verified by TC-235, through the real business-meaning
binder, once that binder exists; until then TC-235 and FR-208-AC-4 through
AC-12 are `planned`/unbacked, and this test case does not change that.

## Test Procedure

Load the published fixture
[`business-meanings-k2.json`](../../proposals/checked-package-v2/fixtures/business-meanings-k2.json)
and its schema
[`business-meanings-k2.schema.json`](../../proposals/checked-package-v2/business-meanings-k2.schema.json).

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| F01 | Validate the fixture against its schema | schema-valid |
| F02 | For each situation FR-208-AC-4 through AC-12 names (identity/operation misuse per meaning, missing-name references, conflicting bindings, cross-meaning member typing, the two `forbidden-pre-read` cases, the `wrong-context` invariant case, and specialization's `malformed-declaration`, `derivation-conflict` and `specialization-cycle` cases), look up its fixture vector by id | a vector exists, tagged to the named AC, and, for a refused situation, carrying the exact `code`/`cause` pair FR-208 names for it |
| F03 | Confirm the fixture's `nodes` list includes each of Order, Money, OrderPlaced, OrderLifecycle, Fulfilment, OrderStore and OrderManagement | present |
| F04 | Confirm an `admitted` vector exists for each of the four specializable business meanings' inheritance case (record value type, event type, state machine, process) | present, and the process vector's scenario states that a process inherits no steps |

## Expected Results

The fixture is schema-valid and its vectors name every refusal situation
FR-208-AC-4 through AC-12 describes, each with its exact code/cause pair.
This test case does not discharge FR-208-AC-4 through AC-12; those criteria
require the real business-meaning binder that TC-235's Test Procedure names,
which does not yet exist in `quire-spec-language`.
