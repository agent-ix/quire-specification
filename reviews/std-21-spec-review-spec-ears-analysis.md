---
id: SR-15251
title: 'EARS analysis of PR #361: FR-470 statement (STD-21)'
type: SpecReview
analysis: ears-conformance
scope: 'agent-ix/quire-specification@8a0ac91; git diff origin/main(0a11d84)...8a0ac91 (PR #361, branch std-21-domain-clause-names): FR-470 (new, AC-1..8), TC-953 (new, R01..R09), spec/spec.md contains edge; context FR-154, FR-208, model-complete.md, shared-grammar.md, TC-939, open PR #360; QSL FR-366/TC-1050 (quire-spec-language read only); ticket STD-21 (QSpec#84)'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-470
  type: reviews
---

# EARS analysis of PR #361: FR-470 statement (STD-21)

## Summary

Ticket: STD-21. EARS check of the one new requirement statement, FR-470's Description. It is event-driven ("When QSL checks a domain clause, it SHALL resolve ..."), with a named subject, one response and a testable object. quire's grammar lint reports no EARS or ambiguous-term warning on FR-470. The eight ACs are concrete single-outcome criteria. Gates: `quire validate --scope . spec/**/*.md` (quire 0.36.2, engine 0.50.2) on the head reports 134 failed documents; origin/main a3a79a3 reports 116. The whole difference is the 18 spec/objects/interfaces documents that #356 fixed on main after this branch's base 0a11d84, and no diagnostic names a touched file. `cargo test --test checked_package_v2 --test backend_provider_v1` passes 23 and 126.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-470 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. FR-470's statement conforms to the EARS event-driven pattern.
