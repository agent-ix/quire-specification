---
id: SR-14713
title: "spec-integrity analysis of STD-236 (QSpec PR #341)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@59636cbbbf03420bfe01001bd43e5744562e8776 (diff against origin/main 97bfd65); spec/functional/source-package/FR-133-extend-language-uniformly.md, spec/functional/source-package/FR-467-define-extension-definition-members.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/objects/interfaces/FR-469-checked-package-extension-nodes.md, spec/spec.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, spec/test-cases/TC-951-checked-package-extension-nodes.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-469
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-951
    type: reviews
---
# SR-14713: spec-integrity analysis of STD-236 (QSpec PR #341)

## Summary

Ticket: STD-236. PR: quire-specification#341, head 59636cb.

FR-469 is internally consistent with FR-322: every code it uses is in FR-322's V2 read vocabulary, the causes pair as FR-272 does (`unknown_required_capability` carries no cause and takes the node id as locus), the types namespace already covers "every other declared node", and the `quire.structural-node/v1` preimage shape (`semantic_type: null`, `recursion: null`, owner, declaration, body) matches the self-typed declaration nodes already published. Ids: FR-469 and TC-951 are not used on main or by any other open QSpec PR (#200, #207, #267, #338 to #352 checked); TC-952 is taken by #351, not by this PR; FR-322 gains no AC id. The PR records no digests, node keys, package_id or JCS preimage values, and adds no fixture JSON, so there is nothing to recompute; the keys can only be checked once `positive-extension-nodes.json` exists. Refusal-code judgment (lead default 2): `unknown_required_capability` is right for QSpec. FR-133 says a consumer that lacks an extension reports "the unknown required capability"; FR-271 defines that code as a package requiring a capability the reader does not know; it is in FR-322's V2 read vocabulary while `unknown_required_feature` is not; and FR-272's `unknown-feature` cause cannot pair with it, because the code carries no cause. Proposed resolution: QSL FR-354 (its refusal table row and FR-354-AC-5) changes to `unknown_required_capability` with no cause, naming the node by `locus`; QSpec keeps the PR's default and adds the `required_features` rule of FND-002. Gates: `quire validate` 134 (baseline 134); FR-322 keeps its existing diagnostics (same as main), FR-469 and TC-951 are clean; cargo `checked_package_v2` + `backend_provider_v1` pass (14 + 117), as expected, because the tests still enforce the 13-tag set.

Units examined:

- `FR-469-AC-1` (examined)
- `FR-469-AC-2` (examined)
- `FR-469-AC-3` (examined)
- `FR-469-AC-4` (examined)
- `FR-469-AC-5` (examined)
- `FR-469-AC-6` (examined)
- `FR-469-AC-7` (examined)
- `FR-469` (examined)
- `FR-322-AC-13` (examined)
- `TC-951` (examined)
- `TC-233` (examined)
- `FR-133-AC-3` (context_only)
- `FR-467-AC-1` (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Major: after merge FR-322 lists `extension` among the current closed node tags and FR-322-AC-13 and TC-233 say the published positive fixture carries all 14 tags, but the published artifacts on the PR head hold 13: `proposals/checked-package-v2/schema.json` has no `extension` tag or `extension_form` form, `fixtures/positive-all-families.json` carries exactly 13 tags, and `tests/checked_package_v2.rs` pins `REQUIRED_NODE_TAGS: [&str; 13]`; FR-469's "Published schema" property names a `positive-extension-nodes.json` that does not exist. The spec and its own schema contradict (a reader that follows the schema refuses an `extension` node), and AC-13 is false on the tree. The ticket asks for FR-322 and its vectors to be updated by the same procedure as other node families; either land the schema, preimage, fixture and test changes listed under Remaining work in this PR, or keep the tag list and AC-13 at 13 until they land. | spec/objects/interfaces/FR-322-checked-package-artifact.md:181-184 |
| FND-002 | low | Minor: FR-469 does not say whether a selected extension also appears in `lock.required_features`. FR-322 refuses unknown required features before consumption (`unknown_required_feature`), so a producer that lists the extension there makes a reader lacking it refuse at the lock with `unknown_required_feature`, never reaching reader join 2's node-level `unknown_required_capability`, which is the code QSL FR-354 states today. State that an extension is carried only by `definition_selections` (not `required_features`), or order the two refusals. | spec/objects/interfaces/FR-469-checked-package-extension-nodes.md:112-121 |

## Verdict

Not mergeable as is: one major finding (FND-001, spec versus published schema, fixture and test) and one minor (FND-002).

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | high | Major: head e75fc63 does not contain origin/main bb6d08a (main gained #338, #348, #339, #340 and #343 since the merge-base 8d59511), and a trial merge conflicts in `dependency-selection-vectors.json` and `fixtures/positive-all-families.json`. #348 (STD-93) changed both files, so the identities this PR recomputed (all-families `package_id` 14ce321a..., dependency-vector `package_id` 42d802da..., closure `test/a` 9cdb7621..., `test/leaf` 14ce321a...) are stale on the merged fixture. Merge or rebase onto main, recompute those identities on the merged all-families fixture, and rerun the gates. | proposals/checked-package-v2/fixtures/positive-all-families.json |
| FND-004 | low | Minor: after merge, main's FR-322 `dependencies` member (added by #348) lists the node forms whose body artifact states its own `dependencies` list (FR-341, FR-342, FR-370, FR-375, FR-402, FR-440, FR-451, FR-464 to FR-466). FR-469 states the extension node's list ("exactly the unique digest-ascending targets of its clause references") but is not in that list or in its example table. Add FR-469 and an `extension`/`extension_form` row when resolving FND-003. | spec/objects/interfaces/FR-322-checked-package-artifact.md |

## Dispositions

Round 1, reviewed at e75fc63294fd81b2a8d46e9d1c241a9018be3b29 (fix commit e75fc63 on top of merge b35638c). Recomputation: an independent Python script (JCS by sorted-key compact JSON, SHA-256) reproduces every key and id on the head. The extension node key 7aa76a0f... is the digest of its `ExtensionFormNode` preimage. `positive-extension-nodes.json` `package_id` 92c0cf44... and `positive-all-families.json` `package_id` 14ce321a... match. The dependency-vector `package_id` 42d802da..., both closure vectors' `test/a` and `test/leaf` ids, and the `dependency_reference` package match. Each `identity_projection` equals its graph without occurrences. Both packages hold 14 tags and `required_features` `["quire.value.complete/v1"]`. Gates on the head: `quire validate` gives 134 (baseline 134). FR-322's diagnostics are the same as main's, and FR-469 and TC-951 are clean. cargo `checked_package_v2` gives 125 passed and `backend_provider_v1` gives 14 passed.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | e75fc63: schema.json `NodeBase`/`NodeProjection` enumerate `extension`, `ExtensionNode` binds form `extension_form`, `BodyBindingRules` binds `ExtensionFormBody` with exactly one `declaration` occurrence; preimage schema adds `ExtensionFormNode`; `positive-all-families.json` carries 14 tags; `positive-extension-nodes.json` exists; `REQUIRED_NODE_TAGS: [&str; 14]`. |
| FND-002 | fixed | e75fc63: FR-469 "Carriage" property and FR-469-AC-8 (TC-951 X17). |
