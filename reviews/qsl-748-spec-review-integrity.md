---
id: SR-9002
title: "integrity review of quire-specification PR 246 (variant types as field, parameter and result types)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@852c515d143bb2fbdce5ed186cd68e686493dfb6; spec/objects/foundation/FR-208-quire-meaning-vocabulary.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/functional/type-model/FR-141-evaluate-text-and-enumerations.md, spec/functional/type-model/FR-150-preserve-model-normalization-provenance.md, spec/functional/type-model/FR-154-admit-domain-package-model.md, spec/test-cases/TC-195-model-normalization-provenance.md, proposals/checked-package-v2/schema.json, proposals/checked-package-v2/node-identity-preimage.schema.json, proposals/quire-v1/definitions/model-complete.md"
review_set: subset
---

## Summary

Ticket: QSL-748. Integrity, consistency and cross-reference review of PR 246 against
QSL FR-360 (quire-spec-language `impl/business-model-meanings`, items 1 to 12 and
cross-repo requirements 1 to 3) and against the rest of QSpec. The whole repo was grepped
for every remaining statement that a variant `typeRef` or a clause naming a variant type
is refused. The checked-operation catalog in quire-verification-contracts `main` was read
for `quire.op.structural.eq` and `quire.op.enum.eq`. The FR-208, FR-154, FR-150 and
FR-322 amendments carry QSL's admission, supersession, naming and refusal rules
faithfully, with QSL's codes and causes. The checked-IR side has three contradictions,
one stale test row elsewhere in the repo, and two identity gaps.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | FR-322-AC-71 (and TC-940 step 2) admits `quire.op.structural.eq` over two `Tier` operands. FR-322 gives a `model`/`variant_type` node family `enum`, and `structural_kind` (FR-322 group table; catalog `quire.op.structural.eq` operands `structural_kind`) does not contain `enum`, so the application fails the fit rule with `ill_typed`/`operator-ineligible`. Enum equality is `quire.op.enum.eq`/`quire.op.enum.ne` (`enum_kind`), the "enumeration equality application" QSL FR-360 item 12 lowers `==` to. Use `quire.op.enum.eq`/`ne` in AC-71 and TC-940. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1450; spec/test-cases/TC-940-variant-type-node-key-is-alias-independent.md:31 | wrong-requirement |
| FND-002 | high | The V2 schemas reject the node FR-322 now defines. `schema.json` `OwnerPresenceRule` lists the `model` forms `object_type`, `record_value_type`, `event_type` and `systems_interface` only, so a `model`/`variant_type` node with a `ModelOwner` fails with "not required owner", and an owner-less one passes. `node-identity-preimage.schema.json` `ModelDeclarationNode.semantic_form` also lacks `variant_type`. FR-322 calls both schemas normative, and AC-71 asserts the reverse of what they do. Following the FR-322-AC-68 precedent for `event_type`, this PR should add `variant_type` to both enums, add AC-68's "both V2 schemas enumerate the form..." clause to AC-71, and add a positive node vector and an owner-less adverse vector. A follow-up ticket is the wrong place for this: the requirement is false until the schema changes. | proposals/checked-package-v2/schema.json:83; proposals/checked-package-v2/node-identity-preimage.schema.json:90; spec/objects/interfaces/FR-322-checked-package-artifact.md:497,678,1450 | wrong-requirement |
| FND-003 | high | TC-195 N04 run 2 still expects `M::V1` in a record field of S1 to be refused `unsupported_construct`/`declaration-form` at check. TC-195 verifies FR-150, which this PR changes to "a model `variant` type referenced by source is admitted (FR-208)". The PR leaves this contradiction in a test it did not touch. Change N04 run 2 to admission. | spec/test-cases/TC-195-model-normalization-provenance.md:57 | wrong-requirement |
| FND-004 | medium | FR-322 states no reader rule for an `enum_value` whose `semantic_type` is a `model`/`variant_type` node. For a `scalar_type`/`enum` node, the reader checks the literal's case against the declaration preimage's `members` (FR-322 `nominal_identity_preimage` join; FR-141). A variant node's `ModelDeclarationNode` preimage names no case, so nothing requires `case` to be a declared case of the selected document's variant type, and nothing names the refusal. Add the join (the case must be a case of the variant type that the owner's selection declares) with its code and cause. | spec/objects/interfaces/FR-322-checked-package-artifact.md:524-534; spec/functional/type-model/FR-141-evaluate-text-and-enumerations.md:86-95 | missing-requirement |
| FND-005 | medium | A case whose name is not a source identifier has no checked identity or key order. FR-208 admits such a case (`Gold Plus`, `2x`), and QSL FR-360 item 11 needs a checked member key for every case. The FR-141 member preimage reuses `EnumMemberPreimage`, whose `case` is `Identifier` (`^[A-Za-z_][A-Za-z0-9_]*$`). FR-144's unordered-enum key is "the case identifier's ASCII bytes", while QSL item 7 cites it as the ascending UTF-8 byte order of the case names. State the member key and the set key for any case name, or restrict variant case names. | spec/functional/type-model/FR-141-evaluate-text-and-enumerations.md:90-95; proposals/checked-package-v2/schema.json:94; spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md:117; spec/objects/foundation/FR-208-quire-meaning-vocabulary.md:363-365 | missing-requirement |
| FND-006 | medium | QSpec already has a model-name resolution rule. model-complete.md and shared-grammar.md say a clause names a declaration by the model alias "then the segments of its IR node identity after the package identity". A case's IR identity is `variant/Tier-Gold`, so that rule gives `M::variant::Tier-Gold`, not `Alias::Tier::Gold`. FR-208's new rule is an unstated exception to it. Add the case-literal form and its refusals, as an exception, to model-complete.md's naming rule (shared-grammar.md:264 points there), and keep FR-208 citing it. This also answers where the resolution rule belongs: QSpec has no FR-317, and model-complete.md is the closest owner. | proposals/quire-v1/definitions/model-complete.md:51-55; proposals/quire-v1/shared-grammar.md:263-268; spec/objects/foundation/FR-208-quire-meaning-vocabulary.md:345-365 | wrong-requirement |
| FND-007 | low | The paragraph after FR-141's new one says that retaining a key while changing "declaration membership" is `invalid_semantic_graph`, and that recomputing gives a new declaration identity. Read onto a variant type, this contradicts TC-940 step 4 and the `ModelDeclarationNode` preimage, under which renaming a case leaves the `Tier` key unchanged. Scope the paragraph to `scalar_type`/`enum` nodes, or state the variant exception. | spec/functional/type-model/FR-141-evaluate-text-and-enumerations.md:97-102 | wrong-requirement |
| FND-008 | low | The PR deletes the only QSpec text that named the member paths `Operation.params[i].typeRef` and `Operation.returns.typeRef`. QSL FR-360 item 8 keeps those paths for the meanings that are still refused (TC-1009 step 4), and QSpec now names no path for a parameter or result that names an event type, state machine, process, persistence interface or namespace. Keep the path bullet, scoped to the refused meanings. | spec/functional/type-model/FR-154-admit-domain-package-model.md:105-110 | missing-requirement |

## Verdict

Changes requested: three high, three medium and two low findings. The following units are
clean and match QSL FR-360:

- the FR-208 meaning row, the Field types bullet and the admit paragraph, including the
  event occurrence-field carve-out, admission under a refused variant declaration, and
  "for no other meaning";
- the unordered rule and the three naming refusals, with QSL's codes and causes:
  `missing_declaration`/`missing-name`, `ill_typed`/`type-mismatch` and
  `ill_typed`/`operator-ineligible`;
- FR-208-AC-15, AC-16 and AC-17;
- FR-322's owner-bearing list, owner table row, step 2 row, step 4 row and family `enum`
  (never `ordered_enum`);
- the FR-141 declaration identity and member identity over the node key;
- the FR-154 row and AC-11, and the FR-150 sentence.

Judgement on FR-154 and FR-150: both amendments are needed and correct, since each
restated the lifted refusal. FR-208 drops nothing from cross-repo requirements 1 to 3. The
only rule beyond them is the unordered and equality restatement, and FR-141 already holds
it. Leaving the unwritable case name to QSL is acceptable: the shared grammar's
`member-name` is an identifier, so such a case does not parse. Its identity gap is
FND-005.
