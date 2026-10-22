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
