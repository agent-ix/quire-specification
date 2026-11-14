---
id: SR-202611141457002
title: "EARS conformance review of the STD-665 statistical.workers registration"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification; PR #654 at a5e9aa28, diff origin/main...HEAD: spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md (Inputs, Workers section, FR-408-AC-15..17), spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md (Behavior 5), spec/objects/interfaces/FR-331-backend-provider-envelope.md (request member list), spec/test-cases/TC-356-regenerative-long-run.md (WK-01..04); compared with QSL FR-191 and FR-255 on agent-ix/quire-spec-language main"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-408, type: references }
  - { target: ix://agent-ix/quire-specification/FR-461, type: references }
  - { target: ix://agent-ix/quire-specification/FR-331, type: references }
  - { target: ix://agent-ix/quire-specification/TC-356, type: references }
---

## Summary

Ticket: STD-665. EARS check of the new requirement statements in FR-408 "Workers" and the
FR-461 Behavior 5 sentence. The provider obligations (refuse 0; merge, charge and stop in merge
order; discard beyond the stop; equal results for every count; cancellation prefix) are well-formed
ubiquitous or event-driven statements with a named system and one response each. The MAY clause
is a permission, not a requirement. One statement is malformed.

## Verdict

PASS with one low finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | "The request SHALL carry `workers`, a positive integer ... defaulting to 1" puts the obligation on the request rather than the provider, and contradicts itself: "SHALL carry" makes the member required while the default and FR-408-AC-16 cover a request that omits it, and "a positive integer" excludes the 0 that the next bullet refuses | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:308-310 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The fix's "QSL FR-191 SHALL NOT contradict it" places a SHALL on another repository's document, with no system that can satisfy it and no criterion; the governing rule ("where the two texts differ this section governs the statistical settings") is enough on its own | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:313-315 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | 86ac612d: the sentence is now descriptive ("The request carries `workers`, a positive integer ... defaulting to 1 when the request names none") with no SHALL on the request; the 0 case is a separate provider obligation. |
