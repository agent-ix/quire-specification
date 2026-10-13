---
id: SR-7874
title: "Base spec review of quire-specification PR #239: string, uuid and timestamp equality operations and literal kinds"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@a76cd32d81bd38dc41025635d6c81ca62c44921c; PR #239 diff against main baa25d9: spec/objects/interfaces/FR-322-checked-package-artifact.md (Operation families, groups and constraints; FR-322-AC-67), spec/test-cases/TC-233 (SE-01..SE-10), spec/complete-v1/tests.md, proposals/checked-package-v2/schema.json, proposals/checked-package-v2/node-identity-preimage.schema.json; cross-checked against FR-149, QSL FR-056, tests/checked_package_v2.rs and quire-verification-contracts#16"
review_set: subset
---
# Base spec review of quire-specification PR #239

## Summary

Ticket: QSL-735 (AC4, AC5). The PR adds six equality identities for the
`string`, `uuid` and `timestamp` families to both V2 schemas and to FR-322,
adds the three families to the literal `value_kind` rule and both schemas'
`value_kind` enums, and adds FR-322-AC-67 and TC-233 vectors SE-01..SE-10.

What holds: the `OperationIdentity` enum is identical in both schemas (160
unique members), the six new identities sit after `quire.op.text.size` in
both, and the four `value_kind` enums (`LeafTerm`, `SemanticTerm`,
`PreimageLeafTerm`, `PreimageTerm`) all match the FR-322 text order. The
operator shape (binary, two same-family operands, `boolean` result, `laws`
`[]`, `mode`/`member`/`leaves` null, no constraints) matches
`quire.op.boolean.eq`/`ne`, and the merged catalog rows in
quire-verification-contracts#16 match it field for field. Adding enum values
only widens the schemas: no existing preimage or golden changes, and no
schema file is hashed into any digest. FR-322-AC-67 and SE-01..SE-10 collide
with no id in the repo or in the 8 other open PRs. The paragraph's
"reflexive, symmetric and transitive" is prose about the equality relation,
not a `laws` entry, so `laws: []` is consistent with `boolean.eq`.

## Verdict

Changes requested. Two high findings: AC-67 names the wrong refusal for a
stray law or mode, and the repo's TC-233 oracle still refuses the three new
literal kinds while no test backs AC-67. Three medium findings: the String
semantics cite an FR-149 row that does not exist, the literal `value`
spelling is unspecified for identity-bearing values, and several AC-67
clauses have no vector.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | FR-322-AC-67 says a `text_profile` law or mode on these identities refuses `operator-ineligible`. FR-322's own reader order refuses an extra law as `invalid_package`/`operation-law-mismatch` and a mode where the entry names no mode kind as `invalid_package`/`operation-mode-mismatch`, both before any `ill_typed`/`operator-ineligible` check. The AC contradicts the requirement it belongs to. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1305, 266-288 | wrong-requirement |
| FND-002 | high | The repo's TC-233 oracle was not updated: its literal admission table (`descriptor`, `value_kind` match) lists no `string`, `uuid` or `timestamp` arm and returns `None`, so SE-09's literals are refused against FR-322's new rule. FR-322-AC-67 has no `#[trace("FR-322-AC-67", "TC-233")]` test, although AC-62 under the same TC-233 has two. | tests/checked_package_v2.rs:1061-1071; spec/complete-v1/tests.md:1014 | correct-requirement-no-evidence |
| FND-003 | medium | The new paragraph grounds `string` equality in "FR-149's row for that native (FR-149-AC-14)" and its lack of ordering in FR-149-AC-16. FR-149 has no String row, and AC-14/AC-16 cover only UUID and Timestamp. String equality (exact Unicode scalar sequence, no normalization) is defined in QSL FR-056 "Checked native String and UUID leaves" (FR-056-AC-23). The citation should point there, or FR-149 should gain a String row. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1120-1131; spec/functional/type-model/FR-149-apply-complete-equality-matrix.md | wrong-requirement |
| FND-004 | medium | No spelling is given for a literal `value` of kind `string`, `uuid` or `timestamp`. The value enters the `quire.application-node/v1` preimage, so `00112233-...-AABBCCDDEEFF` and its lowercase form would key the same constant to two node ids. FR-322 fixes a spelling only for integers (IntegerString). State that a uuid or timestamp literal `value` is its QSL FR-056 K2 canonical payload string (UUID lowercase 8-4-4-4-12) and that a noncanonical spelling refuses. SE-09 names no value either. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1114-1118, 1210-1220; TC-233 SE-09 | missing-requirement |
| FND-005 | medium | Several AC-67 clauses have no TC-233 vector: `uuid` against `string` operands, a `text_profile` law, a `text_profile` mode, an arithmetic identity over these families, and a literal kind mismatch other than `uuid`. SE-07, SE-08 and SE-10 cover one case each. | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:254-267 | correct-requirement-no-evidence |
