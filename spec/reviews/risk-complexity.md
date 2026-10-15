---
id: SR-006
title: "risk-complexity review of quire-specification"
type: SpecReview
analysis: risk-complexity
scope: "spec/spec.md and indexed requirements"
review_set: all
review_date: "2026-09-07"
---

## Summary

Every FR/StR/NFR has a technical-risk and volatility assessment with concrete mitigations. The principal risks are identity/qualification boundaries and first implementation of reference behavior, rather than missing infrastructure being mistaken for a regression.

## Verdict

**CONDITIONAL** — Findings require disposition before the affected implementation or acceptance gate. This is not owner acceptance.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | High-risk or volatile shared identity, runtime validation and consumer qualification work needs bounded acceptance slices and the named mitigations before task generation. | Risk register and top hazards below; [dependency](dependency.md); [evidence](evidence.md) |

## Scope and provenance

Reviewed quire-specification. The owner selected base plus all seven Quoin analyses, and declined the optional intent↔test↔code semantic step in `/gap-analysis`. Ordinary requirements consistency, EARS conformance, and the separate current-code review remain in scope. The assignment prohibits spawning additional agents, so these analyses were performed sequentially by Agent A; no independent reviewer acceptance is implied.

The installed Quoin 0.20.0 `spec-review/SKILL.md` and its analysis skills govern these artifacts. The SpecReview authoring pack was fetched once for this repository and its process skeleton/schema was used. [Provenance](data/provenance.json), [coverage output](data/coverage.json), [advisor output](data/advice.json), and [actual method catalog](data/verification-methods.json) preserve the deterministic inputs. IDs in this report are local to this repository unless qualified.

These are new repositories. Missing formal plans, TC records, suites, and matrices are workflow setup/readiness debt. Unimplemented LC02–LC05 stages and unqualified shared consumers are known remaining work; they are not reported as regressions or hidden stubs. No completion status is fabricated.

## Risk register

| Requirement | Technical risk | Volatility | Drivers | Mitigation |
| --- | --- | --- | --- | --- |
| [FR-001](../functional/foundation/FR-001-select-native-profile.md) | medium | medium | Definition selection must not silently extend the historical digest. | Retain golden profile bytes; qualify a separately accepted amendment. |
| [FR-002](../functional/foundation/FR-002-resolve-owned-declarations.md) | high | high | Identity authority and existing typed-model adapter are external trust boundaries. | Pin exact model/lock/closure contracts; require independent consumer fixtures. |
| [FR-003](../functional/foundation/FR-003-bind-source-correspondence.md) | high | medium | Byte/scalar source correspondence controls evidence attribution. | Generate Unicode, segment and digest adversarial cases. |
| [FR-004](../functional/foundation/FR-004-separate-artifact-identities.md) | high | high | Artifact and semantic identities can be conflated across owners. | Closed refs, collision controls and B/C contract adoption. |
| [FR-005](../functional/foundation/FR-005-guard-optional-values.md) | high | medium | New guarded-definedness implementation across observations. | Independent snapshot-qualified positive/adverse examples. |
| [FR-006](../functional/foundation/FR-006-preserve-control-evaluation.md) | high | high | Control activation, sharing and interruption affect future evidence. | Pin event tuples and interrupted cases before evaluator implementation. |
| [FR-007](../functional/foundation/FR-007-prove-integer-definedness.md) | high | medium | Bounded arithmetic must establish definedness without hidden overflow. | Generate extrema and invalid divisors; independent expected results. |
| [FR-008](../functional/foundation/FR-008-preserve-sequence-semantics.md) | medium | medium | Order/multiplicity differ from set semantics. | Duplicate/reordering examples and generated sequences. |
| [FR-009](../functional/foundation/FR-009-preserve-object-identity.md) | high | medium | Object identity is an evidence trust boundary. | Cross-universe/type/observation collision cases. |
| [FR-010](../functional/foundation/FR-010-evaluate-exact-reachability.md) | high | medium | New exact graph traversal plus resource incompleteness. | Self-loop, cycle, disconnected and exhausted-budget cases. |
| [FR-011](../functional/foundation/FR-011-validate-populations.md) | high | high | Unvalidated or incomplete population can masquerade as a valid context. | Validate closure before evaluation; inject partial validation failures. |
| [FR-012](../functional/foundation/FR-012-bind-operation-observations.md) | high | high | Invocation/pre/post/immutable parameter correspondence crosses artifacts. | Independent pre/post/frame fixtures and conflicting-invocation controls. |
| [FR-013](../functional/foundation/FR-013-validate-operation-frames.md) | high | medium | Frame permission and object-delta mistakes admit invalid transitions. | Creation/deletion/update boundary and wrong-invocation cases. |
| [FR-014](../functional/foundation/FR-014-read-exact-wire.md) | high | high | New closed wire joins multiple independent consumers. | Raw-JSON fuzzing, schema checks and independently written readers. |
| [FR-015](../functional/foundation/FR-015-preserve-canonical-domains.md) | high | high | Digest domain change could reinterpret historical evidence. | Keep legacy canonical tests/corpus and explicit domain tags. |
| [FR-016](../functional/foundation/FR-016-migrate-explicitly.md) | high | high | Mapping scope could overgrant semantic reuse. | Byte-only migration controls; refuse implicit reverse/transitive joins. |
| [NFR-001](../non-functional/NFR-001-preserve-versioned-meaning.md) | high | medium | Compatibility and historical interpretation are evidence-sensitive. | Immutable reviewed golden fixtures and mutation controls. |
| [NFR-002](../non-functional/NFR-002-classify-resource-incompleteness.md) | high | medium | Every new phase must propagate incomplete work correctly. | Budget/cancellation fault injection in each implemented phase. |
| [NFR-003](../non-functional/NFR-003-qualify-reference-bindings.md) | high | high | Accepted authority/content binding is security-sensitive. | Independent negative joins with recomputed digests. |
| [NFR-004](../non-functional/NFR-004-retain-review-and-rights.md) | high | high | Standard artifact terms and promotion policy remain unsettled. | Keep private; require included-artifact rights/acceptance records. |
| [StR-001](../stakeholder/StR-001-native-assessment-trust.md) | high | high | Full native workflow spans unsettled shared interfaces. | Retain a real healthy/violating/refused acceptance slice and exact evidence bindings. |

## Top hazards

1. FR-002/FR-004: selecting or joining the wrong authority/content.
2. FR-006/FR-012: losing invocation or interrupted activation meaning.
3. FR-011/NFR-002: accepting partial validation or exhausted work as a logical result.
4. FR-014–FR-016: wire or migration adoption changing historical interpretation.
5. NFR-004: publishing artifacts before their grant and acceptance are settled.

## Failure-domain cross-check

The register uses the trust, identity, purity and topology boundaries in [failure-domain](failure-domain.md). A high technical-risk score is a planning priority, not a high-severity defect or a claim of a failed test. Stage new technology behind a reviewed bounded slice; keep volatile external contracts isolated in their owning adapters. No concurrency implementation currently warrants Loom adoption.
