---
id: SR-16250
title: "Spec review of quire-specification PR #412: STD-423, FR-331 cross_check member, cross-check-disagreement cause and tool-failure byte-limit payload"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@3def7183957da351c87e80987c36efb5e964f83b; PR #412 (git diff origin/main...3def7183): proposals/backend-provider-v1/schema.json, proposals/backend-provider-v1/vectors.json, spec/functional/ir-proof/FR-349-register-smt-backend.md, spec/functional/ir-proof/FR-354-record-proof-trust-basis.md, spec/functional/tooling/FR-462-speak-the-plugin-wire.md, spec/objects/interfaces/FR-331-backend-provider-envelope.md, spec/test-cases/TC-269, TC-428, TC-431, tests/backend_provider_v1.rs; read for context: QSL PR #1329 FR-290/FR-291, QSpec open PR #391"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-349
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-354
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-462
    type: reviews
---
# SR-16250: Spec review of quire-specification PR #412: STD-423

## Summary

Ticket: STD-423 (QSpec half of STD-372 / QSL #1329). Sub-analyses folded in: integrity, criterion strength (manual, no Jev), cross-repo fidelity with QSL #1329, id uniqueness, scope-deferral check.

Examined:
- FR-331: the `cross_check` paragraph, the inconclusive-cause row, the member list, the carries table, the plugin-wire failure row, and FR-331-AC-39 and AC-40.
- FR-349: the cross-check paragraph and FR-349-AC-3.
- FR-354: the cross-check sentence and FR-354-AC-5.
- FR-462: the byte-limit text and table rows, and FR-462-AC-10.
- Tests: TC-269 TB-08 and TB-11, TC-428 PW-10, TC-431 BP-34 and BP-35.
- Fixtures: the schema's `cross-check-disagreement` arm, the two-arm `FailedCause` and `Result.cross_check`; the positive vector and nine adverse vectors; the two new trace tags.

Main is an ancestor of the head. FR-331-AC-39/40, FR-462-AC-10, TB-11, PW-10 and BP-34/35 are unique across open QSpec PR heads. QSL #1329's table and payloads use these spellings. No scope is deferred.

Gates re-run by this reviewer at 3def7183:
- `quire validate --scope . 'spec/**/*.md'`: 86 failure lines, equal to main.
- `cargo test --test checked_package_v2 --test backend_provider_v1`: 132 and 23 passed, 0 failed.
- QSL conformance `cargo test -p qsl-package --lib checked_v2`, run from QSL #1329's head with QSPEC_DIR at this head: 46 passed, 0 failed.

`git merge-tree` with open #391 conflicts in FR-331 and TC-431. Both PRs append rows after FR-331-AC-36 and BP-32. The PR that merges second must rebase.

## Verdict

**NOT MERGEABLE.** FND-001 is high: FR-349-AC-3 requires a `cross_check` member on an agreeing `refuted` result, and FR-331 and the schema forbid that member there. FND-002 and FND-003 are medium.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-349-AC-3 ends "An agreeing cross-check settles cvc5's result with `cross_check` verdict equal to it". For an agreeing counterexample (cvc5 `refuted`, Z3 `refuted`), that requires a `cross_check` member on a `refuted` result. But FR-331 says "No other result carries `cross_check`", the schema sets `cross_check: false` on every non-`proved` result, and adverse vector `cross-check-on-inconclusive` refuses it. QSL #1329 records the agreeing counterexample "with no `cross_check`". A test of AC-3 cannot pass both rules. Limit the sentence to a `proved` result, and state that an agreeing counterexample carries no member. | spec/functional/ir-proof/FR-349-register-smt-backend.md:92 |
| FND-002 | medium | FR-331 restricts `cross_check` to "a `proved` result of the SMT backend". It also says a `verdict` other than `proved` "records a disagreement over a proof the core certified", since an uncertified disagreement settles `inconclusive` (FR-349). Neither restriction is enforced or tested. The schema admits `cross_check` on any `proved` result, including a plugin's `trusted` proof and an `uncertified` proof with verdict `refuted`. FR-331-AC-39 has no adverse vector for either case, and no reader cause is named. Add both as refused cases (schema or reader, with the `invalid_result` cause) and vectors. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:110-121,721; proposals/backend-provider-v1/schema.json:997 |
| FND-003 | medium | The `cross-check-disagreement` cause admits `primary` `inconclusive`, but FR-349 covers disagreements only "over an `uncertified` proof or over a counterexample". Two cases have no stated settlement: cvc5 `inconclusive` with Z3 `proved` or `refuted`, and a certified proof with Z3 `inconclusive`. The second is presumably kept `proved` with verdict `inconclusive`. Two implementations can settle these differently. State both in FR-349 and add them to AC-3 and TC-269. QSL #1329's cross-check table needs the same rows (SR-16249 FND-005). | spec/functional/ir-proof/FR-349-register-smt-backend.md:63-72 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 4bae564d (fix carried in merge 642d7acd): FR-349-AC-3 now reads "a replayed counterexample with Z3 `refuted` settles `refuted` with no `cross_check`", matching FR-331 and the schema. |
| FND-002 | fixed | 4bae564d: FR-331 says "The reader SHALL refuse before consumption a `cross_check` on any result other than `proved`; on a `proved` result whose `basis` is not `bounded-complete` or `inductive` ... or whose `certification` is `trusted`; and a `verdict` other than `proved` on a proof that is not `certified`". Three schema `if/then` rules enforce it, and vectors `cross-check-on-trusted`, `cross-check-on-non-smt-basis` and `cross-check-disagreement-on-uncertified` test it (FR-331-AC-39, BP-34). Checked by hand: result 2 of `certification-outcome` (trusted, `exhaustive`) is refused by the trusted and basis rules; result 5 (certified `reduced`) by the basis rule; result 5 of the new vector (uncertified `inductive`, verdict `refuted`) by the uncertified rule alone. |
| FND-003 | fixed | 4bae564d: FR-349 states a seven-row table. Checked by hand against FR-354 ("never changes the certification") and FR-331: certified with any Z3 verdict stays `proved`, `certified`; uncertified with `proved` keeps `cross_check` `proved`; uncertified with `refuted` or `inconclusive` settles `inconclusive` with `primary: proved`; a replayed counterexample with `refuted` stays `refuted` with no member; a counterexample with `proved` or `inconclusive` settles `primary: refuted`; cvc5 `inconclusive` with `proved` or `refuted` settles `primary: inconclusive`; and both `inconclusive` keeps cvc5's cause. TB-12 (certified, Z3 `inconclusive`) and TB-13 (cvc5 `inconclusive` against each Z3 verdict) match rows 1, 6 and 7. Positive vector results 5 to 7 match rows 2, 1 and 6. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | The new FR-349 table has no blank line after its last row. Line 77 reads "\| `inconclusive` \| `inconclusive` \| cvc5's `inconclusive` with its own cause, with no `cross_check` member \| When Z3 is not configured in the SMT descriptor, no cross-check". The paragraph that follows (Z3 absent or failing its probe: cvc5 settles alone with the `cross-check-absent{tool}` warning, the rule behind FR-349-AC-6) is therefore parsed as a fourth cell of the table and its continuation, not as prose. Insert a blank line between the table and the paragraph. | spec/functional/ir-proof/FR-349-register-smt-backend.md:77-80 |
| FND-005 | low | Adverse vector `cross-check-on-trusted` patches `certification-outcome` result 2, whose basis is `exhaustive`. The basis rule already refuses that member, so the vector never shows the `trusted` rule refusing on its own. Base it on a trusted proof whose basis passes the basis rule, or state that no plugin `proved` result can carry `bounded-complete` or `inductive`. If so, the trusted rule is redundant and can go. | proposals/backend-provider-v1/vectors.json (cross-check-on-trusted) |
