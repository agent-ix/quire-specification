---
id: SR-17126
title: "Re-verification of quire-specification PR #436 (QSL-856) at 8c472cc1"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@8c472cc135d0264678c6aca61225526928bf3d78; fix commits 41ce93fa, b43a01f2, 3594c1e2, 8c472cc1 over SR-17125; protocol-node-vectors.json, tools/rekey_fixtures.py, dependency-selection-vectors.json, timed-model-node-vectors.json, tests/checked_package_v2.rs"
review_set: subset
---
# Re-verification of PR #436

## Summary

Ticket: QSL-856. Re-check of SR-17125 FND-001 to FND-004 after the fix cycle.

Examined:
- activation.kind literal typing in the three positive fixtures (examined)
- TC-946 K10 and K14 rows against protocol-node-vectors.json (examined)
- tools/rekey_fixtures.py --check and write mode (examined)
- source_map pruning of positive-all-families.json against FR-322 source-map totality (examined)
- TM-23 locus in timed-model-node-vectors.json (examined)
- dependency-selection-vectors.json placeholder digests and the new tc_233 assertion (examined)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The `guard / guard` language-boundary check on the PR head fails (status failure on 8c472cc1). The PR adds `proposals/checked-package-v2/tools/rekey_fixtures.py`, a Python first-party tool, and AGENTS.md says tools, generators and scripts must be Rust with no script exemption. The log was not readable, so the cause is inferred from the diff. Replace the tool with a Rust one (or drop it) before merge, or the required check stays red. | proposals/checked-package-v2/tools/rekey_fixtures.py |
| FND-002 | low | The TC-946 K10 vector row is stale: the TC table reads "an acyclic protocol node adds `recursion_group` but leaves `recursion` `null`" and the vector reads "a protocol node carries a `recursion_group`". The new row-id test checks id presence only, not text. (SR-17125 FND-002 wrongly called K10 missing: it was present with old text; K14 was the missing row, now added and equal to the TC text.) | proposals/checked-package-v2/fixtures/protocol-node-vectors.json |
| FND-003 | low | `rekey_fixtures.py` without `--check` is not safe on every positive fixture: on positive-operation-identities.json it retypes text literals (nodes 32, 33, 36, 37) and changes the package id, although `--check` passes on it. On the three protocol fixtures it is idempotent and reproduces the shipped bytes. The tool should not retype where the fixture types text literals otherwise on purpose. | proposals/checked-package-v2/tools/rekey_fixtures.py |
| FND-004 | low | The TC-946 K14 row (FR-464-AC-15 cyclic admission) is published and tagged, but no positive fixture contains a cyclic component, so the tag `FR-464-AC-15` on the case-count test shows the vector text exists, not that cyclic packages are admitted. | tests/checked_package_v2.rs |

## Verdict

FND-001, FND-003 and FND-004 of SR-17125 are fixed in substance: the literal is typed as text, IR 9a84e65 admits the positive fixtures, TM-23 locus 51 matches the new fixture (QSpec replay and the IR TM-23 test pass), and the old FND-004 tool exists. Measured at 8c472cc1: QSL origin/main 09412b159 with QSPEC_DIR at this head, `cargo test -p qsl-package --lib checked_v2` 59 pass, 0 fail. IR 9a84e65 `conformance_qspec` (scratch wildcard arm for IR's WrongExport compile error) 65 pass, 2 fail: `hybrid::qspec_hybrid_step_precedes_protocol_reference_join` fails identically on QSpec main, and `timed_model::timed_model_published_fixture_round_trips_complete_nodes` (hard-coded 54 nodes, now 56) is IR-side and needs an IR test update (on main it also failed). The adverse mutations of FR-038-AC-112 pass. QSpec `cargo test --test checked_package_v2` 153 pass; the earlier full run failed only `tc_280...`, identical on main. The source-map pruning is correct: the dropped entry named the replaced stub key and every occurrence still has its entry. The source map is not identity input. No other fixture has a source-map entry naming a missing node. Placeholder restoration holds: putting an admitted package id back into the first `3333...` selection fails the new tc_233 assertion ("test/geometry names an admitted fixture"). Verdict: content PASS; the red language-boundary check (FND-001) is the open item for merge.
