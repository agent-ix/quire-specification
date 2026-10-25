---
id: TC-005
title: "Select composed native definitions without fallback"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-001
    type: verifies
---
## Description

Distinguish the adopted historical definition, the superseded replacement
candidate and an adopted composed definition. This is planned compiler
acceptance, not an executed standard test.

## Test Procedure

Select the unavailable
superseded candidate through the actual parser/package reader and assert a
located refusal. Select an adopted composed definition by its identity as the
positive control.

For each `header_selectable_layers` row, compile a unit whose `profile`
declaration names the layer and whose one declaration uses only its forms,
and read the selected definitions. Then compile a unit selecting
`quire.state.core/v1` that declares a named predicate, one that calls
`filter`, and both again selecting `quire.state.queries/v1`. Then compile a
unit whose `profile` declaration names `quire.temporal.complete/v1` and whose
state model writes a `time` member, a `Clock` field and an atomic clock
constraint, and the same unit with the declaration naming
`quire.model.complete/v1`.

## Expected Results

An unavailable definition yields no checked clause and never falls back. A
supported composed selection binds its own definition. The superseded
candidate is never aliased to either selection.

Each layer's unit is admitted, and its selected definitions are the layer
and its `requires` closure and no layer that requires it, written out per
layer as a literal; each state-layer unit's qualification rows are exactly
`edition`, and the complete-value, complete-model and complete-temporal units' rows are
the `package_selection` rules' selection. Under state core the predicate declaration refuses
`unsupported_construct`/`declaration-form` and the `filter` call refuses
`unsupported_construct`/`expression-form`, each naming
`quire.state.core/v1`; under state queries both are admitted.

The `quire.temporal.complete/v1` unit is admitted and its selected definitions are the layer, `quire.model.complete/v1`, `quire.state.graph/v1`, `quire.state.queries/v1`, `quire.value.complete/v1` and `quire.state.core/v1`, written out as a literal; the `quire.model.complete/v1` unit refuses the `time` member `unsupported_construct`/`declaration-form` naming `quire.model.complete/v1` and the required layer `quire.temporal.complete/v1`.
