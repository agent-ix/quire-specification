---
id: SR-002
title: "failure-domain review of quire-specification"
type: SpecReview
analysis: failure-domain
scope: "spec/spec.md and indexed requirements"
review_set: all
review_date: "2026-09-07"
---

## Summary

Trust boundaries, entity keys, purity, and graph/resource failure handling were reviewed. The standard explicitly distinguishes static refusal, missing observation and work incompleteness; the current native CLI/formatter findings are recorded only in the language scope.

## Verdict

**PASS** — No additional findings within this analysis scope. This is not owner acceptance.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No additional failure-policy omission found within the reviewed standard boundary. Selected authorities and independent-consumer behavior remain unqualified dependencies, not assumptions of success. | FR-002–FR-016; IT-002 |

## Scope and provenance

Reviewed quire-specification. The owner selected base plus all seven Quoin analyses, and declined the optional intent↔test↔code semantic step in `/gap-analysis`. Ordinary requirements consistency, EARS conformance, and the separate current-code review remain in scope. The assignment prohibits spawning additional agents, so these analyses were performed sequentially by Agent A; no independent reviewer acceptance is implied.

The installed Quoin 0.20.0 `spec-review/SKILL.md` and its analysis skills govern these artifacts. The SpecReview authoring pack was fetched once for this repository and its process skeleton/schema was used. Provenance, coverage output, advisor output, and actual method catalog preserve the deterministic inputs. IDs in this report are local to this repository unless qualified.

These are new repositories. Missing formal plans, TC records, suites, and matrices are workflow setup/readiness debt. Unimplemented LC02–LC05 stages and unqualified shared consumers are known remaining work; they are not reported as regressions or hidden stubs. No completion status is fabricated.

## Boundary analysis

| Boundary | Policy and examined edge cases | Requirements |
| --- | --- | --- |
| External model / typed adapter | Strict refusal on missing, foreign, stale or ambiguous static bindings; no partially linked package. A owns native linking; C coordinates existing consumers. | FR-002 |
| Entity and evidence identity | Artifact key includes kind, authority, identity and structured revision; byte digest and semantic definition/feature selection are checked separately. Local syntax labels do not grant portable identity. | FR-004 |
| User predicates | Read-only expressions over immutable populations, snapshot-qualified optional facts and invocation parameters; no callback side effects or hidden domain mutation admitted. | FR-005, FR-006, FR-012 |
| Cycles, depth and unavailable data | Reachability tests the target before visited suppression; cycles terminate, disconnected nodes do not invent reachability, work exhaustion remains incomplete. Missing observations are not empty populations. | FR-010, FR-011, NFR-002 |
| Backend / extraction | Unsupported obligations remain addressable. Extraction provides opaque bodies and correspondence; the native compiler owns parsing and lowering uses the existing binder. | FR-003; IT-002 |

## Required follow-through

Use generated identity collisions, stale bytes with recomputed digests, cyclic/disconnected graph cases, exact and exceeded budgets, and interrupted evaluation to exercise these policies. Existing example packets supply candidates, not an independent oracle. The optional gap semantic comparison was not run.
