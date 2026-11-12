---
id: SR-202611101500006
title: "EARS conformance review of QSL-1876 observation binding derivation"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification; spec/objects/observation/FR-261-observation-binding.md (PR #648 diff)"
review_set: subset
---
## Summary

Ticket: QSL-1876. EARS review of the new normative text and FR-261-AC-4. The
derivation and its "SHALL NOT accept caller attribution" are in body prose, not
in FR-261's requirement statement. That statement covers only binding-selection
definition identity, so AC-4 goes beyond it. AC-4 also bundles three claims.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-261's Description (its only EARS statement) covers only a changed binding-contract definition identity; the new derivation and the consumer "SHALL NOT accept caller attribution" are in an unnumbered prose section about admitted records (FR-260's entity), so AC-4 verifies behaviour no requirement statement makes normative | spec/objects/observation/FR-261-observation-binding.md:14,42-44,113 |
| FND-002 | low | FR-261-AC-4 is compound: same node/instance grouping, progress-order occurrence ranking and the no-other-field rule are three separately failing claims in one AC | spec/objects/observation/FR-261-observation-binding.md:113 |

## Verdict

FAIL on FND-001: add an EARS statement for the derivation (in FR-261 or in FR-260,
where admitted records live) and trace AC-4 to it. Split AC-4.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-260 Description now carries a "When a consumer attributes ... SHALL derive ... SHALL NOT accept caller attribution" statement; FR-261-AC-4 is removed and FR-260-AC-5..9 trace to it. |
| FND-002 | fixed | The compound AC is split into FR-260-AC-5 (node), AC-6 (instance), AC-7 (occurrence), AC-8 (field independence) and AC-9 (caller attribution refused). |
