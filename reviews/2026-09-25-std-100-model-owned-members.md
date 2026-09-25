---
id: SR-624
title: "Spec review — STD-100 FR-322 model-owned members"
type: SpecReview
analysis: failure-domain
scope: "STD-100; FR-322 Model-owned members, TC-280, model-member-type-vectors.json, ModelDeclarationNode preimage"
review_set: subset
---
# Spec review — STD-100 FR-322 model-owned members

## Summary

Base checklist plus failure-domain review of commit 552113f, which adds
FR-322's "Model-owned members" rule, FR-322-AC-29 to AC-32, TC-280, its
vectors and the `ModelDeclarationNode` preimage. The review found one high,
seven medium and seven low findings. All are fixed in the follow-up commit
except FND-003, which QSL ADR-013 QC-24 owns, and FND-014, which is accepted
as written.

## Verdict

**PASS after fixes**. The rule is stated, has positive and adverse vectors
and a Rust qualification test (TC-280). Reaching a model-owned `field` member
through an application also needs QC-24's operand-family change (FND-003).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The refusal-vocabulary sentence named the wrong five codes after `missing_import` was added. Fixed: the five codes are named explicitly. | FR-322 `read` |
| FND-002 | medium | The refusal vocabulary pulled in FR-154 and normalization codes without naming them. Fixed: `missing_import`, `unsupported_construct` and `wrong_snapshot` are listed, and the selection-check codes are stated. | FR-322 `read` |
| FND-003 | medium | The only `field` catalog entry, `quire.op.record.project`, takes a `record` operand, and a model entity type has family `reference`. QSL ADR-013 QC-24 owns this operand family, which is outside STD-100. Recorded, not changed. | operation-catalog.json; FR-322 families |
| FND-004 | medium | Step 3 contradicted "never resolves a name by search" and the "before any operation refusal" ordering of `ambiguous-name`. Fixed in both places. | FR-322 `operation`, `declaration` |
| FND-005 | medium | Owner recovery did not require the node's tag and form to match the key, and did not state the stale-key case or match uniqueness. Fixed: the tag and form must match, a stale key refuses `missing-selection`, and FR-321 gives at most one match. | FR-322 step 2 |
| FND-006 | medium | The test oracle did not apply phase 4's most-derived-redefiner rule, and no vector covered it. Fixed: the oracle applies it, step 3 names it, and vector MT-10 covers it. | tests/model_member_types.rs; TC-280 |
| FND-007 | medium | The vector package is a projection of the declaration records, not an IR document, and MS-01 claimed full admission. Fixed: the vector is stated as a projection, uses `typeRef`/`interfaceFeatures`, and MS-01 passes the selection admission checks. | vectors; README; TC-280 |
| FND-008 | medium | A dispatched call's arguments were not checked against the resolved operation's parameters. Fixed: the arity and parameter-type rule is added, with vectors MT-11, MA-10 and MA-11. | FR-322 step 4 |
| FND-009 | low | "Denotes" was ambiguous between anonymous and declared nodes. Fixed: either denotes. | FR-322 Model-owned members |
| FND-010 | low | Constraint cases were underspecified. Fixed: exact constraint sets and `lo <= hi` are required, with vector MA-13. | FR-322 step 4 |
| FND-011 | low | Step 1's stage, refusal choice, limits and step 2's charge were unstated. Fixed. | FR-322 steps 1–2 |
| FND-012 | low | Field multiplicity typing differs from FR-152's end typing. Fixed: the Presence row of `quire.model.complete/v1` is cited. | FR-322 step 4 |
| FND-013 | low | No vector covered `systems_interface`. Fixed: vector M8 and case MA-12 cover it. | vectors |
| FND-014 | low | AC-29 and AC-30 each combine several checks. Accepted as written. | FR-322 ACs |
| FND-015 | low | Two sentences were phrased negatively. Fixed: they are rephrased positively. | FR-322 Model-owned members |
