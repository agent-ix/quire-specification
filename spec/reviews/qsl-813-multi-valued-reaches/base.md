---
id: SR-11753
title: "QSL-813 PR 266 spec review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@bfe973f525f6398aaa5bf4d0813d54393dd526b7; spec/functional/type-model/FR-152-bind-systems-model-structures.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-197-systems-model-typed-binding.md, spec/test-cases/TC-281-model-reference-equality-and-field-projection.md, proposals/checked-package-v2/model-member-type-vectors.json"
review_set: subset
---
# QSL-813 PR 266 spec review

## Summary

Ticket: QSL-813. One pass over `git diff origin/main...HEAD`. Checked against
the merged QSL FR-365 (AC-1 to AC-12) and TC-1048, the QSpec value-accounting
`graph` rows, the merged IR reader, and open PR 265.

Examined:
- FR-152 "Reachability over a relationship end" (examined): the check order,
  the traversal, the charges, the bound, incomplete and dangling rules.
- FR-152-AC-11 (examined): "`reaches(a, b, name)` over a relationship end admits a single-valued and a finite multi-valued destination end of the receiver endpoint's own type, and over a multi-valued end the frontier expands every neighbour in reference-key order, charging `graph.expand` per newly discovered object, `graph.edge` per link occurrence including repeats, and one `graph.result-retain`, with exact required counters completing and one fewer incomplete at the stated charge point."
- FR-152-AC-12 (examined): "An `unbounded` or `ordered` destination end, a destination end of another type and a traversal against the declared direction each refuse with the cause and order stated above; a neighbour count outside the destination bound refuses nothing; an unlisted or incomplete extent gives `incomplete_population`/`incomplete-scope` after the start's `graph.expand`; a dangling neighbour refuses `dangling_reference` before its `graph.edge`."
- FR-322 `quire.op.model.reaches` paragraph and FR-322-AC-57 (examined).
- TC-197 Y14 to Y18 (examined; charges recomputed by hand).
- TC-281 RN-18, RN-20 to RN-27 and the vector file (examined; each expected result recomputed against the fixture document).
- QSL FR-365, TC-1048 (context_only).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | IR's merged reader still refuses a `Set`/`Bag` destination for `quire.op.model.reaches` (only `Reference`/`Option` admit), so after RN-18 flips IR refuses a package QSpec now admits. Nothing goes red (IR does not consume `relationship_navigation_cases`), but the PR names only the QSL follow-up; an IR follow-up is needed too | spec/objects/interfaces/FR-322-checked-package-artifact.md:1467 |
| FND-002 | low | FR-322-AC-57 still says "RN-01 to RN-19 record each"; the cases are now RN-01 to RN-27 | spec/objects/interfaces/FR-322-checked-package-artifact.md:1467 |
| FND-003 | low | FR-152's new section does not say how a dangling or undecidable start `a` decides (QSL FR-365 step 1: before any charge); only neighbours are covered | spec/functional/type-model/FR-152-bind-systems-model-structures.md:192 |
| FND-004 | low | TC-197 Y16's last case ("`p1`-`p2` with `p2` undecidable") names no `reaches` call and no population variant that makes `p2` undecidable, although Y14 declares `p2` | spec/test-cases/TC-197-systems-model-typed-binding.md:97 |

## Verdict

PASS. No major issue. The widening is consistent with QSL: QSL FR-365 accepts
a subset (single-valued only) and its lowered node still satisfies the widened
FR-322-AC-57, so QSpec can widen first. The charge schedule (one
`graph.expand` per newly discovered object with `value_occurrences` = k, one
`graph.edge` per link occurrence including repeats, one `graph.result-retain`,
the bound refusing nothing, incomplete after the start's expand, dangling
before the neighbour's edge) matches `value-accounting.md` lines 44, 128 and
401-409 and QSL FR-365 Evaluation, AC-9 to AC-12 and TC-1048 rows 9, 16, 17
and 20. Y14 (5, 2 and 7 work units; limit 4 at `graph.result-retain`;
`value_occurrences` 2 at `graph.expand` with `next_charge` 3) and Y15 (6 work
units) recompute exactly. Every RN-18 and RN-20 to RN-27 expected result
recomputes against `domain-package-acme-orders.json`. The author's choices are
sound: (1) RN-18 flipping is the intended change and no consumer reads it;
(2) unbounded and ordered stay refused, as in navigation and QSL FR-365;
(3) the bound is not checked at evaluation, as QSL FR-365-AC-12;
(5) `reaches_field` unchanged. No textual conflict with PR 265 (a simulated
merge gives none; its FR-322 hunks are at 1198-1215 and 1472, this PR's at
1043 and 1457). No roadmap language and no ticket ids in spec text.

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | deferred | The IR reader change belongs to quire-contract-ir; the planner routes the IR follow-up to the IR lead, outside this QSpec PR |
| FND-002 | fixed | 21489bac27fb62a7f6a996362877cfd6c52ca09d |
| FND-003 | fixed | 21489bac27fb62a7f6a996362877cfd6c52ca09d |
| FND-004 | fixed | 21489bac27fb62a7f6a996362877cfd6c52ca09d |
