---
id: SR-15253
title: 'Spec integrity analysis of PR #359: remove complete-QSL WASM scope (STD-22)'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification@4540d6a; git diff origin/main(0a11d84)...4540d6a (PR #359, branch std-22-remove-wasm-scope): FR-308 and TC-228 deleted, IT-076, NFR-071, NFR-072, NFR-073, AD-004, docs/v1-capability-inventory.md V1-TOOL-009, docs/v1-complete-architecture.md S9 and quire-wasm paragraph, spec/spec.md; QSL Plan-013 (quire-spec-language read only); ticket STD-22 (QSpec#69)'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/IT-076
  type: reviews
- target: ix://agent-ix/quire-specification/NFR-073
  type: reviews
- target: ix://agent-ix/quire-specification/AD-004
  type: reviews
---

# Spec integrity analysis of PR #359: remove complete-QSL WASM scope (STD-22)

## Summary

Ticket: STD-22 (QSpec#69). PR: quire-specification#359, head 4540d6a, base 0a11d84. The owner's latest ticket comment (2026-09-16) confirms that complete-QSL WASM is not V1 scope and says to continue the correction. The PR does that. It deletes FR-308 and TC-228, removes the V1-TOOL-009 row, drops IT-076's WASM scope and its FR-308 `verifies` edge, drops NFR-073's FR-308 `constrains` edge, and drops WASM from AD-004, NFR-071, NFR-072, NFR-073, S9 and the `quire-wasm` paragraph. It does not modify the quire-wasm repo. On main a3a79a3 all of this text was still present, so the gap claim holds. On the head, a grep for FR-308, TC-228, V1-TOOL-009, wasm, WebAssembly and browser over the whole tree (outside reviews/ and spec/reviews/) finds only Cargo.lock `wasm-bindgen` dependency entries. The inventory doc states no ticket totals that need correcting, and its 51-group invariant is unaffected because V1-TOOL-009 came from the code-surface census. Leaving a gap at V1-TOOL-009 is a reasonable lead default. agent-ix/quire-wasm#6 is already CLOSED NOT_PLANNED. Gates: `quire validate --scope . spec/**/*.md` (quire 0.36.2, engine 0.50.2) on the head reports 134 failed documents; origin/main a3a79a3 reports 116. The whole difference is the 18 spec/objects/interfaces documents that #356 fixed on main after this branch's base 0a11d84, and no diagnostic names a touched file. `cargo test --test checked_package_v2 --test backend_provider_v1` passes 23 and 126.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-308 | spec/functional/tooling/FR-308-expose-bounded-wasm-api.md | examined |
| TC-228 | spec/test-cases/TC-228-wasm-native-parity-and-bounds.md | examined |
| IT-076 | spec/integration/IT-076-native-tool-surface-parity.md | examined |
| IT-076-SC-01 | spec/integration/IT-076-native-tool-surface-parity.md | examined |
| NFR-073 | spec/non-functional/NFR-073-contain-tools-and-plugins.md | examined |
| NFR-071 Scope | spec/non-functional/NFR-071-bound-all-complete-v1-execution.md | examined |
| NFR-072 Scope | spec/non-functional/NFR-072-reproduce-locked-results.md | examined |
| AD-004 | spec/assurance/AD-004-output-mapping-subsystem.md | examined |
| V1-TOOL-009 | docs/v1-capability-inventory.md | examined |
| S9 | docs/v1-complete-architecture.md | examined |
| spec/spec.md FR-308 contains edge | spec/spec.md | examined |
| QSL Plan-013 | agent-ix/quire-spec-language plan/Plan-013-complete-v1-delivery/plan.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | (minor) Nothing under spec/, proposals/, tests/, plan/ or docs/ cites FR-308, TC-228, V1-TOOL-009 or WASM after this PR (outside reviews/). QSL origin/main b453e10 still does: plan/Plan-013-complete-v1-delivery/plan.md:59 (`ix://agent-ix/quire-specification/FR-308`), plan.md:272 (V1-TOOL-009 / FR-308 / TC-228 row), and tasks/Task-053-wasm-parity.md:11-12 (FR-308, TC-228 `verifies`). These targets dangle once #359 merges. The PR body says QSL's Plan-013 is not changed "(QSL's FR-308 is an unrelated protocol-reference requirement)", but these edges target QSpec's FR-308, not QSL's. File the QSL Plan-013 / Task-053 / A07 removal as a tracked follow-up ticket and correct the parenthetical. The ticket's Lane A and delivery-graph item lives there too. Separately, the author's Linear comment lists closing agent-ix/quire-wasm#6 as still open, but it is already CLOSED (NOT_PLANNED). | spec/functional/tooling/FR-308-expose-bounded-wasm-api.md:1 |

## Verdict

Mergeable: no major finding. FND-001 (minor) is about the cross-repo follow-up only. QSL Plan-013 keeps ix:// references to the deleted QSpec FR-308 and TC-228, and the PR body's parenthetical about QSL's FR-308 misattributes them.
