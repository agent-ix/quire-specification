---
id: SR-15880
title: "Spec review of STD-291 step-relation claim node (QSpec PR #377)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@9e408ad866774f8a76ae40fb75ff25909d8b59b0; spec/objects/interfaces/FR-477-checked-package-step-relation-claim-body.md, spec/test-cases/TC-962-checked-package-step-relation-node.md, spec/objects/interfaces/FR-402-hyperproperty-obligation-identity.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/spec.md, proposals/checked-package-v2/schema.json"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-477
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-962
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-402
    type: reviews
---
# SR-15880: Spec review of STD-291 step-relation claim node (QSpec PR #377)

## Summary

Ticket: STD-291. The diff is `origin/main...9e408ad86` (main is an ancestor of the head). It is spec only: new FR-477 and TC-962, one sentence in FR-402, one entry in FR-322's dependency-list sentence, two spec.md lines and `step_relation` in the `ClaimNode` `semantic_form` enum. The reviewer checked it against FR-322 (owner, structural node preimage, dependencies), FR-341 (parameter aggregate and `value`/`parameter` body), FR-342, FR-397, FR-402, FR-451, FR-464's reader order, TC-337 HB-11 and TC-343 SR-05, and the open PR #370 (FR-475).

These claims hold:

- The form is separate and premises are reached by reference: preconditions, postconditions and invariants are their own `state`/`state_clause` nodes, the frame is reached through the anchor's `frame` member (FR-342), and the implementation is a reference to the `correspondence`/`abstraction_relation` node (FR-451 frame key = anchor's context and operation).
- Each execution is an aggregate of alias, operation anchor, parameters in FR-341 postcondition order, preconditions, postconditions and implementation. Reader step 4 requires each premise set to be exact over the package.
- The FR-402 pointer is consistent with FR-402's step-relation identity paragraph; FR-397's "Code claim" request rule matches FR-477 "Code claim request".
- The node is a structural node with a `declaration`, so FR-322's owner rule applies; `claim` nodes may carry a declaration (DeclarationTagRules) and occurrence role `claim` is in the schema enum.
- FR-477 and TC-962 are unused on main and on every other open PR head (14 checked). TC-337 HB-11 and TC-343 SR-05 exist. No status wording, no compatibility layer.

Gates at 9e408ad86: `quire validate --scope . 'spec/**/*.md'` 86 failure lines (baseline), none in a changed file; `cargo test --test checked_package_v2 --test backend_provider_v1` 23 + 119 passed. QSL conformance (`cargo test -p qsl-package --lib checked_v2`, QSPEC_DIR = this head, QSL at #1260's head): 42 passed.

Merge plan: #370 (FR-475) and #375 (FR-476) edit the same FR-322 dependency-list line and the same two spec.md insertion points, and #370 edits the same `ClaimNode` enum line; #365 also edits the FR-322 list line. `git merge-tree` against #377 conflicts for #370, #375 and #365 (clean for #371). Resolution is the union: enum `[..., "hyperproperty", "state_graph", "step_relation", "synthesis_request"]`, FR-322 list in numeric order, and each spec.md row kept.

The FND-001 fix needs an owner or planner decision on how executions get distinct binders; it may need a short /specify pass across FR-341/FR-322 as well as FR-477.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | major: The executions cannot be told apart. `parameters` references the FR-341 postcondition binders of the operation, and reader step 4 requires exactly those nodes. A `value`/`parameter` node carries no owner and is keyed by `{name, level}` and its type alone (FR-322 structural preimage, P4). Two executions of one operation (`x: V::Vault::step, y: V::Vault::step`) therefore reference the same `self`, `result` and `i` nodes, and the body's reads of `x` and `y` are reads of the same nodes. HB-11's `(pre(x.self.l) = pre(y.self.l) and x.i = y.i) implies x.self.l = y.self.l` lowers to a tautology, so the code claim a harness reads is vacuous. <br>Fix, owner or planner decision: give each execution its own binders, for example (a) per-execution `value`/`parameter` nodes whose body also binds the execution variable (or a continuing level across executions), with step 4 checking their types against the postcondition binders instead of node identity; or (b) execution-indexed reads in the body, as hyper bodies index trace variables (`e @ a`). Add an AC and a TC vector: `x`'s and `y`'s `self` nodes differ and HB-11's body is not a tautology. | spec/objects/interfaces/FR-477-checked-package-step-relation-claim-body.md:54-78, spec/objects/interfaces/FR-477-checked-package-step-relation-claim-body.md:128-133 |
| FND-002 | medium | minor: The reader position is not unique. The step-relation step runs "after the abstraction step of FR-451 and before the operation step"; FR-464's protocol step and PR #370's FR-475 state-graph step occupy the same window. A package with a protocol defect and a step-relation defect has no stated first refusal. <br>Fix: order the step against the protocol step (and FR-475's step once #370 lands), for example "after the protocol step". | spec/objects/interfaces/FR-477-checked-package-step-relation-claim-body.md:108-111 |
| FND-003 | low | minor: Properties list `declaration` but not `owner`. A node with a `declaration` that is not nominal must carry its `SourceOwner` (FR-322 `owner`, schema OwnerPresenceRule), and the owner enters the node key. <br>Fix: add `owner — the declaring source unit's SourceOwner (FR-322)` to Properties. | spec/objects/interfaces/FR-477-checked-package-step-relation-claim-body.md:79 |
| FND-004 | low | minor: TC-962 RN-01 says "`result` (when `step` has one)", so the vector leaves the parameter list open. TC-337 HB-11's vault is fixed. <br>Fix: state whether `step` has a result and list the exact parameter nodes. | spec/test-cases/TC-962-checked-package-step-relation-node.md:31 |
| FND-005 | low | minor: FR-322's dependency-list sentence puts FR-477 between FR-451 and the FR-464 range, out of numeric order. <br>Fix: move it after the FR-464 to FR-466 range; this also eases the #370/#375 merge. | spec/objects/interfaces/FR-322-checked-package-artifact.md:553-556 |

## Verdict

Not mergeable until FND-001 is resolved. The rest of the form (by-reference premises, exact premise sets, code-claim request rule, identity through FR-322) is sound. FND-002 should be fixed in the same round; FND-003 to FND-005 are wording.

## New findings (disposition pass 1)

Head 2ea9d5a7e (`origin/main` is an ancestor). Re-derived HB-11 with the fix: `x`'s binders are `self`@0 and `i`@1, `y`'s are `self`@2 and `i`@3, so the four keys differ. `(pre(x.self.l) = pre(y.self.l) and x.i = y.i) implies x.self.l = y.self.l` now reads four distinct nodes and is refuted by two runs from equal pre-states and inputs whose post `l` differs. The vacuity is gone. Continuing levels agree with QSL FR-092 ("a binder's level is its position among the binders in scope").

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | minor: FR-322's new "Body steps" list names the state-graph step (FR-475) and the terminal step (FR-476). Those requirements exist only on open PRs #370 and #375. If #377 lands first, main's order list names two absent requirements. <br>Fix: merge #370 and #375 before #377 (or in one train), or drop the two entries here and have #370 and #375 insert their own. Either way, the lead owns the merge order. | spec/objects/interfaces/FR-322-checked-package-artifact.md:423-434 |
| FND-007 | medium | minor: FR-341 owns the `value`/`parameter` body and defines `level` as "the value's binder level in the 'Operation anchors, aliases and captures' table (`0` for `self`, ...)". FR-477's `y` binders (`self` at level 2) contradict that definition, and FR-341 was not amended. <br>Fix: one sentence in FR-341's `value`/`parameter` body section: a step relation's execution binders take FR-477's continuing levels. | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:131-138 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Each execution has its own binders, with levels continuing across executions. Reader step 4 checks names, types and level continuation. FR-477-AC-5, RN-13 (binders differ, body not a tautology) and RN-14 (shared binders refused). Re-derived: HB-11 reads four distinct nodes. |
| FND-002 | fixed | FR-322 "Body steps" fixes one order: state-graph, temporal, abstraction, terminal, step-relation, protocol. FR-477 places its step after terminal and before protocol. This agrees with FR-451, FR-464, FR-370 and #375's FR-476. See FND-006 for the cross-PR references. |
| FND-003 | fixed | Properties carry `owner`, the SourceOwner that enters the key. |
| FND-004 | fixed | RN-01: `step` takes `i` and has no result; `x` is at levels 0 and 1, `y` at 2 and 3. |
| FND-005 | fixed | FR-477 now follows the FR-464 to FR-466 range. |
