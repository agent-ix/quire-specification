---
id: SR-011
title: "integrity review of the verification reference amendment"
type: SpecReview
analysis: integrity
scope: "FR-017, FR-018, IT-003, TC-001–004, TM-001 and proposals/shared-reference-2-draft"
review_set: all
review_date: "2026-09-08"
---

## Summary

The schema now represents the selected JCS domain and every role requested by B. The new version avoids silently widening the adopted closed schema.

## Verdict

PASS for the scoped specification and producer-qualification design. Independent
consumer adoption, execution and full-standard acceptance are not claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No unresolved consistency or atomicity finding in the amended scope. | schema.json; FR-017; FR-018 |

## Provenance

Actual QUOIN 0.20.0 specify/spec-matrix/spec-review skills and the retained
owner-selected base plus seven analyses apply. Agent A performed these reviews
serially; no independent reviewer or new agent acceptance is implied.

## Traceability

| User need | Requirements | Stakeholder | Verification |
| --- | --- | --- | --- |
| US-004 historical correspondence | FR-017 exact roles | StR-001 | TC-001/002; IT-003 |
| US-004 canonical correspondence | FR-018 selected JCS | StR-001 | TC-003/004; IT-003 |

NFR-001/002/003 are explicitly referenced for immutable meaning, incomplete
outcomes and exact bindings. Their older method-label findings are not declared
discharged by this amendment.

## Consistency probes

Closed version 2 envelope plus version 3 artifact is an explicit breaking
selection, not a fallback for an unsupported old kind. Required features are
decoded sets, while arrays in canonical artifact input preserve order.
Domain/version/algorithm select the digest syntax as one tuple; an IR domain
cannot accidentally admit the JCS-prefixed alternative.

The new canonical domain preserves B's inspected raw SHA-256-over-JCS preimage.
Its textual prefix is not hashed. The synthetic core fixture is explicitly
separate from B's actual report schema, preventing A from authoring a second
portable-report authority or inventing a human review.

The integration test pins a real public Rust API and clean checkout; missing
B code is a setup failure. There is no API pagination, authenticated runtime
call, registry precedence or interactive generator fallback in this scope.

