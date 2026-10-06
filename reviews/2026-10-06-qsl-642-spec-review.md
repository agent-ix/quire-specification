---
id: SR-1303
title: "Spec review of quire-specification PR #192: node-key preimage integers keep one exact spelling (QSL-642)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@87f1173c37baee4a0d6ccdf4e4ab7939dfa6db7c; PR #192 diff against origin/main: spec/objects/interfaces/FR-322, spec/test-cases/TC-430, spec/complete-v1/tests.md, proposals/checked-package-v2/{README.md,node-identity-preimage.schema.json,node-identity-vectors.json,schema.json}"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-430
    type: reviews
---
# Spec review of quire-specification PR #192

## Summary

Ticket: QSL-642. Base spec review with the integrity and failure-domain lenses, scoped to the diff. Judged against the decided option (a): integer values are `IntegerString` at every magnitude; counters are a JCS number up to 2^53-1 and a canonical decimal string beyond; no magnitude refusal; no published key changes.

- FR-322's new "Integers in a node-key preimage" statement matches option (a) and QSL #654's FR-092 word for word in substance.
- All three `integer_encoding_vectors` digests were recomputed with python hashlib over the RFC 8785 bytes (sorted keys, no whitespace): `bc6fe35a...`, `89ec1ac8...` and `f9ea36c5...` all reproduce. `integer-range-u64-maximum` equals QSL FR-092 T13. The `integer-add` `operation_vectors` entry still reproduces, so no published key changes.
- `BeyondSafeIntegerString` was checked with python `re` (all patterns anchored), for every value within ±3000 of 2^53, every digit-prefix boundary of 9007199254740991, and 200,000 random 16-digit values. It accepts exactly the strings for values above 2^53-1. It refuses 2^53-1, `0`-prefixed, `+` and `-` forms, and any string under 16 digits. `jsonschema` validates both recursion vectors as `ApplicationNode`, and refuses `size` 9007199254740992 written as a number, 9007199254740991 written as a string, and a leading-zero ordinal (TC-430 NI-03). `schema.json` parses, and its copy of `BeyondSafeIntegerString` is identical to the preimage schema's.
- The digests are a canonical identity digest that binds a key to its preimage content, which is the allowed exception. That is not ceremony.
- Spec validation (`quire validate --scope . 'spec/**/*.md'`) gives output identical to origin/main (145 pre-existing failing documents). The only difference is one existing warning that moved line number.

Examined:
- FR-322 "Integers in a node-key preimage" statement (examined)
- FR-322-AC-56 (examined)
- TC-430 rows NI-01 to NI-05 (examined)
- node-identity-preimage.schema.json `KeyOrdinal`, `KeySize`, `BeyondSafeIntegerString`, `Recursion`, `OperationMember`, `group_reference` (examined)
- schema.json `KeyOrdinal`, `OperationMember` (examined)
- node-identity-vectors.json `integer_encoding_vectors` (examined)
- README line, complete-v1/tests.md rows (examined)
- open PR #191's FR-322 rows (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-322-AC-56 has the same id as an AC in open PR #191 (QSL-638 owner reader, head bc04ca7). That PR adds FR-322-AC-56 (relationship_end resolution) and AC-57. Whichever merges second gets a duplicate AC id or a hand-resolved conflict. Renumber this AC to the next id free across both PRs (FR-322-AC-58 if #191 lands first), and update TC-430 and the complete-v1 matrix row. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1050 |
| FND-002 | low | The statement says a counter in the wrong form ("a counter at most 2^53-1 written as a string, and a counter beyond it written as a number") is refused by the schema, "and a node keyed from one is not current". "Not current" is not a defined reader outcome. AC-56 checks only the schema. Name the reader's refusal (for example `invalid_package`/`stale-node-key`, as AC-53 uses), or drop the clause. | spec/objects/interfaces/FR-322-checked-package-artifact.md:989-992 |

## Verdict

Mergeable after FND-001 is fixed (a renumber). Fix FND-002 in the same round. The rule matches option (a), the vectors reproduce, the schema pattern is exact, the two schemas agree, no published key changes, and validation matches main.
