---
id: SR-14682
title: "gap analysis of STD-108 changed ACs against TC rows (QSpec PR #339)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@43c4295af9b7507a827bd7f1244513870a8bc674; spec/objects/protocol/FR-290-protocol-claim-kind.md, spec/test-cases/TC-271-capability-kind-candidate-negotiation.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-290
    type: reviews
---
# SR-14682: gap analysis of STD-108 (QSpec PR #339)

## Summary

Ticket: STD-108. Plan completion: not assessed. This is a manual mapping of
each changed AC to its TC rows. The PR is spec only. The TC-271 harness
assertion for CK-09 is listed as remaining work for the code lane.

- FR-290-AC-5 (changed) maps to TC-271 CK-09. CK-09 covers the
  `value-validity` claim at a scalar operation, the result bound taken from
  the application's own type and from an enclosing narrowing, the narrowing's
  absence of a claim, and FR-146 admission being independent of the claim.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-290-AC-5 now says `value-validity` is required "once per occurrence", but CK-09 has one occurrence in each function, so the rule is not exercised: a per-node implementation passes CK-09 just as a per-occurrence one does. Add a body with one application written twice (for example `(x + 1) * (x + 1)`: two claims at `+` and one at `*`). The termination-measure exclusion needs no vector: under FR-146 a measure element is only a parameter, `size(p)` of a collection parameter or a structural parameter, so no scalar operation application can occur in a measure. | spec/test-cases/TC-271-capability-kind-candidate-negotiation.md:45 |

## Verdict

The changed AC has a TC row, but its new per-occurrence clause has no
vector (FND-001).

## Resolution

FND-001: CK-09 gains `h` with body `(x + 1) * (x + 1)`, whose three claims (two at `+`, one at `*`) separate a per-occurrence count from a per-node count. The termination-measure exclusion needs no vector, as the finding states.
