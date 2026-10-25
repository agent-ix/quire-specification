---
id: SR-13669
title: 'Spec review of QSpec PR #319: FR-142 unit-graph topology causes, integrity'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification@820027e65; PR #319 diff origin/main...HEAD: spec/functional/type-model/FR-142-evaluate-quantities-and-units.md,
  spec/test-cases/TC-187-quantity-and-unit-conversion.md; ticket STD-112'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-142
  type: reviews
- target: ix://agent-ix/quire-specification/TC-187
  type: reviews
---

## Summary

Ticket: STD-112. PR quire-specification#319, head 820027e656308de9cd1b790583c8cea0e3b38ac1.

Integrity of the PR against the rest of QSpec and the QSL consumer: id uniqueness, the FR-272/native-diagnostics catalog and pairing, the checked-package V2 reader rules (README, node-identity vectors, FR-322), and QSL FR-091 and qsl-semantics at origin/main 4639aa67d, plus the unmerged QSL-275b and QSL-237/238 branches. Open PRs #318, #320, #267, #207, #200 and #59 do not touch FR-142, TC-187, FR-272 or native-diagnostics. `quire validate --scope . 'spec/**/*.md'` (quire 0.36.2, engine 0.50.2) reports 134 failed documents on main and on the PR head, the same set; FR-142 and TC-187 validate clean. `cargo test --test checked_package_v2 --test backend_provider_v1` passes (114 and 14 tests).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-142 cause table | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| FR-142 root row | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| FR-142 duplicate-root row | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| FR-142 cycle row | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| FR-142-AC-13 (new, line 248) | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| FR-142-AC-5 | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| TC-187 U06 | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
| TC-187 U11a | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
| TC-187 U32 | spec/test-cases/TC-187-quantity-and-unit-conversion.md | context_only |
| FR-142 Dimension and conversion algebra | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | context_only |
| FR-142 unit node keys | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | context_only |
| FR-142-AC-13 (existing, line 249) | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | context_only |
| FR-272 invalid_package and missing_declaration rows | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | context_only |
| native-diagnostics invalid_package / invalid_semantic_graph rows | proposals/quire-v1/definitions/native-diagnostics.md | context_only |
| checked-package V2 unit reader constraints | proposals/checked-package-v2/README.md | context_only |
| unit-decl grammar | proposals/quire-v1/shared-grammar.md | context_only |
| QSL FR-091 Dimension and unit declarations, OQ-11, OQ-12, AC-35 | quire-spec-language spec/functional/FR-091-produce-value-forms-and-assemble-package-declarations.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new criterion reuses the id FR-142-AC-13, which the existing criterion on the next line (a unit and a record of one name coexist; two units of one name refuse `ambiguous-name`; `record Quantity` refuses `invalid_syntax`) already holds and TC-187 U32 already cites. The AC table now has two rows with one id, so U06's and U11a's '(FR-142-AC-13)' and U32's '(FR-142-AC-13)' trace to the same id and no matrix can tell the two criteria apart. `quire validate` does not catch it. Fix: number the new criterion FR-142-AC-15, the next free id (no open PR, #318, #320, #267, #207, #200 or #59, touches FR-142 or TC-187), and change the citations in U06 and U11a. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:248-249 |
| FND-002 | medium | The rule names no component, and the checked-package V2 reader refuses the same shapes with another code. 'The admission of a unit graph SHALL refuse' has no subject, and 'no unit of the checked package' points at a checked package. proposals/checked-package-v2/README.md:184-186 has the strict reader refuse a targetless unit whose scale is not `1/1` or offset not `0/1`, a non-root unit without a target, a zero scale, a cross-dimension target and a cyclic path, and node-identity-vectors.json `root-unit-nonidentity-scale` and `nonroot-unit-missing-target` expect `invalid_semantic_graph`, a code with no cause variant (native-diagnostics). A reader author can take FR-142 to require `invalid_package`/`invalid-value` for that vector. The PR body's 'No other requirement mentions these cases (grep)' misses these. Fix: make the subject the checker admitting a package's source unit declarations (FR-142's Quantity type reference section already uses 'the checker'), and add one sentence that a `quire.checked-package/v2` reader refuses these shapes as `invalid_semantic_graph` (FR-322 `nominal_identity_preimage`), not with these causes. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:61-64 |
| FND-003 | medium | The `definition-cycle` row retains 'the units of the cycle in path order' but fixes no start, so `m` -> `cm` -> `m` can be retained as [m, cm] or [cm, m], and U11b and the new AC both say 'naming both in path order'. FR-272 and native-diagnostics give the `invalid_package` payload as 'the actual field/index path or dependency edges', and `definition-cycle` elsewhere retains edges (a call-graph cycle's call edges in FR-151 order). QSL FR-091 already retains a unit cycle's dependency edges, sorted. Fix: retain the cycle's target edges starting at its least unit, under the unit order that failure-domain FND-001 fixes, and state that start in U11b. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:73 |

## Verdict

Not mergeable: FND-001 is a duplicate AC id. The cause names, codes and pairing match FR-272 and native-diagnostics, and QSL already maps a unit cycle to `definition-cycle` and an unresolved name to `missing-name`, so those two rulings agree with the consumer.

## Dispositions

Round 1, bdacb1420 (head 8ba797093).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fixed in bdacb1420: / FR-142-AC-15 / Over the `L` unit graph of TC-187, each single defect of TC-187 U06 and U11a to U11g refuses with its row's code and cause naming the dimension and units shown, and exposes no conversion value; the orderings are fixed by the vectors U11h to U11k: a unit that has scale zero and an unknown target is `invalid-value`; unit `a` with an unknown target before unit `b` with scale zero is `missing-name`; a self-target beside a second targetless unit in one dimension is `conflicting-definition`; and dimension `L` with a self-target before dimension `T` with a zero scale is `definitio... |
| FND-002 | fixed | Fixed in bdacb1420: The checker SHALL admit no unit mapping of a refused dimension. A refusal refuses the package, as every other refusal of this requirement does, and the `quire.checked-package/v2` strict reader refuses the same shapes, as an already-built package, with `invalid_semantic_graph` (FR-322 |
| FND-003 | fixed | Fixed in bdacb1420: / 6 / the target edges form a cycle (a unit targeting itself included) / `invalid_package`/`definition-cycle` / the cycle's dependency edges, `unit -> target`, in order from the cycle's least unit by declaration order; when the dimension has several cycles, the one that contains the least unit / |
