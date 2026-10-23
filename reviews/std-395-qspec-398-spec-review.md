---
id: SR-16321
title: "spec-review of FR-480 reference literals (quire-specification PR 398)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@9239a52efd; spec/functional/type-model/FR-480-check-and-resolve-reference-literals.md; spec/objects/interfaces/FR-322-checked-package-artifact.md; spec/test-cases/TC-965-reference-literals.md; proposals/quire-v1/shared-grammar.md; proposals/checked-package-v2/schema.json; proposals/checked-package-v2/node-identity-preimage.schema.json"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-480
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/TC-965
    type: references
  - target: ix://agent-ix/quire-specification/FR-381
    type: references
  - target: ix://agent-ix/quire-specification/FR-181
    type: references
  - target: ix://agent-ix/quire-specification/FR-204
    type: references
  - target: ix://agent-ix/quire-specification/FR-361
    type: references
---

# Spec review of FR-480 reference literals (quire-specification PR 398)

## Summary

Ticket: STD-395. One spec-review pass over `git diff origin/main...9239a52efd` (agent-ix/quire-specification#398), checked for the no-drop/no-narrowing/no-deferral rule, testable vectors (derived by hand), id uniqueness across open PR heads, and mergeability. Gate: `quire validate --scope . 'spec/**/*.md'` 86 failure lines (equal to origin/main) and `cargo test --test checked_package_v2 --test backend_provider_v1` pass.

Examined, clean:

- Hex vectors checked: `s1` is bytes 0x73 0x31 (`7331`), `s9` is `7339`, `s3` is `7333`. TC-965 RL-06 invalid values: `7G31` (non-hex), `733` (odd length), `73A1` (uppercase) and empty text, all refused `invalid_package`/`invalid-value`, per the FR-322 text.
- Fixed versus created-object: AC-4's `s1` (admitted by an initial record) is fixed and holds for `s1` only; `s3` (in the universe, admitted by no initial record) is undefined with `absent-key` at the initial state and `s3`'s reference after the creating operation; `s9` (in no universe) refuses `missing-name` before exploration. AC-5: fixed literal admitted in a model-state-barring slot, created-object refused at binding, `deref` refused. FR-381-AC-3 already refuses a literal naming a key of a symmetric population statically, so no ordering conflict with binding-time refusal.
- No `//` comment conflict with the `#` token (comments are `//` in the grammar); `#` is confined to this production.
- Ids FR-480, TC-965 and FR-322-AC-101 are unused on origin/main and every other open QSpec head (FR-481 and TC-966 on main are different artifacts). The ticket's follow-up note about citing the FR from QSL FR-067, FR-104 and FR-093 is code-time bookkeeping, not part of this PR.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - | correct-requirement-no-evidence |

## Verdict

Mergeable by content: no defects in this PR's own delta.

Conflicts with origin/main: FR-322 and spec.md (the FR-322 value_kind sentence and AC table, and the spec index; rebase needed). Open heads: spec.md conflicts with most; the value_kind enum and FR-322 conflict with #403, #405, #407, #408, #410, #413, #417, #418 and #419 (append-style: keep the union of enum members). Gates: `quire validate` 86 failure lines (identical to origin/main); `cargo test --test checked_package_v2 --test backend_provider_v1` pass; QSL conformance `cargo test -p qsl-package --lib checked_v2` (QSL origin/main, QSPEC_DIR a QSpec origin/main plus this PR with the FR-322 and spec.md conflicts resolved by union): 49 passed, 0 failed; the control run on QSpec origin/main alone also gives 49/0. Against the PR head own older base (13 commits behind) the same run fails 2 hybrid-node tests, which is the stale base, not this PR.
