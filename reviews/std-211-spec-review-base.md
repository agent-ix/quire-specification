---
id: SR-13322
title: "Base checklist review of QSpec PR #287 (STD-211)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@1e274e7; spec/functional/analysis/FR-375-declare-model-refinement.md, spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md, spec/functional/analysis/FR-377-decide-model-refinement.md, proposals/quire-v1/shared-grammar.md, proposals/quire-v1/definitions/native-diagnostics.md, spec/test-cases/TC-310-refinement-declaration-admission.md, spec/test-cases/TC-311-population-valued-mapping-expressions.md, spec/test-cases/TC-315-refinement-undetermined-and-replay.md, spec/test-cases/TC-316-ring-buffer-data-refinement.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-376
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-377
    type: reviews
---

## Summary

Ticket: STD-211. Base checklist (ID formats, duplicates, AC-to-TC coverage, matrix visibility) over `git diff origin/main...1e274e7`. `quire validate` exits 0 on the three FRs and four TCs (the two proposal files have no frontmatter, as on main). `quire matrix --scope .` lists every new criterion (FR-375-AC-15..18, FR-376-AC-6..8, FR-377-AC-9..11). No id collides with merged #285 (FR-375-AC-12..14, TC-310 mutations 14-17) or with open #283 (FR-322-AC-75/76, TC-277 BP-14), #286 (FR-149-AC-21, TC-194 E35 only) or #288.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-375-AC-15 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-AC-16 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-AC-17 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-AC-18 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-376-AC-5 | spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md | examined |
| FR-376-AC-6 | spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md | examined |
| FR-376-AC-7 | spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md | examined |
| FR-376-AC-8 | spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md | examined |
| FR-377-AC-9 | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| FR-377-AC-10 | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| FR-377-AC-11 | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| TC-310-m20 | spec/test-cases/TC-310-refinement-declaration-admission.md | examined |
| TC-310-m21 | spec/test-cases/TC-310-refinement-declaration-admission.md | examined |
| TC-311-steps | spec/test-cases/TC-311-population-valued-mapping-expressions.md | examined |
| TC-315-c7 | spec/test-cases/TC-315-refinement-undetermined-and-replay.md | examined |
| TC-315-c8 | spec/test-cases/TC-315-refinement-undetermined-and-replay.md | examined |
| TC-316-row | spec/test-cases/TC-316-ring-buffer-data-refinement.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-377-AC-11 has three clauses but TC-315 covers one. Case 7 holds "an unmapped subtype object whose key sorts first" (the object clause); no case maps a reference to an unmapped object, and no case reads the mapped state's relationship extent (the expected results cite AC-11 only for case 7). The ring-buffer fixture also declares no subtype of `Ring`, and TC-315 does not add one. Add the subtype to the fixture (QSL TC-1086 uses `SubRing` and a `next: Reference<Ring>` field) and vectors for the reference clause and the extent clause. | spec/functional/analysis/FR-377-decide-model-refinement.md:192; spec/test-cases/TC-315-refinement-undetermined-and-replay.md:44-49,72-74 |
| FND-002 | medium | Case 7's first run cannot be built as written. The ring-buffer `Queue` has one field row (`items`), so there is no "earlier row"; and a per-evaluation meter budget of zero (the TC-555 / QSL FR-140-AC-5 device) applies to every row, so the `items` row is itself incomplete and its `only` is never reached. The fourth run ("an evaluator stop injected in the earlier row") is fault injection that a black-box vector cannot perform. Name a fixture variant with a second field row, a way to make only that row refused or incomplete (for example a refused form in it), and drop or re-scope the injected stop. | spec/test-cases/TC-315-refinement-undetermined-and-replay.md:44-49,72-74 |
| FND-003 | low | The last clause of FR-375-AC-17 and TC-310 mutation 20 expect `Some(1)` in a state clause to be "an ordinary call that resolves as any other", which states no outcome. Say what the vector expects (for example `missing_declaration`/`missing-name` at `Some` when no function `Some` is declared, and a call when one is). | spec/functional/analysis/FR-375-declare-model-refinement.md:398; spec/test-cases/TC-310-refinement-declaration-admission.md:113-116,192-195 |

## Verdict

Not mergeable on this lens until FND-001 and FND-002 give FR-377-AC-9 and AC-11 constructible vectors. Every other new criterion parses, names a TC, and that TC carries a matching step: FR-375-AC-15/16 (TC-310 mutations 18/19), AC-18 (mutation 21), FR-376-AC-5 (TC-311 step 2, TC-316), AC-6/7/8 (TC-311 steps 6/7/8), FR-377-AC-10 (TC-315 case 8). The `convert<Integer>` spelling is applied in all three copies of the ring row (FR-376-AC-5, TC-311, TC-316), no stale `s.index = (self.head` copy remains, and it matches QSL #1038's ADR-020 §8 text. The sum bound matches QSL FR-139 (#1038).
