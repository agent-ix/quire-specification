---
id: SR-1306
title: "Code review of PR #194 (QSL-650): tests/backend_provider_v1.rs"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@045b37b3bd08d2f5a55b65b424274e7ac435d922; PR #194 diff against origin/main 65e816f; tests/backend_provider_v1.rs with proposals/backend-provider-v1/{schema.json,vectors.json} as its data"
review_set: subset
---
# Code review of PR #194 (QSL-650)

## Summary

Ticket: QSL-650. This reviews the new TC-431 qualification test, which
validates the backend-provider vectors against the schema and checks the
package-bytes rule. The Rust-review lane is folded in: idiom, panics, trace
tags, and tautology and stub detection. The oracles were measured
independently. Every adverse vector was re-run with Python `jsonschema` 4.26
against its own stage schema, and the error locations were compared with the
patched member.

Examined:
- `encode`/`decode` strict base64url, lines 32-76 (examined)
- `validator` registry for `urn:quire:checked-package:v2`, lines 82-97 (examined)
- `composed` and `apply`, lines 99-159 (examined)
- tc_431_positive_vectors_are_admitted_with_owned_members (examined)
- tc_431_adverse_vectors_are_refused (examined)
- tc_431_package_member_carries_the_v2_bytes_of_its_package_id (examined)
- tc_431_noncanonical_package_spellings_refuse (examined)
- tc_431_vector_byte_members_decode_and_provision_digests_bind (examined)
- tc_431_kani_vectors_cover_every_terminal_value (examined)
- tc_431_item_level_causes_reach_negotiation (examined)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The `package_id` assertion is self-referential. `composed` takes `request.package_id` from the same fixture that supplies the bytes, so lines 255-258 compare the fixture's `package_id` with itself after a base64url round trip. The package identity is never recomputed. Recompute it from the decoded bytes, `sha256(JCS(identity_preimage))` as `checked_package_v2.rs:377` `refresh_package_id` does, and compare that with `request.package_id`. Then AC-18's "whose `package_id` equals the request's" has a real oracle. | tests/backend_provider_v1.rs:101-114, tests/backend_provider_v1.rs:254-258 |
| FND-002 | low | The doc comment says "The strict decoder and the schema's `Base64Url` pattern agree: each non-canonical spelling … is refused by both", and TC-431 BP-04 says the same. The test only calls `decode`. The schema half lives in the adverse-vector test, and nothing checks that the pattern admits every `encode` output. Here, assert that the validator's `Base64Url` refuses the four spellings and admits each round-trip `encode(bytes)`, or narrow the comment and BP-04. | tests/backend_provider_v1.rs:261-273 |
| FND-003 | low | The adverse oracle is `!validator.is_valid` and nothing more. AC-19 calls each adverse vector "a single-member change", but the test does not assert that a patch has exactly one op or that the refusal lies at the patched member. Measured today, all 66 adverse vectors are refused at or directly above their patch path, so the oracle holds. A future vector refused for an unrelated reason would also pass. Assert `patch.len() == 1` and that some `iter_errors` instance path is a prefix of, or lies under, the patch path's parent. | tests/backend_provider_v1.rs:222-231 |
| FND-004 | low | The owners check accepts any `FR-<digits>`, so a typo such as `FR-9999` passes. "Names an FR" is only a format check. Check that the named FR exists, for example that one `spec/**/FR-<n>-*.md` file is present. | tests/backend_provider_v1.rs:185-201 |
| FND-005 | low | The Kani coverage test does not cover every row of the table it is traced to (FR-331-AC-20). It never asserts the `Inconclusive` row's typed cause (`replay-parity`) or the `Unavailable` row's diagnostic `unsupported_projection`/`tool-unavailable` with `detail.absent`. Its `measured` assertion (line 352) has nothing to do with AC-20. Assert one entry per table row, keyed on result and cause or diagnostic, and drop or move the `measured` check. | tests/backend_provider_v1.rs:302-353 |

## Verdict

The code is clean, idiomatic Rust. The strict decoder is correct: it rejects
non-alphabet bytes, `len % 4 == 1` and nonzero trailing bits, and every
`encode` output round-trips. `apply` panics on every malformed patch rather
than skipping it. The schema registry resolves the checked-package URN, so a
dangling `$ref` would fail the build. Trace tags are present and match the
TC-431 rows. Every positive vector is admitted and every adverse one refused
for its stated member, so the oracles are real today. The findings make them
hold on their own in future: FND-001 is a self-referential check, and FND-003
locks the refusal location. All are low. The test is mergeable once the
SR-1304 and SR-1305 findings are fixed, and these five belong in the same fix
round.

## Dispositions

Round 1, reviewed at 346fa49 (fix range 276a7e0..346fa49).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 346fa49 |
| FND-002 | fixed | 346fa49 |
| FND-003 | fixed | 346fa49 |
| FND-004 | fixed | 346fa49 |
| FND-005 | fixed | 346fa49 |
