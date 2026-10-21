---
id: SR-13650
title: 'Spec review of QSpec PR #314: a temporal clause always names an over parameter (STD-186)'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@77228570f; PR #314 diff origin/main...HEAD: shared-grammar.md, FR-361, FR-370, TC-180, TC-303; ticket STD-186'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-370
  type: reviews
- target: ix://agent-ix/quire-specification/FR-361
  type: reviews
- target: ix://agent-ix/quire-specification/TC-180
  type: reviews
- target: ix://agent-ix/quire-specification/TC-303
  type: reviews
---

## Summary

Ticket: STD-186. PR quire-specification#314, head 77228570f92bd4b83ca2dd51e365fd47e8c5d16e.

Base lens with EARS and integrity folded in. Gates: cargo tests pass (12 + 114); quire validate 134 failures as on main, touched files clean. Merges cleanly into main. The refusal example was checked against both `temporal-clause` productions (shared-grammar.md:153 and :481): after `using ident` the next token must be `over`, so `clock` is the unexpected token. The PR body flags (b) versus an optional `over` as an owner-level choice and names the QSL FR-153-AC-3 / FR-161-AC-2 rework (STD-185): verified. FR-370-AC-13 collides with nothing. The test's discrimination is reviewed in SR-13652 (code-review), the empty-universe consequence in SR-13651 (failure-domain).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| shared-grammar over-required paragraph | proposals/quire-v1/shared-grammar.md | examined |
| shared-grammar no-over example row | proposals/quire-v1/shared-grammar.md | examined |
| FR-361 over binding paragraph | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md | examined |
| FR-370 argument 1 | spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md | examined |
| FR-370-AC-13 | spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md | examined |
| TC-180 no-over row | spec/test-cases/TC-180-complete-v1-profile-bundle-accounting.md | examined |
| TC-303 step 11 | spec/test-cases/TC-303-checked-package-temporal-clause-body.md | examined |
| TC-303 expected 11 | spec/test-cases/TC-303-checked-package-temporal-clause-body.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The added sentences ('so the clause is checked once per conforming object; a claim that reads no one object still names one') extend the paragraph that #315 (STD-196) replaces: merging #314 then #315 conflicts in FR-361, and #315's model has one instance per (population, key) for every covering population, so 'once per conforming object' is wrong after both land (an object in two covering populations gives two instances). Fix: coordinate with #315; state the rule in #315's instance terms (one check per instance) or land #315 first and rebase. | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:128-131 |
| FND-002 | low | The grammar refusal is put in TC-180, the complete-V1 bundle-accounting case, whose frontmatter verifies FR-131 and FR-339 only, so FR-370-AC-13's source half traces to a TC with no edge to FR-370. Fix: add a `verifies` FR-370 relationship to TC-180, or move the source-refusal step to TC-303 (which already verifies FR-370). | spec/test-cases/TC-180-complete-v1-profile-bundle-accounting.md:26 |
| FND-003 | low | The new paragraph follows the `commit`/`finish` protocol productions (line 237), not either `temporal-clause` production (lines 153 and 481), so a reader of the temporal grammar does not see it. Fix: move it next to the `temporal-clause` production. | proposals/quire-v1/shared-grammar.md:237 |

## Verdict

Mergeable after FND-001 is coordinated with #315 (and SR-13652 FND-001 fixed); FND-002 and FND-003 are small. The (b) choice is correctly flagged as owner-level in the PR body.

## Dispositions

Round 1, head 13bd4aa05.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fixed in 13bd4aa05: The added FR-361 text is now in #315's instance terms. The textual conflict with #315 remains by design (#315 lands first, #314 rebases); trial merge #315 then #314 conflicts in FR-361 and must be resolved by keeping #315's paragraph and inserting only #314's new sentences; so resolved it reads consistently, apart from the inherited no-instance wording (SR-13653 FND-006). |
| FND-002 | fixed | Fixed in 13bd4aa05: TC-180 now verifies FR-370. |
| FND-003 | fixed | Fixed in 13bd4aa05: The paragraph now follows the first temporal-clause production and names the complete-V1 form too. |
