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

Independent review of PR 443 at 199b4d1d. The `class_catalog` entries are
closed in both V2 schemas and enter the dedicated refinement node preimage,
package identity projection and exact local dependency list. Both schemas
pass Draft 2020-12 metaschema validation and all local references resolve.
Changed FR-375, FR-464, TC-310 and TC-233 pass Quire structural validation;
FR-322 retains its existing frontmatter and typed-fields failures. No
applicable `AssuranceProfile` was found in the authored spec tree.

FR-375-AC-19's admitted `memory drain` example predates this PR and remains
in tension with FR-436 and the new text's statement that the currently
defined source memory class is only `tso`/`flush`. Its disposition requires
the pending owner decision; this review does not treat `drain` as admitted.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved at 199b4d1d: refinement step 4 is limited to already checked keys, declarations, types, selections, catalog order and row joins. FR-464 now runs the full catalog-versus-admitted-protocol graph association check as protocol step 7, after its steps 1 to 6. FR-464-AC-14 and TC-310 CC-11 pin protocol-body, catalog-join and earlier row-mismatch precedence without moving the established body-step order. | FR-375 Checked class catalog and Reader order step 4; FR-464 Reader order and AC-14; TC-310 CC-11 |
| FND-002 | medium | Resolved at 199b4d1d: CCW-01 updates both `refinement_step_rows` and matching generated owner `source_map` regions after source-row reorder, while keeping the canonical body and identities equal. | TC-233 CCW-01; FR-322-AC-129; FR-375-AC-22 |
| FND-003 | medium | Resolved at 199b4d1d: TC-310's wire-variant setup now always recomputes `package_id` while CC-02 retains only the refinement node key; CCW-04 also rebuilds dependencies and recomputes `package_id` before expecting the dedicated stale-key refusal. | TC-310 CC-02 and setup; TC-233 CCW-04; FR-322 Identity and validation |
| FND-004 | medium | Resolved at 199b4d1d: TC-310 CC-10 reads one admitted source-default `tso` package under the default and under an `sc` request override; it requires byte-identical checked catalog and `package_id` with different resolved memory model and obligation identity. | FR-375-AC-23; TC-310 CC-10; FR-434-AC-1/AC-2 |

## Verdict

PASS for the revised specification. All four PR findings are resolved. The
pre-existing `memory drain` contradiction remains an owner decision
independent of these findings.
