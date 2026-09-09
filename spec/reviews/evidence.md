---
id: SR-005
title: "evidence review of quire-specification"
type: SpecReview
analysis: evidence
scope: "spec/spec.md and indexed requirements"
review_set: all
evaluated_revision: "1a49525691f35bf2e8802e9ccce4b27544d4c77e"
review_date: "2026-09-07"
---

## Summary

The mandatory advisor and catalog commands ran successfully. Uncatalogued NFR labels require disposition, and the owner-requested reviewer supplement recommends risk-appropriate methods beyond the current advisor rules.

## Verdict

**CONDITIONAL** — Findings require disposition before the affected implementation or acceptance gate. This is not owner acceptance.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | NFR-001-M-1: authored `Byte comparison` is uncatalogued. Advisor: `performance-benchmarking`. Reviewer proposes `golden-approval-testing`; Compare selected historical bytes; latency is not the obligation. | NFR-001-M-1; [raw advice](data/advice.json) |
| FND-002 | medium | NFR-001-M-2: authored `Adverse consumer tests` is uncatalogued. Advisor: `performance-benchmarking`. Reviewer proposes `contract-testing`; Independent readers reject an unselected cross-version mapping. | NFR-001-M-2; [raw advice](data/advice.json) |
| FND-003 | medium | NFR-002-M-1: authored `Budget-boundary tests` is uncatalogued. Advisor: `performance-benchmarking`. Reviewer proposes `fault-injection`; Exhaust each phase budget and assert incomplete with no Boolean. | NFR-002-M-1; [raw advice](data/advice.json) |
| FND-004 | medium | NFR-002-M-2: authored `Validation-budget tests` is uncatalogued. Advisor: `performance-benchmarking`. Reviewer proposes `negative-abuse-testing`; Interrupt population validation and reject partial acceptance. | NFR-002-M-2; [raw advice](data/advice.json) |
| FND-005 | medium | NFR-003-M-1: authored `Digest mutation tests` is uncatalogued. Advisor: `performance-benchmarking`. Reviewer proposes `negative-abuse-testing`; Mutate bytes under a stale digest and assert refusal. | NFR-003-M-1; [raw advice](data/advice.json) |
| FND-006 | medium | NFR-003-M-2: authored `Independent reference-join tests` is uncatalogued. Advisor: `performance-benchmarking`. Reviewer proposes `contract-testing`; Independent joiner rejects immutable-key content conflicts after digest recomputation. | NFR-003-M-2; [raw advice](data/advice.json) |
| FND-007 | medium | NFR-004-M-1: authored `Rights inventory inspection` is uncatalogued. Advisor: `performance-benchmarking`. Reviewer proposes `inspection`; Review an included-artifact rights inventory and promotion record. | NFR-004-M-1; [raw advice](data/advice.json) |
| FND-008 | medium | NFR-004-M-2: authored `Requirement and issue audit` is uncatalogued. Advisor: `performance-benchmarking`. Reviewer proposes `inspection`; Audit review/implementation status on the owning requirements and tickets. | NFR-004-M-2; [raw advice](data/advice.json) |

## Scope and provenance

Reviewed `quire-specification@1a49525691f35bf2e8802e9ccce4b27544d4c77e`. The owner selected base plus all seven Quoin analyses, and declined the optional intent↔test↔code semantic step in `/gap-analysis`. Ordinary requirements consistency, EARS conformance, and the separate current-code review remain in scope. The assignment prohibits spawning additional agents, so these analyses were performed sequentially by Agent A; no independent reviewer acceptance is implied.

The installed Quoin 0.20.0 `spec-review/SKILL.md` and its analysis skills govern these artifacts. The SpecReview authoring pack was fetched once for this repository and its process skeleton/schema was used. [Provenance](data/provenance.json), [coverage output](data/coverage.json), [advisor output](data/advice.json), and [actual method catalog](data/verification-methods.json) preserve the deterministic inputs. IDs in this report are local to this repository unless qualified.

These are new repositories. Missing formal plans, TC records, suites, and matrices are workflow setup/readiness debt. Unimplemented LC02–LC05 stages and unqualified shared consumers are known remaining work; they are not reported as regressions or hidden stubs. No completion status is fabricated.

## Deterministic advisor result

The actual `quoin advise --repo <scope> --json` run returned 85 obligations: 77 FR acceptance criteria and 8 NFR measurement rows. It reported 0 method-class mismatches, 8 uncatalogued authored labels, and 0 inconclusive rows. All FR `Test` cells match a supported method class; this does not select a concrete suite or prove a criterion.

Every NFR metric received `performance-benchmarking` from the `quantified-threshold` rule. A zero rights violation, zero Boolean after exhaustion, or zero changed historical bytes is not by itself a latency/throughput claim. The owner specifically asked for reviewer judgment beyond the unfinished advisor. The proposed alternatives below are therefore labeled reviewer recommendations, including when the rule returned a recommendation rather than `inconclusive`. No catalog or installed advisor code was changed.

Coverage counts 79 criteria targets, including two stakeholder validation criteria, and 77 unbacked FR verification rows. NFR measurement rows are already represented as obligations; they are not absent merely because the optional NFR-AC section matched nothing. No false-completion status was found.

## Proposed method dispositions

| Obligation | Catalog method | Reviewer rationale | Disposition |
| --- | --- | --- | --- |
| NFR-001-M-1 | golden-approval-testing | Compare selected historical bytes; latency is not the obligation. | Proposed review disposition; not recorded as human-confirmed discharge |
| NFR-001-M-2 | contract-testing | Independent readers reject an unselected cross-version mapping. | Proposed review disposition; not recorded as human-confirmed discharge |
| NFR-002-M-1 | fault-injection | Exhaust each phase budget and assert incomplete with no Boolean. | Proposed review disposition; not recorded as human-confirmed discharge |
| NFR-002-M-2 | negative-abuse-testing | Interrupt population validation and reject partial acceptance. | Proposed review disposition; not recorded as human-confirmed discharge |
| NFR-003-M-1 | negative-abuse-testing | Mutate bytes under a stale digest and assert refusal. | Proposed review disposition; not recorded as human-confirmed discharge |
| NFR-003-M-2 | contract-testing | Independent joiner rejects immutable-key content conflicts after digest recomputation. | Proposed review disposition; not recorded as human-confirmed discharge |
| NFR-004-M-1 | inspection | Review an included-artifact rights inventory and promotion record. | Proposed review disposition; not recorded as human-confirmed discharge |
| NFR-004-M-2 | inspection | Audit review/implementation status on the owning requirements and tickets. | Proposed review disposition; not recorded as human-confirmed discharge |

## Suites warranted by the risks

| Suite scope | Requirements | Catalog methods | Recommended evidence |
| --- | --- | --- | --- |
| Native syntax / source contract | FR-001, FR-003 | property-based-testing; fuzzing; grammar-based-fuzzing | Generated valid source, malformed UTF-8/tokens/delimiters, Unicode byte boundaries, exact map coverage, format token preservation and idempotence. Deterministic seeds/minimized regressions become durable tests. |
| Identity and wire consumers | FR-002–FR-004, FR-014–FR-016 | contract-testing; golden-approval-testing; negative-abuse-testing | Independent producer and consumer fixtures, decoded duplicate keys, changed bytes with stale and recomputed digests, reordered feature sets and unknown versions. Native production readers remain future work. |
| Suite discrimination | FR-003, FR-004, FR-014 | mutation-testing | After tests are bound, remove a range/digest/identity guard or alter token emission and measure whether the suite fails. Current absence of advisor fault-detection advice is not proof this is unnecessary. |
| Failure preservation | FR-010–FR-013, NFR-002 | fault-injection; negative-abuse-testing | Inject budget exhaustion, cancellation, failed I/O and partial-stage output; assert refusal/incompleteness, retained observed events, and no fabricated Boolean. Include actual CLI exit 2/3 cases now. |
| Finite reference behavior | FR-005–FR-013 | bdd-spec-by-example; property-based-testing; metamorphic-testing | Reviewed independent examples and generated finite populations, cycles and boundaries. Reference/backend comparison is qualified only for supported fragments; both agreeing is not an independent semantic oracle. |
| Concurrency, only if introduced | Future shared caches, atomic cancellation or concurrent evaluators | deterministic-simulation; model-checking | Consider Loom when application-owned shared state/synchronization exists. Current Arc-backed immutable source and a joined stack-limit test do not justify adopting a concurrency harness. |

## Tool fit and limits

These are method recommendations, not claims that new harnesses ran or dependencies were installed. [Proptest](https://github.com/proptest-rs/proptest) supports generated cases with shrinking; [cargo-fuzz](https://rust-fuzz.github.io/book/cargo-fuzz/tutorial.html) supplies Rust fuzz targets; [Loom](https://github.com/tokio-rs/loom) explores concurrent Rust executions. Whether a tool fits the repository is reviewer judgment. Select versions, toolchain lanes, budgets and dependency grants when specifying the actual suite. Start with boundary tests, generation and mutation where they answer a concrete question; solver/concolic escalation needs evidence that cheaper methods have stalled.

No `spec/evidence/suites.md`, TC artifacts or matrix currently binds these proposed suites. That is a planning gap in a new repository. Confirm method dispositions in the owning NFR Method cells during remediation, then author the suite/matrix and trace actual tests; do not create a parallel method authority or mark an inspection discharged by a source tag.
