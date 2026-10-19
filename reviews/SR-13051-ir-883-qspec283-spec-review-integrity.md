---
id: SR-13051
title: "IR-883 QSpec PR 283 integrity analysis: closure wording, structure and validation"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@7baa1104b8a37f8606a6c05a9e0c43fba8ddada8 vs origin/main a2230e1; spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/objects/interfaces/FR-323-native-runtime-envelope.md, spec/functional/tooling/FR-307-package-reusable-semantic-libraries.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, spec/test-cases/TC-277-replay-byte-provision.md, proposals/checked-package-v2/README.md"
review_set: subset
---

## Summary

Ticket: IR-883. Structural validation was run with quire 0.36.2 (engine 0.50.2) over the five changed spec files, at the PR head and at origin/main a2230e1. Both runs give the same two structural errors in FR-322 and FR-323: `[value_object] required 'id' (frontmatter_field(id)) is missing` and `semantic.record-invalid ... "fields" is a required property`. The warnings are also the same; only one line number shifts, 1511 to 1524. "2 document(s) failed structural validation" on both. These errors are pre-existing, not introduced by this PR.

A grep for stale "closure" and `dependency_selections` uses found two neighbour interfaces with stale or missing wording (FR-330, FR-331; see SR-13050 FND-003/004). FR-307's Outputs ("dependency closure") and "Resolution takes the transitive import closure" are still correct under a derived closure. FR-375 and FR-322's qualified-name resolution rule refer to the selected (direct) entries, which is consistent. TC-937 is consistent.

## Examined criteria

- FR-322-AC-35 (examined): the full row as amended.
- FR-323-AC-7 (examined): the row as amended.
- TC-233 procedure and expected-results paragraphs (examined); TC-277 BP-06 and its closing note (examined).
- proposals/checked-package-v2/README.md vector inventory (examined).
- FR-307, FR-330, FR-331, FR-375, TC-937 closure/selection wording (context_only).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | proposals/checked-package-v2/README.md describes every member of `dependency-selection-vectors.json` (selections, `package_id`, entry mutations, model selections, order vectors) but not the new `closure_vectors` member. It also leaves out that member's shape: `direct_dependencies`, `dependency_locks` with `base`, `loci`, `dependency_reference` and `use` | proposals/checked-package-v2/README.md:61-73 |
| FND-002 | low | FR-322-AC-35 now combines a schema/order criterion that QSpec tests today with two reader-behaviour clauses (cross-lock diamond, transitive-only reference) that stay planned until IR lands. One row can no longer carry an honest binding. That is why the PR removed its trace tag (SR-13053 FND-001). Splitting the two new clauses into a new FR-322 AC, verified by TC-233 and left untagged, keeps AC-35 honestly tagged | spec/objects/interfaces/FR-322-checked-package-artifact.md:1540 |

## Verdict

**PASS with low findings.** No new structural validation errors. The FR-322/FR-323 errors are pre-existing and identical on origin/main. The "planned" statuses in TC-233 and TC-277 are honest.
