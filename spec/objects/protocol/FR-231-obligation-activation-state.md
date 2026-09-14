---
id: FR-231
title: "Obligation activation state"
type: FR
object: state_machine
relationships:
  - target: ix://agent-ix/quire-specification/FR-055
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-093
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-230
    type: references
  - target: ix://agent-ix/quire-specification/FR-244
    type: references
---
# FR-231: Obligation activation state

## Description

When a trigger scope is assessed, the evaluator SHALL report the obligation's activation as triggered, inactive or activation-unknown (FR-055, FR-093, FR-061).

FR-055 adds that the evaluator "SHALL NOT treat an inactive or unknown
obligation as satisfied or demonstrated", and FR-061 that the result producer
"SHALL NOT classify an inactive obligation as satisfied or demonstrated".

FR-093 states that "Activation, temporal truth and participation shall remain
separate dimensions", and FR-055 states that the evaluator "SHALL keep
activation independent from truth".

## States & Transitions

```mermaid
stateDiagram-v2
    [*] --> triggered : admitted trigger event or whole-execution origin with established captures per FR-055 and FR-093
    [*] --> inactive : trigger scope authoritatively closed with complete input and no admitted trigger per FR-093-AC-5
    [*] --> activation_unknown : open trigger scope or missing or refused trigger evidence per FR-093-AC-5
    [*] --> no_instance : admitted activation guard is false so the evaluator creates no instance and no refusal per FR-093
    triggered --> triggered : repeated delivery of the same semantic trigger with agreeing payload creates no second instance per FR-093-AC-2
    no_instance --> [*]
```

The initial pseudostate represents assessment of a trigger scope; it is not an
emitted activation value or an invented pre-report domain state.

The activation label vocabulary itself is **defined by
[FR-244](FR-244-activation-state.md)**, which holds `triggered`, `inactive` and
`activation-unknown` as a closed vocabulary extracted from the same FR-055 and
FR-061 prose. This artifact defers to FR-244 for the labels, their exact
spellings and their meanings; the entries below record only how this machine
enters each label and are not a second definition of it.

State meanings, as entered by this machine and defined by FR-244:

- `triggered` — an admitted trigger established the instance. FR-061 requires
  that a satisfied or violated truth has "activation is triggered".
- `inactive` — FR-093: "A trigger scope that is authoritatively closed and
  complete without an admitted trigger shall produce inactive activation; it is
  not a true obligation and is not evidence that recovery behavior was
  exercised." FR-061: "Activation is inactive" implies "truth is unavailable and
  the trigger scope is complete under an exact closure authority."
- `activation-unknown` — an open trigger scope or missing/refused trigger
  evidence prevents activation from being known. Completeness retains its
  independently derived FR-295 state; missing evidence is recorded there and a
  refused read remains a separate assessment-execution disposition.
- `no_instance` — FR-093: "The evaluator shall create no instance and no refusal
  when an admitted activation guard is false." This is the absence of an
  obligation instance, not a fourth activation value.

No requirement states a transition out of `inactive` or `activation_unknown`
once reported, so none is drawn here.

## Identity and comparison

- **Identity kind** — *value*. An activation state has no identity of its own; it
  is compared componentwise as an exact label.
- **Key** — the state label itself, matched exactly. Diagnostic prose, display
  name, source path, span, message text, locale and rendering never participate
  (FR-009, FR-047); FR-061-AC-1 requires that the cases "remain discriminable
  without diagnostic prose".
- **Key order** — lexicographic over the label, for collection comparison only.
  It carries no precedence between activation values, and in particular no
  ordering from unknown towards triggered.
- **Causal order** — none. An activation state is a reported fact about an
  instance, not a node in the authored partial order of
  [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md).
- **Absence** — an obligation whose activation has not been reported has an
  absent activation state. Absence compares only to absence; it is never
  defaulted to `inactive`. FR-055 states that "The evaluator SHALL NOT treat an
  inactive or unknown obligation as satisfied or demonstrated."
- **Digest domain** — not applicable; no key component of this object is a
  digest.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-231-AC-1 | A closed and complete trigger scope with no admitted trigger reports `inactive`, and an open scope or missing or refused trigger evidence reports `activation-unknown`. | Test |
| FR-231-AC-2 | Neither `inactive` nor `activation-unknown` is readable as satisfied or demonstrated. | Test |
| FR-231-AC-3 | A false admitted activation guard produces no instance and no refusal, and is not recorded as an activation value. | Test |
| FR-231-AC-4 | An absent activation state compares only to absence and receives no default value. | Test |

## Dependencies

- [FR-055](../../functional/protocol/FR-055-activate-protocol-obligations.md).
- [FR-093](../../functional/temporal/FR-093-bind-temporal-activation-and-captures.md).
- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md).
- [FR-244](FR-244-activation-state.md) defines the activation label vocabulary
  this machine's states are read against.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Axis ruling

FR-055, FR-093 and FR-061 now agree: missing or refused required trigger
evidence yields `activation-unknown`; input completeness retains its separately
derived FR-295 state, assessment execution retains its own
non-completed/refused state, and an open trigger scope remains a separate
progress/closure fact. `inactive`
requires authoritative trigger-scope closure plus complete input and no admitted
trigger. `incomplete` and `open` are not activation values.

## Status

Proposed activation-axis reconciliation for `quire-specification#40`.
