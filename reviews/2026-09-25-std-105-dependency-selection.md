---
id: SR-627
title: "Spec review — STD-105 DependencySelection for checked-package v2 dependency_selections"
type: SpecReview
analysis: failure-domain
scope: "STD-105; PR #159 head 3dc3c1f: FR-322 `dependency_selections` property and FR-307 reference edge, FR-322-AC-35, TC-233 procedure step and matrix row, schema.json `$defs/DependencySelection` and the removal of the `dependency` role from `Selection.role`, proposals/checked-package-v2/README.md, dependency-selection-vectors.json, tests/checked_package_v2.rs tc_233_dependency_selections_carry_each_dependency_package_identity"
review_set: subset
---
# Spec review — STD-105 DependencySelection for checked-package v2 dependency_selections

## Summary

Base checklist plus failure-domain review of PR #159 at 3dc3c1f against
`origin/main` c0f043f. The review covers only the PR diff.

The new entry `{identity, version, package_id}` fits the existing vocabulary.
`identity` and `version` are the `L` and `v` of FR-307's
`import "L" version "v" digest "d"`, spelled as in `ModelRef`. `package_id`
reuses the `PackageId` def, so its domain is checked by the existing const.
Ascending `identity` order and one entry per identity restate FR-307's lock
rule ("one selection per library identity, in ascending identity order").
The schema now types both `PackageLock` and `IdentityPreimage` entries the
same way.

Measured on this branch:

- `cargo test --locked --test checked_package_v2` with target dir
  `target/` inside the worktree: 42 passed, 0 failed. This includes
  `tc_233_dependency_selections_carry_each_dependency_package_identity`, which
  rebuilds the identity preimage and asserts the recorded `package_id`
  `a72ea6b33f2365998aa24d5a226f225f15d5531ae66a922b754cc5c988a90b57`.
  So the recorded id recomputes. The test also shows that changing a
  dependency's `package_id` changes the importing id.
- `quire validate` on the branch and on `main` both report 142 documents
  failing structural validation. After removing path prefixes and line
  numbers, the two logs differ only in the head SHA and in the criterion
  count (710/1421 against 709/1420, the new FR-322-AC-35). The branch adds no
  failures.

Consumers of the removed `dependency` role, from `git grep` on each
`origin/main`:

- IR (b52c263): `crates/quire-contract-model/src/checked_package/v2/vocabulary.rs:294`
  `CheckedSelectionRole::Dependency`. `v2/mod.rs:172,192` still type
  `dependency_selections` as `Vec<CheckedSelection>`.
- QSL (daa0178b): `qsl-package/src/checked_v2/tests.rs:732` and `:1616` build
  `selection("dependency", ...)` entries to test the
  `UnsupportedDependencySelections` refusal.
- CG (e2a5671): none. The `"dependency"` hits in
  `schemas/kani-proof-graph-v2.schema.json` are an unrelated proof-graph edge
  kind.
- QSpec fixtures: none. `fixtures/*.json` carry only empty
  `dependency_selections`.

Fixture placement is sound. `dependency-selection-vectors.json` is a delta
vector over `positive-all-families`, not a package document. It sits beside
`node-identity-vectors.json`, `model-member-type-vectors.json` and
`model-effective-declaration-vectors.json`, where the directory already keeps
vector files. It is authored in QSpec, the owning repository, and consumers
read it by reference through `QSPEC_DIR`, so it is neither a copy nor a
bypass. QSL's C-14 conformance test reads only `fixtures/positive-*.json` and
`adverse.json`. A complete positive package document with non-empty
`dependency_selections` could move into `fixtures/` once IR-287 types the
entry, and QSL-255 lets QSL's I2 reader admit non-empty selections with the
dependency packages supplied as evidence. Such a fixture would also need two
real library packages, because the vector's `3333…` and `4444…` ids are
placeholders.

## Verdict

**PASS after fixes**. FND-001 must be fixed in this PR before merge. The spec
states an ordering and uniqueness invariant that no reader rule enforces, and
IR-287 will implement the reader from this text. FND-002 is fixed in ticket
text, not in this repository. The low findings tighten the prose and the
vectors.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-322 requires one entry per library identity in ascending `identity` order, but no refusal enforces it. `uniqueItems` catches only byte-identical entries. A lock with `test/geometry`@1 and `test/geometry`@2, or with two entries out of order, passes the schema and recomputes its own `package_id`, so a conforming reader admits a non-unified diamond that FR-307 forbids. "Ascending" is also undefined for non-ASCII identities: the test compares Rust `&str` (UTF-8 byte order), while JCS orders keys by UTF-16 code units. No new cause is needed. Fix: state that a second entry of an identity refuses `invalid_package`/`conflicting-definition` (FR-307's diamond cause, with both entries as loci), and that an entry not strictly after its predecessor refuses `invalid_package`/`invalid-value` at that entry's pointer. Define the order as UTF-8 byte order. Add `duplicate-identity` and `misordered` vectors with those outcomes, and add a clause to FR-322-AC-35. Do not reuse `duplicate_selection`/`duplicate-identity`, which FR-271 scopes to domain-package model selections. | spec/objects/interfaces/FR-322-checked-package-artifact.md:62-65; FR-322-AC-35 (line 583); schema.json:29-30; dependency-selection-vectors.json:8; FR-307:95-96 |
| FND-002 | medium | The PR body calls the removed `dependency` role unused, but two repositories consume it. IR's `CheckedSelectionRole::Dependency` (vocabulary.rs:294) and `Vec<CheckedSelection>` typing (v2/mod.rs:172,192) accept it. QSL's `qsl-package/src/checked_v2/tests.rs:732,1616` build `selection("dependency", ...)` entries to reach `UnsupportedDependencySelections`. When IR-287 lands, those QSL tests get a wire-shape refusal instead and fail. IR-287's text does not mention removing the variant, and no QSL ticket covers re-authoring the tests. The removal itself is correct and should stay. Fix: add "remove `CheckedSelectionRole::Dependency`" to IR-287. Add to QSL-255, or a new QSL ticket blocked by IR-287, "re-author checked_v2 tests 732/1616 with a `DependencySelection` entry". Correct the PR body. | schema.json:27; IR vocabulary.rs:294; QSL qsl-package/src/checked_v2/tests.rs:732,1616 |
| FND-003 | low | "The entry is the `package` a `dependency_reference` names" conflicts with FR-322:181, where `package` is the dependency's `quire.package.semantic/v2` identity (a `PackageId`), not the whole entry. The binding sentence also names only `package_id` equality, while FR-307:55-58 binds identity, version and `package_id` and names the refusal for each mismatch. Fix: "A `dependency_reference`'s `package` equals one entry's `package_id`. A supplied dependency package binds to that entry only when its identity, version and recomputed `package_id` equal the entry's; otherwise it refuses as FR-307 states (`stale_dependency`/`revision-mismatch` or `byte-digest-mismatch`)." | FR-322-checked-package-artifact.md:72-75, 179-181; FR-307:55-58 |
| FND-004 | low | Spec style: "It is not a `Selection` or a `DefinitionRef`: a dependency is a checked package, not a definition document" names the superseded shape instead of stating the design. The closed `DependencySelection` shape already says what the entry is. The digest-domain clause after it is a real rule and should stay. Fix: delete the first sentence and write "a `quire.definition.bytes/v1`, `quire.source.bytes/v1` or `sha256-jcs` domain in `package_id` refuses `digest_domain_mismatch`". The spec diff has no roadmap wording. | FR-322-checked-package-artifact.md:69-72 |
| FND-005 | low | Vector `definition-selection-shape` uses `"role": "dependency"`, which this PR removes from `Selection.role`. The entry therefore fails for two reasons, and it would still fail if `DependencySelection` wrongly admitted a `Selection` shape with a valid role. Fix: use `"role": "profile"`, so the vector tests only the shape. | dependency-selection-vectors.json:9 |
| FND-006 | low | Each entry mutation records a reader `outcome` (`refused:malformed_wire` or `refused:digest_domain_mismatch`). The test asserts only `!schema.is_valid(&candidate)`, so a mutation that failed for an unrelated reason would still pass. TC-233's Expected Results ("Every mutation is refused by exactly its named check") do not say what these outcomes mean here. Fix: assert that every schema error's `instance_path` starts with `/{member}/dependency_selections/0`. Add one Expected Results sentence saying that each entry mutation fails the schema at the mutated entry, and that `outcome` is the reader expectation IR-287 consumes. | tests/checked_package_v2.rs:3755-3765; TC-233 Expected Results; dependency-selection-vectors.json:8-15 |

## Dispositions

| ID | Disposition |
| --- | --- |
| FND-001 | Open: fix in this PR before merge. |
| FND-002 | Open: ticket text in IR-287 and QSL. No change in this repository beyond the PR body. |
| FND-003 | Open: fix in this PR. |
| FND-004 | Open: fix in this PR. |
| FND-005 | Open: fix in this PR. |
| FND-006 | Open: fix in this PR. |
