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
located refusal. After an owning semantic ticket explicitly adopts a composed
definition, select its exact version/digests and mutate each independently;
attempt replay of an old semantic subject under the new selection. Use a valid
exact selected definition as the positive control.

For each `header_selectable_layers` row, compile a unit whose `profile`
declaration names the layer and whose one declaration uses only its forms,
and read the selected definitions. Then compile a unit selecting
`quire.state.core/v1` that declares a named predicate, one that calls
`filter`, and both again selecting `quire.state.queries/v1`.

## Expected Results

An unavailable or mismatched
definition yields no checked clause and never falls back. A supported exact
composed selection binds its own definition and new derivation. The superseded
candidate is never aliased to either selection.

Each layer's unit is admitted, and its selected definitions are the layer
and its `requires` closure and no layer that requires it, written out per
layer as a literal. Under state core the predicate declaration refuses
`unsupported_construct`/`declaration-form` and the `filter` call refuses
`unsupported_construct`/`expression-form`, each naming
`quire.state.core/v1`; under state queries both are admitted.
