---
id: SR-014
title: "risk-complexity review of the verification reference amendment"
type: SpecReview
analysis: risk-complexity
scope: "FR-017, FR-018, IT-003, TC-001–004, TM-001 and proposals/shared-reference-2-draft"
review_set: all
evaluated_revision: "9244dcc96c8f0d68ae4213d7d2e6bdebe4d8311b"
review_date: "2026-09-08"
---

## Summary

The highest risks are identity confusion and cross-repository contract drift. A narrow versioned amendment with fixed vectors contains both.

## Verdict

PASS for the scoped specification and producer-qualification design. Independent
consumer adoption, execution and full-standard acceptance are not claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No unmitigated risk blocks producer qualification of this amendment. | TC-002; TC-003; TC-004 |

## Provenance

Actual QUOIN 0.20.0 specify/spec-matrix/spec-review skills and the retained
owner-selected base plus seven analyses apply. Agent A performed these reviews
serially; no independent reviewer or new agent acceptance is implied.

## Scoped risk register

| Requirement | Technical risk | Volatility | Driver and mitigation |
| --- | --- | --- | --- |
| StR-001 | High | Medium | False assurance; keep producer/consumer/outcome claims separate |
| FR-014 | High | Medium | Untrusted wire; strict refusal and version mutations |
| FR-015 | High | Low | Cryptographic identity; preserve established preimages |
| FR-017 | High | Medium | Wrong-role joins; enumerate six roles and substitutions |
| FR-018 | High | Medium | JCS/IR confusion; golden vectors and actual B API |
| NFR-001 | Medium | Low | Historical drift; exact e897f81 byte comparison |
| NFR-002 | Medium | Low | Unbounded inputs; finite file/depth ceilings |
| NFR-003 | High | Low | Stale/content conflict; independent identity-axis mutations |

## Top hazards

A matching byte digest can hide the wrong role: TC-002 separates both checks.
A shared JSON label can hide different canonical preimages: TC-003 preserves
the exact B implementation correspondence and old IR forms.
A disposition can create a circular identity: TC-004 hashes only the validated
core and checks a changed core/wrapper independently.
See the failure-domain review for the corresponding refusal and purity rules.

B's independent contract adoption is a real remaining gate, but does not prevent
A from delivering the schema and vectors. No new concurrency, backend or parser
technology is required to do so.

