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
