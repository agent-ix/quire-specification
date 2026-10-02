---
id: FR-268
title: "Progress and late-data authority"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-113
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-266
    type: references
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
  - target: ix://agent-ix/quire-specification/FR-255
    type: references
  - target: ix://agent-ix/quire-specification/FR-289
    type: references
  - target: ix://agent-ix/quire-specification/FR-294
    type: references
---
# FR-268: Progress and late-data authority

## Description

When silence would settle a deadline, the observation service SHALL settle it only where declared progress and closure authority covers that deadline.

FR-113 Behavior states it directly: "Silence SHALL settle a deadline only when
declared progress/closure authority covers that deadline." FR-113-AC-1 requires
that "A quiet eligible deadline settles only with declared progress authority."

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Selected progress/assertion definition identity | FR-113 Description | Retained exactly before a settled disposition is produced. |
| Declared clock interpretation | FR-113 Description | Typed as a distinguishable fact by [FR-267](FR-267-observation-time-facts.md). |
| Progress authority | FR-113 Description, FR-113-AC-1 | Its coverage is what permits silence to settle a deadline. |
| Captured trigger identity | FR-113 Description | Retained exactly. |
| Late-data cutoff | FR-113 Description | The selected cutoff a late event is classified against. |
| Restoration basis | FR-113 Description | A mismatched basis "SHALL retain an incomplete or refused input rather than settle or restart from it" (FR-113 Behavior). |
| Window applicability | FR-113 Behavior | Progress and closure apply "only to admitted members with instants `start <= instant < end`" of the selected window ([FR-266](FR-266-event-time-window.md)). |

FR-113 Behavior requires that the service "classify a late event against the
selected cutoff rather than silently rewriting an already settled result", and
FR-113-AC-3 that "A late event is retained as late and cannot silently replace
the selected settled result."

The v1 authority identity is derived under `quire.verification.jcs` from the
exact definition identity, subject scope, clock
selection, required source set, boundary convention and restoration basis.
The v1 cutoff identity is derived separately under
`quire.verification.jcs` from the clock selection, cutoff instant and cutoff
rule identity.

## Identity and comparison

- **Identity kind** — *derived*. Authority and cutoff have separate
  `sha256-jcs:<hex>` identities over the preimages above.
- **Key** — the complete derived identity of the selected authority or cutoff.
  All named semantic components participate; display name, source path, span,
  message text, locale and rendering never do.
- **Key order** — bytewise over `definition identity`, for collection
  comparison only. It carries **no causal meaning** and no recency, and FR-113
  Behavior states that "Arbitrary timestamp order SHALL NOT establish
  causality."
- **Causal order** — none. Coverage of a deadline by a progress authority is a
  declared scope relation, not a causal edge, and it never crosses a window
  boundary: FR-113-AC-5 requires that progress and closure "cannot settle or
  contradict a later window through that boundary".
- **Absence** — an absent key component compares only to absence. Where the
  authority does not cover a deadline, silence does not settle it (FR-113
  Behavior, FR-113-AC-1); an uncovered deadline is neither a settled deadline
  nor a violated one. An absent or mismatched restoration basis, scope, clock or
  source set "SHALL retain an incomplete or refused input" (FR-113 Behavior) and
  is never defaulted.
- **Digest domain** — authority and cutoff identities use
  `quire.verification.jcs`. A crossing domain is unequal.

## Late data

A late event is classified against the selected cutoff using FR-294's exact
`on-time` / `late` vocabulary. The classification is a retained fact about the
event; it does not rewrite the settled result it arrives after, and it does not
restart the assessment (FR-113-AC-3, FR-113-AC-4). Contradiction is an
independent completeness consequence under FR-295, never a third timing label.

## Cross-family boundary mapping

FR-289 governs this authority when it settles an inclusive temporal deadline.
The selected half-open carrier end and progress watermark must prove same-clock
coverage strictly beyond the inclusive upper bound, while assessment
membership is filtered to instants at or below that bound. Equality between a
half-open end/watermark and the inclusive deadline is insufficient.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-268-AC-1 | A quiet deadline not covered by a declared progress authority compares unequal to a settled deadline and to a violated one. | Test |
| FR-268-AC-2 | Two authorities differing in definition identity compare unequal. | Test |
| FR-268-AC-3 | An event classified late against the selected cutoff leaves the previously settled result's bytes unchanged. | Test |
| FR-268-AC-4 | A mismatched assertion definition, scope, clock, source set or restoration basis compares only to absence for settlement purposes and does not restart an assessment. | Test |
| FR-268-AC-5 | A native inclusive deadline is covered only through an exact FR-289 mapping; equality of a half-open end/watermark with that deadline cannot settle it. | Test |
| FR-268-AC-6 | Any authority scope, clock, required-source set, boundary or restoration-basis mutation changes the authority identity; any cutoff clock, instant or rule-identity mutation changes the cutoff identity. | Test |
| FR-268-AC-7 | Missing, foreign-domain, duplicate or unbounded authority/cutoff inputs refuse before settlement or timing classification. | Test |

## Dependencies

- [FR-113](../../functional/observation/FR-113-preserve-time-progress-and-late-data.md)
  establishes the retained inputs, the silence rule, the late-data cutoff and
  the mismatch rule.
- [FR-266](FR-266-event-time-window.md) supplies the half-open window that
  bounds progress and closure applicability.
- [FR-267](FR-267-observation-time-facts.md) types the time facts the authority
  is interpreted against.
- [FR-255](../temporal/FR-255-bounded-temporal-interval.md) records the
  inclusive temporal convention this authority is asked to cover, and the
  conflict with the half-open observation convention.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) supplies the digest
  domain rule.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Authority boundary

Progress proves source coverage through a boundary. Closure fixes an assessment
scope. Completeness accounts for required facts. They are independent inputs;
no one of them derives either of the others.

## Status

Proposed progress/cutoff identity closure for `quire-specification#40`.
