---
id: SR-12850
title: "Base review of PR #278: FR-153 population named by qualified name"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@eb852d41d0c5ee5510b19e373df41555210a0105; PR #278 diff against origin/main 1da546f: FR-153 Behavior + FR-153-AC-12, TC-198 L14, FR-383-AC-13, TC-326 step 7, proposals/quire-v1/shared-grammar.md:450"
review_set: subset
---
# Base review of PR #278 (STD-172)

## Summary

Ticket: STD-172. Checklist pass over the five changed files: ID formats, AC
testability and test-case oracle strength. IDs are well formed and do not
collide: FR-153-AC-12 and TC-198 L14 are the next free ids, and no open PR
(#267, #273, #279 for STD-171, #280 for STD-173, #207, #200, #59) touches
FR-153, TC-198, TC-326 or FR-383. #279 edits shared-grammar.md only at lines
573 and later, so it does not overlap line 450. The refusal codes exist in
native-diagnostics.md: missing_declaration/missing-name (line 66),
ill_typed/operator-ineligible (line 71) and incomplete_population/incomplete-scope
(line 78).

## Verdict

No high findings. Two medium findings are oracle gaps in the new test vectors,
and there is one low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-198 L14 lists three forms (allInstances, pre(allInstances), lookup ... absent empty) "in" an invariant, a pre clause and a post clause, but it does not say which form goes in which clause. pre(e) in an invariant or a precondition is a forbidden-pre-read (QSL FR-104-AC-4), so read literally the vector expects a refusal it never states. The expected column also gives no result for the lookup at all: present a2 at pre, none at post, under L07. So a mutant that reads lookup at the wrong observation passes. The invariant's current snapshot is not supplied either, because L07 is an invocation (pre/post only), yet the vector expects [a1, a2, b1]. | spec/test-cases/TC-198-closed-model-lookup.md:72 |
| FND-002 | medium | TC-326 step 7 now writes allInstances<Counter>(M::counters), but the TC-323 subject it builds on declares no model alias M. The type argument is also unqualified, while FR-153 requires T to resolve from a qualified name through the alias (QSL FR-155-AC-2 and FR-391-AC-1 write allInstances<M::Counter>). As written, the step resolves M to nothing and refuses missing-name before any footprint is derived, so the case-7 oracle cannot be reached. | spec/test-cases/TC-326-receiver-scoped-frames-and-footprints.md:42; spec/test-cases/TC-323-partial-order-reduction-vectors.md:22 |
| FND-003 | low | L14 tests only = and let as value uses. FR-153 also refuses the name as an operand of any other operator. QSL TC-1076 row 4 also covers size(M::P) and a clause body that is the bare name; adding one of those would catch a checker that special-cases = only. | spec/test-cases/TC-198-closed-model-lookup.md:72; spec/functional/type-model/FR-153-query-closed-model-environments.md:131-132 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | L14 now gives Q and R the bodies allInstances<M::A>(M::PA) and lookup<M::A>(M::PA, r2) absent empty. Those bodies are a Set and an Option, not a Boolean, so read literally each clause refuses ill_typed/non-boolean-root before any expected value can be observed. The same vector then expects a body that is M::PA to refuse operator-ineligible, so the vector does treat "body" literally. Wrap each form in a Boolean, as I's count(...) = 3 does, or say "a sub-expression of the body". | spec/test-cases/TC-198-closed-model-lookup.md:72 |
| FND-005 | low | New FR-153-AC-14 also requires unclosed-subtypes for an uncovered subtype through a named population, but L14 exercises only the open extent and the missing row. L02 covers unclosed-subtypes for the parameter binding p only. Add an M::P case, for example allInstances<M::A>(M::PA2) with PA2 of L12. | spec/test-cases/TC-198-closed-model-lookup.md:72; spec/functional/type-model/FR-153-query-closed-model-environments.md:189 |

## Dispositions

Round 1, reviewed at 561f23fcf64f179b9632072134dd9d7574abdfb0. Changed spec files validate.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
