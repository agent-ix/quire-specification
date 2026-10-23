---
id: SR-16349
title: "spec-review of FR-478 convert text to an exact integer or decimal (owner decision, lead default) (quire-specification PR 395)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@a3b9cd66; spec/functional/type-model/FR-478-convert-text-to-an-exact-number.md; spec/test-cases/TC-964-text-converts-to-an-exact-integer-or-decimal.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-478
    type: references
  - target: ix://agent-ix/quire-specification/TC-964
    type: references
---

# Spec review of FR-478 convert text to an exact integer or decimal (owner decision, lead default) (quire-specification PR 395)

## Summary

Ticket: STD-365. One spec-review pass over `git diff origin/main...a3b9cd66`: standing rule (no drop, narrowing or deferral), testable ACs with hand-derived vectors, id uniqueness across all open PR heads, QSpec name cross-check, mergeability, and the repo gate.

Examined, clean:

- FR-478-AC-1/2 derived by hand against the stated grammar: integer `0|-?[1-9][0-9]*`; decimal adds `.digits`, no `-` before zero; `1.50` is coefficient 150, scale 2; `1.555` into scale 2 refuses `decimal_out_of_domain`; `-0`, `05`, `+5`, `.5`, `5.`, `1.0` (integer target) are undefined; `５` converts only under nfkc.
- Accounting order in AC-5 matches the Accounting section; FR-146/FR-149/value-accounting edits are consistent.
- QSL conformance (`cargo test -p qsl-package --lib checked_v2`, QSpec main + this PR): 49 passed. Against the PR head alone, 2 hybrid tests fail only because the head lacks the 15 main commits (hybrid, STD-409..415).

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Narrowing by lead default: the text-to-number operation is specified for integer and decimal targets only, and `Rational`, `Float64`, quantity and every other target is a permanent type-check refusal (`ill_typed`/`operator-ineligible`). The ticket is "text-to-number" and calls the integer-and-decimal scope a lead default and the whole operation an owner decision. Standing rule: a scope cut by lead default is a blocker; Peter must rule or the other numeric targets are specified here. | spec/functional/type-model/FR-478-convert-text-to-an-exact-number.md | missing-requirement |
| FND-002 | low | spec/spec.md lists every recent FR in its `contains` relationships (FR-467, FR-469, FR-470, FR-476, FR-477, FR-481; #393 adds FR-479) but #395 does not add FR-478. | spec/functional/type-model/FR-478-convert-text-to-an-exact-number.md | wrong-requirement |

## Verdict

Not mergeable: FND-001 (high, standing rule). Merge-tree against main is clean.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 at d92ea803: FR-478 specifies every numeric target for `Text[..]` and `String` operands, with grammar, outcomes, accounting, twelve identities and TC-964 vectors TN-01 to TN-22. |
| FND-002 | still-open | At d92ea803 `spec/spec.md` still has no `contains` relationship and no index row for FR-478. |
