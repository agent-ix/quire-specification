---
id: FR-247
title: "Complete-global-conformance closure state vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-059
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-115
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-116
    type: specifies
---
# FR-247: Complete-global-conformance closure state vocabulary

## Description

When a global-conformance claim is reported, the result producer SHALL retain exactly one complete-global-conformance closure state from this closed vocabulary plus the exact required execution, branch and workflow closure-premise identities, while any other claim retains `not-required` with no global premise set.

## Values

| Value | Description |
| --- | --- |
| `closed` | The global closure record is closed. Complete global success requires `closed` together with every exact required execution, branch and workflow closure premise (FR-059, FR-061). |
| `open` | The global closure record is open. It prevents complete global success and remains represented (FR-059, FR-061). |
| `incomplete` | The global closure record is incomplete. It prevents complete global success and remains represented (FR-059, FR-061). |
| `contradicted` | The global closure record is contradicted. It prevents complete global success and remains represented (FR-059, FR-061). |
| `not-required` | The claim is not global conformance. The record is explicitly `not-required` and carries no global premise set; a supplied global premise under `not-required`, a global claim marked `not-required`, or another progress/closure axis relabelled `not-required` is refused (FR-061, FR-115, FR-116). |

## Identity and comparison

- **Identity kind** — *value*. A closure-state label has no identity of its own
  and is compared componentwise as an exact string. The *record* it belongs to
  additionally carries the exact required execution, branch and workflow
  closure-premise identities (FR-059, FR-061).
- **Key** — the label itself. Matching is exact. Display name, source path, span,
  message text, locale and rendering never participate; FR-061 states a strict
  reader "SHALL NOT infer independence from distinct display names or
  implementations". A premise set is compared by its exact premise identities: an
  omitted, duplicated, cross-wired or scope-mismatched premise is refused
  (FR-059-AC-5, FR-061-AC-10, FR-115).
- **Key order** — lexicographic over the label, for collection comparison only. It
  carries no progression from `open` to `closed` and no causal meaning. No
  requirement states an order over the premise identities within a premise set,
  so none is declared here.
- **Causal order** — none over the labels. The required execution, branch and
  workflow premises are authored protocol facts; their order is the authored
  partial order of FR-052 and is never derived from key order, timestamp or
  source position.
- **Absence** — an absent closure record compares only to absence. It is never
  defaulted to `open` or to `not-required`: FR-061 refuses "a global claim without
  its complete-global-conformance closure record", and a non-global claim must
  carry `not-required` explicitly rather than by omission.
- **Digest domain** — not applicable; this key contains no digest. See
  [`ix://agent-ix/quire-specification/FR-201`](../foundation/FR-201-canonical-identity-domain.md).

## Five independent axes

FR-061 states that this record is the fifth of five independent facts:
"Decision-scope progress and closure and surrounding-execution progress and
closure are four independent facts with exact scope, authority and boundary
identities. Complete-global-conformance closure is a fifth independent fact for a
global-conformance claim". FR-061 then forbids substitution outright: "The result
producer SHALL NOT use completeness, assessment execution or any other
progress/closure fact as a substitute for any closure axis."

FR-059 is equally explicit: "Complete global success requires `closed` and every
exact required premise; a closed decision scope, settled per-obligation truth,
completed assessment or closed surrounding execution cannot substitute for a
missing or non-closed global premise."

[FR-115](../../functional/observation/FR-115-report-activation-participation-and-adequacy.md):
"While complete-global-conformance closure is open, incomplete or contradicted,
the adapter SHALL NOT report complete global success from a closed decision
scope, closed surrounding execution or a decisive witness/counterexample alone."

This artifact therefore types **only** the fifth axis. The other four axes are
retained as separate properties of the canonical protocol result
([FR-240](FR-240-canonical-protocol-result.md)) and each carries one member of
FR-284's closed scope-progress-and-closure vocabulary.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-247-AC-1 | Each member's value equals the exact label string in the table. | Inspection |
| FR-247-AC-2 | A global-conformance claim without a closure record, and a non-global claim carrying a global premise set, are each refused. | Analysis |
| FR-247-AC-3 | Complete global success is unavailable unless the state is `closed` and every exact required premise is present; omitting, duplicating, cross-wiring or scope-mismatching a premise prevents it. | Analysis |
| FR-247-AC-4 | `not-required` substituted for a global claim or for another progress or closure axis is refused. | Analysis |
| FR-247-AC-5 | An independently settled per-obligation truth is unchanged by any state of this record. | Analysis |

## Dependencies

- [FR-059](../../functional/protocol/FR-059-assess-finite-global-conformance.md)
  establishes the four states and the required premise set.
- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md) adds
  `not-required` for a non-global claim and the five-axis independence rule.
- [FR-115](../../functional/observation/FR-115-report-activation-participation-and-adequacy.md)
  and
  [FR-116](../../functional/observation/FR-116-map-observation-results-to-consumers.md)
  restate the record at the observation-result and consumer boundaries.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Status

Accepted complete-V1 global-closure contract. Implementation and qualification
remain tracked separately.
