---
id: SR-852
title: "Integrity review of PR #175: probabilistic properties and dense time"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@d7919e0e5510d1e8f2e5830b4acb5138beaad0e5; PR #175 diff against origin/main a18859d; ids, index rows, test matrices, cross-references, shared-grammar.md, simulation-sampler.md, native-diagnostics.md, FR-271, FR-272; open sibling PRs #168, #169, #170, #173, #174"
review_set: subset
---
# Integrity review of PR #175

## Summary

Ticket: STD-137, with STD-139. FR-405..FR-421 and TC-350..TC-374 are free
on main and in every other Wave B branch. TC-375 and later belong to PR #173.
Each new FR has a spec.md index row and implements a user story. Every new
and amended AC has a matrix row that points at a behaviour TC. No ticket id
appears outside References.

`quire validate` was run on each changed file. The 17 new functional FRs
and the 25 new TCs pass. The failures that remain are classes main already
has: object files report "required 'id' missing", and the matrices report
"Coverage Status" against "Status".

`cargo test --locked --no-fail-fast` fails only the two `tc_233` tests.
These are the known digest tests that #174 deletes; they are a merge item,
not a finding here.

## Verdict

**Changes needed.** Five medium and five low findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC ids clash with open sibling PRs: FR-272-AC-14 (#174), FR-090-AC-8 (#170), FR-181-AC-7 (#170, #168), FR-255-AC-6 (#170), FR-331-AC-11 (#170) and FR-161-AC-8 (#169). The FR-181 sampler text and TC-210 are also rewritten by #168, #170 and #173, and FR-349, TC-266 and FR-271 by #174. Whichever PR merges second must renumber its ACs and drop its duplicate edits. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:178 |
| FND-002 | medium | The shared grammar's `probabilistic-clause` has no `over` parameter. FR-406, FR-407 and FR-408 quantify over "every binding of its `over` parameter", TC-357 BF-02 uses two bindings, and `temporal-clause` has `over`. As written, a binding cannot be stated. | proposals/quire-v1/shared-grammar.md:540-542; spec/test-cases/TC-357-bonferroni-and-symmetry.md:25 |
| FND-003 | medium | `temporal-clause` still requires `'clock', string`. FR-252, FR-416-AC-6 and TC-368 TB-01 require a claim with no `clock` member to bind the default. The grammar was not changed to make `clock` optional. | proposals/quire-v1/shared-grammar.md:143-145; spec/functional/temporal/FR-416-evaluate-timed-temporal-claims.md:142 |
| FND-004 | medium | The sampler definition does not say whether a draw-free choice (`n = 1`, such as one enabled operation or a one-value support) uses up a choice index. The answer changes every later preimage in the step, so two implementations would draw differently. Items 1 and 2 of "Choices within a step" are also alternatives (with or without a workload), yet the text calls the list an "order". | proposals/quire-v1/definitions/simulation-sampler.md:25-48 |
| FND-005 | medium | FR-408-AC-10 settles a mean-of-fraction SPRT request as "refused `NotStatistical`". `NotStatistical` has no diagnostic code, family, stage or FR-331 value in FR-272 or native-diagnostics, so the AC has no exact expected result. | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:99,192 |
| FND-006 | low | The grammar and the vectors disagree in several places. Decimal literals (`0.95`, `alpha 0.01`) are not `exact-value`. FR-419 says `fixed-priority`, the grammar says `fixed_priority`. TC-359 writes `probability of … >= 0.5`. `past[0, 300 s]` uses durations, but `interval` takes integers. `fraction … over [uint, h]` admits a lower bound that only 0 has a meaning for. `releases τ` has no production. A partial confidence set cannot be parsed, yet FR-407 gives it the typed cause `partial-confidence`. The timed-interval note says "numeric literal", but `rational(…)` starts with an identifier. TC-369 TS-11 omits the alias and the unit. | proposals/quire-v1/shared-grammar.md:520-583 |
| FND-007 | low | FR-408, FR-411 and FR-416 cite "the FR-363 rule" and FR-363 in their relationships. FR-363 exists only in PR #170, and FR-161 still cites "FR-341's labels", which #170 renames FR-360. #175 must merge after #170 and be rebased onto it. | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:150; spec/functional/temporal/FR-161-analyze-infinite-trace-properties.md:126 |
| FND-008 | low | FR-405 says a random parameter has exactly one distribution, but gives no typed refusal for a second `random` declaration of the same parameter. | spec/functional/analysis/FR-405-declare-probabilistic-models.md:62-63 |
| FND-009 | low | `priority` is optional in the `task` grammar. FR-419 does not say what a fixed-priority claim does when a task has no priority, yet TC-372 assumes "priorities by period". | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:44-52 |
| FND-010 | low | The FR-271 `unsupported_projection` row was edited to drop "pinned", but it keeps the "(revision `1-draft.5`)" catalog-revision marker. That is tracking ceremony, and it can go in #174's sweep. | spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md:62 |

## Dispositions

Round 1, reviewed at 9ed6fab8b33842c1fef83d918d683f84d2b4de04 (fix commits 96a98644 and 9ed6fab8).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | deferred | Merge item, not a content defect: the coder renumbers the clashing ACs when #175 rebases after #170, #168 and #169 merge. #174 has merged, so its clashes resolve on the next rebase onto main. |
| FND-002 | fixed | 96a98644 |
| FND-003 | fixed | 96a98644 |
| FND-004 | fixed | 96a98644 |
| FND-005 | fixed | 96a98644 |
| FND-006 | fixed | 96a98644 |
| FND-007 | deferred | Merge item: #175 merges after #170 and rebases onto it. FR-161:126 still cites "FR-341's labels", which the rebase renames to FR-360, and the FR-363 citations resolve once #170 is on main. |
| FND-008 | fixed | 96a98644 |
| FND-009 | fixed | 96a98644 |
| FND-010 | still-open | The FR-271 `unsupported_projection` row still carries "(revision `1-draft.5`)". #174 has merged and main still has the marker, so the sweep this finding deferred to did not remove it. Drop the marker from the row #175 edits. |

Round 2, reviewed at addbd0986dcee88165668b4bb956d8f33569e816 (rebased onto main ae02648b; commits b56235f8 and addbd098).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-010 | fixed | b56235f8 |

Round 3, reviewed at fcfcd6b67c3f9767d0d3447d146a117520ee5df7.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fcfcd6b6: Rebased onto main; none of the clashing FRs holds a duplicate AC id. |
| FND-007 | fixed | fcfcd6b6: FR-161 cites FR-360; FR-363 is on main. |
