---
id: SR-11264
title: "Spec review of quire-specification PR #259: relationship instances in navigation and frames"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@5b310712f339feea75e6172d20e25195eefe555a; spec/functional/type-model/FR-152-bind-systems-model-structures.md, spec/functional/foundation/FR-013-validate-operation-frames.md, spec/objects/interfaces/FR-340-checked-package-frame-location.md, spec/test-cases/TC-197-systems-model-typed-binding.md, spec/test-cases/TC-236-frame-grant-population-scope.md, proposals/quire-v1/definitions/value-accounting.md"
review_set: subset
---
# Spec review of quire-specification PR #259

## Summary

Ticket: QSL-794. One pass of spec-review with the contradiction, ambiguity,
completeness, testability and spec-integrity sub-analyses over
`git diff origin/main...HEAD`. Spec-only PR. Only major issues are reported;
mediums and lows are recorded in the Verdict.

Compared against merged QSL (origin/main): FR-364 "Runtime navigation reads
only the extent of its relationship" steps 1 to 5 and "Where each refusal
fires"; FR-106 checks 7, 8 and 11 (step 11.5); FR-115 Frame witness;
TC-1034 rows 1 to 12.

Examined:
- FR-152 "Navigation" runtime paragraph (lines 143-158) and FR-152-AC-10 (examined)
- value-accounting.md model-reference paragraph (lines 384-398) and the `collection.result-retain` / `model.navigate` rows (examined)
- FR-013 Behavior relationship sentences and FR-013-AC-10 (examined)
- FR-340 relationship-grant paragraph (lines 133-140) and FR-340-AC-11 (examined)
- TC-197 fixture Y navigation bullet, Population Q, Y07, Y08, Y09, Y13 (examined)
- TC-236 Vector 6 and Expected Results (examined)
- FR-013-AC-4, spec.md 10.1, FR-352 frame_witness paragraph (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

**PASS: no blocking findings.**

Verified, step by step:
- Navigation order matches QSL FR-364 steps 1-5 and value-accounting.md: `model.navigate`, then the extent-completeness decision with no later charge, then the targets. FR-152, FR-152-AC-10 and the value-accounting text all place the incomplete outcome after `model.navigate`.
- Y13 run 1 (unlisted) and run 2 (`complete: false` with two links): `complete(R)` is false, so the result is `incomplete_population`/`incomplete-scope` retaining `Sys/composition` and `s1`, after 1 work unit. This matches QSL TC-1034 row 9.
- Y13 run 3 (complete, no links, destination `Pump` `{0,3}`): this is multi-valued with 0 targets. That means no `collection.visit`, then `collection.bound`, the bound holding for 0, then `collection.result-retain`: 3 work units. This is QSL FR-364 step 5 with an empty selection. The result units are not stated, but they are derivable as `occ(empty Set)` from the `collection.result-retain` row.
- Y13 run 4 (complete `Pump/housing`, no links, `self.home` from `p1`, destination `Tank` `[1,1]`): this is single-valued, so the count bound is decided with no charge, giving `below-minimum` with count 0 after `model.navigate` alone. This matches QSL FR-364 step 4 and TC-1034 row 7.
- Y07-Y09 with Q listing both extents complete: the charges and results are unchanged, and the extent listing is now what makes them reachable. Y07 base `[p1, p2]` is 5 work units. The limit-4 run stops at `collection.result-retain`. The 4-target run states its own extent: 6 work units, `above-maximum`. The `p4` dangling run is 4 work units, decided before the bound. Dangling depends on population closure, not extent completeness, so its behaviour is unchanged from merged #258. Y08's `undirected` `self.pump` from `t1` reads `sources` of the complete housing extent `{p1, p2}`, giving count 2 and `above-maximum`. Without the complete listing it would read incomplete, so the PR's Q text is needed. Y09's `t9` dangling run is unchanged.
- FR-013 / FR-340 / TC-236 Vector 6 agree with QSL FR-106 check 11 step 5. The grant covers both ends. Relationships are compared only when both snapshots list them complete. A link with an end at a deleted object is left out. A link with an end at a created object is an added instance that needs the grant. The rule that `creates` grants only the new object (whose fields are exempt as a creation) does not conflict with this, because a link is not a field of the created object.
- No tracking, hash or version ceremony was added. The pre-existing FR-340 structural failure and the FR-013 "effective" wording were not worsened.

Recorded (medium/low, not blocking):
- Medium: value-accounting.md line 391 now conditions "a field or relationship-end navigation" on "the observation's extent of the relationship". A field navigation has no extent, so a literal reading makes every field navigation `incomplete`. Scope the clause to a relationship end.
- Medium: TC-236 Vector 6 does not state that `t2` (and `p2`) exist in the pre snapshot. If `t2` were absent from pre, it would be a `Tank` creation that `creates Pump` does not grant, and FR-106 11.1 would pre-empt the link check.
- Medium: QSL FR-115 (merged) routes only check 11 steps 1-4 and has no `FrameWitness` change variant for a link, so a relationship `frame_violation` in a `Frame` run has no defined witness. This is a QSL-side gap, not this PR's text.
- Medium: an unlisted or incomplete relationship is "not compared", so the frame verdict can be valid while a change is unobserved. This is in tension with spec.md 10.1 ("missing observations remain incomplete"), but it matches merged QSL FR-106 11.5.
- Low: "the first changed link" in TC-236 Vector 6 relies on QSL FR-106 11.5's instance order (source then target reference key). FR-013 states no order.
- Low: Y13 states no result units for the empty Set.
