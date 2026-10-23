---
id: SR-16277
title: "spec-review of cardinality termination measure by removal filters (quire-specification PR 403)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@01d94924; spec/functional/expressions/FR-146-check-total-pure-functions.md; spec/test-cases/TC-191-total-pure-function-checking.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-146
    type: references
---

# Spec review of cardinality termination measure by removal filters (quire-specification PR 403)

## Summary

Ticket: STD-401. One spec-review pass over the PR's own delta (`git diff origin/main...01d94924`), checked for the no-drop/no-narrowing/no-deferral rule, testable vectors (key vectors derived by hand), id uniqueness across open PR heads, and schema/vector consistency.

Examined, clean:

- Soundness, hand-derived: a cardinality element is a natural number and the lexicographic order over naturals is well founded. Each removal shape is false exactly for occurrences equal to `e` under the FR-144 membership relation, and `contains(p, e)` proves at least one exists, so the argument's bound count is at most `size(p) - 1`. The third shape `not contains(d, x)` with `contains(d, e)` drops at least `e`. A same-kind `convert` keeps the count.
- Float handling is sound against #419: `x != e` over a float-bearing element is `numericEqual` (NaN != NaN drops nothing; +0/-0 are one under `=` but two members), so excluding it and using `bitIdentical` or `not contains(d, x)` is correct.
- P13 (1) 4 calls, returns 1; (2) 0; (3) 1; (9) 1, then `cardinality_out_of_bound`; (12) lexicographic; (13) {qNaN,+0} minus qNaN, then `contains` false, returns 0. All hand-derived and match.
- Gates at 01d94924: quire validate 86 failure lines; cargo test checked_package_v2 134 passed, backend_provider_v1 24 passed. No test-parsed vector changed. This head includes #405 (eaed9b73).

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Merge-order dependency. P13 (13) to (15) and the float clauses of the strictly-smaller rule assume #409 (Set<Float64> and Set<R> with a float leaf admitted) and #419 (`!=` over a float-bearing type typed as numericEqual). #403's base is main, where both declarations and `x != e` over R are `ill_typed`/`operator-ineligible`. On main, (13) and (14) are type errors and (15) refuses `ill_typed`, not `unproved-decrease`. Fix: stack #403 on #419, or merge it only after #419 and say so in the PR. | spec/test-cases/TC-191-total-pure-function-checking.md | wrong-requirement |
| FND-002 | medium | STD-401's scope names "tail / remove-one". Every admitted decreasing form removes all occurrences equal to `e`. A positional walk of a Sequence or Bag with duplicates (process one occurrence, recurse on the rest) has no decreasing form, because removing one occurrence changes the result. Either add a one-occurrence form (for example a tail or a remove-first of a sequence parameter under `size(p) > 0`), or have the lead confirm that remove-all covers the ticket. Lead default, not an owner answer. | spec/functional/expressions/FR-146-check-total-pure-functions.md | missing-requirement |

## Verdict

Mergeable after FND-001, FND-002 (medium) are fixed or dispositioned by the lead.

## Dispositions

Round 1, at 2ef818ee. Hand-derived P14: walk(sequence[7,7,7],0) = 3 after 4 calls; contains-guarded walk(sequence[5,5],5) = 6 after 2 calls; w1(sequence[1,2]) = 0; drain(bag[1,1,2],1) = 2 after 3 calls; drain(sequence[1,2,1],1) = 2; Bag<Float64> {+0,+0,-0} with k = +0 gives 2 (bit identity); FR-145 bounds dropFirst [max(a-1,0),max(b-1,0)] and removeOne [max(a-1,0),b] match Q13/Q14. The three identities (sequence.drop_first, ordered_set.drop_first, collection.remove_one) are in both schemas.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | #403 is now based on spec/std-397-ieee-negate-equality (#419), which carries #409's float elements and #419's float `=`/`!=`, so P13 (13)-(15) type-check as written. |
| FND-002 | fixed | A *one-occurrence removal* is `removeOne(p, e)` (FR-145) for a stable path `e` with the membership fact `contains(p, e)` ... A *tail* is `dropFirst(p)` (FR-145) for a `p` of `Sequence` or `OrderedSet` kind with a fact at that call site that `size(p) >= 1` |
