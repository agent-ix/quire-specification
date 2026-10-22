---
id: SR-15255
title: 'Gap analysis of PR #359: removed FR-308 obligations and IT-076 (STD-22)'
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-specification@4540d6a; git diff origin/main(0a11d84)...4540d6a (PR #359, branch std-22-remove-wasm-scope): FR-308 and TC-228 deleted, IT-076, NFR-071, NFR-072, NFR-073, AD-004, docs/v1-capability-inventory.md V1-TOOL-009, docs/v1-complete-architecture.md S9 and quire-wasm paragraph, spec/spec.md; QSL Plan-013 (quire-spec-language read only); ticket STD-22 (QSpec#69)'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/IT-076
  type: reviews
---

# Gap analysis of PR #359: removed FR-308 obligations and IT-076 (STD-22)

## Summary

Ticket: STD-22. The PR adds or changes no AC. Its removed ACs (FR-308) go together with their only verifier (TC-228), so no TC row is left orphaned and no AC is left untested. IT-076 still verifies FR-300 through SC-01..03, and no other artifact named IT-076 as covering FR-308. On QSpec, no test under tests/ or src/ referenced FR-308 or TC-228. Gates: `quire validate --scope . spec/**/*.md` (quire 0.36.2, engine 0.50.2) on the head reports 134 failed documents; origin/main a3a79a3 reports 116. The whole difference is the 18 spec/objects/interfaces documents that #356 fixed on main after this branch's base 0a11d84, and no diagnostic names a touched file. `cargo test --test checked_package_v2 --test backend_provider_v1` passes 23 and 126.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-308 | spec/functional/tooling/FR-308-expose-bounded-wasm-api.md | examined |
| TC-228 | spec/test-cases/TC-228-wasm-native-parity-and-bounds.md | examined |
| IT-076 | spec/integration/IT-076-native-tool-surface-parity.md | examined |
| IT-076-SC-01 | spec/integration/IT-076-native-tool-surface-parity.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean.
