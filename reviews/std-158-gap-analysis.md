---
id: SR-13822
title: "Gap analysis of QSpec PR #327 (attempt type after 'attempts N of'), STD-158"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@6d395af97ee35847031fdf3f413c29b1e98e9a2a; git diff origin/main...6d395af: new FR-239-AC-8 against TC-380 rows (CP-11) and the verifies edge TC-380 -> FR-239; orphan search over spec/, proposals/, tests/, plan/; ticket STD-158"
review_set: subset
date: 2026-10-22
relationships:
  - target: ix://agent-ix/quire-specification/FR-239
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-380
    type: reviews
---

## Summary

Ticket: STD-158. PR: quire-specification#327, head 6d395af. Method: quoin gap-analysis, planless (plan completion not assessed), restricted to acceptance criteria and test-case rows because the PR changes only spec files. Tooling: quoin 0.28.3, quire 0.36.2 (engine 0.50.2), spec-artifacts-process@b82e42e.

One AC is new (FR-239-AC-8, Test (TC-380)). It has a TC row (TC-380 CP-11, which cites FR-239-AC-8) and a `verifies` edge from TC-380 to FR-239. No AC, row or vector was changed or deleted, so nothing is left referring to a removed id. QSpec has no Rust test traced to TC-380 or FR-239. The executable binding lives in QSL (TC-1058 rows 15a-15c, qsl-semantics protocol_obligation tests).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-239-AC-8 | spec/objects/protocol/FR-239-compensation-retry-policy.md | examined |
| TC-380 CP-11 | spec/test-cases/TC-380-compensation-worked-example.md | examined |
| TC-380 | spec/test-cases/TC-380-compensation-worked-example.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | TC-380 CP-11 changes only the attempts line, so the retry binders stay `M::RefundAttempt` while the attempt type becomes `Integer`, `Set<...>` or `M::Mode`. The row then requires "no refusal at the retry binders". No QSpec AC states that binder comparison is skipped when the attempt type is refused. FR-239-AC-8 says nothing about the binders. The QSL checker does skip it (`attempt_type` returns None, so binders are not compared), and TC-1058 row 15c avoids the question by typing each binder as the attempt type as written. Fix: write each variant's retry binders as the attempt type, as TC-1058 row 15c does, or add the binder behaviour to FR-239-AC-8. | spec/test-cases/TC-380-compensation-worked-example.md:74 |
| FND-002 | low | TC-380's Description scopes the test to exploring `Pay` (states, registration, activation, `cattempt`/`cend`, commit cutoff and interval verdicts). It does not mention the check-time attempt-type refusal that CP-11 now covers. Add the attempt-type position check to the Description. | spec/test-cases/TC-380-compensation-worked-example.md:20-22 |

## Verdict

Coverage is complete: every new AC has a TC row and a trace edge, and there are no orphans. Two low findings concern how exactly the new row is stated.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1, fix commit d06dfb4. Each TC-380 CP-11 variant now writes `attempts 1 of T` and `retry (e: T, l: T) { false }`, with `T` being `Set<M::Total>[0, 3]`, `Integer` and `M::Mode` as written, which matches TC-1058 row 15c. FR-239-AC-8 also now states that the checker "compares no retry binder with a refused type", so the row's "no refusal at the `retry` binders" traces to an AC. |
| FND-002 | fixed | Round 1, fix commit d06dfb4. The TC-380 Description now ends "and check that the attempt type written after `attempts N of` parses whatever its form and is admitted only as a record or event type." CP-11 cites both FR-239-AC-8 and FR-239-AC-9. |
