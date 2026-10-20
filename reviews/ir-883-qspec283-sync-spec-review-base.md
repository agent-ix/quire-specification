---
id: SR-13416
title: "Spec review — direct selections after main sync"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; PR #283 964c28e vs origin/main 7c66d67; FR-307, FR-322, FR-323, FR-330, FR-331, TC-227, TC-233, TC-277, TC-937, dependency-selection vectors, tests/checked_package_v2.rs"
review_set: subset
---

## Summary

Ticket: IR-883. Direct-lock semantics, recursive closure, reference scope, replay input closure, ContractPackage diamond order, new TC links and main-side AC renumbering were checked. Prior SR-13050 findings remain fixed on 964c28e.

## Examined units

| ID | Role | Path | Excerpt |
| --- | --- | --- | --- |
| FR-307-AC-7 | examined | spec/functional/tooling/FR-307-package-reusable-semantic-libraries.md | For a chain A imports B and B imports C, producing A emits exactly B in A's `dependency_selections`, while B emits exactly C in its own lock; A's derived closure includes B and C. Recompiling A from its source produces a different lock and does not certify an A lock that also lists C as canonical producer output, although a reader does not enforce direct-only form from the lock alone. |
| FR-322-AC-35 | examined | spec/objects/interfaces/FR-322-checked-package-artifact.md | A `dependency_selections` entry is a closed `DependencySelection` `{identity, package_id}` whose `package_id` is a `quire.package.semantic/v2` `PackageId`: the published package carrying two such direct entries, one per identity in ascending identity order in both its lock and identity preimage, is admitted by the V2 schema and its recorded `package_id` recomputes from them, and changing a depende |
| FR-322-AC-75 | examined | spec/objects/interfaces/FR-322-checked-package-artifact.md | A diamond with two direct dependencies whose recursively derived closures reach one library identity at different `package_id`s refuses `invalid_package`/`conflicting-definition`, with both importer-rooted selection-path JSON pointers as loci. A `dependency_reference` naming a transitive-only package refuses `missing_declaration`/`missing-selection` even when that package is in the derived closure |
| FR-322-AC-76 | examined | spec/objects/interfaces/FR-322-checked-package-artifact.md | In the published closure vectors, each direct dependency's recorded `package_id` recomputes by JCS SHA-256 from its base fixture's `identity_preimage` with that dependency's direct selections substituted; the diamond's two locks select the same transitive identity with distinct recorded package IDs, and the reference vector names a package in the derived closure absent from the importer's direct s |
| FR-323-AC-7 | examined | spec/objects/interfaces/FR-323-native-runtime-envelope.md | A replay request's `package` reference names the proved package's own `sources` and one `dependencies` entry per identity in its derived closure, in ascending UTF-8 identity order, each carrying that dependency's `identity`, `package_id` and own lock `sources`, including transitive-only dependencies. The executor refuses by the first rule of its order: swapped or repeated entries `invalid_package` |
| FR-330-AC-11 | examined | spec/objects/interfaces/FR-330-contract-package.md | A reader `refused` result is a diagnostic at stage `package_read` carrying the reader's code and cause unchanged, every returned code having an FR-271 row and the category its code and cause map to; its `path` is the FR-274 `json_path` location and a node-level refusal carries the `locus` payload. A wrong outer contract version is `unknown_contract_version` and bytes that are not well-formed JSON  |
| FR-330-AC-12 | examined | spec/objects/interfaces/FR-330-contract-package.md | A reader `incomplete` result is `stage_limit_exceeded` at stage `package_read`, category incomplete, with the cause the table gives for its `limit_kind` (`bytes`, `nodes`, `edges`, `occurrences`, `work`), `bound` its `limit`, `actual` its `consumed`, the location its `path` (none for `bytes`) and `setting` the value `CheckedPackageReadLimits.<limit_kind>`; an exact-limit input is admitted and the  |
| FR-330-AC-13 | examined | spec/objects/interfaces/FR-330-contract-package.md | The ContractPackage reader derives every direct input's closure recursively and refuses a diamond that reaches one library identity at two different `package_id`s as `invalid_package`/`conflicting-definition` with both selection-path JSON pointers rooted at `/package/source/lock/dependency_selections` as loci, before graph-reference checks. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

**PASS for the reviewed diff.** The normative changes remain internally consistent. Main-side FR-330 diagnostic AC-11/12 and FR-322 AC-77/78 were preserved; the new FR-330 diamond check is AC-13.
