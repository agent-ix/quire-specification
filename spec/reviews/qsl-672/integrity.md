---
id: SR-5663
title: "QSL-672 QSpec admission consistency"
type: SpecReview
analysis: integrity
scope: "quire-specification@732fce3f405a1ec877aa21acd424a73482250719; shared grammar; FR-147; FR-039; FR-044; FR-149; TC-192; state-core definition; QSL FR-091"
review_set: subset
---

## Summary

Integer grammar recognition, complete-value admission, selected quotient laws, Euclidean modulus and dead-branch refusal are consistent with companion FR-091. The newly expanded rational-slash guidance still contradicts the definedness requirement it cites.

## Verdict

**FAIL** — a compiler author still has two contradictory admission rules for expected-rational slash.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Expected-rational slash now permits exact integer promotion without a complete-value guard, but FR-044 Behavior and AC-3/AC-9 prohibit implicit conversion under every profile. State-core also expressly refuses implicit conversion. Scope the promotion exception and reconcile these normative requirements before implementation. | proposals/quire-v1/shared-grammar.md:1070; spec/functional/foundation/FR-039-normalize-exact-rational-literals.md:25; spec/functional/foundation/FR-044-check-composed-numeric-definedness.md:29; FR-044-AC-3; FR-044-AC-9; spec/functional/type-model/FR-149-apply-complete-equality-matrix.md:192; proposals/quire-v1/definitions/state-core.md:38 |

## Analysis

Primary changed unit: FR-039 Behavior. Its new wording says FR-149 permits exact `n/1` promotion when a rational result type is expected, without naming the complete-value prerequisite. Shared grammar likewise places that exception immediately after its carried-forward admission text. TC-192 DIV-18 selects complete value and expects promotion, but directs result/nonzero/domain checks to FR-044, which still rejects implicit conversion under every profile. A coder cannot decide the intended profile boundary from an agreed normative rule.

This finding is the same semantic defect as SR-5662 FND-001, examined through the integrity lens, not a second independent defect. The old FR-044 blanket statement already conflicted with FR-149; the current alignment needs to resolve it because it explicitly invokes both contracts.

The selected division laws remain unchanged: truncating and floor remainder signs agree with the signed test table; Euclidean remainder is nonnegative and less than the absolute divisor. `mod` does not inherit a selected truncating/floor law. Missing or conflicting `div`/`rem` selections refuse before evaluation. Zero, bounded exposed-member errors, accounting exhaustion and finite-provider negotiation remain separate. Infix forms preserve source locations and reachability never suppresses profile refusal. No new extension API, side effect or graph traversal was introduced.

Trace chain reviewed: StR-002 -> US-010 -> FR-147 -> TC-192 DIV-17/DIV-18; QSL FR-091-AC-39 and resolved OQ5 consume the same admission decision. No new relationship-edge changes or applicable AssuranceProfile were present. Base validation passed structurally; the computed matrix does not establish implemented coverage.


## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 84c229da6a31fc31d9ddf9249847b3bc98316a5b: FR-039/shared grammar now expressly limit promotion to complete value, and FR-044 Behavior/AC-3/AC-9 carry the same exception. State-only refusal remains explicit; TC-044 and DIV-18 supply paired profile controls. |

## Delta verdict

**PASS** for semantic integrity at 84c229da6a31fc31d9ddf9249847b3bc98316a5b. No new arithmetic/profile contradiction found. Base checklist findings are recorded separately in SR-5662.

Examined shared grammar, state-contract scope, FR-039 Behavior, FR-044 Behavior and AC-3/AC-6/AC-9, FR-147-AC-11, TC-044 and TC-192 DIV-17/DIV-18; read FR-149 rational promotion and state-core admission as context. Exact promotion is limited to expected-rational `/` under complete value. Ambient conversion and state-only integer slash still refuse, and quotient normalization, nonzero proof and rational result bounds still apply. Existing div/rem law selection and Euclidean mod meaning remain unchanged. No implementation or executable coverage is claimed.


## Delta confirmation (disposition pass 2)

**PASS** retained at 9b709948c39ecc40210589ba5ec418ef5a81c295. The complete delta from the semantic-reviewed head changes only planned trace arguments and Verification labels. The semantic rules and paired admission controls remain unchanged; no new integrity finding.
