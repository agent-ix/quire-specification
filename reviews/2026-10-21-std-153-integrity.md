---
id: SR-14686
title: "integrity review of STD-153 identity-keyed selection wording and unsupported_construct causes (QSpec PR #343)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@726b8cccbedda4546e5882c574464c487fc09fd1; proposals/checked-package-v2/schema.json, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/objects/interfaces/FR-340-checked-package-frame-location.md, spec/objects/interfaces/FR-342-checked-package-operation-anchor-body.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, spec/test-cases/TC-280-model-owned-member-types.md, spec/test-cases/TC-281-model-reference-equality-and-field-projection.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-340
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-342
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-280
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-281
    type: reviews
---
# SR-14686: integrity review of STD-153 (QSpec PR #343)

## Summary

Ticket: STD-153, items 1 and 2, plus items 3 and 4 from the 2026-10-02
comment. Diff reviewed: `git diff origin/main...726b8cc` (7 files).

Re-measure claims, checked independently against main `97bfd65`:

- Item 1. FR-307-AC-6 binds `import "test/geometry"` by identity alone and
  refuses a `version` or `digest` token, so no change is needed. The four
  AC rewrites (FR-322-AC-38, FR-340-AC-13, FR-342-AC-4, FR-342-AC-9) match
  their vectors. M2 is `Order` owned by `acme/billing` (TC-280 MA-05, MW-08),
  RE-12 cites FR-322-AC-38 and FF-06 cites FR-340-AC-13. A repo-wide grep
  finds no remaining "unselected ... version" wording.
- Item 2, confirmed. FR-322 "Model-owned members" step 1 and FR-322-AC-32
  refuse only a digest domain other than `sha256-jcs`
  (`stale_dependency`/`digest-domain-mismatch`), a missing document
  (`missing_import`/`missing-selection`) and a different identity
  (`invalid_model_binding`/`wrong-model-selection`). A same-identity document
  under the selected digest passes. `stale_dependency`/`content-mismatch`
  exists only for `dependency_selections` packages (FR-322 lines 125-137).
- Item 3, confirmed. FR-272's `unsupported_construct` row lists exactly
  `declaration-form`, `expression-form` and `not-yet-implemented`
  (FR-272-AC-20). The main schema's `cause_tag` enum had none of the three.
  At the head, the enum adds the three, and a new `DiagnosticCausePairing`
  branch restricts them to `unsupported_construct`. It is reached through
  `Diagnostic.allOf`. Every other use in spec/ and proposals/ (FR-208,
  FR-001, TC-946 R7 to R9, TC-330 SG-R14, TC-310, FR-452,
  backend-provider-v1 vectors) pairs them with `unsupported_construct`, so
  the restriction contradicts nothing.
- Item 4, confirmed. `LeafSegment` in both `schema.json` and
  `node-identity-preimage.schema.json` admits `recursion:(0|[1-9][0-9]*)`.
  FR-322 "Structural leaf walk" defines the recursion leaf (`laws` `[]`,
  `mode` `null`). `tests/checked_package_v2.rs` asserts
  `walk_recursion_leaf(&["field:next", "inner", "recursion:0"])`.

Gates: `quire validate` reports 134 failing documents (main 134). FR-322 and
FR-340 have the same diagnostics as on main, with line numbers moved. The
other touched files have none. `cargo test --test checked_package_v2 --test
backend_provider_v1` passes, 117 + 14.

Merge note (not a finding): `git merge-tree` of this head with PR #347
(STD-237) conflicts in FR-322. Both PRs edit adjacent AC rows (AC-37 there,
AC-38 here). Each PR merges cleanly onto main alone.

Pre-existing, outside this diff: the V2 `cause_tag` enum has no
`wrong-model-selection`, the cause FR-322-AC-32 uses. It is the same kind of
gap as item 3.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The rewritten precondition "The lock selects the identity `acme/orders` only." follows the bullet that introduces the second projection, `acme/runs`. Read literally, it covers the `acme/runs` cases too, and AP-01 ("Resolves `Counter/bump`") would then be `missing-selection`. Scope the sentence to the `acme/orders` fixture, or state what the `acme/runs` cases select. | spec/test-cases/TC-280-model-owned-member-types.md:78 |

## Verdict

Faithful to the ticket. All four re-measure claims hold. The schema change
is correct and contradicts no other pairing. One precondition is scoped
ambiguously (FND-001).

## Resolution

FND-001: the TC-280 precondition scopes the lock selection per projection: `acme/orders` for the `acme/orders` projection and `acme/runs` for the `acme/runs` projection.
