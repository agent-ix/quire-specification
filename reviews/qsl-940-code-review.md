---
id: SR-13699
title: 'Code review of QSpec PR #323: inline integer literal operands in checked-package-v2 fixtures (QSL-940), code-review with rust-review lane'
type: SpecReview
analysis: code-review
scope: 'agent-ix/quire-specification@de9af077e; git diff origin/main...de9af077e (PR #323, branch qspec-inline-literals, merge base fc98b5e): proposals/checked-package-v2/fixtures/positive-two-owners-a.json, proposals/checked-package-v2/fixtures/positive-two-owners-b.json, proposals/checked-package-v2/fixtures/positive-operation-identities.json, proposals/checked-package-v2/node-identity-vectors.json, tests/checked_package_v2.rs; ticket QSL-940'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
- target: ix://agent-ix/quire-specification/TC-233
  type: reviews
---

## Summary

Ticket: QSL-940. PR: quire-specification#323, head de9af077e (the branch tip; GitHub's `refs/pull/323/head` and the PR's `headRefOid` still read a313cd8 at review time, the commit before AC-92). Method: code-review with the rust-review lane folded in. The PR rewrites the integer operator applications of three positive packages and two operation vectors so their integer literal operands are inline `literal` terms, removes the two literal nodes of `positive-two-owners-a/b.json` with their projection and `source_map` entries, rekeys the changed nodes and package ids, and adds `tc_233_integer_operator_literal_operands_are_inline_in_the_published_packages`.

Evidence, all from an independent Python recomputation (RFC 8785 for these ASCII-key, float-free values is `json.dumps(sort_keys=True, separators=(',', ':'))`, then SHA-256), not the suite's `rekey_stale_nodes`:

- The script reproduces QSL FR-092 E12 (`101e991c...`) and E13 (`9b59de5f...`) at QSL PR #1161 head ca37b7138, and E12's inline operand is byte-identical to the operand shape written here.
- Every application-bearing node in all nine positive fixtures keys to its `quire.application-node/v1` preimage: `three` `326cccf7...` in both two-owner packages, `integer.mul` `4dbceabb...` and `integer.add` `ddf357d4...` in the operation package. On origin/main the same script gives the old keys `c1c2df65...`, `2f6193a7...` and `8d709246...`.
- Every application node's `dependencies` equals its unique ascending reference targets plus member declarations and populations; both rewritten `integer.add`/`integer.mul` nodes and `three` have `[]`.
- Each `identity_projection` equals the graph nodes without `occurrences`, and each `package_id` equals SHA-256 of the JCS `identity_preimage` (`c9e77cda...`, `89ff2500...`, `1ccf2e8d...`).
- Every node occurrence has exactly one `source_map` entry and no entry names an absent node; no remaining ordinal referred to the removed literal nodes.
- Every `operation_vectors` and `integer_encoding_vectors` digest is current; every `operation_mutations` `retained_sha256` equals its base vector and every `rekeyed_as` equals the patched base (`integer-add-to-mul` lands on `4dbceabb...`). The package nodes equal the `integer-add`/`integer-mul` vectors.
- No integer `add`/`sub`/`mul`/`negate`/`eq`/`ne` application in any published JSON references a `value`/`literal` node; `positive-union-nodes.json`'s `integer.mul` operands are parameters. No old key or old package id survives anywhere in the repository.
- `cargo test --test checked_package_v2 --test backend_provider_v1`: 116 and 14 passed.
- Mutations (new test / full gate): restore the reference form in `-a` (fails / 2 fail), `-b` (fails / 2 fail), the operation package (fails / 5 fail); restore the old vectors file (passes / 4 fail); change inline `1` to `5` without rekey (passes / 3 fail); a non-Integer inline `type` (fails); drop `add` or `mul` from `OPERATORS` (fails); drop `sub`, `negate`, `eq` or `ne` (passes / all pass); delete the retained literal node `7` (passes / 15 fail).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| `three` application, both packages | proposals/checked-package-v2/fixtures/positive-two-owners-a.json, positive-two-owners-b.json | examined |
| `integer.mul`, `integer.add` applications and retained literal nodes | proposals/checked-package-v2/fixtures/positive-operation-identities.json | examined |
| `integer-add`, `integer-mul` vectors and their mutations | proposals/checked-package-v2/node-identity-vectors.json:110-111,130-149 | examined |
| `tc_233_integer_operator_literal_operands_are_inline_in_the_published_packages` | tests/checked_package_v2.rs:14615-14666 | examined |
| FR-322-AC-92 | spec/objects/interfaces/FR-322-checked-package-artifact.md:1840 | examined |
| `application_node_preimage`, `is_application_node`, `application_dependencies`, `positive_fixtures` | tests/checked_package_v2.rs:687-720,2224-2248,12342 | context_only |
| QSL FR-092 E12/E13, FR-093 Literal row | quire-spec-language@ca37b7138 spec/functional/FR-092-key-type-parameter-and-declared-nodes.md:882-896, FR-093-lower-checked-value-expressions-to-fr-322-terms.md:139 | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-322-AC-92 and the test name six operators, but the published packages hold literal operands only under `integer.add` and `integer.mul`. Dropping `sub`, `negate`, `eq` or `ne` from `OPERATORS` passes the whole gate, so the `eq`/`ne` rule (QSL FR-358 step 6) and the `sub` shape of QSL E12 have no published vector here. | tests/checked_package_v2.rs:14621-14628; spec/objects/interfaces/FR-322-checked-package-artifact.md:1840 |
| FND-002 | low | AC-92's clause that the literal nodes for `7` and `-2` remain in `positive-operation-identities.json` is not asserted by the bound test. Deleting the `7` node passes the new test; only unrelated tests fail on the dangling `div`/`rem` references. | tests/checked_package_v2.rs:14663-14665; spec/objects/interfaces/FR-322-checked-package-artifact.md:1840 |
| FND-003 | low | The PR body's first section still says no spec file is touched and that the FR-322 statement is "Not added here", which contradicts the head commit de9af077e and the body's own later "Normative owner" section. GitHub's PR head also still reads a313cd8, the commit without AC-92. | quire-specification#323 description |
| FND-004 | low | The `integer-add-missing-operand` mutation still writes its single operand as a `reference` to the integer literal node `9f42fd25...`, a form FR-322 now forbids for an `integer.add` operand, so the vector differs from its base in operand form as well as arity. An inline `7` would isolate the arity refusal. | proposals/checked-package-v2/node-identity-vectors.json:149 |

## Verdict

Not clean, but every changed identity is correct. An independent recomputation confirms every rekeyed node key, its preimage, its `dependencies`, each `identity_projection` and each `package_id`, and the same algorithm reproduces QSL #1161's E12 and E13. The inline operand shape is QSL E12's. The removed literal nodes leave no dangling reference, projection entry or `source_map` entry, and the retained `7`/`-2` nodes are still referenced by `div`/`rem`. The test can fail. Restoring the reference form in any of the three packages, a non-Integer inline type, or dropping `add` or `mul` each fails it. The rest of the gate catches a value change without a rekey. FND-001 is the one gap worth closing: four of the six operators have no published vector. FND-002 to FND-004 are small.

## New findings (disposition pass 1)

Head 406c078b5. Evidence for this round:

- The independent script recomputes all six `inline_literal_cases` digests. It rebuilds each `reference_form_sha256` from the same preimage with references to the `7` (`9f42fd25...`) and `-2` (`f4fe653e...`) literal nodes. Each case's `operation` equals its preimage's operation identity. The `eq`/`ne` cases are typed at the Boolean node `99643906...`, with operator class `binary` as the `text.eq` vectors use.
- The fixture recheck over all nine positive packages is unchanged and clean.
- Mutations, new test / full gate:
  - Remove `sub`, `negate`, `eq` or `ne` from the FR-322 paragraph: each fails / 1 fails.
  - Delete the `inline-sub`, `-negate`, `-eq` or `-ne` case: each fails / 1 fails.
  - Delete the `7` literal node: fails.
  - Reference an `integer_range` literal node from `integer.add`: fails at the reference check.
  - Three mutants survive (FND-006): relabel `inline-sub`'s preimage as `integer.add` and rekey it; set `reference_form_sha256` to `00..00`; type an inline operand at Boolean and rekey it.
- Every reader of `node-identity-vectors.json` in this suite and in QSL indexes a named section. QSL's `deny_unknown_fields` applies only to the preimage structs, so the new top-level `inline_literal_cases` key breaks no consumer.
- QSL conformance at ca37b7138 with `QSPEC_DIR` at this head fails two tests that pass against QSpec origin/main (FND-005). The control run against main fails only the two known tests, c14 and i2.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | Landing this PR breaks two QSL conformance tests at QSL #1161 head ca37b7138 that pass against QSpec main. `conformance_fr322_application_keys_match_qspec_operation_vectors` fails with "integer-add: preimage does not decode: data did not match any variant of untagged enum BodyTerm". `conformance_emitted_application_nodes_match_qspec_positive_fixtures` fails because QSL's `integer.mul` golden case emits reference operands where the fixture now has literals. Both already fail at de9af077e. QSL needs a matching change, or a landing order that keeps `make conformance` green. | proposals/checked-package-v2/node-identity-vectors.json:110-111; proposals/checked-package-v2/fixtures/positive-operation-identities.json; quire-spec-language qsl-semantics/src/check/node_key/tests.rs:833; qsl-package/src/emit/tests/golden.rs:402 |
| FND-006 | low | The `inline_literal_cases` checks do not tie a case to its operator or to its reference form. The test never compares `operation` with the preimage's `body.operation.identity`, so six relabelled copies of one application would pass. It checks `reference_form_sha256` only for inequality and never recomputes it. It checks inline operands for `term: literal` but not for `value_kind` or type. One surviving mutant each. | tests/checked_package_v2.rs:14694-14718 |
| FND-007 | low | The PR body no longer says that the FR-322 rule and its reader-admission sentence are lead defaults. The "Normative owner" section was dropped and nothing replaced it. The body also does not describe `inline_literal_cases` or the operator list the test reads from FR-322. | quire-specification#323 description |

## New findings (disposition pass 2)

Head 1eba298ac. No new code-review finding. Evidence for this round:

- The three FND-006 mutants now fail the gate: relabelling `inline-sub`'s preimage as `integer.add` with a rekey, an arbitrary `reference_form_sha256`, and an inline operand typed at Boolean with a rekey. Each fails 1 test.
- AC-93 mutants fail too. Replacing `positive-integer-literal-references.json` with the inline two-owner package fails 1 test, and so does giving its `three` node a stale key.
- The new fixture is byte-identical to origin/main's `positive-two-owners-a.json`. The independent script finds its keys, projection, `package_id` (`c9821ebe...`) and `source_map` current.
- `cargo test --test checked_package_v2 --test backend_provider_v1`: 117 and 14 passed.
- QSL #1161 head 63543b3f5 conformance, with `QSPEC_DIR` at this head and again at this head without the new fixture: the same four failures both times. These are FND-005's `conformance_fr322_application_keys_match_qspec_operation_vectors` and `conformance_emitted_application_nodes_match_qspec_positive_fixtures`, plus the known `c14` and `i2`. `c14` and `i2` read the positives in name order and verify `positive-integer-literal-references.json` before they fail at `positive-operation-identities.json`. Run over the new fixture alone, `c14` passes, and `i2`'s loop verifies it. The golden test reads a fixed file list. The new fixture breaks nothing in QSL.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |
| FND-005 | still-open | Routed to the QSL #1161 coder. At QSL 63543b3f5 both QSL conformance tests still fail against this head; at ca37b7138 they passed against QSpec main. |
| FND-006 | fixed | - |
| FND-007 | fixed | - |
