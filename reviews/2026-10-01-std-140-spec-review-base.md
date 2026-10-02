---
id: SR-840
title: "Base review of PR #173: protocol transition system and weak memory"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@559a409fa92622132b84dc6df275d8fb612ddb5a; PR #173 diff against origin/main; FR-425..FR-439, FR-049, FR-052, FR-181, FR-220, FR-228, ADR-006, TC-052, TC-210, TC-375..TC-394, spec/spec.md, spec/complete-v1/tests.md, spec/protocol/tests.md"
review_set: subset
---
# Base review of PR #173

## Summary

Ticket: STD-140, with STD-138. This pass covers the checklist: IDs, index
rows, traceability, and AC-to-TC coverage. FR-425..FR-439 and
TC-375..TC-394 do not collide with main or with any other Wave B branch.
Every new FR implements a user story (US-006, US-008 or US-011). Every new
AC has a row in TM-009 or TM-004 that points at a behaviour TC. The TCs are
concrete vectors with expected results; the SB, IRIW and split-shipment
counts were spot-checked by hand and match. Ticket ids appear only in
References. Running `quire validate` on each changed file shows only the
four failures that main already has (complete-v1/tests.md,
protocol/tests.md, FR-220, FR-228).

The gaps are amended or added behaviour that no TC exercises, and edits to
FR-181 and TC-210 that other lanes are also making.

## Verdict

**Changes needed.** Four medium findings and one low finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-049-AC-3 now admits an invariant-and-variant repeat and refuses one with neither premise. TC-171, the test it points at, was not updated: it still tests only "a repeat maximum of three" and "repeat maximum zero". | spec/functional/foundation/FR-049-bind-native-choreography-syntax.md:86; spec/test-cases/TC-171-bind-causal-control-values.md:18 |
| FND-002 | medium | FR-426 adds a settling rule for a `repeat` with no maximum (no counter; enter the body while the guard is true). FR-426-AC-4 and TC-376 FL-06 test only the maximum form, so the new rule has no AC and no behaviour vector. | spec/functional/protocol/FR-426-apply-protocol-steps.md:60,149 |
| FND-003 | medium | Commit 559a409 adds normative undefined-evaluation behaviour: FR-433 replay of a `kind: undefined` counterexample, and FR-439's rule that an undefined claim is refuted at bound-limited states. Neither has an AC or a TC vector. | spec/functional/protocol/FR-433-replay-protocol-counterexamples.md:91; spec/functional/memory/FR-439-bound-replay-and-report-weak-memory-runs.md:68 |
| FND-004 | medium | FR-181's sampler sentences, FR-181-AC-2 and TC-210's ecosystem-lock section are outside STD-140/STD-138. The Q3 and Q4 Wave B branches rewrite the same sentences with different text, and Q4 also rewrites TC-210. Removing the ceremony is correct, but three divergent versions will conflict. Leave this edit to one lane. | spec/functional/runtime/FR-181-simulate-finite-executions.md:37,128,141; spec/test-cases/TC-210-finite-simulation-reproducibility.md |
| FND-005 | low | ADR-006 now records a Decision, but its Consequences section still says "Until the question is answered, … an implementation can satisfy one side only". | spec/decisions/ADR-006-protocol-bounded-control-profile.md:71-75 |

## Dispositions

Round 1, reviewed at 47c10e8ea16abb2545234dc09dedefca37bbf273.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 47c10e8 |
| FND-002 | fixed | 47c10e8 |
| FND-003 | fixed | 47c10e8 |
| FND-004 | fixed | 47c10e8 |
| FND-005 | fixed | 47c10e8 |
