---
id: SR-13310
title: "Base spec review of QSpec PR #286 (STD-210)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@5f25fd2; spec/functional/type-model/FR-149-apply-complete-equality-matrix.md, spec/test-cases/TC-194-complete-equality-matrix.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-149
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-194
    type: reviews
---

## Summary

Ticket: STD-210. Base checklist over `git diff origin/main...HEAD` (2c2918b..5f25fd2): FR-149 prose on ordering operators, FR-149-AC-21 and TC-194 E35. The change makes `<`, `<=`, `>` and `>=` between `Integer` and `Int[..]` operands (and between two `Int[..]` operands) accepted, with no expected type and no coercion. Compared against QSL origin/main a2d68af2e: `qsl-semantics/src/check/check.rs` `ordering` (both operands `is_integer`, which covers `Integer` and `Int(_)`, give `OrderedKind::Integers`) and `arithmetic` (integer `+`/`-`/`*` with no hint is typed `Integer`), and QSL FR-104 line 159-162 and FR-381 line 227-229, which state the same rule. No ids added or renumbered; no collision with open PRs #283 or #287 (neither touches FR-149 or TC-194; #287's FR-376-AC-5 uses `convert<Integer>` under `=`, consistent with the untouched STD-157 equality rule). No `proposals/checked-package-v2` or schema data changed. No hash, digest or pin use introduced.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-149 ordering prose | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md | examined |
| FR-149 equality expected-type rule (STD-157) | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md | context_only |
| FR-149-AC-13 | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md | context_only |
| FR-149-AC-19 | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md | context_only |
| FR-149-AC-21 | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md | examined |
| TC-194 E34 | spec/test-cases/TC-194-complete-equality-matrix.md | context_only |
| TC-194 E35 | spec/test-cases/TC-194-complete-equality-matrix.md | examined |
| TC-194 E36 | spec/test-cases/TC-194-complete-equality-matrix.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | TC-194 E35 explains `earlier.tally + 1 <= later.tally` as "`earlier.tally + 1` keeps the type its operands give it" without naming the type, and FR-149 states no type for an integer arithmetic node that receives no expected type. A reader can take "the type its operands give it" as `Int[0,3]`, which would carry a range obligation. QSL types the node `Integer` (`arithmetic`, no hint). Say "`earlier.tally + 1` is typed `Integer`" in E35, and optionally state the unhinted type in the FR-149 ordering sentence. | spec/test-cases/TC-194-complete-equality-matrix.md:70; spec/functional/type-model/FR-149-apply-complete-equality-matrix.md:221-222 |
| FND-002 | low | FR-149-AC-21 says `<`, `<=`, `>` and `>=` are accepted between `Integer` and `Int[..]` operands, but TC-194 E35 exercises only `<=`, `<` and `>=`; no vector uses `>`. Add `n > later.tally` (or `later.tally > n`, which also covers the `Int[..]`-on-the-left order). | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md:334; spec/test-cases/TC-194-complete-equality-matrix.md:70 |

## Verdict

Mergeable; both findings are low and need not block. The rule now reads the same in the FR-149 prose, FR-149-AC-21, TC-194 E35, QSL FR-104 and FR-381, and the QSL checker. The STD-157 `=`/`!=` expected-type rule and its negative cases (AC-19, AC-20, the first eight E35 vectors, E36) are unchanged. AC-21's `Test (TC-194)` link resolves to E35. The Float ordering refusal (AC-13, E30) is unaffected, because the acceptance covers only integer operands.
