---
id: SR-607001
title: "spec-review of QSpec STD batch (quire-specification PR 607)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; PR 607 head 900187e8; FR-322 (AC-74, AC-309, AC-371), FR-487 (AC-13), TC-940, TC-978, proposals/checked-package-v2/model-tuple-union-vectors.json"
review_set: subset
---

# Spec review of QSpec STD batch (PR 607)

## Summary

Tickets: STD-550, IR-1297, STD-450, QSL-1463 (FR-487 call closure). `quire validate`: FR-487, TC-940, TC-978 exit 0; FR-322 fails with the same two errors on origin/main (missing id, record-invalid), so pre-existing. FR-322-AC-371 and FR-487-AC-13 are unused; open PR 606 uses FR-322-AC-342..370, no id collision, but a trial merge of 606 conflicts textually at the tail of the FR-322 AC table (both append; resolve by ordering 342..370 then 371); FR-487 merges clean. STD-550: the stage owner named (join stage 3 of FR-322-AC-275, RM-POP-MAX) matches the reader order text, and the Contract IR citations (FR-053-AC-4, FR-038-AC-279, AdmittedUnbounded) exist. The vectors file: all 27 wire_cases refusals and 29 intake refusals carry a stage; TW-21/TW-22 (invalid_semantic_graph, graph) is consistent with stage 1. STD-450 not diffed (no edit).

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-322-AC-371 says every refusal in the "tuple/union intake and wire vectors" carries a stage and a refusal without one is a replay failure, but 12 refusals in position_cases (TP-03, TP-07), union_value_cases (TU-04..08), enum_value_cases (TE-02..04) and equality_cases (TQ-02, TQ-04) have none. Name the sections (intake_cases, wire_cases) or add stage to those 12. | FR-322-AC-371; model-tuple-union-vectors.json | wrong-requirement |
| FND-002 | low | FR-322-AC-371 lists TW-01 to TW-05 as schema, but TW-05 does not exist (ids run TW-04, TW-06); it omits TW-10 and TW-26 (schema) and TW-21/TW-22 (graph), so the stage of those is stated only in the file. | FR-322-AC-371 | wrong-requirement |
| FND-003 | medium | FR-487-AC-13 has no QSpec-side observable: the reader recomputes no key and refuses nothing for it, and TC-978 says vectors are authored later with the producer; no MP-nn vector is cited, so nothing in this repo can fail it. It also bundles an unrelated claim (summary cell not counted in capacity, State fields unchanged). Split, and state the producer-side verifier (QSL FR-251-AC-26 / TC-1081) or cite a reader-visible vector. | FR-487-AC-13; TC-978 | correct-requirement-no-evidence |
| FND-004 | low | FR-487 key rule covers a helper by "the index segments of the body-root node"; say whether that is the body root's node_id (content-bearing) or the graph position, since FR-322 index segments are graph positions that reordering rewrites. | FR-487 Plan-root key; FR-322 operand index segments | wrong-requirement |

## Verdict

Mergeable after FND-001 and FND-003 are addressed (small, same files); neither is a soundness defect. FR-322-AC-74 (consumer projection AdmittedUnbounded, stage 3 owner) and FR-322-AC-309 (declared helper keeps its declaration path) are consistent with the reader order and with QSL FR-251/FR-514. Rebase on 606 resolves the one textual conflict.
