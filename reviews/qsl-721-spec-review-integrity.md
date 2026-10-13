---
id: SR-9043
title: "Integrity review of QSpec PR #251 (QSL-721 per-declaration specialization-cycle refusals)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@82cba376a03882c3cac383cfc9c880b4964727eb; proposals/quire-v1/definitions/model-complete.md, spec/test-cases/TC-196-conformance-redefinition-and-dispatch.md, spec/test-cases/TC-235-business-construct-meaning-binding.md; cross-reference sweep over spec/, proposals/, tests/"
review_set: subset
---

## Summary

Ticket: QSL-721. The PR adds and removes no IDs. It edits three rows (TC-235
M17, M20; TC-196 R01) and one paragraph of model-complete.md.

Cross-reference sweep: spec/, proposals/ and tests/ were searched for
"specialization-cycle", "supertype cycle", "exactly once", "one refusal" and
"uniqueness" near cycle statements. FR-150 (:108-109), FR-208 Generalization
(:232-233), FR-271, FR-272 (:65) and native-diagnostics.md (:70) state the cause
without a cardinality, so they remain correct. FR-151-AC-2 ("refuses with its
named cause and the contributing declarations") is compatible. FR-151's "one
refusal per strongly connected component" (:215) is about `definition-cycle`,
not supertypes. TC-195 N14 already uses one refusal per node for variant-type
cycles. No ADR mentions specialization cycles. tests/ has no hit. One stale
statement remains, listed below.

Traceability: TC-196 verifies FR-151, whose AC-2 covers the inheritance-cycle
refusal. The TC-235 cycle runs cite FR-208-AC-12, which says nothing about
cycles (see FND-002).

`quire validate` (quire 0.36.2, engine 0.50.2) on all three changed files: both
TC files pass. model-complete.md fails on "required 'type' is missing from
frontmatter". That failure exists on main too, because the PR does not touch
the frontmatter. The DuplicateArchetype, DuplicateInverseEdge and
semantic.inline-data-schema warnings are known tooling noise.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | TC-196's procedure text still says "Preserve ... R01's cycle uniqueness". R01 now expects two refusals, one at `A` and one at `B`, so "uniqueness" reads as the old single-refusal rule and contradicts the edited row. The intended invariant is probably "a cycle is detected and reported at its first closing extension only; a later closing with the same edge set is charged, not reported again". That needs restating, or the clause needs to go. | spec/test-cases/TC-196-conformance-redefinition-and-dispatch.md:80 | wrong-requirement |
| FND-002 | low | TC-235 M17 run 3 and M20 run 4 trace their per-declaration specialization-cycle refusals to FR-208-AC-12. That AC names no cycle rule; only FR-208's Generalization body (:232-233) and FR-151-AC-2 do. The edited oracle therefore backs no AC that states it. TC-235 does not verify FR-151. Pre-existing, but carried forward in the rows this PR rewrites. | spec/test-cases/TC-235-business-construct-meaning-binding.md:96, :99 | correct-requirement-no-evidence |

## Verdict

Two findings. One medium: a stale "cycle uniqueness" clause in TC-196 that
contradicts the edited R01. One low: the TC-235 cycle runs trace to an AC that
does not state the cycle rule. The IDs are unchanged and the sweep is otherwise
clean. The model-complete validate failure is the pre-existing missing
frontmatter type.

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | d69d51507ffd0a3843d4d9f91c6d7ba8e2bdebee: TC-196:80 now preserves "R01's cycle reporting (detected and reported at the first closing extension only; later closings charged, not reported)", which agrees with the edited R01 and model-complete. |
| FND-002 | fixed | d69d51507ffd0a3843d4d9f91c6d7ba8e2bdebee: FR-208-AC-12 now states that a supertypes cycle among these meanings is refused specialization-cycle once per declaration on the cycle, each refusal listing the cycle's keys. This changes the AC's content, not a matrix row, and it agrees with model-complete. The TC-235 M17 and M20 citations now trace to an AC that states the rule. The FR-208 validate output is identical to main's. |
