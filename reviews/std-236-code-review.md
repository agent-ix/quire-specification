---
id: SR-15560
title: "code review of STD-236 published extension definitions (QSpec PR #365)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@b0ad3e9dc8e3f153b9e52764d4c5bc34d79ca585 (diff against origin/main 35fc4ea); tests/checked_package_v2.rs, proposals/checked-package-v2/extension-definitions.json, proposals/checked-package-v2/extension-definitions.schema.json, proposals/checked-package-v2/extension-definition-vectors.json, proposals/checked-package-v2/README.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-469
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-951
    type: reviews
---
# SR-15560: code review of STD-236 published extension definitions (QSpec PR #365)

## Summary

Ticket: STD-236. PR: quire-specification#365, head b0ad3e9. This review covers the Rust lane (rust-review) over `tests/checked_package_v2.rs` and the new published JSON files. The rest of the restored #341 content is byte-identical to #341, which SR-14713, SR-14714 and SR-14715 reviewed, so it is not re-reviewed here.

Units examined:

- `FR-469-AC-9` (examined), bound to `tc_951_published_definitions_are_fr_467_members_and_withholding_one_refuses_its_fixture`
- `published_extension` and `held_retention`/`held_quota`/`held_policy` (examined)
- `extension-definitions.json` against IR `CheckedArtifactRef` and `CheckedExtensionDefinition` at quire-contract-ir main (includes be1ae59 IR-960) (examined)
- `extension-definitions.schema.json` (examined)
- `extension-definition-vectors.json` XD-01 to XD-04 (examined)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Minor: `tc_951_published_definitions_...` builds the held set from the hard-coded list `[held_policy(), held_quota(), held_retention()]` instead of from the entries of `extension-definitions.json`. If the file publishes a fourth definition, the test never holds it, so the test does not exercise the FR-469 loader rule ("holds the published definition whose reference equals each selection"). Build `all` from the file's `definitions` entries through `published_extension`. | tests/checked_package_v2.rs:16353 |

## Verdict

One minor finding and no major findings. The published JSON deserializes into IR's `CheckedArtifactRef`/`CheckedExtensionDefinition` (`deny_unknown_fields`, `type` renamed to `type_ref`), and its values equal IR's conformance `retention_definition`/`quota_definition`/`policy_definition`. A QSL loader can therefore deserialize each entry with serde from QSPEC_DIR at run time, with no vendored copy. The schema is stricter than IR on `minItems` and non-empty text, and FR-467 Members supports that. XD-02/XD-04 locus and pointers match the fixtures (node 5 and node 34, digest 7aa76a0f...1832). Tests: `cargo test --test checked_package_v2 --test backend_provider_v1` passes with 23 and 128 tests.

## Dispositions

Round 1, reviewed at 345c3426a68a36982cfd9fbe23ab35f7052a0988. Gates: quire validate gives 86, and cargo `checked_package_v2` (23 passed) and `backend_provider_v1` (129 passed) pass.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 345c342: `published_extensions()` maps every `definitions` entry of the file through `held_entry`, and `tc_951_withholding_a_published_definition_refuses_its_fixture` holds `let all = published_extensions();`. |
