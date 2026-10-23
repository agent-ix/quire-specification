---
id: SR-16221
title: "Spec review of quire-specification PR #399: FR-272 causes for invalid_quire_context and unimplemented_capability"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@ee68a9810ec04083a4aa22f51f9813953e74b3d7; PR #399 (git diff origin/main...ee68a981): proposals/quire-v1/definitions/native-diagnostics.md, spec/objects/foundation/FR-272-typed-diagnostic-cause.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, tests/checked_package_v2.rs; read for context: QSpec FR-271, FR-300; QSL PR #1306 FR-298 and PR #1316 FR-294/FR-401, QSL main FR-286"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-272
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-233
    type: reviews
---
# SR-16221: Spec review of quire-specification PR #399

## Summary

Tickets: STD-349 and STD-350 (the QSpec half of QSL #1306 and #1316). Sub-analyses folded in: integrity, criterion strength (manual, no Jev), cross-repo fidelity with QSL #1306/#1316, id uniqueness, scope-deferral check. Examined: the new `invalid_quire_context` and `unimplemented_capability` rows in FR-272 and native-diagnostics, FR-272-AC-24 and FR-272-AC-25, the TC-233 procedure sentence and `verifies` edges, and the two new tests. Main is an ancestor of the head. FR-272-AC-24 and AC-25 are unique across open QSpec PR heads. No other QSpec or QSL text uses either code with a cause, so closing the sets breaks nothing. The decisions are flagged as lead defaults.

Fidelity:
- The `unsupported-render-form` cause, with `view`/`form` and the six view kinds, matches QSL #1306's FR-298.
- `backend-not-supplied` (`backend`) and `untranslatable-entry` (`backend`, `location`, first construct in clause-node order) match QSL #1316's FR-294 `UnsupportedCause` and FR-401.

Gates re-run by this reviewer at ee68a981:
- `quire validate --scope . 'spec/**/*.md'`: 86 failure lines, equal to main.
- `cargo test --test checked_package_v2 --test backend_provider_v1`: 134 and 23 passed, 0 failed.
- QSL conformance `cargo test -p qsl-package --lib checked_v2`, run from QSL #1306's head with QSPEC_DIR at this head: 46 passed, 0 failed.

## Verdict

**MERGEABLE.** Merge it before or together with QSL #1306 and #1316. The one finding is low. If SR-16220 FND-007 on #1316 is resolved by adding a construct kind to `Untranslatable`, this PR's `untranslatable-entry` payload keys must gain the same key in the same round.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The `unimplemented_capability` set is now closed to three causes. QSL main FR-286's existing use, "An engine the request names that the driver has not built (AOT or JIT, where only the interpreter exists) ... `unimplemented_capability`", could be `backend-not-supplied` or `unimplemented-meaning`, and no text says which. Two producers can emit different causes for the same failure. Name FR-286's case in the `backend-not-supplied` description, or add a QSL FR-286 line in #1316. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:109 |
