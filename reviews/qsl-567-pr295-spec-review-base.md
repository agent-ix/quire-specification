---
id: SR-13414
title: "Base review of QSpec PR #295: FR-365-AC-8 order-distinguishing vector"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@c861f5c; spec/functional/temporal/FR-365-build-canonical-temporal-counterexample.md, spec/test-cases/TC-300-strong-fairness-refinement-depth.md; ticket QSL-567"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-365
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-300
    type: reviews
---

## Summary

Ticket: QSL-567 (follow-up of quire-specification#294; the branch and title carry no ticket id, and the coordinator tracks it under QSL-567). PR: quire-specification#295 at c861f5c (c861f5c1f78259171d930023aa32132558f26865), scoped to `git diff refs/remotes/origin/main...c861f5c1f` (origin/main fbb9636, #294 merged). The diff adds FR-365-AC-8, a three-state SCC with two visibility pairs that is meant to tell FR-365's one-step refinement apart from refining one pair at a time, and a TC-300 step and expected result for it.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-365-AC-8 | spec/functional/temporal/FR-365-build-canonical-temporal-counterexample.md | examined |
| TC-300-step-AC-8 | spec/test-cases/TC-300-strong-fairness-refinement-depth.md | examined |
| FR-365-components-and-order | spec/functional/temporal/FR-365-build-canonical-temporal-counterexample.md | examined |
| FR-365-AC-7 | spec/functional/temporal/FR-365-build-canonical-temporal-counterexample.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The vector does not separate the two rules under the order FR-365 states. FR-365 enumerates "SCCs and their sub-components in discovery order under FR-181 canonical breadth-first order". Here breadth-first discovery from 0 gives 0, 1, 2. If pair B is refined alone first, `1 → 0` is removed. The remainder splits into `{0}`, `{1}` and `{2}`, each with a self-loop. `{0}` is discovered first, keeps only `0 → 0`, has no edge of either pair, and passes. So B first also gives `{0}`, not `{1}`. `{1}` comes first only if sub-components are taken in Tarjan completion order, where the sink `{1}` finishes first. FR-365 does not use that order, and it is not breadth-first discovery. My oracle (`oracle2.py`) confirms it: under breadth-first discovery ranking, one-step, A-first and B-first all give `{0}`; under Tarjan completion order, B-first gives `{1}`. So a TC-300 test built from this AC passes for an implementation that refines one pair at a time, which is what the PR body says AC-8 must catch. The TC-300 expected result "`{0}`, not `{1}`" has the same problem. Fix: use a vector that separates the rules under breadth-first discovery order. The oracle found one, renumbered so that state numbers are the discovery order: states 0-3, `1` and `3` accepting, edges `0 → 1` (pair B stale), `0 → 2`, `1 → 1`, `1 → 2`, `2 → 0` (pair A stale), `2 → 3`, `3 → 2` and `3 → 3`, no fresh loads. One step removes `0 → 1` and `2 → 0`, and the first passing component is `{1}`. B first leaves `{0, 2, 3}` first, which then fails pair A and refines to `{2, 3}`. A first ends at `{1}`. Also state in FR-365 how sub-components are ordered, for example by their earliest-discovered state in canonical breadth-first order, since the two orders give different first components (54 and 6 distinguishing graphs in 100,000). Finally, give the AC only the observable result. "Refining pair B alone first would pass ..." describes a rule the system does not have, and no test can fail it. Keep the contrast in the TC or the PR as rationale. | spec/functional/temporal/FR-365-build-canonical-temporal-counterexample.md:139 |

## Verdict

Not mergeable: one medium finding. The AC's stated result is correct. One step removes `0 → 1`, `0 → 2` and `1 → 0`, and the first passing component is `{0}` under either ordering of sub-components. But the contrast the vector exists for, `{1}` under B-first, holds only in Tarjan completion order, not in the breadth-first discovery order FR-365 states, so the test cannot fail the implementation it is meant to reject. AC-8 agrees with AC-7 and FR-365's one-step refinement text. Its TC-300 step and expected result are placed as AC-7's are. It needs no change to the refinement rule. `quire validate --scope .` exits 0 on both files, with output identical to main. `quire matrix` lists FR-365-AC-8 as untagged, like every FR-365 AC.

## New findings (disposition pass 2)

Against head 85383b3 (85383b39b712d5a12cc7705fd59c7e2bbd9e021f), with origin/main at fbb9636. The committed `reviews/` copies equal the reviewer's round-1 files. `quire validate --scope .` exits 0 on all four touched files, and for FR-365 and TC-300 the output is identical to main. `cargo test --test backend_provider_v1 --test checked_package_v2` passes 11 and 108. `quire matrix` lists FR-365-AC-8 as untagged, like every FR-365 AC. Oracle (`pr295-r2-oracle.log`) with breadth-first discovery from 0, successors in `e`-label order (which gives discovery 0, 1, 2, matching the AC): one step `{1}`, A first `{2}`, B first `{1}`.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | TC-300's rationale for AC-8, "refining pair A alone first would pass `{2}`, so only the one-step rule gives `{1}`", is not quite right. Refining pair B alone first also gives `{1}`: removing `e4`, `e5` and `e6` leaves `{0}`, `{1}` and `{2}`, and `{1}` passes. So the vector catches a one-pair-at-a-time implementation only when it takes A first. Write "refining pair A alone first would pass `{2}`" and drop "so only the one-step rule gives `{1}`", or note that the B-first order also gives `{1}`. The expected result `{1}` itself is correct. | spec/test-cases/TC-300-strong-fairness-refinement-depth.md:58 |

## Dispositions

Round 1. Against head 320c05e (320c05e5022afce739664267de8632366c5f23ac), with origin/main still at fbb9636. `quire validate --scope .` exits 0 on FR-365 and TC-300, with output identical to main. `cargo test --test backend_provider_v1 --test checked_package_v2` passes 11 and 108. `quire matrix` lists FR-365-AC-8 as untagged, like every FR-365 AC. Oracle on the new vector (multigraph, `e7` a second `2 → 2`): with ranks by state number or by breadth-first discovery with successors by target, one step gives `{1}`, A first `{2}` and B first `{1}`. With breadth-first discovery and successors in `e`-label order, every mode gives `{2}`. In Tarjan completion order, every mode gives `{1}`.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | Partly fixed. The ordering rule (earliest-discovered state, not completion order) is now stated, and the new vector separates the two rules when state 1 is discovered before state 2. But the AC's own edge labels put `e2` `0 → 2` before `e3` `0 → 1`. Under FR-181, a state's successors are visited in canonical transition order, so a reader who takes the `e`-numbers as that order discovers 2 before 1. Then one step, A first and B first all give `{2}` (oracle, `pr295-r1-oracle.log`): the test expects `{1}` and fails a correct implementation, and the two rules are not separated. If ranks follow state numbers or targets, one step gives `{1}` and A first `{2}`, as the AC states. Remaining: (1) make the discovery order unambiguous, for example by swapping the labels so that `e2` is `0 → 1` and `e3` is `0 → 2`, or by stating that the transition order from 0 visits `0 → 1` first. (2) The ordering rule lives only in an AC. Put it in FR-365's Components and order paragraph (sub-components ranked by their earliest-discovered state in FR-181 canonical breadth-first order), and have the AC test it. (3) "Whereas refining pair A alone first would pass `{2}`" still describes a rule the system does not have. Move it to the TC as rationale. |

Round 2, against head 85383b3.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
