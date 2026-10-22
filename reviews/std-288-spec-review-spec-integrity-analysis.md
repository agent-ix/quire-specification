---
id: SR-15841
title: "spec-integrity analysis of STD-288 (QSpec PR #371)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@59998eb9349b6680d3d8d499276d7042527b53a5; spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, spec/test-cases/TC-217-complete-contract-ir-lowering.md, proposals/checked-package-v2/README.md, proposals/quire-v1/definitions/native-diagnostics.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-233
    type: reviews
---

## Summary

Ticket: STD-288. The scope is `git diff origin/main...59998eb`. The review checked the FR-322 `identity_preimage`, `package_id` and `semantic_graph` properties, the protocol-carrier paragraph, the owner paragraphs, "Owner member schema", AC-14, AC-48, AC-49, AC-51, AC-53, AC-54, AC-95 and AC-96, TC-233's procedure and GO-04, TC-217's cross-field sentence, the README and native-diagnostics.

The derivation rule is deterministic. Entry `i` is node `i` with `occurrences` removed and every other member unchanged. Under JCS, member order does not matter. Because `NodeBase` is closed, "every other member" is exactly the old `NodeProjection` member set, so a nominal node's `body` stays in `package_id`. All 12 positive fixtures keep their `package_id`, recomputed independently.

The remaining `identity_projection` mentions in FR-322, FR-451, FR-464, FR-375, TC-931 and TC-936 describe the projection as a member of the package-id preimage, and they still hold under the derived rule.

The AC rewrites keep their intent:

- AC-48, AC-49, AC-53, AC-54 and AC-96 lose only the "projection patched" step, which can no longer happen.
- AC-51 loses the projection-owner refusal. Under a derived projection that refusal is structurally impossible, and its schema-shape refusals stay.
- AC-95's negative moves from "projection out of graph order" to "nodes reversed, recorded id kept". Both test that order binds the id.

This matches IR-275's direction: Fix 5 says "derive the projection at read time", and AC-5 says existing fixtures keep their `package_id`.

`quire validate --scope . 'spec/**/*.md'` reports 86 failure lines, the required baseline.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 identity_preimage / package_id properties | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322 semantic_graph graph-order paragraph | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322 protocol_reference carrier paragraph | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322 Owner member schema | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-14 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-48 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-49 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-51 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-53 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-54 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-95 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-96 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| TC-233 identity procedure and GO-04 | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md | examined |
| TC-217 cross-field sentence | spec/test-cases/TC-217-complete-contract-ir-lowering.md | examined |
| package key paragraph | proposals/checked-package-v2/README.md | examined |
| refusal order preamble | proposals/quire-v1/definitions/native-diagnostics.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The protocol-carrier paragraph now reads "Include `R` in the graph and, at the same position in the identity projection the reader derives, and in the source map (...), then recompute `package_id`". The sentence is garbled. It also tells the author to place `R` into a structure that nobody ships and that the reader derives. Say "Include `R` in the graph, which puts it at the same position in the derived identity projection, and in the source map (...)". | spec/objects/interfaces/FR-322-checked-package-artifact.md:263-266 |
| FND-002 | low | FR-322-AC-14 now holds two independently failing behaviours in one row. One is the recomputation of `identity_preimage` and `package_id`, with the excluded and named-member edits. The other is the `shipped_projection_cases` schema and `malformed_wire` refusal. A failure of the second cannot be traced apart from the first. An appended AC for the shipped-projection refusal would keep AC-14 atomic and follow the append-only AC convention. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1933 |

## Verdict

Two low findings. The contract change is consistent, deterministic and identity-preserving.
