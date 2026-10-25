---
id: SR-44301
title: "Base review of refinement class provenance"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification PR 443; FR-375, FR-322, TC-310, TC-233 and both checked-package-v2 schemas"
review_set: base
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/TC-310
    type: references
  - target: ix://agent-ix/quire-specification/TC-233
    type: references
---

# Base review of refinement class provenance

## Summary

Independent review of PR 443 at 2a4b7f3d. The `class_catalog` entries are
closed in both V2 schemas and enter the dedicated refinement node preimage,
package identity projection and exact local dependency list. Both schemas
pass Draft 2020-12 metaschema validation and all local references resolve.
Changed FR-375, TC-310 and TC-233 pass Quire structural validation;
FR-322 retains its existing frontmatter and typed-fields failures. No
applicable `AssuranceProfile` was found in the authored spec tree.

FR-375-AC-19's admitted `memory drain` example predates this PR and remains
in tension with FR-436 and the new text's statement that the currently
defined source memory class is only `tso`/`flush`. Its disposition requires
the pending owner decision; this review does not treat `drain` as admitted.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Class-catalog reader step 4 says it reconstructs the complete catalog from an **admitted** protocol graph under FR-464 to FR-466, but FR-322 and FR-464 run the protocol body step after refinement. At the refinement step those protocol semantics have not been admitted, so the specified input and mixed-defect refusal precedence are not defined. State exactly which already checked graph facts the catalog step can inspect and give a vector with both a catalog defect and a later protocol-body defect, preserving the established body-step order. | FR-375 Checked class catalog and Reader order step 4; FR-322 Body steps; FR-464 Reader order |
| FND-002 | medium | CCW-01 reorders source rows and changes only `refinement_step_rows` regions, yet FR-322 requires each generated owner `source_map` region to equal that sidecar region. An actual admitted source-reorder vector must update both occurrence regions and sidecar regions while retaining the same canonical body and identities. | TC-233 CCW-01; FR-322-AC-129; FR-375-AC-22 |
| FND-003 | medium | The retained-node-key mutations CC-02 and CCW-04 change identity-bound catalog content but do not explicitly recompute `package_id`; TC-310's setup expressly excludes retained-key cases from that recomputation. FR-322 verifies package identity before graph admission, so the expected dedicated `stale-node-key` refusal is not reached as written. Recompute `package_id` while retaining only the refinement node key. | TC-310 CC-02 and setup; TC-233 CCW-04; FR-322 Identity and validation; FR-375 Reader order step 1 |
| FND-004 | medium | AC-23 promises that a request memory override changes neither checked catalog nor package id, but the new CC/CCW vectors never supply two requests with different resolved memory models against one source-default package. Add that before/after request vector, checking unchanged source catalog/package id and distinct obligation identity, without replacing the existing source-default and malformed-memory cases. | FR-375-AC-23; TC-310 CC-01 to CC-09; TC-233 CCW-01 to CCW-05; FR-434-AC-1/AC-2 |

## Verdict

Changes requested in PR 443. The pre-existing `memory drain` contradiction
remains an owner decision independent of these findings.
