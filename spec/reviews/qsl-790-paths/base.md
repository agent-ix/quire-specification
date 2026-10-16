---
id: SR-11408
title: "Spec review of quire-specification PR #261: field and member steps, clause-set results, relationship-end navigation paths"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@ed67cdb4c7b33326b91d948f1fbb2e0f490cc68c; spec/objects/foundation/FR-207-runtime-value-path.md, spec/objects/foundation/FR-208-quire-meaning-vocabulary.md, spec/test-cases/TC-235-business-construct-meaning-binding.md, spec/test-cases/TC-429-value-path-vocabulary.md"
review_set: subset
---
# Spec review of quire-specification PR #261

## Summary

Ticket: QSL-790. One pass of spec-review with the contradiction, ambiguity,
completeness, testability and spec-integrity sub-analyses over the PR diff
(`origin/main...HEAD`). Spec-only PR, so no code review. Only major issues
block. Mediums and lows are recorded in the Verdict.

Oracles, merged QSL `origin/main`: FR-107 (the receiver path, read paths,
clause sets, FR-107-AC-7 to FR-107-AC-18), TC-1039, TC-1040 (steps 6, 7, 9),
FR-363, FR-364, FR-265-AC-7, TC-1033 fixture. QSpec rules: FR-047-AC-9,
FR-208 (record value invariants, state machine invariants, State-machine
context anchors, Locations, FR-208-AC-10, FR-208-AC-11).

Examined:
- FR-207 step table, the new member/field paragraph, the new
  Relationship-end navigation section, FR-207-AC-12 (examined)
- FR-208 record-value invariant location prose (examined)
- TC-235 M13, M16 (run 3), M22 (examined); M15 runs 1 to 3 (examined, as the
  neighbour of M16 run 3 under the same clause-set rule)
- TC-429 VP-03, VP-06 (examined)
- FR-047-AC-9, FR-208-AC-10, FR-208 Locations (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | TC-235 M15 run 2 expects "exactly one result", the precondition `false` at `p1`. M15's own runs 1 and 3 report `Consistent` at `p1` and `q1` in the same evaluation, and FR-208-AC-10 says a state-machine invariant holds in the pre and post observation of every invocation. QSL FR-107's clause-set rule never skips invariants. So under the clause-set rule that this PR now states in M16 run 3, run 2 also yields `Consistent` `true` at `p1` and at `q1`. "Exactly one result" cannot be reached. Fix: scope run 2 as M16 run 3 is scoped, so the clause set of `place` (precondition and postcondition) reports exactly one result, and `Consistent` is `true` at `p1` and at `q1`. | spec/test-cases/TC-235-business-construct-meaning-binding.md:94 |

## Verdict

FAIL, one blocking finding (FND-001).

Traced and correct:
- FR-207 step table and the new paragraph: a selection from a declared object
  is `member`, from a record value or a state-machine context is `field`. This
  matches QSL FR-107 read paths, FR-265-AC-7 and TC-1040 steps 1 to 9.
- FR-208 location prose agrees with it. FR-208 Locations (context fields as
  `field`) is unchanged and consistent.
- TC-235 M13: the three object-reached paths are now `member`, still in key
  order (discount, history, quoted). The parameter path is unchanged.
- TC-429 VP-03 matches TC-1040 step 1's `[member parent, option_value,
  reference object(root), member versionNumber]` pattern.
- FR-047-AC-9 is unchanged and M22 still agrees with it: the parameter path
  orders before the receiver path.
- TC-235 M22: the parameter read now comes from `place`'s postcondition at
  `q1`, matching TC-1040 step 7 (`FlagsPlaced`). The receiver half matches
  TC-1040 step 6, and the row does not claim the invariant's observation is
  admitted from a snapshot. Replacing `ensures` adds no declaration, so 25
  charges holds.
- TC-235 M16 run 3: `3 < 3` is false, so the clause set has one entry and
  outcome `False`, and the postcondition is skipped. This matches FR-107's
  clause-set rules and TC-1039 step 1 (`up34`). A process invariant is
  evaluated at invariant observations (FR-208-AC-10), so leaving it out of the
  set is right.
- FR-207 Relationship-end navigation, FR-207-AC-12 and TC-429 VP-06 match
  FR-107's navigation bullet, FR-107-AC-17 and TC-1040 step 9
  (`(N, object(t1), [member level])`, no entry for `self.home`). They also
  match FR-363 and the TC-1033 fixture: `home` is the near (source) end role,
  and the destination end `Tank` supplies `t1`.

There is no tracking, hash or version ceremony in the diff. I found no
leftover `field` used where `member` is meant: grep finds no other `field`
step on an object-rooted path in `spec/`.

Recorded (not blocking):
- medium: the multi-object sentence in Relationship-end navigation ("located
  once for each referenced object") has no AC or vector. It also does not say
  how a quantifier element bound from a navigation result is rooted.
- medium: VP-03 does not say that `self` is a declared object. `member owner`
  is right only if it is.
- low: "a single member step" in Relationship-end navigation reads as the
  whole path. A deeper selection such as `self.home.config.x` extends that
  step.
- low: FR-207 says "records no entry", and "entry" is a QSL `reads()` term
  that QSpec does not define.
- low: FR-207-AC-12 and VP-06 do not name the root observation.
