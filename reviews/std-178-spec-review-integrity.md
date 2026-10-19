---
id: SR-12897
title: "Integrity review of PR #282: strong refinement fairness against FR-362, FR-377 and FR-380"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@88bf7df; FR-375 Surface syntax, Fairness rows, Checked-package node, AC-10; FR-362 Syntax and resolution, Weak and strong fairness, Identity; FR-377 Abstract progress, Refinement; FR-380 Fairness implication; FR-432; shared-grammar protocol-step rows; quire-spec-language FR-137, TC-542, ADR-020 section 1 and CO-3"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-362
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-380
    type: reviews
---

## Summary

Ticket: STD-178. PR quire-specification#282. Checks that FR-362's strong
fairness applies unchanged to a refinement fairness row, that the requirements
that read `F_A` and `F_C` stay consistent once those sets can hold strong
constraints, and that QSL FR-137-AC-4, TC-542 and ADR-020 section 1 now agree
with FR-375.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-380's fairness implication is stated with no kind: "Every constraint of `F_P` is in the refinement's `F_A`, or is the `whole` form of an `each` constraint in `F_A`." Before this PR every `F_A` entry was weak, so kind could be ignored; now `F_A` can hold strong constraints. If an implementer matches the "`whole` form" on operation and granularity only, `F_P` = (strong, op, whole) is accepted against `F_A` = (weak, op, each), and the transfer is unsound, because weak does not imply strong. The rule also misses the sound case (strong in `F_A` implies weak in `F_P`, FR-362 "Strong implies weak"), so that transfer settles `transfer-not-preserving`. QSL ADR-020 CO-3 has the same wording. | spec/functional/analysis/FR-380-transfer-claims-through-refinement.md:79-80; spec/functional/temporal/FR-362-apply-fairness-constraints.md:117-122 |
| FND-002 | low | A kindless `fair` means `weak` in a temporal clause (FR-362: "An unwritten kind is `weak`") but is refused in a refinement row (FR-375-AC-10). Choosing this is allowed, but FR-375 gives no reason for the difference. Open QSL PR #970 already accepts a kindless row and reads it as `Weak` (its FND-004 owner question), so that PR has to change. | spec/functional/analysis/FR-375-declare-model-refinement.md:71-80,210; spec/functional/temporal/FR-362-apply-fairness-constraints.md:54,65 |

## Verdict

Mergeable. FR-362's semantics carry over to a refinement row unchanged:
FR-375 resolves each row to the FR-362 triple (kind, operation,
granularity), and FR-377's stutter-tail rule ("enabled at the tail's mapped
state and no step of the tail takes it") is correct for both kinds, because
the mapped state on a stutter tail is constant. The node body carries `kind`,
so the weak and strong forms differ in identity, as FR-362's Identity section
requires. QSL FR-137-AC-4, TC-542 step 4 and ADR-020 section 1
(`fair (weak | strong) [each | whole]`) now agree with FR-375. FND-001 is a
one-sentence propagation fix to FR-380 that should go in with this PR or in a
tracked follow-up. Outside the diff, and not this PR's to fix: FR-375's
header is `'abstract', ident`, a model alias only, but FR-432 admits an
abstract protocol subject. QSpec does not spell how a declaration selects
one. See the QSL scope-boundary review SR-12901 FND-002.
