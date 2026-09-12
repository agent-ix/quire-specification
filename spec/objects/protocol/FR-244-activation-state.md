---
id: FR-244
title: "Activation state vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-055
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-062
    type: specifies
---
# FR-244: Activation state vocabulary

## Description

When a result reports whether an obligation activated, the strict reader SHALL match the activation state as an exact string from this closed vocabulary, in which neither `inactive` nor `activation-unknown` classifies an obligation as satisfied or demonstrated.

## Values

| Value | Description |
| --- | --- |
| `triggered` | The trigger was established and the dependent obligation exists with its exact workflow, role, effect, predicate, capture and evaluation-anchor identities (FR-055). Required for truth `satisfied`, `violated` or `pending`, and for an obligation-path protocol-adequacy demonstration (FR-061). Projects to E02 `activation.status` `activated` (FR-062). |
| `inactive` | No trigger was admitted while the trigger scope is authoritatively closed and complete (FR-055, FR-093). Assessment execution is `completed`, truth is `unavailable`, and the surrounding subject execution may remain open (FR-061). It is not evidence that recovery behavior was exercised (FR-093). Projects to E02 `not-activated` (FR-062). |
| `activation-unknown` | The trigger scope is open, or trigger evidence is missing or refused (FR-055, FR-093). Truth is `unavailable` and completeness is open or incomplete with the missing trigger dependencies (FR-061). Projects to E02 `unknown` (FR-062). |

## Identity and comparison

- **Identity kind** — *value*. An activation label has no identity of its own and
  is compared componentwise as an exact string.
- **Key** — the label itself. Matching is exact. Display name, source path, span,
  message text, locale and rendering never participate; FR-061 states a strict
  reader "SHALL NOT infer independence from distinct display names or
  implementations". The identity of the *activated obligation instance* is a
  separate matter owned by FR-093: it is "identified by the clause subject and
  semantic trigger-event identity, not by timestamp, display name or equal
  payload alone".
- **Key order** — lexicographic over the label, for collection comparison only. It
  carries no precedence and no causal meaning.
- **Causal order** — none over the labels. The trigger event that produces
  `triggered` participates in the authored partial order of FR-052; that order is
  never derived from key order, timestamp or source position.
- **Absence** — an absent activation state compares only to absence. It is never
  defaulted to `inactive`, which is a positive assertion that the trigger scope is
  closed and complete without a trigger.
- **Digest domain** — not applicable; this key contains no digest. See
  [`ix://agent-ix/quire-specification/FR-201`](../foundation/FR-201-canonical-identity-domain.md).

## Recorded divergence: activation spelling

- [FR-055](../../functional/protocol/FR-055-activate-protocol-obligations.md) Outputs:
  "Triggered, inactive or activation-unknown obligation states with their exact
  dependencies". [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md)
  uses `triggered`, `inactive` and `activation-unknown` verbatim in its
  result-combination rules.
- [FR-093](../../functional/temporal/FR-093-bind-temporal-activation-and-captures.md)
  Outputs: "An inactive scope, an activated obligation with immutable typed
  captures, or a located incomplete/refused activation result", and FR-093-AC-5:
  "A closed-complete trigger scope with no trigger reports inactive; an open scope
  or missing/refused trigger evidence reports activation-unknown".
- [FR-062](../../functional/protocol/FR-062-project-results-to-portable-verification.md)
  treats `activated` as the **E02 target** name for the source state `triggered`:
  "Activation `triggered` / `inactive` / `activation-unknown` | `activation.status`
  is respectively `activated` / `not-activated` / `unknown`."

FR-093 writes "an activated obligation" where FR-055 and FR-061 write
`triggered`. No requirement states that FR-093's "activated" is the FR-061 source
label rather than the FR-062 target label. This artifact lists only the three
FR-055/FR-061 source labels and records the divergence without resolving it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-244-AC-1 | Each member's value equals the exact label string in the table, and the E02 target names `activated` / `not-activated` / `unknown` do not match a source activation state. | Inspection |
| FR-244-AC-2 | An `inactive` or `activation-unknown` obligation is not read as satisfied or demonstrated. | Analysis |
| FR-244-AC-3 | An absent activation state compares only to absence and is never defaulted to `inactive`. | Analysis |
| FR-244-AC-4 | Removing trigger telemetry moves activation toward `activation-unknown` and cannot improve truth or adequacy. | Analysis |

## Dependencies

- [FR-055](../../functional/protocol/FR-055-activate-protocol-obligations.md) establishes
  the three obligation states.
- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md) establishes the
  activation combination rules.
- [FR-062](../../functional/protocol/FR-062-project-results-to-portable-verification.md)
  establishes the E02 target names, which are not source labels.
- [FR-093](../../functional/temporal/FR-093-bind-temporal-activation-and-captures.md)
  supplies the divergent temporal spelling recorded above.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
