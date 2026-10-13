---
id: SR-8698
title: "integrity review of quire-specification PR 242 (variant type rules)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@4861c728ee997b31b28a9bb94ef37785b471a4be; spec/functional/type-model/FR-154-admit-domain-package-model.md, spec/objects/foundation/FR-208-quire-meaning-vocabulary.md, spec/test-cases/TC-235-business-construct-meaning-binding.md"
review_set: subset
---

## Summary

Ticket: QSL-739. Integrity and consistency review of the PR 242 diff: FR-154's three new
declaration-refusal rows and variant prose, FR-154-AC-11, FR-208's Field types sentence,
`variant-type/v1` per-meaning bullet and FR-208-AC-15, and TC-235 vectors M24 to M28.
Each rule, cause, node and member path was compared with the source of truth, QSL FR-056
"Variant types", FR-056-AC-24..28 and TC-994..997 on quire-spec-language
`impl/business-model-meanings`, and with the QSL code that implements them
(`qsl-semantics/src/model/intake.rs` `read_variant_type`, `normalize.rs`,
`tests/it/variant_type.rs`) and the FCD reader (`semantic-ir/src/rules.rs`).

Most rules match QSL exactly: the no-case path, the spanless-case path
(`TypeDefinition.variants[i].origin` at the type node, as TC-996 step 2), the mis-minted
identity at the case node and `Variant.identity`, the payload at the case node and
`Variant.payloadType`, the field typeRef at the field node and `Field.typeRef`, and the
duplicate-name `conflicting-binding` at the type node and `TypeDefinition.variants[i]`
with the later case's other checks skipped. Charges match: QSL charges one
`normalize.record` per IR node and counts each `variants` entry as a node
(`source_record_count`), so M24 to M28 (24, 24, 22/24/22, 25/24, 25) are right. IDs are
unique across all remote branches and open PRs, the three-dot diff has no removals, and
`quire validate` reports no new error. Six findings remain, below.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | TC-235 M26 run 3 sets the `status` shape to `record` and also empties `Tier.variants`, but expects only the `TypeDefinition.kind` refusal. QSL reports both `TypeDefinition.kind` and `TypeDefinition.variants` for that input (test `an_empty_and_mis_shaped_variant_type_reports_both`), as FR-154's "every check of the table that fails reports" requires. Drop the empty list (TC-996 step 3 changes only the shape; then 24 charges), or expect both refusals in that order. | spec/test-cases/TC-235-business-construct-meaning-binding.md:105 | wrong-requirement |
| FND-002 | medium | The FR-154 payload row says a case with a `payloadType` refuses `declaration-form` "whether or not it resolves". QSL FR-056-AC-25 and TC-995 scope this to a resolving payload ("the reader must already resolve the payload type"), and the FCD reader refuses an unresolved one first with `UNRESOLVED_VARIANT_PAYLOAD` (semantic-ir `rules.rs:1218-1226`), so intake never sees it. Drop "whether or not it resolves", or state that an unresolved payload is the reader's refusal. | spec/functional/type-model/FR-154-admit-domain-package-model.md:86 | wrong-requirement |
| FND-003 | medium | FR-154's row, its prose bullet and FR-154-AC-11 refuse only a field `typeRef` that names a variant type. FR-208's Field types sentence covers a field or parameter, and QSL code also refuses an operation parameter and a result (test `an_operation_signature_naming_a_variant_type_refuses_as_malformed`). FR-154 names no node or path for those. Widen the FR-154 row and AC to field, parameter and result, with their paths, or narrow FR-208. | spec/functional/type-model/FR-154-admit-domain-package-model.md:87 | wrong-requirement |
| FND-004 | low | FR-154's variants prose sends a non-`enumeration` shape to the node-validity row but names no member path for it. FR-154-AC-11 also leaves the shape case out. M26 run 3 asserts `TypeDefinition.kind` (as QSL TC-996 does), which FR-154's text does not support. Add `TypeDefinition.kind` at the type node to the prose list, and add the shape case to AC-11. | spec/functional/type-model/FR-154-admit-domain-package-model.md:95 | missing-requirement |
| FND-005 | medium | FR-154 reports one node's failures in table order. The payload row (`declaration-form`) comes before the malformed-variant row, so a case with both a mis-minted identity and a payload reports `Variant.payloadType` first. QSL reports `Variant.identity` first, in member-path order (`read_variant_type` checks identity, origin, then payloadType). The same holds for a conflicting case at `variants[i]` (the later row) when a later case has a spanless `variants[j].origin`. State the in-node order for variant refusals, or reorder the rows to match QSL. | spec/functional/type-model/FR-154-admit-domain-package-model.md:86-89 | wrong-requirement |
| FND-006 | low | The `variant-type/v1` bullet is added to "Per meaning" under "Business model meanings", which says "The six business model meanings are..." and does not list variant-type. A blank line also splits it from the list. Move it out of that section, or update the count and list. | spec/objects/foundation/FR-208-quire-meaning-vocabulary.md:318 | wrong-requirement |

## Verdict

Changes requested: one high finding (M26 run 3 expectation), three medium findings and
two low findings. Clean units: the M24, M25, M27 and M28 expectations and charges, the
no-case, spanless-case, identity, payload-path and duplicate-name paths, the
reader-first duplicate-identity rule, the variant slot identity text (consistent with
FR-154's Node identities paragraph), FR-208-AC-15's causes, and the absence of any
effective-identity statement (QSL-740 owns it). Author question 1: stating the field
`typeRef` rule as the current rule is consistent with QSL FR-056, which states it as a
SHALL and calls it a placeholder only in prose. Leaving the QSL-738 note in the PR body
is fine, apart from FND-003's scope. Author question 2: the spanless-case path matches
TC-996 step 2 and the QSL code exactly.
