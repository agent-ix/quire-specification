---
id: SR-404
title: "Evidence-method review of the composed native Quire v1 baseline"
type: SpecReview
analysis: evidence
scope: "FR-020, FR-030–062, FR-090–095, FR-110–116, FR-130; StR-002; NFR-010/020/021/040/050–052; TM-003/004/006/007 and their TC/IT contracts; proposals/quire-v1"
review_set: all
evaluated_revision: "788b161bd1b7fd5d632cc42c949f497914df3ce6"
review_date: "2026-09-10"
---

## Summary

The scoped advisor returned one method-class mismatch, ten uncatalogued NFR metric labels and four inconclusive FR criteria among 274 native obligations. The existing planned cases cover the major semantic distinctions, but observation resource/provenance controls and one temporal TC binding need correction; all recommendations below are reviewer judgment for the existing plan, not implementation or qualification claims.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | NFR-020-M-1 is authored Analysis; the advisor proposes performance-benchmarking only from quantified-threshold. A benchmark cannot establish that every caller-controlled collection/traversal is bounded. Reviewer proposes inspection of the complete boundary/charge/termination inventory, with TC-062 testing its declared limits. Owner B: settle the catalog method and bind this inventory explicitly; TC-062 alone enumerates known limits and cannot find an omitted collection. | NFR-020-M-1; TC-062; TM-004 |
| FND-002 | low | NFR-040-M-1 uses uncatalogued Property test. Reviewer proposes property-based-testing, with TC-118 independently calculating checked horizon/history bounds and forcing overflow at each composition boundary. Owner E: record the declared catalog method. | NFR-040-M-1; NFR-040-AC-1; TC-118 |
| FND-003 | low | NFR-040-M-2 uses uncatalogued Mutation test. Reviewer proposes fault-injection as the primary exhausted-run check in TC-118, supplemented by mutation-testing of Boolean fallbacks. Measuring mutant detection alone does not establish the original exhaustion behavior. Owner E: record that distinction in the method disposition. | NFR-040-M-2; NFR-040-AC-2; TC-118 |
| FND-004 | low | NFR-040-M-3 uses uncatalogued State-transition test. Reviewer proposes model-based-test-generation for activation/retention/settlement transitions, with TC-118's forced required-history/capture loss. Owner E: record the catalog method and retain the explicit incomplete alternative. | NFR-040-M-3; NFR-040-AC-3; TC-118 |
| FND-005 | low | NFR-040-M-4 uses uncatalogued Integration test. Reviewer proposes integration-testing for the existing restoration/limit identity controls in TC-118. Owner E: use the declared label and preserve the separate identity-change refusal. | NFR-040-M-4; NFR-040-AC-4; TC-118 |
| FND-006 | medium | NFR-050-M-1 uses uncatalogued Boundary tests and maps only to TC-142/144. Those cases cover missing membership and generic exhausted retention, but do not exercise every event-size, active-key, history and population/window limit at exact and one-over values. Reviewer proposes fault-injection. Owner F: extend the existing cases with each named boundary and assert typed non-success with the actual bound and subject. | NFR-050-M-1; TC-142; TC-144; TM-007 |
| FND-007 | medium | NFR-050-M-2 uses uncatalogued Resource tests. No mapped TC measures retained state against each declared bound or detects allocation/retention before refusal. Reviewer proposes property-based-testing with an independently observed accounting bound. Owner F: add exact/one-over measurements to TC-142/144 so a correct refusal with excessive retained state still fails. | NFR-050-M-2; TC-142; TC-144; TM-007 |
| FND-008 | low | NFR-051-M-1 uses uncatalogued Replay differential tests. Reviewer proposes metamorphic-testing across batch and incremental executions of the same admitted inputs, retaining the existing satisfied, missed-deadline and adverse TC-144 controls. Agreement tests consistency and cannot replace the independent semantic oracle. Owner F: record the catalog method. | NFR-051-M-1; TC-144 |
| FND-009 | medium | NFR-051-M-2 uses uncatalogued Provenance inspection, while TM-007 binds only TC-144, whose expected result checks dispositions without testing omitted provenance identities. Reviewer proposes contract-testing: explicitly bind suitable TC-145/146 identity controls or extend TC-144 to remove each required input identity and require refusal/non-reproducibility. Owner F: ensure identical truth with incomplete provenance cannot pass this metric. | NFR-051-M-2; TC-144; TC-145; TC-146; TM-007 |
| FND-010 | low | NFR-052-M-1 uses uncatalogued Dependency inventory. Reviewer proposes inspection using TC-147's complete executable/dependency inventory and forbidden-runtime controls; a dependency list alone does not cover generated executables or fixture audits. Owner F: record the catalog method. | NFR-052-M-1; TC-147; TM-007 |
| FND-011 | low | NFR-052-M-2 uses uncatalogued Build/configuration inspection. Reviewer proposes inspection using TC-147's owned-path/toolchain inventory and independently wrong-toolchain control. Owner F: record the catalog method; no build or runtime audit was performed by this review. | NFR-052-M-2; TC-147; TM-007 |
| FND-012 | low | The advisor is inconclusive for FR-039-AC-2. Reviewer chooses unit-testing: TC-039 checks rational(1,0) and rational(0,0) against the independent zero-denominator refusal before any result. The authored Test class can remain; owner A records this method disposition. | FR-039-AC-2; TC-039 |
| FND-013 | low | The advisor is inconclusive for FR-042-AC-1. Reviewer chooses unit-testing of the exact pre-version/post-version/immutable-delta correspondence in TC-042, with typed invocation/observation identities inspected. The authored Test class can remain; owner A records this method disposition. | FR-042-AC-1; TC-042 |
| FND-014 | low | The advisor is inconclusive for FR-113-AC-2. Reviewer chooses contract-testing: TC-143's same-event/ingestion-time, causal-order and clock-uncertainty controls must retain four distinct fields/premises through the real consumer boundary. The authored Test class can remain; owner F records this method disposition. | FR-113-AC-2; TC-143; IT-050 |
| FND-015 | low | The advisor is inconclusive for FR-114-AC-1. Reviewer chooses metamorphic-testing for TC-144's batch/incremental comparison, with the satisfied and missed-deadline expectations grounded independently in E's selected semantics. The authored Test class can remain; owner F records this method disposition. | FR-114-AC-1; TC-144; IT-050 |
| FND-016 | medium | FR-091-AC-5 names TC-116, but TM-006's coverage and summary bind TC-115. TC-115 supplies inclusive response/progress controls; TC-116 supplies lifecycle controls. Owner E: reconcile the criterion and matrix to the intended case or explicitly bind both, and retain complete empty existential/universal window and at-deadline event expectations. | FR-091-AC-5; TC-115; TC-116; TM-006 |

## Deterministic inputs and limits

Commands executed successfully against the frozen native scope were
`quire coverage --scope . --json`,
`quire properties --scope . 'spec/**/*.md' --json`, `quoin advise --json`, and
`quoin catalog methods --json`.
Tools were Quoin 0.23.1 and Quire 0.31.0
(`cli 4f6ed024`, `engine 0.46.0@ca7362d4`); the installed evidence skill is from
the Quoin plugin 0.22.5. The actual installed catalog, not a remembered method
table, supplied every method name above.

The native advisor was filtered by the exact requested requirement IDs: 251 FR
acceptance criteria, four NFR-040 acceptance criteria and nineteen NFR metrics,
274 total. It reported one mismatch, ten uncatalogued labels and four
inconclusive rows. All 255 FR/NFR acceptance criteria use the catalog's Test
class. A supported class does not identify a concrete method or prove a case.

Reproducibility digests for the complete read-only outputs are:

| Output | SHA-256 |
| --- | --- |
| Native advisor JSON | `f3a84902595e70e8c672a2472729e906a34334370f58fc32a6e61d25139077b4` |
| Native coverage JSON | `9154f01e73100c253fe169496936731922455ee3a4701a8a55f866d0653c8654` |
| Native properties JSON | `6f15a95f82f79bb29449f31be09754960096be2f4a06b56cd9c72980d416272d` |

The advisor recommends performance-benchmarking for all nineteen NFR metrics
because their numeric targets match `quantified-threshold`. Zero invalid truth,
missing identity or foreign runtime is a correctness/policy threshold, not a
latency/throughput requirement. Reviewer judgment therefore selects the actual
discriminating methods even when the rules were not inconclusive, as requested
for this coherent review. No benchmark campaign is inferred from that match.

Likewise, `network-exposed` on FR-049-AC-1/7, FR-050-AC-2 and FR-091-AC-1/5
does not establish a running attack surface: these are typed role/relationship
and temporal endpoint semantics. The existing contract/boundary cases are the
appropriate first controls. `supply-chain` advice on result provenance or
duplicate receipts does not call for an SBOM instead of identity assertions.
Temporal model-checking/runtime-monitoring recommendations concern applicable
meaning, not authorization to add a second monitor or a general theorem campaign.
No evidence-derived fault-detection recommendation is interpreted as an
unverified-requirement finding.

The Coverage Status/Status catalog mismatch is known. Automated backed totals
are not used as authored coverage, and no catalog column or matrix status was
renamed. All new cases remain Planned; historical state/TL/JCS results do not
discharge composed-profile, strict-reader or bridge obligations.

## Criterion-grounded verification strategy

The following reviewer selections elaborate the existing cases where method
choice matters. Passing obligations do not receive Findings rows. These are
ways to implement the existing planned controls after contract agreement, not
new executable tools, mandatory solver campaigns or evidence-store artifacts.

| Criteria and existing cases | Catalog method and discriminating oracle |
| --- | --- |
| FR-020-AC-7–10; TC-020 | contract-testing: validate every required capability/evidence reference against an independently authored inventory, plus absent-family and absent-population controls. A synthetic complete inventory tests the reporter only. |
| FR-030-AC-1–8, FR-035-AC-1–8, FR-130-AC-1–6; TC-030/035 and IT-010 | contract-testing plus metamorphic-testing: independent typed closure/role expectations; mutate one digest domain or declaration identity; preserve static subject under runtime-only changes. Raw bytes, static meaning and assessment input remain distinct comparisons. |
| FR-032-AC-1–4; TC-032 | golden-approval-testing plus negative-abuse-testing: retained old bytes and explicit new selections, including stale-rule-byte rejection. Equal source/formula text cannot substitute for equal selected definitions. |
| FR-036-AC-1–5, FR-037-AC-1–6, FR-038-AC-1–5; TC-036–038 | unit-testing with independently authored token/tree/binding expectations; property-based-testing for byte spans and identity invariants. Grammar-based-fuzzing may extend the same declared grammar later; it cannot replace the positive precedence/namespace controls. |
| FR-039/041/042/044/045 criteria; TC-039/041/042/044/045 | unit-testing and property-based-testing: exact rational normalization/bounds, ordered duplicate occurrences and every sum prefix, pre-qualified reads and exact guard identity. Use independent arithmetic/reference examples; do not assert the producer's own result as its oracle. |
| FR-043-AC-1–5; TC-043 | model-based-test-generation over explicitly finite graphs, with isolated/self-loop/cycle/long-path controls and an independent positive-length reachability oracle. Exhaustion is incomplete, not unreachable. |
| FR-046-AC-1–6 and FR-047-AC-1–5; TC-046/047 | contract-testing and negative-abuse-testing: mutate invocation/frame/population and diagnostic cause/stage identities independently; changing English text cannot change machine classification. |
| FR-048-AC-1–8, FR-049-AC-1–12; TC-049/170–174 | contract-testing over independently authored source trees, causal/value-availability graphs and two capture stages. Boundary examples distinguish holds(true) from temporal true, retry maximum from work budget, false constraint from timeout, and finish from closure. |
| FR-050–059; TC-050–059; IT-020/021 | model-based-test-generation and contract-testing over the finite authored control graph; enumerate permitted small interleavings and one-axis invalid histories. This tests global conformance, not realizability or actual distributed deployment. |
| FR-060-AC-1–5; TC-060 | contract-testing and fault-injection for each requested claim and a typed backend failure; assert no stronger claim or fallback is created. Unimplemented advanced analysis stays unsupported. |
| FR-061-AC-1–9; TC-061 | contract-testing plus property-based-testing over valid result-combination rows and forbidden one-axis mutants. Compare exact support/lineage and unchanged historical bytes; retain early decisive truth independently from closure and adequacy. |
| FR-062-AC-1–4; TC-064 and IT-021 | contract-testing through both actual strict readers; independently compute the projection identity preimage and all simultaneous FR-007 findings. Preserve protocol adequacy in its canonical source instead of relabeling it E02 observation adequacy. |
| FR-090–094; TC-110–116/118 | property-based-testing and unit-testing grounded in exact profile equations, with clock/origin/late-contradiction and resource controls. Mutating every settlement axis tests more than a Boolean truth table. |
| FR-095-AC-1–7; TC-113/117; IT-040/041 | contract-testing and property-based-testing over the already-declared finite formula/trace population with an independent native oracle. Record population cardinality and resource stops; a partial run cannot claim exhaustive correspondence. No current TL past/timestamped support is inferred. |
| FR-110–116; TC-140–146; IT-050/051 | contract-testing and metamorphic-testing at binding, replay, result and output boundaries. Wrong-order links, missing membership, foreign progress, stale definitions and parser-accepted lossy output are distinct negative controls. Preserve F's integrated settlement/support and supersession axes. |

## NFR method dispositions

These are proposed dispositions, not human-confirmed evidence discharge. The
owning requirement's Method/Verification cell remains authoritative; this review
does not create a second registry or change those cells.

| Obligation | Reviewer method | Existing planned artifact and exact observation |
| --- | --- | --- |
| NFR-010-M-1 | fault-injection | TC-048: insufficient next work charge produces no affected completed result. |
| NFR-010-M-2 | negative-abuse-testing | TC-048: missing required input/live work cannot be dropped to report success. |
| NFR-010-M-3 | property-based-testing | TC-048: observed allocations/expansions never exceed the declared charge. |
| NFR-020-M-1 | inspection | Complete bound/charge/termination inventory; bind it explicitly alongside TC-062. |
| NFR-020-M-2 | fault-injection | TC-062: each exact limit admits eligible work and one-over produces typed resource-incomplete. |
| NFR-020-M-3 | negative-abuse-testing | TC-062: action-capable test double receives zero calls. |
| NFR-021-M-1 | metamorphic-testing | TC-063: equivalent map/independent-event presentations retain ordered semantic results. |
| NFR-021-M-2 | metamorphic-testing | TC-063: each changed semantic dependency changes the corresponding identity. |
| NFR-021-M-3 | contract-testing | TC-063: no reproduced result omits exact input/backend provenance. |
| NFR-040-M-1 | property-based-testing | TC-118: independent checked bounds and deliberate overflow. |
| NFR-040-M-2 | fault-injection | TC-118: forced exhaustion emits no Boolean; mutation-testing measures fallback discrimination. |
| NFR-040-M-3 | model-based-test-generation | TC-118: required capture/history survives until settlement or explicit incomplete loss. |
| NFR-040-M-4 | integration-testing | TC-118: changed restoration/limit identity cannot reuse a stale result. |
| NFR-050-M-1 | fault-injection | TC-142/144: add each named exact/one-over observation boundary. |
| NFR-050-M-2 | property-based-testing | TC-142/144: add measured retained state for every declared bound. |
| NFR-051-M-1 | metamorphic-testing | TC-144: same admitted input yields equal settled dispositions across both modes. |
| NFR-051-M-2 | contract-testing | Explicitly bind omitted-identity controls in TC-144/145/146. |
| NFR-052-M-1 | inspection | TC-147: complete owned executable/dependency inventory and foreign-runtime controls. |
| NFR-052-M-2 | inspection | TC-147: owned-path Rust/toolchain inventory and wrong-toolchain control. |

StR-002's Demonstration and Inspection methods suit its enterprise example and
capability/ownership inventory. They validate stakeholder meaning separately
from test execution.

No suites were executed or added. The existing TC/IT documents supply the
planned corpus; concrete Rust harness bindings and evidence recording remain
future implementation work after the exact contract findings are resolved.
There is no inference that a missing executable test, solver, monitor or suite
artifact is a regression in this specification-only cycle.
