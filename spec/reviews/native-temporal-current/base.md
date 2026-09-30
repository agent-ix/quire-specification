---
id: SR-300
title: "Base review of native temporal lifecycle correspondence"
type: SpecReview
analysis: base
scope: "FR-090 through FR-095, NFR-040, TM-006, TC-110 through TC-118, IT-040 and IT-041"
review_set: all
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-090, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-095, type: reviews }
---
## Summary

The E-owned native temporal family was reviewed against the integrated B result
contract and F observation drafts at `d104cf9`. The remediation at `2f347b8`
makes activation, runtime sample absence, progress monotonicity, decisive open-
scope settlement and native-to-TL result correspondence explicit and testable.
Native Quire remains the sole editable clause authority; TL is an internal
generated/evaluation target for the exact supported domain.

## Verdict

This verdict records the evaluated revision above. Later A/F integration
dispositions are in [the shared foundation](../../../proposals/quire-v1/shared-foundation.md#historical-integration-findings);
they supersede the open integration instructions below without changing this
review's historical observations.

**PASS for the E-owned contribution.** No E-owned blocking finding remains.
The composed profile is not ready for adoption until F preserves the revised
result dimensions through its consumer contract and A mints an exact successor
temporal definition/profile revision with refreshed digests. Those are existing
owner boundaries, not permission for E to edit their artifacts.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Resolved: an untriggered scope was always inactive, even when the trigger scope was open or required trigger evidence was missing. It now becomes inactive only after exact closed-complete trigger scope; other cases remain activation-unknown with their execution/completeness state. | FR-093-AC-5; TC-116 | wrong-requirement |
| FND-002 | high | Resolved for E: decisive temporal truth now carries independent decision-scope and surrounding-execution progress/closure, settlement basis and exact decision support through the shared result and TL correspondence records. | FR-091-AC-4; FR-094-AC-3/7; FR-095-AC-7 | missing-requirement |
| FND-003 | medium | Resolved: an absent runtime sample under a valid fixed-sample binding is incomplete, while a malformed or missing clock definition/binding refuses before evaluation. | FR-090; FR-095-AC-6; TC-113 | wrong-requirement |
| FND-004 | medium | Resolved: same-binding progress regression and same-revision conflict now produce a typed contradiction/refusal and cannot roll back prior state. | FR-094-AC-8; TC-115 | correct-requirement-no-evidence |
| FND-005 | medium | Resolved: the exhaustive correspondence claim now has a closed formula/interval/trace/anchor/lifecycle population and requires population identity/cardinality or a truthful resource stop. | FR-095-AC-1; TC-117 | correct-requirement-no-evidence |
| FND-006 | high | Open integration gate: F's consumer contract and FR-115/116 still omit decision-scope versus surrounding-execution closure, settlement basis, exact decision support and supersession. | observation-output-mapping-contract.md; FR-115; FR-116; TC-145/146; IT-051 | missing-requirement |
| FND-007 | low | Open integration gate: temporal definition `1-draft.2` is an immutable earlier candidate and retains the pre-correction FR-061 note. A must mint a successor revision and refresh every dependency/file digest rather than silently editing it. | definitions/temporal-common.md; definitions/README.md; checksums.sha256 | correct-requirement-no-evidence |

## Checklist results

| Check | Result |
| --- | --- |
| Source authority | Every temporal artifact keeps native Quire as the only authored language and TL as generated internal output. |
| Semantic profiles | Event-position, fixed-sample and timestamped-window clocks, origins, closure rules and unsupported mappings remain distinct. |
| Lifecycle/result agreement | Assessment execution, activation, both closure axes, truth, settlement basis, exact decision support, completeness and supersession cannot substitute for one another. |
| Test coverage | TM-006 truthfully maps every E criterion to planned cases; new lifecycle, monotonicity and finite-census mutations are explicit. |
| Completion honesty | All cases remain planned; no parser, evaluator, bridge, qualification or adoption claim is made. |
| Structural validation | Quire 0.31.0 reports 213/213 documents grammar-clean and zero findings after remediation. |

## Review set

| Analysis | Artifact |
| --- | --- |
| base | [base](base.md) |
| failure-domain | [failure-domain](failure-domain.md) |
| integrity | [integrity](integrity.md) |
| dependency | [dependency](dependency.md) |
| evidence | [evidence](evidence.md) |
| risk-complexity | [risk-complexity](risk-complexity.md) |
| scope-boundary | [scope-boundary](scope-boundary.md) |
| ears-conformance | [ears-conformance](ears-conformance.md) |
