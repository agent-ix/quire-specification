---
id: FR-281
title: "Claim strength"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/engineering-assurance/FR-022
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-115
    type: specifies
---
# FR-281: Claim strength

## Description

When a result retains the strength of a claim, the result producer SHALL carry
exactly one distinct source-bound value from the closed claim-strength
vocabulary owned by Engineering Assurance
([FR-022](ix://agent-ix/engineering-assurance/FR-022))
rather than collapsing strengths into one success fact.

Engineering Assurance FR-022
owns its members, their exact wire spellings, their meaning and the typed
refusal of any other label. This artifact states how a Quire result carries
that vocabulary: its source binding, its identity discipline and its
cardinality on an advanced result.

FR-061 states the retention requirement: "The result producer SHALL retain
each claim strength from the Engineering Assurance FR-022 vocabulary as a
distinct source-bound value."

## Properties

| Property | Meaning |
| --- | --- |
| `strength` | Exactly one Engineering Assurance FR-022 wire spelling, carried unchanged. The members, their spellings and their meaning are those of FR-022; a Quire result adds no member, alias, spelling or rendering of its own. |
| source binding | The source that established the strength. The strength is **source-bound** and does not transfer to another claim (FR-061). |

## Identity and comparison

- **Identity kind** — *value*. A strength label has no identity of its own and
  is compared componentwise as an exact string.
- **Key** — the wire spelling itself, matched exactly. Display name, source
  path, span, message text, locale and rendering never participate.
- **Key order** — lexicographic over the wire spelling, for collection
  comparison only. **It is not a strength ranking.** FR-061 requires these be
  retained as *distinct* values, not as points on a scale, and Engineering
  Assurance FR-022 declares the vocabulary unordered.
- **Causal order** — none.
- **Absence** — a claim retained without a strength has an absent strength,
  which compares only to absence and is never defaulted to `observed` or to any
  other member.
- **Digest domain** — not applicable; the source binding that establishes a
  strength may carry a digest, whose domain is selected under
  [FR-201](../foundation/FR-201-canonical-identity-domain.md).

## Cardinality and relation

FR-061 requires exactly one strength on every advanced result. There is no
ordering, dominance or automatic promotion among members. A result requiring
more than one independently supported strength is represented as independent
claims/results, not as a multi-valued strength.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-281-AC-1 | Each carried strength equals exactly one Engineering Assurance FR-022 wire spelling. | Test |
| FR-281-AC-2 | Two claims with different strengths remain distinct and neither is promoted to the other. | Test |
| FR-281-AC-3 | Key-order adjacency between two strengths does not establish that one dominates the other. | Test |
| FR-281-AC-4 | An advanced result with zero, multiple or unknown strengths is refused. | Test (TC-061) |

## Dependencies

- Engineering Assurance
  [FR-022](ix://agent-ix/engineering-assurance/FR-022) owns the claim-strength
  vocabulary, its wire spellings and the refusal of unknown labels.
- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md)
  requires the strengths be retained distinct and source-bound.
- [FR-115](../../functional/observation/FR-115-report-activation-participation-and-adequacy.md)
  retains claim strength as a separate fact.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Status

Proposed cardinality closure for `quire-specification#40`; implementation
evidence remains planned under TC-061.
