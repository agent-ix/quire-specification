---
id: FR-094
title: "Interpret temporal progress, history and closure"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-008
    type: implements
  - target: ix://agent-ix/quire-specification/FR-091
    type: references
  - target: ix://agent-ix/quire-specification/FR-092
    type: references
---
## Description

When an evaluator consumes a validated progress or closure assertion, it shall
apply that assertion only to the named clock, subjects, interval and history
boundary and shall preserve the distinction between an open prefix, a complete
closed execution and an incomplete closed input.

## Inputs

An active temporal obligation; its required future horizon and past-history
bound; validated progress/completeness/closure assertions supplied by the
observation authority; and any earlier result identities affected by them.

## Outputs

An updated temporal disposition with exact premises, or a typed contradiction,
incomplete or refused result. Historical result bytes are retained when a later
assertion invalidates their premises.

## Behavior

Progress shall be monotonic within its identified binding.

Progress shall cover an inclusive boundary.

A timestamped watermark at `W` permits settlement of a
deadline `D` only when `W >= D` and completeness covers every required source,
subject and valuation through `D`. A fixed-sample progress assertion advances
sample ticks even when no business predicate is true. Event-position time shall
not advance during silence merely because wall time passed.

A complete closed execution shall authorize the selected closed-boundary rule.

An open prefix shall leave unobserved future continuations possible.

The evaluator shall classify end-of-file, consumer shutdown or a file labelled
closed without complete required observations as an incomplete input.

The evaluator shall not apply false extension or finite-window empty truth to
an incomplete input.

Past evaluation shall require history through its computed lower boundary or an
authoritative execution origin. A later record inside an interval previously
declared complete is a completeness contradiction supplied by Agent F's
contract. The evaluator shall supersede or invalidate affected results without
rewriting their historical bytes. The v1 evaluator shall not infer watermark
authority or automatically repair settled history.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-094-AC-1 | A timestamped response exactly at an inclusive deadline participates before progress settles the window; the same response after the deadline does not. | Test (TC-115) |
| FR-094-AC-2 | With no business event, complete timestamp/sample progress through a deadline settles the applicable bounded obligation, while event-position time does not advance. | Test (TC-115) |
| FR-094-AC-3 | Open prefix, complete closed execution and closed incomplete file produce three distinct dispositions from otherwise identical visible observations. | Test (TC-116) |
| FR-094-AC-4 | Progress for a foreign clock, subject, source set or binding cannot settle the obligation. | Test (TC-115) |
| FR-094-AC-5 | A late completeness contradiction identifies the assertion and affected historical result; no earlier bytes or profile identity are restamped. | Test (TC-116) |
| FR-094-AC-6 | Missing predecessor/history on restart requires replay or an admitted state-restoration contract and cannot resume as a fresh complete stream. | Test (TC-118) |

## Dependencies

- Agent F owns the validity, provenance and replay form of observation,
  progress, completeness and contradiction assertions. This requirement owns
  only their temporal interpretation.
- Agent B owns the shared result-dimension wire contract.
