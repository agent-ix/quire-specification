---
id: SR-418
title: "Native temporal vertical-slice architecture review"
type: SpecReview
analysis: architecture-evaluation
scope: "AD-001, AD-002, Plan-002, quire-spec-language compiled-protocol and temporal runtime boundaries"
review_set: all
review_date: "2026-09-11"
relationships:
  - { target: ix://agent-ix/quire-specification/AD-002, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-090, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-095, type: reviews }
---
## Summary

This review follows the authored native temporal subject through the compiler,
observation boundary, runtime, Contract-IR and TL boundary. The current design
has one clear ownership and dependency direction: native semantics govern the
derived bridge rather than a backend selecting meaning. The runtime and tests
already demonstrate bounded evaluation, pointwise connectors, activation and
resource dispositions; the cross-repository authenticated seam is not yet
implemented, so the architecture is ready for staged implementation but cannot
claim end-to-end native-temporal support.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The compiler emits `quire.compiled-protocol/1`; its temporal body has no temporal-definition digest or clock parameters. A runtime could therefore receive a profile name without authenticating sequence authority, sample epoch/period/unit or timestamp unit. AD-002 correctly requires `/2`; Task-002 is the smallest repair and blocks execution admission. | `quire-spec-language/spec/functional/FR-042-publish-compiled-protocol-artifacts.md`; `quire-spec-language/spec/reviews/native-temporal/scope-boundary.md`; AD-002; Plan-002 Task-002 |
| FND-002 | high | The current compiler/runtime temporal implementation has no accepted FR-025 predicate projection or FR-026 correspondence interface. Directly exposing its Boolean positions to TL would conflate typed predicate evaluation, clock/history semantics and formula truth. Task-004 then Task-006 are correctly ordered; no partial bridge should be added to the runtime. | `quire-contract-ir` issues #63/#64; FR-095; Plan-002 Task-004/Task-006 |
| FND-003 | medium | The runtime requires externally supplied trace/progress structures, while the authoritative observation/replay contract is separately owned. Without Task-003, silence, duplicate receipt and late-data results cannot be attributed to one cross-repository authority. Keep the current runtime result as an internal execution boundary and require identified F assertions at the integration edge. | `quire-spec-language/src/temporal/`; FR-093; FR-094; Plan-002 Task-003 |
| FND-004 | low | No architectural direction violation was found in the native runtime itself: its `Option<Truth>`/typed errors and test vectors distinguish non-Boolean stops from truth, matching AD-002’s failure rule. The remaining work is authentication and cross-boundary integration, not replacement of the evaluator core. | `quire-spec-language/src/temporal/result.rs`; `quire-spec-language/tests/composed_temporal_evaluation.rs`; NFR-040 |

## Scenarios examined

- **Success:** an admitted bounded future subject with total Boolean valuations
  reaches the native runtime; pointwise truth is evaluated without TL source
  parsing.
- **Failure:** absent/changed semantic selection at the compiler-to-runtime
  boundary cannot yet be detected from `/1`; `/2` is the required repair.
- **Recovery:** an exhausted or incomplete native runtime path returns a typed
  non-Boolean disposition rather than fabricating a verdict.
- **Change:** altering a sample period, closure interpretation or profile must
  change the subject/correspondence identity; the Plan-002 support table makes
  the later bridge refuse until it can prove that relation.

## Recommendation

Accept AD-002 and Plan-002 as the single implementation architecture, subject
to human review. Start only Task-001 and the two independent foundations after
it; Task-005/006 must remain gated by `/2`, observation authority and FR-025.
