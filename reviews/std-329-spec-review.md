---
id: SR-16182
title: "Spec review of quire-specification PR #381: STD-329, FR-414 check-time window refusals"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@280f1eff0; PR #381 (git diff origin/main...280f1eff0): spec/functional/temporal/FR-414-evaluate-window-aggregates.md, spec/test-cases/TC-366-window-aggregates.md"
review_set: subset
evaluated_revision: "280f1eff0"
relationships:
  - target: ix://agent-ix/quire-specification/FR-414
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-366
    type: reviews
---
# SR-16182: Spec review of quire-specification PR #381: STD-329, FR-414 check-time window refusals

## Summary

Ticket: STD-329. Reviewed `git diff origin/main...HEAD` at 280f1eff0 (FR-414 'Check-time refusals', FR-414-AC-9, AC-10, TC-366 WA-11, WA-12, References) against QSL FR-194 at main (PR #1248's S3 bullets and FR-194-AC-7, AC-8) and FR-414's existing AC-4 and AC-6. Every code, cause and locus matches QSL FR-194 line for line: model-subject term `unsupported_construct`/`expression-form` at the term; wrong-kind bound `ill_typed`/`unit-mismatch` at the first such bound; `a > b` `invalid_package`/`invalid-value` at the whole window; `quantile` and `min_count 0` `invalid-value` at the literal; threshold `unit-mismatch` at the threshold. The three several-fault rules (source order, a wrong-kind bound suppresses `a > b`, a model-subject term skips the window checks) are QSL's, stated as behaviour. AC-9 adds 'under event-position' to the `past[5, 2]` vector, which QSL leaves implicit; that is a tightening, not a divergence. Each AC-10 vector is an 'exactly these refusals' oracle that fails on a missing or extra refusal or the wrong order. FR-414-AC-9/AC-10 and WA-11/WA-12 are absent from main and from every open QSpec PR. Gates at the head: validate 86 failure lines (main 86); cargo 23 + 119 passed; TC-366 is not test-parsed, so no conformance run applies. origin/main is an ancestor; no textual conflict with the other six PRs. Lead defaults are flagged.

Method: quoin spec-review (base), with integrity, EARS, criterion-strength (by hand: no Jev client) and failure-domain checks folded in; code-review of the `tests/` change where present is folded in here. Gap-analysis does not apply (spec, vectors and trace tags only).

Units examined:

- `FR-414 Check-time refusals` (examined): spec/functional/temporal/FR-414-evaluate-window-aggregates.md
- `FR-414-AC-9` (examined): spec/functional/temporal/FR-414-evaluate-window-aggregates.md
- `FR-414-AC-10` (examined): spec/functional/temporal/FR-414-evaluate-window-aggregates.md
- `TC-366 WA-11` (examined): spec/test-cases/TC-366-window-aggregates.md
- `TC-366 WA-12` (examined): spec/test-cases/TC-366-window-aggregates.md
- `FR-414-AC-4` (context_only): spec/functional/temporal/FR-414-evaluate-window-aggregates.md
- `FR-414-AC-6` (context_only): spec/functional/temporal/FR-414-evaluate-window-aggregates.md

## Verdict

**MERGEABLE.** No findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
