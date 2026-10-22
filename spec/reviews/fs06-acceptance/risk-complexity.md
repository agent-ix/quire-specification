---
id: SR-556
title: "Risk and complexity review of FS06 output mapping"
type: SpecReview
analysis: risk-complexity
scope: "common loss/resource behavior and semantic volatility in the three selected target profiles"
review_set: subset
review_date: "2026-09-14"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-121, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-122, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-123, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-124, type: reviews }
  - { target: ix://agent-ix/quire-specification/NFR-060, type: reviews }
---
# Risk and complexity review of FS06 output mapping

## Summary

PASS. The profile boundaries isolate the highest-volatility target semantics and
turn uncertainty into explicit conditional/unrepresented/refused records rather
than broad implementation promises.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-017 | high | Resolved: OCL null/invalid, one-based indexing, collection kinds and wider integer semantics could alter native truth. The profile admits total exact cases only and explicitly classifies the rest. | FR-122; OCL profile; TC-151 |
| FND-018 | high | Resolved: SysML/KerML could blur where model declarations come from or mechanically split implication into assumption/requirement. Both require an exact selected structural correspondence. | FR-123; SysML/KerML profile; TC-152 |
| FND-019 | high | Resolved: FRET timing phrases could erase clock units, interval endpoints or finite-window completeness. Every temporal mapping is conditional on exact retained clock/capture/completeness facts; unsupported intervals have no substitute. | FR-124; FRETish profile; TC-153 |
| FND-020 | medium | Resolved: aggregate expansion and cancellation could expose partial or smaller output. NFR-060 requires checked arithmetic, admission/reservation and no partial package. | NFR-060; FR-125; TC-154 |

## Volatility allocation

- Common identities, loss states and package invariants are stable shared rules.
- Target syntax/correspondence lives only in its versioned profile and mapper.
- Parser/library evolution is isolated in replaceable structural observers.
- Broader collections, model features, temporal bounds and protocol mappings
  require new reviewed profile revisions instead of widening `1-draft.1` silently.
