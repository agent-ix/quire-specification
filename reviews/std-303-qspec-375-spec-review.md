---
id: SR-15820
title: "Spec review of quire-specification PR #375 (STD-303): checked-package model terminal declaration node"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@e7b9545400407b8ac059c40b45dcb65d665f41c4; PR #375 diff against main 5ee0071f; FR-476, TC-961, FR-322 dependencies list, proposals/checked-package-v2/schema.json TemporalNode, spec/spec.md; checked against FR-322, FR-341, FR-361, FR-366, FR-368, FR-370, FR-402, FR-465 and quire-spec-language FR-124"
review_set: subset
---
# Spec review of quire-specification PR #375

## Summary

Ticket: STD-303. The PR adds FR-476, the `temporal`/`terminal` node of a model `terminal` declaration (body `model`, `mode`, `parameters`, `condition`; declaration path `<model alias>, terminal`; reader order and refusals), TC-961 (MT-01 to MT-12), `terminal` in the schema's `TemporalNode` enum, an FR-322 list entry and a spec.md index row.

Examined:
- FR-476 Description, Properties, Identity, Reader order, FR-476-AC-1 to AC-4 (examined)
- TC-961 procedure and vectors MT-01 to MT-12 (examined)
- schema.json `TemporalNode` enum line; FR-322 `dependencies` body-artifact list; spec.md rows (examined)
- FR-322 declaration-name rule and `dependencies` rebuild rule; FR-341 parameter aggregate and `value`/`parameter` body; FR-361 model subject; FR-366; FR-368 request members; FR-370 reader order; FR-402 `subjects`; FR-465 "Terminal" (context_only)
- quire-spec-language FR-124 `self` typing and classifier rule (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-476 binds a `self` parameter typed `Reference<M::T>` but no QSpec artifact says what `self` means in a terminal predicate. FR-366 never mentions `self`. Which type `T` is (the one object type the model's populations cover, covered by exactly one population), and how the classifier evaluates a `self`-reading `P` (true for every object of that population, vacuous when there is none, false if false for any, else undefined), live only in QSL FR-124. FR-476 says the node is the only carrier that a classifier and replay read, so a QSpec consumer cannot evaluate such a node. The reader also checks only that each `parameters` target is a `value`/`parameter` node. It does not check that the target is `self` at level 0 typed by that object type, or that there is at most one target, which FR-341-AC-4 checks for a state clause. Put the `self` meaning in FR-366 (or FR-476), add the reader checks, and add vectors for them. | spec/objects/interfaces/FR-476-checked-package-model-terminal-declaration-body.md:41-44; :82-88 |
| FND-002 | medium | The TC-961 procedure for MT-07 to MT-12 says only "recompute its key". MT-09, MT-10 and MT-11 change the body's `reference` targets, so the node's `dependencies` no longer equal the targets. FR-322 then refuses `invalid_semantic_graph` at `/dependencies` (and a stale projection entry or `package_id`) before the terminal step runs, so the expected refusals are never reached. MT-09's first case also needs an `expression` node that MT-02's package does not hold, and MT-11 assumes `self.value` is its own `expression` node. Say that each mutation rebuilds `dependencies`, patches the projection entry, recomputes `package_id` and adds any node it references, as TC-233 does. | spec/test-cases/TC-961-checked-package-model-terminal-node.md:23-25; :40-44 |
| FND-003 | medium | FR-322 already refuses two nodes carrying one `qualified_name` in the types namespace as `ambiguous_declaration`/`ambiguous-name`, "before any operation refusal". Two `terminal` nodes for one model alias always share the path `M`, `terminal`. FR-476's separate later check ("no two `temporal`/`terminal` nodes reference one `model` node") is therefore either unreachable or runs in an order the spec does not fix, with possibly different loci. MT-12 passes either way, so it cannot tell. Choose one owner: state that FR-322's name rule refuses the duplicate, or fix the order and loci of FR-476's check. | spec/objects/interfaces/FR-476-checked-package-model-terminal-declaration-body.md:96-99 |
| FND-004 | low | Identity says a model subject's identity "binds the node id of the model's terminal node". FR-361 already makes the checked package part of the subject, and FR-368's `subject` member has no terminal member, so this binding is the `package_id`. As written it reads as a second binding with no stated place. Reword it to "through the package identity (FR-361)". | spec/objects/interfaces/FR-476-checked-package-model-terminal-declaration-body.md:67-72 |
| FND-005 | low | Mode fit refuses "at the binding that breaks the rule". It gives no order for an `any` node whose `parameters` is non-empty and whose `condition` is a reference. State body order (`parameters`, then `condition`). | spec/objects/interfaces/FR-476-checked-package-model-terminal-declaration-body.md:85-88 |
| FND-006 | low | The FR-322 list puts FR-476 before "FR-464 to FR-466", which breaks the list's ascending order. #370 (FR-475) and #377 (FR-477) edit the same line in the same way. Put the entry after FR-466. | spec/objects/interfaces/FR-322-checked-package-artifact.md:553-556 |

## Verdict

The node form is sound. The `when`/`any` mode with a condition exactly for `when` matches FR-465's protocol `terminal` member. The aggregate body, with the condition a reference to an `expression` node, is justified because a nested application is outside FR-322's body grammar. The layout-defect code differs from FR-465 (`invalid_semantic_graph`, not `malformed_wire`), which fits a node body rather than an argument. The owner-nested path `<model alias>, terminal` is unique under FR-366's one-per-alias rule, and the written name is correctly presentation-only. Leaving the `using` profile out of the body is consistent with FR-341. The `model` target is FR-402's alias `model` node, which the PR flags as ungrounded. AC-1 to AC-4 are testable, and each maps to MT vectors that can fail. The ids FR-476 and TC-961 are unique on main and across the 13 other open PRs.

Gates at e7b95454: `quire validate --scope . 'spec/**/*.md'` gives 86 failure lines (baseline), and FR-476 and TC-961 validate clean. `cargo test --test checked_package_v2 --test backend_provider_v1` passes (23 + 119). The QSL conformance run (`qsl-package --lib checked_v2`, QSPEC_DIR at this head) is reported separately.

Merge order: #375 merges first. #370 and #377 each conflict with #375 in FR-322's list line and in spec.md, and with each other in schema.json's `ClaimNode` line. Resolve each as main plus its own entry. **Not mergeable until FND-001 is fixed, and FND-002 and FND-003 should be fixed in the same round.**

## Dispositions

Round 1, reviewed at 06cb0bdc88354ddfe860de207a6ddc7f3ee84175 (fix commit 6ca9dc60).

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | FR-366 now gives `self` its type `T`, its per-object evaluation, the vacuous case and both refusals (FR-366-AC-9, TC-297 step 9). FR-476 step 4 checks `self` at level 0 and its type, `parameters` holds at most one entry, and MT-13 to MT-15 test this. |
| FND-002 | fixed | The TC-961 procedure now adds the referenced nodes, sets `dependencies`, rewrites the projection entry and recomputes `package_id`, as TC-233 does. MT-09 and MT-11 name the added nodes. |
| FND-003 | fixed | The node carries no `declaration`, as the schema's `DeclarationTagRules` require for a `temporal` node, so no FR-322 name rule precedes the check. FR-476 step 5 alone owns the duplicate refusal, at the second node. |
| FND-004 | fixed | Identity now binds the subject through the package identity (FR-361), and the request carries no separate terminal member. |
| FND-005 | fixed | Mode fit runs in body order and names `parameters` for an `any` node with both defects (MT-14). |
| FND-006 | fixed | FR-476 now follows FR-466 in the FR-322 list. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | medium | FR-322's new "Body steps" paragraph links FR-475 (the state-graph step), but FR-475 exists neither on main (c1033008) nor in this PR. It arrives only with #370, whose own version of the same paragraph has no terminal step, while #377's adds a step-relation step. As it stands, #375 merged first leaves a dangling link. Either merge #370 before #375, or drop the state-graph clause here and let #370 add it. The merge-order plan must also name one final text of the paragraph covering FR-475, FR-370, FR-451, FR-476, FR-477 and FR-464. | spec/objects/interfaces/FR-322-checked-package-artifact.md:423-434 |
| FND-008 | low | The node's occurrences have role `declaration`, but the node carries no `declaration`. FR-322's rules for a "declared node's `declaration` occurrence" (owner and source agreement) do not apply to it. The other undeclared clause nodes use role `claim` (FR-341, FR-370). Use `claim`, or state why a `declaration` occurrence on an undeclared node is admitted. | spec/objects/interfaces/FR-476-checked-package-model-terminal-declaration-body.md:58-60 |
| FND-009 | low | The second MT-15 case types `self` `Reference<M::D>` "for a second object type `D` of `M`". If `D` has a population, FR-366 binds no `self` for `M` at all (several covered types), so the expected refusal and its cause are unclear. State that `D` is covered by no population. | spec/test-cases/TC-961-checked-package-model-terminal-node.md:48 |
| FND-010 | low | The new FR-366 bullet refuses `self` with `ambiguous_declaration`/`ambiguous-name` when two populations cover `T`, but FR-366-AC-9 and TC-297 step 9 test only the two-object-type `missing-name` branch. Add the two-population case. | spec/functional/temporal/FR-366-check-deadlock-freedom.md:136 |

Round 2, reviewed at 76bda9f43a8b1def1cc4b411ff63c5254309fa83 (FND-001 to FND-006 stay fixed from round 1).

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-007 | fixed | The "Body steps" paragraph now names the steps without links (FR-475, FR-370, FR-451, FR-476, FR-477, FR-464), so nothing dangles, and it is byte-identical in #370 (f326f663) and #377 (1fd2750c), so it merges cleanly in any order. |
| FND-008 | fixed | The occurrences have role `claim`, as FR-341 and FR-370 use, and FR-476-AC-1 and MT-01 say so. |
| FND-009 | fixed | MT-15's `D` is now covered by no population, so `C` stays FR-366's `T`. |
| FND-010 | fixed | FR-366-AC-9 and TC-297 step 9 now include the two-populations-of-`Job` case, refused `ambiguous_declaration`/`ambiguous-name` at `self` and naming both. |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-011 | medium | MT-16 ("one unit declaring MT-01 twice for `M`") is compiled through the Rust boundary, because the procedure's edit range is MT-07 to MT-15. The checker refuses a second `terminal` declaration for one alias (FR-366-AC-6), so no package and no node exist, and its expected "one node with two `claim` occurrences" never happens. The reader rule of FR-476 step 5 is never exercised. Make MT-16 a reader mutation (MT-01's package with a second `claim` occurrence added to the node and the `source_map`, with `package_id` recomputed) and extend the edit range to MT-16. | spec/test-cases/TC-961-checked-package-model-terminal-node.md:24; :49 |
