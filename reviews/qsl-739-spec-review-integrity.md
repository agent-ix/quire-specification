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

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 763efef: M26 run 3 now keeps the cases (24 charges, one refusal at `TypeDefinition.kind`); new run 4 (shape and empty list, 22 charges) expects `TypeDefinition.kind` then `TypeDefinition.variants`, as QSL `an_empty_and_mis_shaped_variant_type_reports_both` |
| FND-002 | fixed | 763efef: the payload row reads "a `payloadType` that resolves", and the prose states that the reader refuses an unresolved payload before intake |
| FND-003 | fixed | 763efef: the FR-154 row, prose and AC-11 cover a field `typeRef`, an operation parameter and an operation result, and M29 tests them; how the new text names those refusals is FND-008 |
| FND-004 | fixed | 763efef: the prose names `TypeDefinition.kind` at the type node, and AC-11 lists the shape case |
| FND-005 | fixed | 763efef: an order statement replaces table order for the variant rows; M25 run 3 (identity then payload on one case node) matches QSL; the order across nodes is FND-009 |
| FND-006 | fixed | 763efef: variant-type moved to its own `## Variant types` section, which says it is not one of the six business model meanings |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-007 | high | M29 expects 26 charges, but the correct count is 25 (K2 21 + `Tier` 1 + 2 cases + operation `setTier` 1). A parameter and a result are not charged IR nodes. M13 (26 = 21 + clause + 3 fields + operation, parameter `amount` not counted) and M22 set the precedent. QSL `source_record_count` counts only types, populations, and their fields, operations, relationships, clauses and variants. | spec/test-cases/TC-235-business-construct-meaning-binding.md:108 | wrong-requirement |
| FND-008 | medium | The new text says a parameter or result naming a variant type is refused "naming the operation node and, in the diagnostic, the parameter or the result; it carries no member path", and M29 asserts "no member path". This describes QSL's current gap, not the target. QSL `signature_type_refusal` returns a bare `ModelRefusalCause::MalformedDeclaration` with no node at all (`refusal_node` returns None), so even "naming the operation node" is not typed. Both break FR-154's rule that every declaration refusal retains the IR node identity and origin, and an assertion over typed fields cannot tell parameter from result. Fix: state a typed node and member path, for example the parameter node `<operation identity>/<param>` or the operation node with a parameter path and a result path (AD-006 already uses `Operation.params` and `returns`), and open a QSL ticket to carry them typed. | spec/functional/type-model/FR-154-admit-domain-package-model.md:109-111 | wrong-requirement |
| FND-009 | medium | The new statement orders each case's failures as "identity, source span and payload", but the identity and payload refusals name the case node while the source-span refusal names the type node. FR-154 reports in node order, and QSL stable-sorts intake refusals by node (`refusal_node` in normalize `build`). So `ix://.../Tier` (span, conflict, shape, empty) reports before any `ix://.../variant/...` case-node refusal, whatever the case order. For a case with a wrong identity and no span, QSL reports the span first; the text says identity first. Fix: say the order applies within one node and node order still applies across nodes. M25 run 3 and M27 run 3 stay correct either way. | spec/functional/type-model/FR-154-admit-domain-package-model.md:117-120 | wrong-requirement |
| FND-010 | low | FR-208's new `## Variant types` section has two problems. Its second sentence is a fragment ("A type definition whose construct has shape `enumeration`, with ... (FR-154).") with stray two-space indentation left over from the bullet. It also does not carry the round-1 fixes: it still says any case with a `payloadType` is `declaration-form`, without "that resolves", and FR-208-AC-15 still names only a field `typeRef`, not a parameter or result. | spec/objects/foundation/FR-208-quire-meaning-vocabulary.md:318-330 | wrong-requirement |
| FND-011 | low | M27 run 3 appends a fourth case `Silver` with no source span but does not state its identity. The expected "two refusals" holds only if the identity is the minted `ix://test/orders/variant/Tier-Silver`; any other identity adds a `Variant.identity` refusal. State the identity. | spec/test-cases/TC-235-business-construct-meaning-binding.md:106 | wrong-requirement |
