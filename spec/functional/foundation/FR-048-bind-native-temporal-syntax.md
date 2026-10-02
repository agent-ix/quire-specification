---
id: FR-048
title: "Bind native temporal syntax to typed value environments"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-005
    type: implements
  - target: ix://agent-ix/quire-specification/FR-034
    type: references
  - target: ix://agent-ix/quire-specification/FR-090
    type: references
  - target: ix://agent-ix/quire-specification/FR-093
    type: references
  - target: ix://agent-ix/quire-specification/AD-012
    type: references
  - target: ix://agent-ix/quire-specification/FR-367
    type: references
---
## Description

When linking a native temporal declaration, the frontend SHALL retain its typed
formula structure and explicit current-input, clock-role, activation and capture
bindings under the selected temporal profile.

## Inputs

The located native syntax tree, exact domain package and profile imports, shared checked
predicates and the declaration's binding requirements.

## Outputs

A source-bound temporal declaration with typed Boolean leaves and explicit
environment requirements, or a located syntax, scope, type or profile refusal.

## Behavior

The frontend SHALL parse each `holds` argument with the common value-expression
grammar.

The frontend SHALL keep temporal formula nodes distinct from ordinary Boolean
values.

The linker SHALL resolve current-input and trigger types to exact declaration keys in the selected domain package.

The linker SHALL restrict the trigger binder to activation/guard/capture scope.

The evaluator SHALL initialize captures in source order at the activation anchor.

The evaluator SHALL consume immutable capture values at later instants.

The binder SHALL resolve the clock role to one exact compatible observation
binding at execution.

When a backend cannot preserve the declaration's meaning, the bridge SHALL
report the unsupported mapping against the retained native subject.

The frontend SHALL parse the unbounded temporal operator grammar edition
(no interval bound) defined by [the shared grammar](../../../proposals/quire-v1/shared-grammar.md#unbounded-temporal-operator-grammar-edition-infinite-trace)
and accepted by [AD-012](../../assurance/AD-012-unbounded-temporal-operator-grammar.md)
only where the clause selects `quire.temporal.infinite-trace/v1`. Under any
other selected profile the same recognized form receives `unsupported_construct`
at its exact construct span, owning declaration and selected profile, as AD-003's
required diagnostic dispositions state for a recognized form prohibited by the
selected profile.

Where the clause selects the infinite-trace profile, the frontend SHALL also
parse the bracketed interval form on every interval-capable
operator, nested with the interval-less form
([FR-367](../temporal/FR-367-evaluate-interval-operators-under-infinite-trace.md)),
the clause's fairness constraints
([FR-362](../temporal/FR-362-apply-fairness-constraints.md)) and a model's
`terminal` declaration ([FR-366](../temporal/FR-366-check-deadlock-freedom.md)).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-048-AC-1 | Native future/past forms and explicit Boolean leaves parse with the declared precedence, original spans and common expression trees; unparenthesized binary temporal chains refuse. | Test (TC-049) |
| FR-048-AC-2 | Negative, fractional and unbounded intervals refuse syntax; reversed integer bounds refuse admission; zero-width inclusive bounds remain distinct from an empty interval. | Test (TC-049) |
| FR-048-AC-3 | The timed-refund declaration binds current and trigger inputs separately; activation-only names outside captures, self/result/pre ambient access, shadowing and forward capture reads refuse. | Test (TC-049) |
| FR-048-AC-4 | Two semantic triggers with equal amounts retain separate capture environments; later input changes and duplicate receipts do not replace a capture or create another trigger identity. | Test (TC-049) |
| FR-048-AC-5 | Missing, foreign or incompatible clock/subject/view bindings remain unavailable or refused without inserting a default clock, sample or empty population. | Test (TC-049) |
| FR-048-AC-6 | On a complete one-position false-extension trace, always[0,1] true is true and always[0,1] holds(true) is false; native-to-TL lowering cannot fold away this distinction. | Test (TC-049) |
| FR-048-AC-7 | A temporal expression in a shared Boolean argument and a non-Boolean holds root refuse, while the same admitted query/predicate expression retains its common type semantics inside holds. | Test (TC-049) |
| FR-048-AC-8 | A source-valid timestamped or past-time declaration retains its native identity when the current TL mapping reports unsupported; no profile or clock substitution occurs. | Test (TC-049) |
| FR-048-AC-9 | An interval-less `always`/`eventually`/`until`/`release`/`once`/`historically`/`since`/`triggered` spelling is admitted only when the clause selects `quire.temporal.infinite-trace/v1`; the identical spelling under any other selected profile receives `unsupported_construct` at its construct span with the owning declaration and selected profile. | Test (TC-238) |
| FR-048-AC-10 | Under `quire.temporal.infinite-trace/v1`, `always (holds(f) implies eventually always[0,10] holds(h))` parses with the interval-less and bracketed forms nested; under every profile, `[a,*]` is a syntax refusal. | Test (TC-238) |

## Dependencies

- [Native temporal grammar and scope rules](../../../proposals/quire-v1/shared-grammar.md).
- [Temporal interpretation](../temporal/FR-090-select-temporal-profile-and-clock.md).
- [Activation and captures](../temporal/FR-093-bind-temporal-activation-and-captures.md).
- [Bridge correspondence](../temporal/FR-095-preserve-native-tl-correspondence.md).
- [AD-012](../../assurance/AD-012-unbounded-temporal-operator-grammar.md) accepts
  the unbounded temporal operator grammar edition this requirement admits.
- [Planned shared matrix](../../composed-foundation/tests.md).

This is A's source integration contribution. E owns temporal meaning; spec artifacts declare
the domain model and F owns observation binding contracts. Exact definition artifacts and complete
baseline review remain required before dependent implementation.
