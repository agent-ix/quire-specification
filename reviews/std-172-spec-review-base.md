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
| FND-004 | fixed | - |
| FND-005 | fixed | - |

Round 2, reviewed at e8958629d320a4f782260e6c521e1fb1baf212be. The FND-004 and
FND-005 rows above are round 2's; FND-001 to FND-003 needed no new row. The
named-population unclosed-subtypes case FND-005 asked for is present. Its
expected result is new finding FND-007.

Round 3 (final record pass), reviewed at 2b006cd52014ce6dc331bf6bc429eaeb9a182a35.
`lookup<M::A>(M::PA, r2) absent refused = r2` is valid. `lookup ... absent
absence-mode` is a single `primary` (shared-grammar.md:361-363), so `= r2` compares
the refused-mode `Reference<M::A>` with `r2`, the same pattern L08 uses.
`absent empty = none` compares the `empty`-mode `Option<Reference<M::A>>` with the
`none` primary (shared-grammar.md:350). FR-149 makes two `none` values one equal pair,
so R's lookup over the post population, where `a2` is deleted, is `true`. The `[B]`
case with `b1` only now holds. `b1` is admitted because `B` covers itself. `A`
conforms to `M::A` and `[B]` does not cover it, so `unclosed-subtypes` names `A`
(FR-153:89-92). L02 is corrected the same way. Changed spec files validate.

| FND | outcome | reason |
| --- | --- | --- |
| FND-006 | fixed | - |
| FND-007 | fixed | - |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | L14's Q and R bodies compare the lookup with some(r2), but QSpec has no some form. shared-grammar.md:350 has 'none' as a primary and no some constructor, and the only some( in spec/ and proposals/ is this vector. So some(r2) resolves as a call to an undeclared function, missing_declaration/missing-name, and Q and R refuse instead of evaluating true. Write the present case as QSpec does elsewhere: present(lookup<M::A>(M::PA, r2) absent empty) and value(lookup<M::A>(M::PA, r2) absent empty) = r2, or use the refused mode, lookup<M::A>(M::PA, r2) absent refused = r2, as L08 does. | spec/test-cases/TC-198-closed-model-lookup.md:72; proposals/quire-v1/shared-grammar.md:350 |
| FND-007 | medium | The new L14 case, PA with member types [A] and P1 without b1, querying allInstances<M::A>(M::PA), expects unclosed-subtypes naming the uncovered type B. FR-153 says "A member type covers itself and every type that conforms to it", and B conforms to A, so [A] covers B and subtype closure holds for M::A. L12 relies on that same coverage, admitting b1 under [A] through one ancestor edge. A correct engine therefore returns the set [a1, a2] and fails the vector. L02's second case already has this expectation on main and needs the same fix. For an uncovered subtype, use member types that do not cover the queried type, for example PA with member types [B] queried as allInstances<M::A>(M::PA), where A itself is uncovered. | spec/test-cases/TC-198-closed-model-lookup.md:72; spec/test-cases/TC-198-closed-model-lookup.md:60; spec/functional/type-model/FR-153-query-closed-model-environments.md:89-92 |
