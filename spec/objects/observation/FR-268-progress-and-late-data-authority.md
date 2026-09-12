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
| Revision | FR-113 Description | Retained exactly. |
| Byte digest | FR-113 Description | Retained exactly; its domain is selected under [FR-201](../foundation/FR-201-canonical-identity-domain.md). |
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

## Identity and comparison

- **Identity kind** — *value*. The authority is a selection, compared
  componentwise; FR-113 requires the selection to be retained and gives it no
  identity separable from it.
- **Key** — (`progress/assertion definition identity`, `revision`,
  `byte digest`). FR-113 Description names exactly this identity/revision/digest
  triple as retained, and FR-113-AC-4 makes a mismatched "assertion definition,
  scope, clock, source set, or restoration basis" non-settling, so a change to
  the selection is not the same authority. Equality is componentwise over the
  key and nothing else; display name, source path, span, message text, locale
  and rendering never participate.
- **Key order** — lexicographic over (`definition identity`, `revision`,
  `byte digest`), for collection comparison only. It carries **no causal
  meaning** and no recency: a higher revision is not a successor of a lower one
  in any settlement sense, and FR-113 Behavior states that "Arbitrary timestamp
  order SHALL NOT establish causality."
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
- **Digest domain** — the progress/assertion definition byte digest carries a
  selected domain under `ix://agent-ix/quire-specification/FR-201`. An identity
  crossing domains is unequal regardless of byte equality.

## Late data

A late event is classified against the selected cutoff (FR-113 Behavior). The
classification is a retained fact about the event; it does not rewrite the
settled result it arrives after, and it does not restart the assessment
(FR-113-AC-3, FR-113-AC-4).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-268-AC-1 | A quiet deadline not covered by a declared progress authority compares unequal to a settled deadline and to a violated one. | Analysis |
| FR-268-AC-2 | Two authorities differing in definition identity, revision or byte digest compare unequal. | Analysis |
| FR-268-AC-3 | An event classified late against the selected cutoff leaves the previously settled result's bytes unchanged. | Analysis |
| FR-268-AC-4 | A mismatched assertion definition, scope, clock, source set or restoration basis compares only to absence for settlement purposes and does not restart an assessment. | Analysis |

## Dependencies

- [FR-113](../../functional/FR-113-preserve-time-progress-and-late-data.md)
  establishes the retained inputs, the silence rule, the late-data cutoff and
  the mismatch rule.
- [FR-266](FR-266-event-time-window.md) supplies the half-open window that
  bounds progress and closure applicability.
- [FR-267](FR-267-observation-time-facts.md) types the time facts the authority
  is interpreted against.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) supplies the digest
  domain rule.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

1. FR-113 does not state the identity domain of the progress/assertion
   definition byte digest.
2. FR-113 names a "late-data cutoff" and requires a late event to be classified
   against it, but states neither the cutoff's own key nor the set of
   classifications a late event may receive. No closed vocabulary is enumerated
   here, because the prose names none.
3. FR-113 names "progress authority" as a retained input without stating what
   constitutes one or how two authorities are compared beyond the selection
   triple above.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
