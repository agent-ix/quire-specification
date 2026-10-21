---
id: SR-13660
title: 'Code review of QSpec PR #317: observed-arm vectors and TC-431 test (STD-147)'
type: SpecReview
analysis: code-review
scope: 'agent-ix/quire-specification@ae1ebe4d4; tests/backend_provider_v1.rs tc_431_adverse_vectors_are_refused; proposals/backend-provider-v1/schema.json TemporalCounterexample; proposals/backend-provider-v1/vectors.json observed-counterexample-outcome and its five adverse patches; ticket STD-147'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-364
  type: reviews
---

## Summary

Ticket: STD-147. PR quire-specification#317, head ae1ebe4d4bebbb8a79e0e511d511391587a91ca1.

I removed each new schema clause separately and ran the suite, plus a throwaway probe test (local only, reverted) that ran each adverse vector and some extra documents through the whole-document validator. Results:

- Removing the whole conditional fails `tc_431_adverse_vectors_are_refused`. So does removing only `then.initial_state`, only `then.over_binding`, only `then.kind`, only `then.prefix` or only `else.prefix`. Each of the five adverse patches is refused by exactly its own clause and is admitted when that clause is removed. This confirms the author's three mutation claims and extends them to every patch.
- Removing `then.loop` or `else.loop` leaves the whole suite green.
- The positive `observed-counterexample-outcome` and `temporal-outcome` stay admitted.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| tc_431_adverse_vectors_are_refused | tests/backend_provider_v1.rs | examined |
| tc_431_positive_vectors_are_admitted_with_owned_members | tests/backend_provider_v1.rs | context_only |
| TemporalCounterexample arm conditional | proposals/backend-provider-v1/schema.json | examined |
| observed-counterexample-outcome | proposals/backend-provider-v1/vectors.json | examined |
| observed-counterexample-names-an-initial-state | proposals/backend-provider-v1/vectors.json | examined |
| observed-counterexample-with-a-model-step | proposals/backend-provider-v1/vectors.json | examined |
| observed-counterexample-deadlock-kind | proposals/backend-provider-v1/vectors.json | examined |
| observed-counterexample-over-binding | proposals/backend-provider-v1/vectors.json | examined |
| model-counterexample-with-a-document-step | proposals/backend-provider-v1/vectors.json | examined |
| FR-364-AC-10 | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Two of the new schema clauses are never exercised. Deleting `then.loop` (observed `loop` items require `document`) or `else.loop` (model `loop` items require `transition`/`post_state`) leaves the suite green. Every adverse patch targets `prefix/0`, and the observed positive has `loop: []`. A probe confirms that without the clause an observed counterexample with a model step in `loop`, and `temporal-outcome` with a `document` step in `loop`, are both admitted. The new paragraph says "only `document` steps", which includes the loop. Fix: add two adverse vectors, an observed counterexample whose `loop` holds a model step, and `temporal-outcome` with `/counterexamples/0/counterexample/loop/0` replaced by a `{document}` step, and add both to BP-23 and AC-10. | proposals/backend-provider-v1/schema.json:500; proposals/backend-provider-v1/schema.json:504 |
| FND-002 | low | Two parts of the new rule have no vector. First, the exclusion of `time-lock` from the observed arm: replacing `kind` with `time-lock` alone is refused by the older `final_delay` conditional, so a discriminating vector must also add `final_delay`. Verified: with `then.kind` removed, an observed `time-lock` plus `final_delay` is admitted. Second, the observed arm's admission of the undefined-evaluation envelope. FR-364-AC-10 names neither. Fix: add an adverse vector `observed-counterexample-time-lock-kind` that replaces `kind` and adds `final_delay` (this needs two patch operations, so either allow two in this test or base it on a time-lock positive with `trace` switched to observed), and an observed envelope positive. Extend AC-10 to cover both. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:205; proposals/backend-provider-v1/schema.json:497 |
| FND-003 | low | The `FR-364-AC-10` tag sits on the generic adverse loop, which iterates over whatever adverse vectors exist and does not check for the five FR-364 ones. Deleting any or all of them keeps the test green with the tag still present. The admission half of AC-10 ("admits an observed-arm formula counterexample ... with two `document` steps") is checked by `tc_431_positive_vectors_are_admitted_with_owned_members`, which carries no FR-364-AC-10 tag. Fix: tag the positive test with FR-364-AC-10 as well, and assert by name that the BP-23 vectors are present, both the positive and the adverse ones. | tests/backend_provider_v1.rs:292; tests/backend_provider_v1.rs:219-223 |

## Verdict

Not mergeable as is, because of FND-001: two of the seven new schema clauses can be deleted without any test noticing. Apart from that, the five adverse patches each discriminate their own clause, and the test change is a tag only.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | The refusal half of FR-364-AC-10 is now traced only to `tc_431_the_arm_coupling_vectors_are_present`. Its doc comment says "the coupling vectors are refused", but it only checks that the names exist. The test that does assert refusal, `tc_431_adverse_vectors_are_refused`, lost its `FR-364-AC-10` tag in this round. Fix: restore the tag on `tc_431_adverse_vectors_are_refused` (keeping it on the presence test is fine), or make the presence test validate each named adverse vector, and correct the doc comment. | tests/backend_provider_v1.rs:812-816; tests/backend_provider_v1.rs:287-294 |
| FND-005 | low | No vector exercises the observed branch's plural `state_keys` exclusion. Removing `{"required": ["state_keys"]}` from the observed branch's `where` clause leaves the suite green. A probe confirms that an observed envelope with `where.state_keys` is then admitted. Fix: add an adverse vector `observed-undefined-evaluation-with-state-keys`, and name it in AC-10 and BP-23. | proposals/backend-provider-v1/schema.json:500 |

## Dispositions

Round 1, head 0f1cba012.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `observed-counterexample-with-a-model-step-in-the-loop` and `model-counterexample-with-a-document-step-in-the-loop` were added. Removing `then.loop` or `else.loop` now fails `tc_431_adverse_vectors_are_refused`, and each vector is admitted only when its own clause is removed (verified). |
| FND-002 | fixed | `observed-counterexample-time-lock-with-final-delay` replaces the whole counterexample in one op, so the `final_delay` conditional is satisfied and only `then.kind` refuses it (verified admitted with `then.kind` removed). The positive `observed-undefined-evaluation-outcome` is admitted. AC-10 now names both. |
| FND-003 | fixed | `tc_431_positive_vectors_are_admitted_with_owned_members` carries FR-364-AC-10, and the new `tc_431_the_arm_coupling_vectors_are_present` asserts the two positive and eleven adverse vectors by name. The tag moved off the refusal test, which is FND-004. |
