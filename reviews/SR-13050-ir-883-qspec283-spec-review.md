---
id: SR-13050
title: "IR-883 QSpec PR 283 spec review: direct dependency selections and derived closure"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@7baa1104b8a37f8606a6c05a9e0c43fba8ddada8 vs origin/main a2230e1; spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/functional/tooling/FR-307-package-reusable-semantic-libraries.md, spec/objects/interfaces/FR-323-native-runtime-envelope.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, spec/test-cases/TC-277-replay-byte-provision.md, proposals/checked-package-v2/dependency-selection-vectors.json; context FR-330, FR-331"
review_set: subset
---

## Summary

Ticket: IR-883 (owner ticket for QSL-879). The requirement is the IR-883 decision comment (option (a)). It is treated as data and re-measured here. The PR states most of the decision correctly:

- `dependency_selections` lists the libraries the unit imports directly (FR-322, FR-307).
- The closure is derived, and a cross-lock diamond refuses `invalid_package`/`conflicting-definition`.
- The entry shape, order and duplicate rules are unchanged.
- The spec says "direct only" is a recompile-checked canonical form that the reader does not enforce (FR-322:130-132).
- A transitive-only `dependency_reference` refuses `missing_declaration`/`missing-selection` (FR-322:463-465).
- The bytes/`package_id` change is stated, with no compat layer.
- TC-277 BP-06 still requires every source in the closure, transitive-only ones included, to reach the executor. FR-323 now lists every closure identity in `dependencies`, which is stronger than before.

The defects are in how the derivation is worded, in the locus form, and in two neighbouring interfaces the PR did not update (FR-330, FR-331). FR-323 replay rules 2 and 4 are also inconsistent, and FR-307 has no criterion that makes the producer list direct imports only.

## Examined criteria

- FR-322 `dependency_selections` bullet (examined), FR-322:96-136.
- FR-322 `dependency_reference` bullet (examined), FR-322:451-466.
- FR-322-AC-35 (examined).
- FR-307 Description and Behavior (examined), FR-307:109-123; FR-307-AC-1..6 (examined: none states the direct-only producer form).
- FR-323 `package.dependencies` and replay rules 1-5 (examined), FR-323:30-70; FR-323-AC-7 (examined).
- TC-233 procedure and expected results (examined); TC-277 BP-06..BP-09 and the closing note (examined).
- FR-330 dependency inputs (context_only, FR-330:418-436); FR-331 `request.dependencies` (context_only, FR-331:149-151).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-322 derives the importer's closure from "its direct entries and the closure recorded in each admitted dependency package's own lock", but under this PR no lock records a closure, only direct entries. As written, the derivation stops at depth two. It should say the closure is derived recursively from each admitted dependency's own direct entries (or from that package's derived closure), and that an admitted package exposes its derived closure to an importer | spec/objects/interfaces/FR-322-checked-package-artifact.md:124-126 |
| FND-002 | medium | "both dependency paths as loci" is never defined. Every other selection refusal uses JSON-pointer loci (`/lock/dependency_selections/i`). The new vector records loci as identity sequences `[["test/a","test/leaf"],["test/b","test/leaf"]]`, which include neither the importer nor a pointer. Neither FR-322 nor FR-307 fixes the form, so the IR reader cannot be checked against the vector deterministically | spec/objects/interfaces/FR-322-checked-package-artifact.md:128-130; proposals/checked-package-v2/dependency-selection-vectors.json:39 |
| FND-003 | medium | FR-331 `request.dependencies` still carries "the v2 bytes of each package the lock's `dependency_selections` names". With direct-only locks this leaves out transitive-only packages, which the backend needs in order to admit each direct dependency (FR-322 requires every selected dependency's admitted package, recursively). This is a stale use of the old closure meaning | spec/objects/interfaces/FR-331-backend-provider-envelope.md:149-151 |
| FND-004 | medium | FR-330 (ContractPackage reader) takes one admitted dependency per direct entry and checks identity and `package_id` per entry, but has no cross-path diamond check. Two direct dependencies that are each validly admitted can still disagree on a shared transitive identity, so FR-330 needs the same `conflicting-definition` rule the decision assigns to `validate_dependencies` | spec/objects/interfaces/FR-330-contract-package.md:418-436 |
| FND-005 | medium | FR-323 replay rule 2 gains the clause "the selected identity and `package_id` must match the selecting package's own lock, including locks of recompiled dependencies", which has no refusal code or cause and overlaps rule 4. Rule 4 still says "the entry's `package_id` is the one the proved package's lock pinned", which is false for a transitive-only entry, whose pin is in a dependency's lock | spec/objects/interfaces/FR-323-native-runtime-envelope.md:51-53,56-63 |
| FND-006 | medium | No criterion makes a producer emit direct-only selections. FR-307-AC-1..6 and FR-322-AC-35 all still pass for a compiler that keeps listing the full closure. That is the O(n^2) behaviour IR-883 removes, and the canonical form the spec says is "checked by recompilation". FR-307 needs an AC for a chain A->B->C in which A's lock carries only B, plus a recompile check that refuses or flags an over-listed entry | spec/functional/tooling/FR-307-package-reusable-semantic-libraries.md:119-123,135-139 |
| FND-007 | low | The change statement "a package's `package_id` changes iff its former closure differs from its direct set (depth at least three), and then every package whose dependency tree contains one changes" contradicts itself: an importer whose closure equals its direct set still changes when a dependency's id changes, so "iff" is wrong. "depth at least three" has no unit (packages or edges). The statement is also migration prose inside a normative member bullet | spec/objects/interfaces/FR-322-checked-package-artifact.md:132-135 |
| FND-008 | low | TC-277 BP-08 still says "the two `dependencies` entries swapped", but BP-06 now has three entries (two direct, one transitive-only), so which pair to swap is unstated | spec/test-cases/TC-277-replay-byte-provision.md:46 |

## Verdict

**FAIL as-is (draft): medium findings open.** The decision is carried through faithfully at the FR-322/FR-307 core, and BP-06 is not weakened. FND-001..FND-006 should be fixed in this PR before the IR implementation is coded against it. FND-003 and FND-004 are stale or missing in neighbour interfaces that use `dependency_selections` and fall within the decision's blast radius.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-009 | low | FR-331 now says `request.dependencies` carries the recursively derived closure, but the published backend-provider schema's `dependencies` description still reads "the v2 bytes of each package the lock's dependency_selections names", the old direct-only wording the FR-331 fix replaced | proposals/backend-provider-v1/schema.json:210 |
| FND-010 | low | FR-323 rule 4 and FR-323-AC-7 add a refusal: a mismatch between an entry and the selecting package's direct lock refuses `stale_dependency`/`content-mismatch` naming the selecting lock pointer. No TC-277 BP row exercises it. BP-07 still says the entry's `package_id` is "the one the proved package's lock pinned", which is false when the edited entry is the transitive-only one | spec/test-cases/TC-277-replay-byte-provision.md:45; spec/objects/interfaces/FR-323-native-runtime-envelope.md:62-68 |

## Dispositions

Disposition pass 1, against head 78abc809a95ca92c2e426e0d78569566d1f81aec (fix commit 73aec7e).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-322 now derives the closure "recursively from its direct entries and each selected dependency's derived closure", says an admitted dependency exposes that closure, and says no lock records closure entries |
| FND-002 | fixed | FR-322 defines each locus as one importer-rooted JSON pointer, `/lock/dependency_selections/{i}` then `/lock/dependency_selections/{j}` per hop, with the final entry the conflicting one. The vector loci are now `/lock/dependency_selections/0/lock/dependency_selections/0` and `/lock/dependency_selections/1/lock/dependency_selections/0`, which match. FR-330 roots the same form at `/package/source/lock/...` |
| FND-003 | fixed | FR-331 `request.dependencies` now carries every package in the recursively derived closure, transitive-only ones included (the schema description residue is FND-009) |
| FND-004 | fixed | FR-330 adds the recursive diamond check with importer-rooted JSON-pointer loci before graph references, plus FR-330-AC-11 (Test (TC-937, planned)) and a planned TC-937 case |
| FND-005 | fixed | Rule 2 now states only the binding ("each import binds to the entry named by its selecting package's own direct lock"). Rule 4 drops "pinned by the proved package's lock" and gives the selecting-lock mismatch an explicit `stale_dependency`/`content-mismatch`. BP-06 still requires every transitive source (residue is FND-010) |
| FND-006 | fixed | FR-307 gets producer prose and FR-307-AC-7 (chain A->B->C: A emits exactly B, and recompiling does not certify an A lock that also lists C), with TC-227 L10 planned. The AC can fail: a closure-listing producer violates it |
| FND-007 | fixed | The "iff" is gone. Depth is defined in edges, and the text says importers change recursively when a direct dependency's id changes |
| FND-008 | fixed | BP-08 now swaps the first and second entries, with the third left in place |
