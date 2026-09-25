---
id: SR-632
title: "Spec review — QSL-255 dependency input, import digest, replay dependencies and dependency_reference"
type: SpecReview
analysis: failure-domain
scope: "QSL-255; branch qsl-255-dep-design head 076a2fd against origin/main: FR-307 import digest spelling, library identity, supplied libraries and FR-307-AC-6; FR-322 library identity sentence, the `dependency_reference` wire and preimage term and FR-322-AC-36; FR-323 replay `package` reference `sources`/`dependencies` and FR-323-AC-7; schema.json and node-identity-preimage.schema.json (`dependency_reference` term, preimage `PackageId`); proposals/checked-package-v2/README.md; TC-227 L09, TC-233 procedure and expected results, TC-277 BP-06..BP-08; complete-v1 matrix rows; tests/checked_package_v2.rs tc_233_a_dependency_reference_enters_the_referencing_node_id"
review_set: subset
---
# Spec review — QSL-255 dependency input, import digest, replay dependencies and dependency_reference

## Summary

Base checklist plus failure-domain review of `git diff origin/main...HEAD` at
076a2fd. Only the diff was reviewed. QSL ADR-015 was read for context only.

What holds up:

- The wire term `{term: "dependency_reference", package, node}` is the same
  closed shape in both schemas. The preimage schema's new `PackageId` def is
  byte-identical to `schema.json`'s (same domain const, algorithm const and
  `HexDigest` pattern `^[0-9a-f]{64}$`).
- The new test changes only the callee of the published `function-call`
  vector. It checks all four mutations against both schemas and shows that
  `package` and `node` each move the node id. That matches FR-322-AC-36 and
  the TC-233 procedure step.
- Library identity as an unstructured non-empty string, compared by UTF-8
  bytes, matches FR-322's existing `dependency_selections` order rule. TC-227
  L09 tests that `test/geometry/extra` does not bind for `test/geometry`.
- FR-323's `stale_dependency`/`byte-digest-mismatch` for a dependency whose
  recompiled `package_id` differs agrees with FR-307's cause for a
  `package_id` difference.
- `invalid_package`/`conflicting-definition`, `missing_import`/`missing-selection`,
  `missing_declaration`/`missing-name` and `invalid_package`/`invalid-value`
  all exist in `quire.native.diagnostics/v1`
  (proposals/quire-v1/definitions/native-diagnostics.md:79-82).
- The matrix gains FR-307-AC-6 → TC-227, FR-322-AC-36 → TC-233 and
  FR-323-AC-7 → TC-277. The diff has no roadmap wording.

Measured:

- Catalog `quire-verification-contracts` 61f4a44
  `checked-operation-catalog-v1.json`: `quire.op.function.call` has
  `operands: ["function"]`, `rest: "any_term"` and `result: "return:0"`, as the
  change assumes.
- `quire validate --scope . <the six changed spec files> --summary`: FR-322
  and FR-323 fail structural validation (`[value_object] required 'id'`,
  `semantic.record-invalid`). Both failures exist on `main` and the diff does
  not cause them.
- Code spelling: the catalog's selected inventory is
  `quire-spec-language` f444d03 `src/diagnostic.rs`, where
  `Code::InvalidDigest.as_str()` is `"invalid-digest"`. `invalid_digest` does
  not occur anywhere on QSpec `origin/main`.

## Verdict

**PASS after fixes.** FND-001 through FND-004 must be fixed in this PR before
merge:

- FND-001: the import-digest refusal uses a code spelling the catalog does
  not have.
- FND-002: one of FR-323's refusals cannot happen, because FR-307 refuses
  first.
- FND-003: the `return:0` join cannot be expressed for a return type that
  the dependency declares.
- FND-004: the new reader rules have no place in the reader's
  first-refusal order.

FND-005 and FND-006 fill coverage and failure-mode gaps. The rest are low
wording fixes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-307, FR-307-AC-6 and TC-227 L09 spell the import-digest refusal `invalid_digest`. The catalog's selected code is `invalid-digest`, and the catalog says hyphenated and underscored spellings are exact and are not aliases. As written, the spec names a code that does not exist. | FR-307:58, FR-307-AC-6, TC-227 L09; native-diagnostics.md:24-30; quire-spec-language f444d03 src/diagnostic.rs:142 |
| FND-002 | high | FR-323 says an entry list that differs from the recompiled package's `dependency_selections` refuses `invalid_package`/`invalid-value`. That cannot happen for a removed entry or a changed version. A removed entry leaves an import with no supplied library, which FR-307 refuses first as `missing_import`/`missing-selection`. An entry whose `version` differs from its import is FR-307's `stale_dependency`/`revision-mismatch`. TC-277 BP-08's "one entry removed" case therefore expects the wrong refusal. | FR-323:35-41, FR-323-AC-7; TC-277 BP-08; FR-307:62-68 |
| FND-003 | high | For a callee reached by `dependency_reference`, FR-322 makes the `return:0` result "that function node's result type, which the referencing node's `result_type` names by node id in the importing graph". A `NodeRef` must name a node of the importing graph. A return type the dependency declares, such as a record `g::Point`, is keyed with its `declaration`. So the importing graph can hold that id only by copying the declaration node. The copy would become an export of the importing package (FR-307) and would clash as `ambiguous-name` with a local `Point`. The rule is sound only for types whose node ids do not depend on the package. | FR-322:214-218, FR-322:494-500 (`result_type` must be that same type node); FR-307 export rule |
| FND-004 | medium | The two `dependency_reference` refusals (`missing_declaration`/`missing-selection` for an unselected `package`, and `missing_declaration`/`missing-name` for a `node` that is not a declaration) have no place in the reader's first-refusal order. The catalog says a V2 reader "emits exactly one diagnostic for the first refusal it finds" in that order. Two readers could report different first refusals for the same package. | FR-322:199-215; native-diagnostics.md:119-160 |
| FND-005 | medium | FR-322-AC-36 tests only schema admission and preimage sensitivity. The new reader rules have no AC and no TC vector: the `missing-name` refusal, a `function` operand satisfied only when the target is a `function` node, the `return:0` result join, and "never listed in `dependencies`". This breaks the Coverage and Error Path rules. The operand family of a `dependency_reference` that is not a callee (a library value constant passed through `rest: any_term`, or a dependency relation) is also not stated. FR-322:472-474 defines family only for a local `reference`. | FR-322-AC-36; FR-322:210-218, 472-474; TC-233 |
| FND-006 | medium | Unstated failure mode: a `dependency_reference` or a `dependency_selections` entry whose dependency package is not supplied to the reader. FR-322 resolves the reference "only against the named dependency's admitted package", but no refusal covers the case where no admitted package is present. `missing-name` presumes that one is. | FR-322:189-201, 212-214 |
| FND-007 | low | FR-307 still says "A supplied library whose identity, version or `package_id` differs from the import". Libraries are now supplied and bound by identity, so an identity difference is an unsupplied identity, which is `missing_import`/`missing-selection`. The word "identity" in that sentence describes a case that cannot happen. | FR-307:68-71 |
| FND-008 | low | FR-307 gives no locus for the duplicate-supplied-library refusal, and none for `missing_import`. TC-227 L09 says "at the import" for the second, and names no locus for the first. The catalog requires a referring locus. | FR-307:65-68; TC-227 L09; native-diagnostics.md:79 |
| FND-009 | low | FR-307 refuses "any other spelling" of `d`, but FR-307-AC-6 and L09 test only the `sha256:` prefix. That leaves the Constraint Boundary rule unmet for uppercase hex, 63 and 65 characters. Existing L01 case 2 ("a digest of L's raw source bytes") gives no spelling, so its expected `byte-digest-mismatch` now depends on the digest being written as bare hex. | FR-307:55-58, FR-307-AC-6; TC-227 L01, L09 |
| FND-010 | low | FR-323's first `package` sentence still says the reference carries "the `RawSourceRef` digest of every source and definition document a replay recompiles". The new shape `{package_id, contract_version, sources, dependencies}` has no definition-document member, because lock `sources` are `RawSourceRef`s and definitions are `definition_selections`. FR-323-AC-7 also says "naming that entry's `identity` and both identities" where it means both `package_id`s, and FR-323 does not say which `version` the executor supplies a recompiled dependency under. | FR-323:23-30, FR-323-AC-5, FR-323-AC-7; FR-322 `PackageLock` |
| FND-011 | low | The "reference targets" in FR-322's `application_node_preimage` dependency derivation now has to exclude `dependency_reference`. The exclusion is stated only in the new paragraph, far from the derivation it changes. A reader implementing the graph-shape check from the derivation sentence would list the target and refuse a valid package as `invalid_semantic_graph`. | FR-322:176-180, FR-322:210-212 |

## Finding detail and fixes

### FND-001 — `invalid_digest` is not a catalog code

Fix: replace `invalid_digest` with `invalid-digest` in FR-307 line 58, in
FR-307-AC-6 and in TC-227 L09. Leave out any cause, because the catalog
defines no required cause for `invalid-digest`. QSL ADR-015 D-2 and QSL TC-446
use the same wrong spelling. Tell the QSL session, but do not edit those files
from this PR.

### FND-002 — FR-323's entry-list refusal against FR-307's

Fix FR-323 by replacing the last sentence of the `package` bullet with:

> The executor supplies each recompiled dependency under its entry's
> `identity` and `version`. An import whose identity no entry supplies, or
> whose version differs from its entry's, refuses as FR-307 states
> (`missing_import`/`missing-selection` or
> `stale_dependency`/`revision-mismatch`). Entries not in strictly ascending
> UTF-8 byte order of `identity`, or an entry that no import of the
> recompiled package reaches, refuse `invalid_package`/`invalid-value` at
> `/package/dependencies`.

Make the same change to FR-323-AC-7. In TC-277:

- Change BP-08 to "entries swapped; then an extra entry no import reaches",
  expecting `invalid_package`/`invalid-value` at `/package/dependencies`.
- Add BP-09, "one entry removed", expecting
  `missing_import`/`missing-selection` at the import.
- Add BP-10, "an entry's `version` changed", expecting
  `stale_dependency`/`revision-mismatch`.

Trace BP-09 and BP-10 to FR-323-AC-7.

### FND-003 — `return:0` join for a dependency-declared return type

Fix: let a type position name a dependency node. In `schema.json` and the
preimage schema, change `result_type` of `application` (and a node's
`semantic_type`) from `NodeRef` to
`oneOf [NodeRef, {package: PackageId, node: NodeId}]`. Then state in FR-322:

> A result the catalog derives from a `dependency_reference` operand names
> the dependency's type node. When that node is keyed independently of its
> package (a builtin scalar or structural type with no `declaration`), the
> `result_type` is the importing graph's node of that id. Otherwise it is
> `{package, node}` naming the type node in the dependency's admitted
> package, and the importing graph carries no copy of it.

Add a TC-233 vector for a call to a dependency function that returns a
declared record type.

If the owner rules instead that Complete V1 dependency functions return only
package-independent types, write that rule as a positive statement ("a
function reached by `dependency_reference` has a result type whose node
carries no `declaration`"). Add its refusal (`ill_typed`/`operator-ineligible`)
and a vector for it.

### FND-004 — place the dependency-reference refusals in the reader order

Fix: in native-diagnostics.md step 7, list the two `dependency_reference`
refusals as the first per-argument checks of an application argument, before
`operation-mode-type-mismatch` and `operator-ineligible`. Use the same
pre-order as the model-owned-member steps. A `dependency_reference` outside
any application gets its refusals checked with step 7 at its node in
ascending node-id digest order. Add one FR-322 sentence pointing to that
position.

### FND-005 — AC and vectors for the reader rules

Fix: add FR-322-AC-37:

> A call whose callee is a `dependency_reference` to a selected dependency's
> `function` declaration node is admitted with its `result_type` equal to
> that function's result type and with no entry for the target in
> `dependencies`. A callee naming a non-declaration node of the dependency
> refuses `missing_declaration`/`missing-name`. A callee naming a declared
> non-`function` node refuses `ill_typed`/`operator-ineligible`. A
> `dependencies` list that includes the target refuses
> `invalid_semantic_graph`. A `package` absent from `dependency_selections`
> refuses `missing_declaration`/`missing-selection`.

Add TC-233 vectors for each clause and a matrix row. In FR-322:472-474, add:

> a `dependency_reference` has the family a `reference` to its target node
> would have, read in the dependency's admitted package

### FND-006 — dependency package not supplied to the reader

Fix: add to FR-322's `dependency_selections` bullet:

> A reader admitting a package with a non-empty `dependency_selections` is
> supplied each selected dependency's admitted package. An entry with none
> supplied refuses `missing_import`/`missing-selection` at that entry's
> pointer, before any `dependency_reference` check.

This reuses FR-307's cause and the model-selection precedent in step 2 of the
reader order. Add one vector for it.

### FND-007 — stale sentence in FR-307

Fix: change the sentence to "A supplied library whose version or `package_id`
differs from the import is …". The two causes stay as they are.

### FND-008 — loci for supplied-library refusals

Fix: in FR-307, say that the duplicate refusal retains both supplied
libraries' identity and version as loci, and that `missing_import` is
reported at the import's identity string. Add both loci to TC-227 L09.

### FND-009 — digest-spelling boundaries

Fix: extend L09 and FR-307-AC-6 with an uppercase-hex digest, a 63-character
digest and a 65-character digest, each expecting `invalid-digest` at the
digest string. Change L01 case 2 to "the bare 64-hex SHA-256 of L's raw
source bytes".

### FND-010 — FR-323 wording

Fix FR-323 as follows:

- Replace the first sentence of the `package` bullet with "exact FR-322
  checked-package reference; a replay request adds the `sources` and
  `dependencies` members below". Say that definition and domain-package
  bytes are reached through `byte_provision` by the digests the recompiled
  sources name.
- In AC-7, change "both identities" to "its `package_id` and the recompiled
  one".
- The supplied-version rule is covered by the FND-002 text.

### FND-011 — dependency derivation sentence

Fix: in FR-322's `application_node_preimage` bullet, change "the unique
digest-ascending reference targets and member declarations of its body" to
"the unique digest-ascending `reference` term targets and member
declarations of its body; a `dependency_reference` contributes none".

## Dispositions

| ID | Disposition |
| --- | --- |
| FND-001 | Open: fix in this PR. Tell the QSL session about ADR-015 D-2 and TC-446. |
| FND-002 | Open: fix in this PR. |
| FND-003 | Open: fix in this PR. The design choice belongs to the spec owner. |
| FND-004 | Open: fix in this PR. |
| FND-005 | Open: fix in this PR. |
| FND-006 | Open: fix in this PR. |
| FND-007 | Open: fix in this PR. |
| FND-008 | Open: fix in this PR. |
| FND-009 | Open: fix in this PR. |
| FND-010 | Open: fix in this PR. |
| FND-011 | Open: fix in this PR. |

## Dispositions

All findings are fixed in commit 56f3934.

- FND-001: `invalid-digest` in FR-307, FR-307-AC-6 and TC-227 L09. The QSL
  ADR-015 and TC-446 spelling is fixed in the QSL change.
- FND-002: FR-323 and FR-323-AC-7 take the suggested wording; TC-277 BP-08
  is swapped entries and an unreached entry, and BP-09 and BP-10 are added.
- FND-003: fixed by the positive rule. A `dependency_reference` names a
  `function` whose parameter and result types carry no `declaration`, so
  the importing graph holds its result type under the same id; any other
  target refuses `ill_typed`/`operator-ineligible`.
- FND-004: FR-322 and native-diagnostics step 7 place the checks.
- FND-005: FR-322-AC-37 with TC-233 vectors and a matrix row; the family
  sentence is added.
- FND-006: the supplied-dependency sentence is added to FR-322 and covered
  by FR-322-AC-37.
- FND-007 to FND-011: fixed as suggested.
