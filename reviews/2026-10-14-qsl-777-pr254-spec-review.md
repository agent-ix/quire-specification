---
id: SR-9124
title: "Spec review of quire-specification PR #254: protocol_reference occurrences"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@80cf2fb983ea4ec8108656d57c59d3b5689bcae5; PR #254 diff vs main: spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-931-contract-package-single-semantic-graph.md, spec/test-cases/TC-943-a-protocol-reference-parses-resolves-and-binds-by-key.md, spec/functional/protocol/FR-463-refer-to-a-declared-protocol-in-an-expression.md, proposals/checked-package-v2/README.md"
review_set: subset
---
# Spec review of quire-specification PR #254

## Summary

Ticket: QSL-777. One spec-review pass over 80cf2fb vs main (1 commit), with
the integrity and failure-domain checks folded into this one file. Paired
QSL text read at quire-spec-language main 9c53506b2: FR-092 (protocol
reference nodes, AC-16/17/18), FR-218, FR-308, TC-1021.

Examined:
- Identity claim. FR-322 `identity_preimage` excludes source regions, and
  `identity_projection` excludes source occurrences (Properties). FR-322-AC-14
  states that editing a source-map region or a node source occurrence leaves
  `package_id` unchanged. The structural-node preimage schema has no
  `occurrences` member. So "occurrences enter neither the key nor the package
  identity" is correct, and so is the worked example (`R.node_id.digest` and
  `package_id` stay the same) (clean).
- Refusal codes. `invalid_source_map` and `invalid_semantic_graph` are both in
  the V2 refusal vocabulary, as bare codes with no cause tag.
  `invalid_semantic_graph` for two nodes with one `node_id` matches
  FR-322-AC-10, which covers noncanonical or repeated graph order. Using
  `invalid_source_map` for a repeated source-map key fits FR-271's meaning
  (a non-monotone source map). FR-322 has no other refusal for a repeated
  source-map key, so nothing conflicts (clean; see the recorded note).
- Schema. Removing `maxItems: 1` is the only cap on the carrier's
  occurrences. Checked with jsonschema against positive-all-families plus
  carrier R: two `expression` occurrences pass; zero occurrences fail; a
  `declaration` role fails; a repeated ordinal passes the schema, so it is a
  reader refusal as V5 says (clean).
- "Exactly one" wording. Searched spec/ and proposals/ for "exactly one" or
  "one occurrence" together with protocol_reference. Only the worked example
  remains, and it is now qualified. FR-330 and FR-087 have no
  occurrence-count text (clean).
- TC binding. FR-322-AC-66 is verified by TC-931 (V1..V6) and TC-943. Each
  new clause of AC-66 has a vector (clean).
- quire validate on scratch copies. TC-931, TC-943 and FR-463 are clean. The
  two FR-322 errors (missing `id`, record-schema `fields`) reproduce on the
  origin/main copy, so they are pre-existing (clean).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The FR-322 carrier text says a carrier whose occurrences or source_map entries repeat a (node_id, role, ordinal) key "is refused as stated in FR-322-AC-66". AC-66 states the refusal only for two source_map entries with one key. A repeated ordinal inside the node's own `occurrences` array has no stated code or locus. TC-931 V5's first variant ("R carries two expression occurrences with ordinal 0") expects `invalid_source_map` "at the later source_map entry". That locus does not exist when the source_map holds one entry (R, expression, 0). As written, this case of the vector cannot be produced, and IR-785 has no rule to implement for it | spec/objects/interfaces/FR-322-checked-package-artifact.md:150-156, spec/objects/interfaces/FR-322-checked-package-artifact.md:1465, spec/test-cases/TC-931-contract-package-single-semantic-graph.md:99 |
| FND-002 | medium | QSL mismatch, to be carried by the QSL follow-up. QSL FR-092's protocol reference node rule still says "occurrences: exactly one, of role expression". QSL FR-218's rationale also says the IR reader admits exactly one occurrence. So the QSL follow-up has more to do than drop the FR-218 repeated-reference refusal. It must also amend the FR-092 bullet and add a two-reference emission criterion (one node, two occurrences) to FR-092 with a TC-1021 step. Nothing needs to change in this PR | quire-spec-language spec/functional/FR-092-key-type-parameter-and-declared-nodes.md:418, quire-spec-language spec/functional/FR-218-check-every-protocol-control-construct-at-s3.md:146-154 |
| FND-003 | medium | TC-931 V2 (the two occurrences and source_map entries in reverse order, admitted) depends on an ordering rule that FR-322 does not state. The worked example says to include R in the source map "in canonical order". FR-322 defines no canonical order for one node's occurrences or for source_map entries of one node, and no refusal for a different order. The expected result is plausible but not backed by the FR text. Recorded | spec/test-cases/TC-931-contract-package-single-semantic-graph.md:96, spec/objects/interfaces/FR-322-checked-package-artifact.md:182-183 |
| FND-004 | medium | TC-931 V6's first variant ("R twice, each with one occurrence") leaves open whether the source_map holds one entry or two (R, expression, 0). With two entries the package also fails V5's rule, and FR-322 does not order the source-map check against the graph check, so the expected `invalid_semantic_graph` is not certain. State one source_map entry. Recorded | spec/test-cases/TC-931-contract-package-single-semantic-graph.md:100 |
| FND-005 | low | The nearest earlier source-map refusal in FR-322 (AC-52, a moved declaration region) uses `invalid_package`/`invalid-value` at the source_map entry. The new rule uses bare `invalid_source_map`. Both are valid codes for different defects. Recorded | spec/objects/interfaces/FR-322-checked-package-artifact.md:1451 |

## Verdict

**FAIL, 1 blocking finding (FND-001).** The amendment is correct otherwise.
Identity is unaffected by occurrences, as AC-14 and the preimage
definitions show. The two refusal codes exist and match how FR-322 handles
the same classes of defect. The schema change is the only cap and behaves as
V1, V3, V4 and V5 say. The FR-322 validation errors are pre-existing.

Fix for FND-001: state the refusal for a repeated (`expression`, ordinal)
inside the carrier's `occurrences` array in AC-66, giving its code and
locus. A pointer at the later occurrence matches V5's rule. Then point V5's
first variant at that locus, or drop that variant.

FND-002 is not a defect in this PR. It is the scope of the QSL follow-up.
FND-003, FND-004 and FND-005 are recorded and do not block.
