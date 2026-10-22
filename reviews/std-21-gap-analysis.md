---
id: SR-15252
title: 'Gap analysis of PR #361: FR-470-AC-1..8 against TC-953 (STD-21)'
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-specification@8a0ac91; git diff origin/main(0a11d84)...8a0ac91 (PR #361, branch std-21-domain-clause-names): FR-470 (new, AC-1..8), TC-953 (new, R01..R09), spec/spec.md contains edge; context FR-154, FR-208, model-complete.md, shared-grammar.md, TC-939, open PR #360; QSL FR-366/TC-1050 (quire-spec-language read only); ticket STD-21 (QSpec#84)'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-470
  type: reviews
- target: ix://agent-ix/quire-specification/TC-953
  type: reviews
---

# Gap analysis of PR #361: FR-470-AC-1..8 against TC-953 (STD-21)

## Summary

Ticket: STD-21. This is a planless gap analysis of FR-470-AC-1..8 against TC-953 R01..R09. AC-1 maps to R01, AC-2 to R02, AC-3 to R03 and R04, AC-4 to R05, AC-5 to R06, AC-6 to R07, AC-7 to R08 and AC-8 to R09. Every changed AC has a TC row that cites it. This is a spec-only PR, so no code is tagged yet; the QSL implementation is the follow-up the author names. R08 inherits the wrong refusal code that integrity SR-15250 FND-001 reports. Gates: `quire validate --scope . spec/**/*.md` (quire 0.36.2, engine 0.50.2) on the head reports 134 failed documents; origin/main a3a79a3 reports 116. The whole difference is the 18 spec/objects/interfaces documents that #356 fixed on main after this branch's base 0a11d84, and no diagnostic names a touched file. `cargo test --test checked_package_v2 --test backend_provider_v1` passes 23 and 126.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-470 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| FR-470-AC-1 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| FR-470-AC-2 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| FR-470-AC-3 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| FR-470-AC-4 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| FR-470-AC-5 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| FR-470-AC-6 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| FR-470-AC-7 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| FR-470-AC-8 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| TC-953-R01 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |
| TC-953-R02 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |
| TC-953-R03 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |
| TC-953-R04 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |
| TC-953-R05 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |
| TC-953-R06 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |
| TC-953-R07 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |
| TC-953-R08 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |
| TC-953-R09 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | (minor) FR-470's Description puts transition guards (FR-208) and operation `post` entries inside the scope of a domain clause. Every AC and every TC-953 vector uses an invariant or a `pre` entry. Nothing tests that a state machine's guard clause, or a `post` entry, resolves through its owner's self alias. Extend AC-2, with a vector, to a `post` entry and a guard of a state machine listed by the namespace. | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md:20 |

## Verdict

One minor coverage gap (FND-001): guards and `post` entries are in FR-470's scope but untested. Every AC has a vector.
