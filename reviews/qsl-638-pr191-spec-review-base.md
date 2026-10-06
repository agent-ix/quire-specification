---
id: SR-1302
title: "Spec review of PR #191 (QSL-638): owner wire spec edits"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@b6292c68792ca76b68f122c9e11a8b49f356509e; PR #191 spec edits against origin/main 60630b0: FR-322 member-identity sentence, FR-322-AC-29, FR-322-AC-54, TC-233 abstraction step, TC-280 MW-06; with FR-322 owner join, Owner member schema and FR-440 catalog entry as context; cross-checked against quire-contract-ir 8eb9837b owner.rs"
review_set: subset
---
# Spec review of PR #191 (QSL-638)

## Summary

Ticket: QSL-638. This review covers the spec edits that came with the code: the new FR-322 sentence on model-owned member identity, the AC-29 and MW-06 change (the owner join now preempts the stale key), and the AC-54/TC-233 change from a `rust_type` edit to a `frames` `function` edit. The FR-322 owner-join text and "Owner member schema" were read as the rules the reader implements. The FR-440 `union_arms`/`arm_body` text was read as the contract for the reader's union arms work.

Examined:
- FR-322 lines 197-200, "A model-owned member (a node carrying a `ModelOwner`) resolves by the member's identity in the selected document, never by a name in the node's body, which is empty; a `relationship_end` member of a model declaration node resolves the same way." (examined)
- FR-322-AC-29, revised parenthetical on the other-tag key (examined)
- FR-322-AC-54 and TC-233 abstraction step, `function` segment of the `frames` entry (examined)
- TC-280 MW-06, owner-join outcome equal to MW-07 (examined)
- FR-322 derived-shape key stage owner join, `ModelOwner` on a clause function (examined)
- FR-322 "Owner member schema", the projection-owner sentence (examined)
- FR-440 catalog entry, `arm_body` and `union_arms` (context_only)
- FR-341 clause kinds (context_only)
- FR-451 Reader order step 1 (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The owner join accepts a clause function's `ModelOwner` only as "an operation member, or an object type for an invariant clause", but FR-322 defines no wire member from which a reader reads a clause function's clause kind. TC-280's MW-11 to MW-14 carry an undefined `clause` binding with a text literal. IR 8eb9837b (`owner.rs` `clause_kind`) reads that binding and accepts only `invariant`, `precondition` and `body`. It refuses `postcondition`, which FR-341 lists as a clause kind, and refuses a function with no or two `clause` bindings. QSpec's readers ignore the kind. Specify the member (name, literal shape, allowed values) and the refusal when it is missing. This is a cross-repo mismatch: IR aligns to the specified values. | spec/objects/interfaces/FR-322-checked-package-artifact.md:281-288 |
| FND-002 | medium | The new member-identity sentence is inaccurate in three ways. "A model-owned member (a node carrying a `ModelOwner`)" conflates a `model_member` term with its declaring node. "The node's body, which is empty" is false for a clause function, which carries a `ModelOwner` and a non-empty body. "Resolves by the member's identity" contradicts "Model-owned members" step 3, which matches the member's `name` against the last `/` segment of each effective member's IR node identity. Restate it, for example: "A `model_member` whose declaring node is a model declaration node resolves through that node's `owner` in the selected document (steps 2-3), never through the node's body, which is empty; a `relationship_end` member on a model-owned relationship resolves its end the same way." | spec/objects/interfaces/FR-322-checked-package-artifact.md:197-200 |
| FND-003 | low | "Owner member schema" says "An identity projection entry carries exactly its node's owner" but gives no refusal for an entry whose `owner` is well-shaped and differs from its node's. The schema cannot express that equality, so the `malformed_wire` sentence does not cover it. IR 8eb9837b refuses it `stale_dependency` at `/identity_preimage/identity_projection/{i}/owner`. Name the code and locus, or point to the general projection-mismatch refusal, so QSpec and IR agree by spec rather than by accident. | spec/objects/interfaces/FR-322-checked-package-artifact.md:594-595 |

## Verdict

The edits under review are coherent with the reader. MW-06 now matches the join-before-key order in FR-322's derived-shape stage, and the AC-29 parenthetical says why. The AC-54/TC-233 `frames` `function` edit changes no `dependencies` and no canonical order, so FR-451 step 1 is the check that fails, as the AC states. The parenthetical has left one AC-29 clause without a vector (SR-1301 FND-001). FND-001 is a real spec gap that has already produced an IR/QSpec divergence. FND-002 is a misleading normative sentence. Fix both in this PR. FND-003 is pre-existing text in the section under review. Fixing it is cheap and keeps the two readers aligned.

## Dispositions

Round 1, reviewed at b6910d8 (fix range b6292c6..b6910d8), against the plan-lead rulings as stated. Clause-function kinds are QSL FR-094's `clause` text binding (`invariant`, `precondition`, `body`), and `postcondition` is refused as a clause-function kind. `state`/`state_clause` nodes carry no owner. A projection owner that differs from its node's owner refuses `invalid_package`/`invalid-value` at `/identity_preimage/identity_projection/{i}/owner`, ahead of the owner join. Cross-repo: IR 8eb9837b refuses that projection case `stale_dependency` (`owner.rs` `validate_owner_schema`), so IR must change code to match the ruling. That is an IR ticket, not a QSpec finding.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 0f4002a |
| FND-002 | fixed | f4f815a |
| FND-003 | fixed | f4f815a |

## New findings (disposition pass 2)

Over the spec edits in b6910d8..d8f3dbc: FR-154 identity paragraph and AC-9/AC-10, FR-322 relationship_end text, step 1 document shape, AC-29/30/44/55/56/57, TC-233, TC-280, TC-281. The rulings are taken as decided. FR-154-AC-10 and FR-322-AC-57 match IR ce634ee6 FR-038:983-1135 in codes, loci and order. FR-154-AC-10's linear order is FR-154's table order, which IR defers to.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | FR-154's identity paragraph is meant to state FCD FR-095's slots (ruling 3), but it omits the constrained-field alias, `ix://<pkg>/<Name><Field>` with no slot segment. The selected document's value type `OrderTotal` has exactly that identity, and TC-280 calls it "the alias FR-095 mints". FR-154 says "A type's identity is `ix://<package identity>/<artifact id>`", which `OrderTotal` is not. Add the alias row. | spec/functional/type-model/FR-154-admit-domain-package-model.md:108-124 |
| FND-005 | low | FR-154 cites "`FR-095` lines 107 to 115" in another repository. The range is already wrong at FCD 739ad24 and origin/main 68c0acba: the slot list runs from line 105 (type) to line 116 (step), so the cited range drops the type, alias and step rows. A line-number citation across repos drifts with every edit. Cite FR-095's "Node identities" section instead. | spec/functional/type-model/FR-154-admit-domain-package-model.md:109 |
| FND-006 | low | FR-154-AC-9 says the document "carries exactly FR-095's identity forms" and lists three relationships, but the document declares four: `Order-twin-Order`, used by RE-ends-9 and RN-12, is missing from the list. The test checks all four. Add it, or drop the list. | spec/functional/type-model/FR-154-admit-domain-package-model.md:158 |

## Dispositions (round 3)

Round 3, reviewed at e938a77 (fix range d8f3dbc..e938a77).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | e938a77 |
| FND-005 | fixed | e938a77 |
| FND-006 | fixed | e938a77 |

## New findings (disposition pass 4)

Scoped to the origin amendment bc04ca78..5555986. The FR-154 origin paragraph, AC-3, AC-7, AC-8 and AC-10, FR-152:83 and TC-234 K1 and its Expected Results conform to the ruling and to FCD `common.schema.json` `$defs.origin`/`sourceLocus`/`generatedOrigin`.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | medium | Text that still promises an artifact id and span for FCD declaration refusals remains in: FR-154's own Description ("refusing any invalid node with its source artifact and span"); TC-234 rows I03, I08 and I09 ("artifact `A` and the row's span", "artifact `B` and its span", "artifact `sys-pump` and its span"); TC-195 N08; TC-197 Y06 ("each naming the node, its artifact and its span", the FR-152 refusals this amendment changed); TC-235's vector preamble and Expected Results; IT-070-SC-03; and AD-006 step 3. Restate each as the IR node identity and its `origin`. | spec/functional/type-model/FR-154-admit-domain-package-model.md:26-29; spec/test-cases/TC-234-domain-package-model-intake.md:45,50,51; spec/test-cases/TC-195-model-normalization-provenance.md:61; spec/test-cases/TC-197-systems-model-typed-binding.md:80; spec/test-cases/TC-235-business-construct-meaning-binding.md:74-75,105; spec/integration/IT-070-artifact-bundle-to-checked-package.md:54; spec/assurance/AD-006-model-graph-state-contracts.md:89-91 |
| FND-008 | low | "A FCD document carries no artifact id" reads as contradicting the declaration-refusal table and FR-154-AC-8, which still check "a type definition's artifact id" against `^[A-Za-z][A-Za-z0-9_]*$`. Say that the document carries no separate artifact-id member and that the id is the `<Name>` segment of the IR node identity, so the id check has a defined input. | spec/functional/type-model/FR-154-admit-domain-package-model.md:94 |

## Dispositions (round 5)

Round 5, reviewed at 6fd6acf (fix range 5555986..6fd6acf). FND-007's nine named places are all fixed. A sweep of spec/ for "artifact" next to "span" found five more places that promise an artifact id and span for FCD model-intake refusals or declarations, outside the list FND-007 named.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-007 | still-open | The nine places named are fixed in 6fd6acf. Five remain: FR-321:24 (an intake declaration "keeps its source artifact and span"), interface_003-model-selection:19 (intake "refuses with the artifact and span"), FR-272:61 (model-intake `missing_declaration`/`missing-name` carries "artifact id and span"), FR-272:65 ("every model intake and model feature variant adds the source artifact id and span of the IR node it names") and TC-197 Y09:83 (intake refusal naming "the artifact and the span"). Restate each as the IR node identity and its `origin`. |
| FND-008 | fixed | 6fd6acf |

## Dispositions (round 6)

Round 6, reviewed at efa7bdb (fix range 6fd6acf..efa7bdb, spec only). A fresh sweep of spec/ and proposals/ for "artifact" next to "span" leaves no FCD model-intake text promising an artifact id or span. The coder's exclusions are right:
- model-complete:142 and TC-235 M03 are a non-quire clause kept with its own text span, not a refusal locus.
- FR-450 and TC-411 refuse QSL `abstraction-decl` source at QSL source spans, not FCD intake.
- AD-006:169 is the quire-rs extraction stage, before the FCD lift.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-007 | fixed | efa7bdb |

## New findings (disposition pass 6)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-009 | low | The Identity row of `quire.model.features/v1` in `model-complete.md` still says "A member's identity is its owner's identity, `/` and the member name". FR-154 (round 2, ruling 3) and IR-661 nest only fields, operations and parameters, and mint relationships, clauses, constraints, variants, states, transitions and steps under their own global slot. This is the generic owner-nested rule that IR ce634ee6 FR-038 calls an "upstream prose conflict". Restate the row as FR-154's slot rule, or point to it. | proposals/quire-v1/definitions/model-complete.md:153 |

## Dispositions (round 7)

Round 7, reviewed at d6aa159 (fix range efa7bdb..d6aa159, spec only).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-009 | fixed | d6aa159 |
