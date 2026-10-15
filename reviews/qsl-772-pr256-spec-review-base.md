---
id: SR-11174
title: "base review of quire-specification PR 256 (Decimal type-link operands in FR-323 and TC-278)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@56199f93d1674ccaa468eef27e14832a99b6e082; spec/objects/interfaces/FR-323-native-runtime-envelope.md (item 6, Qualification, AC-15, AC-16), spec/test-cases/TC-278-replay-envelope-members.md (RE-29)"
review_set: subset
---

## Summary

Ticket: QSL-772. Base checklist over the three edited units, plus a grep of
the whole spec tree at the head for every other place that counts or names
type-link operands ("4 Integer", "8 Rational", "type-link", "type-comparison").

Count: the only operand counts are FR-323 item 6 (now 4/8/4), FR-323-AC-16
(now "4 Integer, 8 Rational, 4 Decimal") and TC-278 RE-29 (now lists the
Decimal pair). FR-323-AC-15's "3 Integer, 6 Rational" counts value-in-domain
operands, a different rule, and correctly has no Decimal entry. The other hits
are in an older review file. No enumeration was missed.

RE-29: the Decimal row is producible. A Decimal pair with each coefficient in
turn at B and at B+1 matches what the code checks, and "the other three within
B" carries over through "likewise".

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The Qualification paragraph after item 8 still says a B check "does not qualify Decimal", although item 6 and AC-16 now make Decimal type-link coefficients B-bounded operands. A reader cannot tell whether Decimal type-link comparison is covered by B. The sentence should say it does not qualify Decimal value-in-domain membership. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:261-264 |

## Verdict

The count, the operand list and the RE-29 vector are consistent across FR-323
and TC-278. One sentence in the Qualification paragraph now contradicts item 6.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 4001c8ea0dd1b81994ed4d031eb04f30623c0986 |
