---
id: SR-2026103002
title: "Base review of the TW-27 model-body vector"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; proposals/checked-package-v2/model-tuple-union-vectors.json (TW-27); spec/objects/interfaces/FR-322-checked-package-artifact.md (owner join stage paragraph, AC-140, AC-153); spec/test-cases/TC-940 step 6; tests/checked_package_v2.rs (derived_shape_semantic_refusal, TC-940 wire replay)"
review_set: base
---

## Summary

Ticket: none (no tracker id in branch or title); PR quire-specification#540.
Reviewed the diff of `origin/main...origin/fix/tw-27-model-body-vector`.
I applied the TW-27 patch to `fixtures/positive-model-tuple-union.json` and
validated it with Python jsonschema against `schema.json`. It is schema-valid.
The `node_id` is unchanged, every edge resolves, and the literal's `type` (the
`scalar_type`/`boolean` node) resolves.
FR-322's `dependencies` rules treat a literal's `type` as a type annotation, so
`dependencies: []` stays correct and stage 1 reports nothing. The `Pair` owner
still joins in stage 2. The first refusal is therefore
`invalid_package`/`invalid-derived-shape` at `/semantic_graph/nodes/0/node_id`,
as the vector states.
The 27 wire-case ids are unique. AC-153's cause-bearing and cause-free lists
expand to exactly the vectors' sets.
Merged against #539 with `git merge-tree`, the two branches produce no conflict.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | `assert_eq!(ids.len(), 27, "every published wire case ran")` is a count pin. The loop already runs every case, any 27 vectors satisfy it, and it forces an edit on every added vector (it will collide with #539's planned TW-28). Delete it and keep the uniqueness assert | tests/checked_package_v2.rs:14326 | implementation-bug-despite-evidence |
| FND-002 | medium | The new reader-model arm `(Some("model"), _) if owner.kind == "model"` misses three forms FR-322 also lists as model declaration nodes with the empty aggregate body: `model`/`model_root` (`ModelRootOwner`), `relation`/`relationship` and `relation`/`population` (`ModelOwner`, tag `relation`). The reader model admits a non-empty body on them. Excluding source-owned declared nodes is correct, because FR-322 says a node with a `declaration` carries a `SourceOwner` and is not a model declaration node. The arm comment's "(a `ModelOwner` node)" is not the same set | tests/checked_package_v2.rs:3005-3009; FR-322 lines 776-782; FR-322-AC-82; FR-322-AC-134 | correct-requirement-no-evidence |
| FND-003 | medium | FR-322 does not place the model declaration body check in the numbered stage order. Stage 3 names only reference, population and compound unit. The check's sentence, which this PR extends, sits in the "owner join stage" bullet, but the reader model runs it in stage 3, after every owner joins. A package with a non-empty `Pair` body and an unjoined owner on a later node gets a different first refusal depending on the reading. Name model declaration nodes in stage 3's list | spec/objects/interfaces/FR-322-checked-package-artifact.md:466-468,492-504 | missing-requirement |

## Verdict

TW-27 is correct: it is schema-valid, it has one defect, and the stated code,
cause and locus are the first refusal. The FR-322, AC-140, AC-153 and TC-940
step 6 citations are accurate, and the PR closes the TC-940 step 6 gap left by
TW-05's deletion. The PR is not mergeable as-is:
- FND-001: delete the count pin.
- FND-002: widen the reader-model arm to FR-322's full list of model declaration
  nodes.
- FND-003: place the check in stage 3's list.
