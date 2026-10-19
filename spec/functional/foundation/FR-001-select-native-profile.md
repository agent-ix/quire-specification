---
id: FR-001
title: "Select an explicit native language profile"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-001"
    type: traces_to
  - target: "ix://agent-ix/quire-specification/FR-375"
    type: references
---
# FR-001: Select an explicit native language profile

## Description

When a source unit declares its language and profile, the conforming frontend shall select the exact admitted definition.

## Inputs

UTF-8 source with language tag, edition and semantic profile declarations.

## Outputs

Selected definition identity or a located unsupported-profile diagnostic.

## Behavior

The implementation target is `ix:native`, edition `0-draft`,
`state-finite/0-draft`, defined by the `proposals/state-core` definition and
schema files. Grammar admission remains distinct from successful typing,
execution and backend support.

FS01 accepts `Quire` as the product/language name, `native Quire` as its
descriptive source-language name and `ix:native` as its stable machine identity.
It also accepts the composed `1-draft` profile hierarchy recorded by
[AD-003](../../assurance/AD-003-native-quire-language-charter.md). When a
frontend recognizes that hierarchy, the frontend shall select semantic profiles
by exact identity without treating recognition as compiler or backend support.

The earlier `state-finite/1-draft` replacement proposal is superseded as an
adoption candidate. Its bounded admission constraints are carried into the
composed state definitions and [FR-019](FR-019-admit-native-source-profile.md),
but its identity is neither enabled nor aliased to a historical or composed
profile. When a selected definition is unavailable, the frontend shall refuse
at the selected source location without falling back to another definition.

### Header-selectable layers

Every AD-003 state and value/model layer is header-selectable:
`quire.state.core/v1`, `quire.state.queries/v1`, `quire.state.graph/v1`,
`quire.value.complete/v1` and `quire.model.complete/v1`. The
`header_selectable_layers` rows of the
[complete-value lock](../../../proposals/quire-v1/definitions/complete-value-lock.json)
give each layer's identity, the layers it requires, its admitted-form
definition and the FR-290 kind and extent its witness requests.

- When a `profile` declaration names a layer's identity, the frontend shall
  select that layer and every layer its `requires` closure names, and no
  layer that requires it.
- When the unit's selected layers include the lock's `package_selection`
  `layer` (`quire.value.complete/v1`), the frontend shall select the
  qualification rows the lock's `always`, `conditional` and `exactly_one`
  rules select for the unit; otherwise it shall select exactly the
  `without_layer` rows (`edition`).
- When a declaration's `using` names that profile's alias, the frontend
  shall admit the declaration under exactly the selected layer's
  admitted-form set, which includes the sets of the layers it requires.
- When a refinement declaration's `using` names the infinite-trace profile
  alias, the frontend shall admit its row expressions under exactly
  `quire.value.complete/v1`, `quire.model.complete/v1` and their `requires`
  closure, which that profile requires when it governs a refinement; a row
  expression form outside that set is refused as the expression-form bullet
  below states.
- If such a declaration uses a declaration form outside that set, then the
  frontend shall refuse it with `unsupported_construct`, cause
  `declaration-form`, at the declaration, naming the selected layer.
- If such a declaration uses an expression form outside that set, then the
  frontend shall refuse it with `unsupported_construct`, cause
  `expression-form`, at the expression, naming the selected layer.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-001-AC-1 | A unit selecting the admitted initial definition receives that exact definition identity. | Test (TC-005) |
| FR-001-AC-2 | A unit selecting an unknown profile receives unknown_profile. | Test |
| FR-001-AC-4 | An unavailable selected profile receives a located refusal; selecting it cannot execute under another profile or reuse that profile's semantic subject. | Test (TC-005) |
| FR-001-AC-5 | For each of the five layers, a unit whose `profile` declaration names the layer and whose declarations use only that layer's forms is admitted, and its selected definitions are the layer and its `requires` closure and no layer that requires it; a state-layer unit selects the `edition` row and no other qualification row, and a complete-value or complete-model unit selects the rows the `package_selection` rules select. | Test (TC-005) |
| FR-001-AC-6 | A unit selecting `quire.state.core/v1` that declares a named predicate refuses `unsupported_construct`/`declaration-form` at the declaration, and one that calls `filter` refuses `unsupported_construct`/`expression-form` at the call, each naming `quire.state.core/v1`; both units selecting `quire.state.queries/v1` are admitted. | Test (TC-005) |
| FR-001-AC-7 | A refinement governed by the infinite-trace profile admits a `mod` row in a unit whose own declarations select no layer; a row form outside `quire.model.complete/v1` and its `requires` closure refuses `unsupported_construct`/`expression-form` at the expression, naming the layer. | Test (TC-310) |

## Dependencies

- [US-001](../../usecase/US-001-understand-native-meaning.md) supplies the user need.
- [Detailed contract or implementation evidence](../../../proposals/state-core/profile.md) supplies the scoped context.
- [Native-source amendment](../../../proposals/native-source-1-draft/README.md) preserves the superseded candidate and its carried-forward constraints.
- [AD-003](../../assurance/AD-003-native-quire-language-charter.md) fixes the language name, profile hierarchy and support-stage boundaries.
- The `header_selectable_layers` rows of the [complete-value lock](../../../proposals/quire-v1/definitions/complete-value-lock.json) list the layers.
- [FR-375](../analysis/FR-375-declare-model-refinement.md) admits refinement
  declarations; how their rows obtain complete-value and complete-model layers
  is pending an owner ruling stated there.

## Status

Accepted FS01 selection and evolution requirement. The released historical
profile implements a bounded subset; each composed semantic profile and backend
retains its independently tracked implementation and qualification status.
