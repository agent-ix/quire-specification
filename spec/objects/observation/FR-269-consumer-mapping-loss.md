---
id: FR-269
title: "Consumer mapping-loss classification"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-116
    type: specifies
---
# FR-269: Consumer mapping-loss classification

## Description

When a selected consumer cannot represent a required fact, the consumer adapter SHALL classify the mapped obligation as conditional, unrepresented or refused rather than as preserved.

FR-116 Behavior states the closed set: "The adapter SHALL classify each mapped
obligation as preserved, conditional, unrepresented, or refused; incomplete,
pending, and refused outcomes are not Boolean success."

## Values

| Value | Description |
| --- | --- |
| `preserved` | The required fact is represented in the selected consumer. FR-116 Behavior: "missing representation SHALL NOT be classified as preserved." |
| `conditional` | The mapping holds only under a retained condition. One of the three classifications a mapping loss must take when the consumer cannot represent a required fact (FR-116 Behavior). |
| `unrepresented` | The source fact is retained and the consumer does not represent it. FR-116-AC-2: "An unsupported or lossy mapping is visible as conditional, unrepresented, or refused." |
| `refused` | The mapping is refused. FR-116 Behavior: "incomplete, pending, and refused outcomes are not Boolean success", and FR-116-AC-3: "A consumer cannot convert pending, incomplete, or refused into a passed Boolean result." |

The set is closed by FR-116 Behavior. No member is added here, and `preserved`
is not reachable by omission.

## Identity and comparison

- **Identity kind** — *value*. A classification label has no identity of its
  own; it is compared componentwise as an exact label.
- **Key** — the label itself. Matching is exact. Equality is componentwise over
  the label and nothing else; display name, source path, span, message text,
  locale and rendering never participate, and FR-116-AC-4 requires that
  "Parsing, serializing, or accepting emitted target text does not alter a
  mapping outcome or establish semantic preservation."
- **Key order** — lexicographic over the label, for collection comparison only.
  It carries no precedence, no severity ranking and no causal meaning; in
  particular `conditional` is not "closer to" `preserved` than `unrepresented`
  is, because no requirement ranks them.
- **Causal order** — none. A classification records the outcome of one mapping
  and participates in no causal structure.
- **Absence** — a mapped obligation with no classification is explicitly absent
  a classification, and absence compares only to absence. It is never defaulted
  to `preserved`: "missing representation SHALL NOT be classified as preserved"
  (FR-116 Behavior). An incomplete, pending or refused outcome is likewise never
  promoted to a Boolean success (FR-116 Behavior, FR-116-AC-3).
- **Digest domain** — not applicable; a classification label carries no digest.
  The definition and profile identity/revision/digest selections it is emitted
  beside are retained under FR-116 Behavior, which requires the adapter to
  "refuse definition/profile identity, revision, or digest drift rather than
  emit a mapping under a current default"; those digests carry domains selected
  under `ix://agent-ix/quire-specification/FR-201`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-269-AC-1 | Each member's value equals the exact label named by FR-116 Behavior, and no fifth member exists. | Inspection |
| FR-269-AC-2 | A required fact the selected consumer cannot represent does not classify as `preserved`. | Analysis |
| FR-269-AC-3 | A mapped obligation with an absent classification compares only to absence and receives no default member. | Analysis |
| FR-269-AC-4 | Accepting or round-tripping emitted target text does not change a mapping's classification. | Analysis |

## Dependencies

- [FR-116](../../functional/observation/FR-116-map-observation-results-to-consumers.md)
  establishes the closed classification set and the no-promotion rules.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) supplies the digest
  domain rule for the selections emitted beside a classification.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [Observation result consumer and output-mapping contract](../../../proposals/quire-v1/observation-output-mapping-contract.md)
  supplies the consumer mapping context.

## Open gap

1. **The consumer itself is not typed here, because no requirement establishes
   its identity.** FR-116 requires the adapter to retain "the requested subject,
   capability, immutable source/binding/progress/definition/profile
   identity-revision-digest selections" and to deliver to "an executable or
   exported consumer", but states no identity, key or order for a consumer or
   for a consumer mapping. Only the loss classification is typed.

2. **Bare "adequacy" has no stated referent.** FR-061 establishes two disjoint
   adequacy domains and forbids crossing them: observation adequacy states
   whether the exact observations required "are `adequate`, `inadequate`,
   `unknown` or `not-assessed`"; protocol adequacy states whether the exact
   declared obligation and case set is "`demonstrated`, `not-demonstrated`,
   `inapplicable` or `unknown`"; and "The result producer SHALL NOT use either
   adequacy domain as evidence for the other."

   FR-115 Description then lists, unqualified, "activation, participation,
   completeness, adequacy, claim strength, provenance" as separate retained
   facts, and FR-115-AC-4 repeats the bare word among the independently readable
   facts. FR-116 omits an adequacy axis from its retained-fact list entirely.

   Which of FR-061's two disjoint domains FR-115's bare "adequacy" denotes, and
   whether FR-116's omission is deliberate, is not stated by any requirement.
   This artifact records the contradiction and resolves neither side. The
   adequacy vocabularies themselves are owned by the result object family, not
   by this one.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
