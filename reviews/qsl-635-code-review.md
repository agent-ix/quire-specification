---
id: SR-1296
title: "Code review of PR #189: real FR-092 structural keys in checked-package-v2 fixtures"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@07cc7cf2f663b8b8c739ba5251ea7b59acf60fd6; proposals/checked-package-v2/fixtures/*.json, proposals/checked-package-v2/node-identity-vectors.json, proposals/checked-package-v2/dependency-selection-vectors.json, tests/checked_package_v2.rs"
review_set: subset
---
# Code review of PR #189 (QSL-635)

## Summary

Ticket: QSL-635. Diff `origin/main...07cc7cf` (main 2b2dd28): five positive
fixtures and the node-identity vectors rekeyed, a new
`positive-recursive-records.json`, four `derived_shape_key_mutations` rows in
`adverse.json`, and a derived-shape stage added to the test reader in
`tests/checked_package_v2.rs`. The Rust lane was applied.

What was checked and holds:

- No placeholder node key is left. A scan of every 64-hex string in the
  positive fixtures and `node-identity-vectors.json` finds no repeated-byte
  digest in any node id, semantic type, reference target or vector. Main had
  `7f7f...` and `b2b2...` (7 each in all-families alone). The repeated-byte
  digests that remain are only `lock.sources[].digest` and
  `source_map[].regions[].source.digest` (`2222...`, `6666...` in the
  untouched nominal fixture). Those are source-byte digests, not node keys,
  and no source bytes exist to derive them from. The `ffff...` in
  `node-identity-vectors.json` is the deliberate `absent-entry` frame
  mutation.
- The keys reproduce independently. A separate Python JCS/SHA-256
  implementation of the FR-092 `quire.structural-node/v1` preimage (owner
  `{kind: "source", authority, identity}` from the node's declaration
  occurrence for declared nodes) recomputed every non-nominal,
  non-application, non-group node: all-families 24/24, clause-operations
  10/10, control-operations 4/4, operation-identities 39/39, union-nodes
  12/12, recursive-records 1/1, 0 mismatches. The FR-092 T2 Integer preimage
  gives `07f6dca9...e32` and every fixture's Integer node carries that key.
- The recursive fixture is QSL emitter output in substance: its five group
  members carry exactly QSL FR-092's golden vectors G2, G3 (`List` group)
  and G7, G8, G9 (`Tree` group) as of QSL a9cfe1113, the commit the PR
  names.
- The four tamper rows refuse for the right reason. Each keeps the
  `integer_range` node id, patches the body in the graph and the projection
  at the same index, and recomputes `package_id`. The test asserts the
  projection agrees and the schema admits the package, then asserts
  `StaleNodeKey` at that node. The range is not an application node, so only
  the new derived-shape stage can produce that refusal.
- `Example::Flag`: the old body was a `literal true` typed at the node
  itself, so a type's body was a value of that type. FR-092's table gives a
  `Boolean` scalar the body `aggregate{[]}`. The new body is that, and
  `declaration` plus `owner` still separate the declared `Flag` from the
  builtin T1. The value `true` still lives in the `enabled` literal. The
  meaning is unchanged, and the shape is now correct.
- Hardcoded digests became lookups that still assert real things.
  `published_digest` asserts exactly one node matches. Every old byte
  matches its new role on main (clause b1 object_type, b2 reference,
  b3 parameter, b4 frame, b5 anchor, dd boolean literal, aa boolean scalar;
  all-families 20/91/92/95/dd; operation f2 rational, a3 text, f1 range,
  b0 decimal literal 1.25). The frame tests now sort entries by real digest,
  so the "misordered" vectors are still descending.
- `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/*.md'`
  (quire 0.36.1, engine 0.50.1) gives 145 failed documents on both main
  2b2dd28 and head 07cc7cf. The sorted outputs are byte-identical. The PR
  touches no spec/ file.

The probe test run (`cargo test --locked` with an in-group tamper probe) was
cancelled after more than 20 minutes behind six other jobs in the
machine-wide build lock. FND-001 rests on reading the code, which is
unambiguous.

## Verdict

**Request changes.** The positive fixtures now carry real keys, which is what
the ruling asked for. However, the reference reader still admits a tampered
bound inside a recursion group (FND-001), which is the exact IR-627 class.
The new tests are also traced to an AC that names a different refusal code
(FND-002).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The test reader's derived-shape stage skips recursion-group members: `is_derived_shape` requires no `recursion_group`, and no other stage re-keys a non-application group member. A `Tree` `collection_bounds` widened from `[0, 3]` to `[0, 1000]`, with the node id kept and the projection and `package_id` recomputed, therefore reads `Ok`. The recursive fixture is never passed through `read_package_semantics`, and no in-group tamper row exists. `group_member_key` already computes the in-group key, so the stage can call it. | tests/checked_package_v2.rs:2061-2084, tests/checked_package_v2.rs:2153-2163, tests/checked_package_v2.rs:9932 |
| FND-002 | medium | The new tamper and key tests carry `#[trace("TC-233", "FR-322-AC-10")]`, but AC-10 says stale-body keys refuse as `invalid_semantic_graph`. The tests assert `invalid_package`/`stale-node-key`. FR-322 also has no normative text for the derived-shape stage: which shapes it covers, its place after the application-key stage, or the in-group case. The reader order exists only in test code, and IR implements against the spec. | spec/objects/interfaces/FR-322-checked-package-artifact.md:845, spec/objects/interfaces/FR-322-checked-package-artifact.md:230-233, tests/checked_package_v2.rs:9758 |
| FND-003 | medium | all-families `Example::Small` is a `bounded_domain`/`integer_range` with body `aggregate{[]}` (no `min`/`max`) whose `semantic_type` is `Example::Flag`, a boolean. FR-092 gives an integer range bindings `min` and `max` over the Integer node. QSL's emitter cannot produce this node, yet it is rekeyed and published as a conformant positive, which teaches IR to admit a boundless range over Boolean. The shape predates this PR, which re-blesses it. | proposals/checked-package-v2/fixtures/positive-all-families.json:17 |
| FND-004 | low | The ruling says keys come from QSL's own key function or emitter. The PR body says a throwaway, uncommitted generator computed the non-recursive fixtures' keys. The result is correct: the in-repo test recomputes every structural key, and an independent recomputation matched all of them. Only the method differs from the ruling. | proposals/checked-package-v2/fixtures/positive-operation-identities.json |

## Dispositions

Round 1, reviewed at 4bf7e9c834a6e29695621be5233aec8c19359d67 (fix commits
9630eb4 and 4bf7e9c over 07cc7cf). The independent Python FR-092
recomputation was re-run on the head: all-families 24/24, clause-operations
10/10, control-operations 4/4, operation-identities 39/39, recursive-records
1/1, union-nodes 12/12, 0 mismatches, and no repeated-byte node id. No build
was run. The new reader stage was read against FR-322's new text. One new
defect, an ungrouped declared structural node that no stage re-keys, is
recorded once in SR-1298 FND-001 because its root is the FR-322 stage text.
Routed, not raised here: the placeholder nodes QSL cannot emit
(all-families `Example::Flag` and nodes 5-22 typed at it; the
operation-identities record at node 39 and nodes 10-12). QSL-638 rebuilds
those fixtures from emitter output.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 9630eb4 |
| FND-002 | fixed | 9630eb4 |
| FND-003 | fixed | 9630eb4 |
| FND-004 | accepted-no-change | The keys are now cross-checked against QSL's own node_key/group_keys (coder: 89 keys, 0 mismatches) and independently by this review; how the keys were generated does not change their content. |
