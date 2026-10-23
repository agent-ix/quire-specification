---
id: SR-16320
title: "spec-review of min_count is optional, 1 when omitted (quire-specification PR 397)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@9b1caa5e4a; spec/functional/temporal/FR-414-evaluate-window-aggregates.md; spec/test-cases/TC-366-window-aggregates.md; proposals/quire-v1/shared-grammar.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-414
    type: references
  - target: ix://agent-ix/quire-specification/TC-366
    type: references
  - target: ix://agent-ix/quire-specification/FR-352
    type: references
---

# Spec review of min_count is optional, 1 when omitted (quire-specification PR 397)

## Summary

Ticket: STD-377. One spec-review pass over `git diff origin/main...9b1caa5e4a` (agent-ix/quire-specification#397), checked for the no-drop/no-narrowing/no-deferral rule, testable vectors (derived by hand), id uniqueness across open PR heads, and mergeability. Gate: `quire validate --scope . 'spec/**/*.md'` 86 failure lines (equal to origin/main) and `cargo test --test checked_package_v2 --test backend_provider_v1` pass.

Examined, clean:

- Vectors derived by hand on `x` values 1, 2 (event-position): `count over past[0, 9] min_count 2` at position 0 sees one value (insufficient-data{1,2}) and at position 1 two values (true for `>= 1`); the filtered `count ... where holds(e.x > 5)` has count 0 at position 1 (insufficient-data{0,1} without `min_count`, `true` for `<= 2` and `false` for `>= 1` with `min_count 0`); `sum ... min_count 0 = 0` is true. The `min`, `max`, `fraction` and `quantile` terms keep refusing `min_count 0` (no value over an empty set), and FR-414-AC-9/WA-11 keep the quantile-then-`min_count` double refusal.
- The two spellings of the default (`min_count` omitted and `min_count 1`) have one checked term, node id and obligation identity; no other artifact in the PR head states `min_count >= 1` as mandatory (FR-352 only quotes the cause payload).
- Count and sum admitting `min_count 0` is a lead default recorded in the ticket; it adds behaviour and removes none. No deferral wording.
- The QSL half is #1334 and is outside this batch. No ids added other than FR-414-AC-11/AC-12 and TC-366 WA-13/WA-14 (unused elsewhere).

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - | correct-requirement-no-evidence |

## Verdict

Mergeable by content: no defects in this PR's own delta.

Merge-tree against origin/main: clean. Open heads: spec.md-only conflict with #398 and #393; the old stale heads #267, #207, #59 conflict with everything. Gates: `quire validate` 86 failure lines (identical to origin/main); `cargo test --test checked_package_v2 --test backend_provider_v1` pass. The PR edits no schema or fixture, so no QSL conformance run was needed.
