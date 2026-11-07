---
id: SR-2026110616950
title: "Spec review of QSL-1695: locality-preserving operand references"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification branch spec/qsl-1695-locality-refs (commits 415a353b, 2cc8d99e) against origin/main 5e3fade2: FR-322 Operand index segments and FR-322-AC-95, AC-278..AC-282, AC-392..AC-395; FR-201, FR-330, FR-336, FR-375, FR-487 adjustments; TC-233 NI table, TC-934, TC-937; checked-package-v2 schema.json, README.md, node-index-vectors.json and converted vectors; tests/support/operand_slots.rs and the adapted tests in tests/"
review_set: base
---
# Spec review of QSL-1695: locality-preserving operand references

## Summary

The change moves operand references out of node bytes. A node-held id writes each operand as an operand slot `{"operand": k}`. The slot names entry `k` of the holding node's row in the new required `semantic_graph.operand_rows`, and the row is numbered by first occurrence in RFC 8785 order. A document-held id keeps index segments `{"node": i}`. FR-201 identity remains the rebuilt structural key and is never a digest. The core contract holds together. Within one node, equal wire values mean equal keys, because a row's entries must be distinct and an admitted graph has distinct keys. The reader steps are rows, placement, range, numbering, then cycle, and each has one locus. The "Following a reference" bullet is a contract the IR reader can implement.

What I checked:

- I wrote an independent Python implementation of FR-322 Reader checks steps 1 to 5. I applied every RFC 6902 patch in `node-index-vectors.json` to `fixtures/positive-dependency-call.json`. All 15 schema-valid cases (NI-08..12, NI-15..24) give the stated step and locus. NI-22 and NI-23 pass the operand checks, and NI-23's duplicate key is correct by construction.
- AC-278..282, AC-392..395 and the TC-233 rows agree with the vectors. AC-392..395 are unique and next-free (the highest on main is AC-391).
- Every `semantic_graph` in `proposals/` (any depth) passes the five steps.
- All 88 vector node fragments that carry `_operand_row`/`operand_rows` are numbered by first occurrence.
- I applied 154 converted vector patches to their bases, and none adds an operand defect. I did not replay the patches with `move` operations or the intake cases whose base is not a package.
- No new pin, digest or tracking record is introduced.

I found one blocking finding and three major ones. The blocking finding: the new NI test reads its base in index view, so it fails. The major ones: step 3 and the locality claim mishandle foreign refinement keys, and the harness's numbering check is quadratic on NI-13.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | **Blocking: the NI test checks the index view, not the published form.** `let base: Value = parsed_fixture(POSITIVE_DEPENDENCY_CALL)` now returns `operand_slots::index_view`. In that view node 2's path holds `{"node": 0}`, so `operand_refusal(&base)` returns a placement defect at `/semantic_graph/nodes/2/node_id/path/3/node`, and the "base passes" assertion fails. Every patch is then applied to the index form, so NI-08 refuses at node 3's index segment instead of its stated locus. Fix: `let base: Value = serde_json::from_str(POSITIVE_DEPENDENCY_CALL).expect(..)`, and keep `viewed = index_view(&base)` for the key comparisons. | tests/checked_package_v2.rs:20476 |
| FND-002 | high | **Major: the numbering step is quadratic and NI-13 will not finish.** Step 4 loops over every row and, for each row, scans every event (`for event in &events` inside `for (index, row) in rows`). For the 100,001-node chain that is about 2×10^10 iterations. Fix: bucket the slot events by `holder` in one pass, for example `Vec<Vec<(&String, u64)>>`, then check each row against its own bucket. | tests/checked_package_v2.rs:20350 |
| FND-003 | high | **Major: step 3 checks foreign-key index segments against the wrong graph.** The `node` of an FR-375 external `{package, node}` key is document-held and resolves against the dependency's graph. FR-375 checks its range when the dependency is bound, and FR-375-AC-35 (RW-09) requires that check. Step 3 checks "an index segment not below the length of `semantic_graph.nodes`", which is the importing graph, with no exception. A literal reader therefore refuses a valid key whose index is at least the importer's node count but below the dependency's. The harness does the same: `holder: None` goes to `index >= nodes.len()`. Fix: in step 2 and step 3, exclude the `node` of a foreign `{package, node}` key and point to FR-375's bind-time check. In `operand_events`, give foreign segments their own holder kind, which steps 2 and 3 skip. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1536; spec/functional/analysis/FR-375-declare-model-refinement.md:427; tests/checked_package_v2.rs:20312 |
| FND-004 | high | **Major: the foreign refinement key leaves locality narrowed.** An FR-375 external key that names a derived dependency node sits inside an importer node but writes the dependency's graph indexes. Adding unrelated declarations to the dependency therefore changes that importer node's bytes. "Locality" admits this in a parenthesis ("no graph index of its own graph"). FR-322-AC-392 and README.md state it with no exception ("hold no graph index"). The ruling asked for no narrowing. Fix: either write that key's operands as operand slots through the holding node's row, naming the imported operand nodes in the importer's graph (which Scope already requires to be present), or get an explicit owner ruling for the exception and state it in AC-392 and the README. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1435; spec/objects/interfaces/FR-322-checked-package-artifact.md:4668; proposals/checked-package-v2/README.md:11 |
| FND-005 | medium | **The AC-95 harness never checks the published form of a reordered graph.** `reordered` reorders the index view and leaves `operand_rows` stale. `node_index_refusal` and `read_with` run on that view, so the reordered package with its rows reordered, as FR-322-AC-95 states it, never meets `operand_refusal` or the V2 schema. Fix: also assert `operand_refusal(&slot_view(&candidate)) == None` and V2 schema validity. | tests/checked_package_v2.rs:20682 |
| FND-006 | low | **FR-322-AC-36 still describes the old NI-07.** It says "a `node` whose `path` holds an index segment (`NI-07`)", but NI-07 now patches in an operand slot. Fix: "holds an operand segment (an operand slot, `NI-07`)". | spec/objects/interfaces/FR-322-checked-package-artifact.md:4402 |
| FND-007 | low | **FR-336 has stale "as written" wording.** The corpus text says an id resolves "exactly as that package writes the id". The package writes node-held ids with slots, and corpus ids are document-held. Fix: "written as a document-held id of that package". | spec/objects/interfaces/FR-336-conformance-corpus.md:185 |
| FND-008 | low | **"Index segment of X" appears outside FR-322's Notation.** FR-487:45-51 and TC-979:24 still derive paths from "the index segment of" X. FR-322's Notation rule rewrites that phrase only "in this artifact". Fix: write "operand segment" in FR-487 and TC-979, or widen Notation to every artifact that cites it. | spec/objects/interfaces/FR-487-checked-package-monitor-plan-body.md:45; spec/test-cases/TC-979-declaration-less-function-node-keys.md:24 |
| FND-009 | low | **The locality vector is not read by the full reader.** NI-24 and AC-392 claim only "passes the operand checks". FR-383-AC-15 depends on full admission of the grown graph. Fix: expect NI-24 to be admitted by the full reader (`read_with`), not only by the operand checks. | spec/objects/interfaces/FR-322-checked-package-artifact.md:4668 |
| FND-010 | low | **A file-inventory count floor (ceremony).** `assert!(packages >= 20, ..)` counts published files. Fix: `packages > 0`. | tests/checked_package_v2.rs:20452 |
| FND-011 | low | **The fragment row convention exists only in the README.** The `<member>_operand_row` / `<prefix>_operand_rows` siblings appear in README.md, and `index_view` depends on them, but no TC states the convention. Fix: add one sentence to the TC-233 vector preamble. | proposals/checked-package-v2/README.md:18 |

## Verdict

The design and its normative text are sound, and the vectors agree with the reader checks exactly. Fix FND-001 and FND-002 before the harness is compiled. Fix FND-003 in the spec and the harness before merge. FND-004 needs either the slot form for foreign keys or an explicit owner ruling. The low findings are wording, traceability and a count floor.
