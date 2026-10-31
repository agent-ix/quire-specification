---
id: SR-2026103004
title: "Spec review of quire-specification PR #550 (QSL-1314): restored frame_mutations vectors"
type: SpecReview
analysis: integrity
date: 2026-10-31
scope: "agent-ix/quire-specification@83b8d78db03dc5aac715b49b4b1322d6bb73300d; PR #550 git diff origin/main...83b8d78d (proposals/checked-package-v2/node-identity-vectors.json, proposals/checked-package-v2/README.md, tests/checked_package_v2.rs); checked against FR-340, FR-322 Structural node identity, TC-233, and quire-spec-language origin/main qsl-package/src/checked_v2/tests.rs conformance_fr340_frame_mutations_match_qspec_vectors and xtask/src/conformance.rs"
review_set: subset
relationships:
- target: ix://agent-ix/quire-specification/FR-340
  type: reviews
- target: ix://agent-ix/quire-specification/TC-233
  type: reviews
---
# Spec review of quire-specification PR #550

## Summary

Ticket: QSL-1314. The PR restores `node-identity-vectors.json` with only `frame_mutations`: 37 vectors with structural `{package, path, ordinal}` node ids and `expected_locus_node_id`. It re-derives the order-dependent vectors under FR-322's componentwise node-id order, restores the QSpec test `tc_233_frame_mutations_refuse_through_the_full_reader_with_their_recorded_cause`, and adds a README bullet.

Every vector was compared with its pre-#518 version (`040c28f3^`), with each 64-hex digest mapped to its structural node id.

Examined:
- FR-340 "Canonical ordering and reader precedence" (examined)
- FR-340-AC-3, AC-4, AC-6, AC-8, AC-9, AC-16, AC-17 (examined)
- FR-340-AC-1 (examined)
- FR-322 "Structural node identity", the ascending node-id order (context_only)
- TC-233 frame steps, lines 150-192 and 645-660 (examined)
- proposals/checked-package-v2/README.md:116-129 (examined)
- all 37 `frame_mutations` vectors (examined)
- QSL `conformance_fr340_frame_mutations_match_qspec_vectors` and `xtask/src/conformance.rs` (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `scope-conflict-listed-receiver-first` lists the `every-object` entry before the `receiver` entry, so it is identical to `scope-conflict-every-object-and-receiver`. The pre-#518 vector listed `receiver` first. That order is the only one where the conflict also has to outrank a scope-order defect in `modifies`. The regeneration sorted it away. FR-340-AC-16 and TC-233:192 require the conflict "whichever order they are listed in", and no vector now lists the receiver entry first. Swap the two `modifies` entries back. | proposals/checked-package-v2/node-identity-vectors.json:1333-1386 |
| FND-002 | medium | TC-233 still says to "read each `deletes_scoped_parameter_cases` vector with a model attached" (FR-340-AC-17, the state-step `missing-name` for an unknown `parameter` entry). That section (DS-01..) was in the pre-#518 file, carried no digest, and is not restored here or anywhere else. The README bullet also claims AC-17 for this file. Restore the section with its test, or remove the step from TC-233. | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:192 |
| FND-003 | low | The README bullet claims the file covers "FR-340-AC-1 to AC-9". The file holds only refusals. AC-5 and AC-7 are schema failures, and the AC-1 admission is not in it. The following paragraph says vectors replace `modifies`/`creates`/`deletes` but does not mention `deletes_scoped`, which seven vectors also replace. | proposals/checked-package-v2/README.md:116-122 |
| FND-004 | low | The top-level `"version": "quire.checked-semantic-node-vectors/v1"` is read by nothing. The QSpec test and QSL's conformance test read only `frame_mutations`. It is a version tag with no consumer, so it fails the value test. Delete it. | proposals/checked-package-v2/node-identity-vectors.json:2 |

## Verdict

The order-dependent vectors are right under the componentwise order.
- `deletes-misordered` lists `[Session, Account]`. `Example/Session` sorts after `Example/Account`, so this refuses `invalid_semantic_graph` at the frame node.
- `precedence-group-outranks-node-id` puts the object type `Session` in a `modifies` relationship entry and the event type `Deposited` in `deletes`. Position decides, so the result is `Session` even though `Deposited` has the lower id.
- `precedence-node-id-within-group` has `[Session, Account]` relationship entries in `modifies`. The tie breaks by order key, so the result is `Account`, and the meaning-join outranks the misorder.
- In `cross-frame-lower-node-id-wins`, the second frame's path `["state","frame",{node 13}]` is a proper prefix of the published frame's path, so it is visited first. Its `deletes` lists `[Account, ConfigVersion, Session]`, and `Config/...` sorts before `Example/...`, so its order defect is reported ahead of the published frame's `missing-name`.
- The two deletes_scoped order vectors, the scope-conflict precedence vectors and every wrong-meaning vector follow FR-340.

No digest, hash, pin or key member remains, and no 64-hex string appears in the file.

The file has the shape QSL reads: `dependencies`, `modifies`, `creates`, `deletes`, an optional `deletes_scoped` and `second_frame`, `expected_code`, `expected_cause` (null maps to None) and `expected_locus_node_id`. QSL's test is still `#[ignore]`d pending this file. Un-ignoring it is QSL's follow-up.

The restored QSpec test passes, 1 passed and 0 failed. `quire validate` reports two structural errors on FR-340, but this PR does not touch that file. TC-233 validates.

**Not mergeable until FND-001 is fixed.** Fix FND-002 in the same PR. FND-003 and FND-004 are small edits.
