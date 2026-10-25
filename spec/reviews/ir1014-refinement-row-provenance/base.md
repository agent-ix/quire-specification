---
id: SR-44201
title: "Base review of refinement step row provenance"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification PR 442; FR-322, FR-375, TC-233, TC-310, checked-package-v2 schema and vectors"
review_set: base
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-375
    type: references
  - target: ix://agent-ix/quire-specification/TC-233
    type: references
  - target: ix://agent-ix/quire-specification/TC-310
    type: references
---

# Base review of refinement step row provenance

## Summary

Independent review of PR 442 at 86e2210f against the existing refinement
occurrence and reader-order contracts. No applicable `AssuranceProfile` was
found in the authored spec tree. The sidecar schema is closed, requires each
entry's owner, ordinal and region, and excludes the sidecar from node and
package identity. The producer owns the original source-row claim; the reader
checks wire consistency without reparsing source. The 24 illustrative vectors
cover admitted source reorder, ineligible refinements, malformed sidecars and
region mismatches. Schema metaschema validation and `git diff --check` pass.
Changed FR-375, TC-233 and TC-310 validate; FR-322 retains its existing
missing frontmatter `id` and typed-fields structural errors.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The normative body-step order inserts refinement between abstraction and terminal, but AC-121 still enumerates the complete order without refinement. TC-233 BS-01 also names the valid baseline steps without refinement and tests only later hybrid/timed/protocol precedence. Update the order criterion and a mixed-defect vector to pin `abstraction < refinement < terminal`; retain the existing later-step checks. | FR-322 Body steps and AC-121; TC-233 BS-01; FR-375 Reader order |

## Verdict

Changes requested: resolve FND-001 in this PR and revalidate the changed
specification. The finding concerns the acceptance and test contract; it
does not call for a change to the sidecar's closed wire shape.
