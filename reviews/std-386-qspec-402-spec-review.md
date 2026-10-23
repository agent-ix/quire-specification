---
id: SR-16336
title: "spec-review of TC-381 IB-07 activation guard reads the trigger record only (quire-specification PR 402)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@901c148e; spec/test-cases/TC-381"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-428
    type: references
  - target: ix://agent-ix/quire-specification/FR-464
    type: references
  - target: ix://agent-ix/quire-specification/FR-143
    type: references
  - target: ix://agent-ix/quire-specification/FR-146
    type: references
---

# Spec-review of TC-381 IB-07 activation guard reads the trigger record only (quire-specification PR 402)

## Summary

Ticket: STD-386. One spec-review pass over `git diff origin/main...901c148e`, checked for the no-drop/no-narrowing/no-deferral rule, testable vectors (key vectors derived by hand), id uniqueness across all open PR heads, cross-spec names, and mergeability against main.

Examined, clean:

- The edit stops reading model state in the guard, which is what FR-464 "Slots that bar model state" requires of the activation guard, and the settlement (`refuted`, `undefined-evaluation`, one-state prefix, `where` naming the activation guard, `activate` not disabled) is FR-428-AC-7. No deferral language.

Merge status: origin/main 0eef131f is not an ancestor; `git merge-tree` is clean; no other open head touches TC-381. (Old heads #267, #207, #59 conflict with every PR.)

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The new guard `value(t.n) = 0` has no `present(t.n)` fact. QSpec FR-143 and FR-146 give `value(r.f)` without a proved `present(r.f)` a static `undefined_expression`/`unproved-presence` refusal when the body is linked, and QSL FR-104 "Definedness facts" and FR-107 apply the same obligation to state clauses ("a division and a `value(..)` carry nonzero and presence obligations"). If the activation guard goes through that check, the unit is refused before model checking and the vector never reaches `undefined-evaluation`. The corpus produces undefined guards with a `mod` whose divisor can be zero (as QSL FR-126-AC-9 and FR-128-AC-6 do) or with `lookup ... absent undefined` (as TC-380 CP-10 does), which the activation guard may not use. Fix: derive undefinedness from a form S3 does not discharge, for example a `mod` by a trigger field over `Int[0, 1]`, or cite the rule that exempts an activation guard from the presence obligation. The vector also gives `Tick` a field while the procedure says "Tick with one value"; say that IB-07 redefines `Tick`. | spec/test-cases/TC-381-per-trigger-activation-and-instance-budget.md:53 | wrong-requirement |

## Verdict

**Still needed, not stale: the QSpec half of STD-386 is not on main** (main's TC-381 IB-07 still has the `lookup<M::Gate>` activation guard that contradicts FR-464). The QSL half, #1324, merged. **Mergeable after FND-001 (medium) is fixed or dispositioned.** Gate `quire validate --scope . 'spec/**/*.md'`: 86 failure lines (baseline). No test or fixture parses TC-381 (searched tests/ and proposals/), so I did not run cargo. `git merge-tree` against main is clean (main is not an ancestor). Only #402 touches TC-381 among the open heads.

## Dispositions

| FND | Round | Outcome | Reason |
| --- | --- | --- | --- |
| FND-001 | 1 | fixed | Fixed in head 7d4612a2; the fix agrees with the finding and the vectors were derived by hand. |

Round 1 verdict: Finding fixed, no new defect; mergeable (merge-tree clean; validate 86).
