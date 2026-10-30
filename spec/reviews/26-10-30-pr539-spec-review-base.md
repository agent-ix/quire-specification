---
id: SR-2026103001
title: "Base review of the TW-06 to TW-09 owner-only vectors"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; proposals/checked-package-v2/model-tuple-union-vectors.json (TW-06..TW-09); spec/objects/interfaces/FR-322-checked-package-artifact.md (stage-order and owner-join locus paragraphs); spec/test-cases/TC-940 (context)"
review_set: base
---

## Summary

Ticket: none (no tracker id in branch or title); PR quire-specification#539.
Reviewed the diff of `origin/main...origin/fix/tw-06-09-structural-vectors`.
Each of TW-06 to TW-09 now applies one `replace` of
`/semantic_graph/nodes/{n}/owner/node` to `fixtures/positive-model-tuple-union.json`.
Applied with Python and validated against `proposals/checked-package-v2/schema.json`
(jsonschema): all four are schema-valid, as the vectors state, and every
`semantic_type`, `dependencies`, body node reference and `source_map` `node_id`
resolves. On `main` the same check found six dangling edges per case. FR-322's
owner join (stage 2, ascending node-id) reports `missing_declaration`/
`missing-selection` at `/semantic_graph/nodes/{n}/owner` first: stage 1 has no
defect to report (TW-00 is admitted and no application node changed), and the
reader takes the declaration from `owner.node`, not from the `node_id` path. No
digest, pin or stored hash is added.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | TC-940 step 6 still expects a model declaration node with a non-empty body to be refused (`invalid_package`/`invalid-derived-shape` at `node_id`), but no published vector covers it since TW-05 was deleted in #518; adverse.json's derived-shape cases cover only population bounds | spec/test-cases/TC-940-variant-type-node-key-is-alias-independent.md:65-67,98-100; FR-322 tuple Refusals paragraph | correct-requirement-no-evidence |
| FND-002 | medium | The stage-order paragraph this PR edits still cites FR-322-AC-169, which is Withdrawn. With TW-06 to TW-09 now carrying one defect each and NK-13 removed, no live AC or vector has both an owner defect and a dangling `semantic_type`/`dependencies` edge, so a reader that runs stage 4 before stage 2 passes every published vector | spec/objects/interfaces/FR-322-checked-package-artifact.md:461,481-485,3171 | correct-requirement-no-evidence |
| FND-003 | low | The new sentence says the owner join is "the vector's one defect", but FR-322 also says a model node's `node_id.path` is `[node_tag, semantic_form, <FCD identity>]` and "a function of the owner alone". In TW-06 to TW-09 the path keeps naming Pair or Shape while `owner.node` names another node, so the package breaks that producer rule too. Say that the reader does not compare the path with `owner.node`, and that the owner join is the one defect the reader checks | spec/objects/interfaces/FR-322-checked-package-artifact.md:489-493,764,1284,1400-1402,1527 | wrong-requirement |

## Verdict

The vector rewrite is correct: (a) every edge resolves in all four patched
packages, (b) the stated code, cause and locus are the first refusal FR-322's
stage order gives, and (d) no hash, digest or pin was added. The edited prose
in FR-322 at lines 481-493 correctly states the stage order and the reason the
`node_id` is kept. FND-003 is a wording fix. FND-001 and FND-002 are coverage gaps:
FND-001 was already flagged by the author, and FND-002 is evidence this PR removes
without replacing. The rest of the edited prose and all four vectors were
examined and are clean.
