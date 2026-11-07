---
id: SR-20261107109301
title: "Base review of the QSL-1093 hyperproperty body change"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification branch spec/qsl-1093-fr402, diff origin/main...HEAD: FR-402 (quantifier Groups, subjects, body wrapper, slot parameters, reader order, AC-5..AC-9), FR-370 (placement, reader order, AC-21), FR-322 body-steps list, FR-481 step position, TC-346 HI-08..HI-15; read against FR-322 Body grammar, FR-272, FR-476 and FR-477"
review_set: base
relationships:
  - target: ix://agent-ix/quire-specification/FR-402
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-370
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-346
    type: reviews
---

## Summary

Ticket QSL-1093. This review covers only the committed diff
`origin/main...HEAD` of branch `spec/qsl-1093-fr402`. It uses the base
checklist and runs no analysis lenses.

What holds:

- **Grammar.** The new `quantifiers` shape is legal in FR-322 "Body grammar".
  The chain is a Member `binding`, then a Tuple (an aggregate of Groups),
  then a Group (`binding`s of text Leaves, then `reference` Leaves), then a
  `binding`, then a Leaf. That is the deepest chain the grammar allows.
  `subjects`, `parameters`, `match` and `align_skip` are Groups, and `body`
  is a Member `binding` of a Leaf.
- **Placement.** FR-370's placement rule now admits a fairness reference as
  a quantifier Group `reference` member. Every other position in a
  hyperproperty body is misplaced (`ill_typed`/`operator-ineligible` at the
  node), and so is every formula reference there. This matches FR-402's
  Reader order, which relies on the earlier temporal step, and it matches
  the wrapper, which is a separate `temporal_clause` node.
- **Step order.** FR-322 "Body steps" lists the hyperproperty step between
  the step-relation step and the hybrid step. FR-402 "Reader order" and
  FR-481 say the same.
- **Pointers.** `B/members/3/value/members/{g}/members/{m}` names quantifier
  Group `g`, member `m`: `quantifiers` is binding 3. HI-12(b)'s `.../members/0/members/3`
  is `a`'s first fairness reference, after `variable`, `quantifier` and
  `alias`. `/body/arguments/4` is the wrapper's fairness argument, argument
  5 of FR-370.
- **Causes.** Every code and cause used exists in FR-272:
  `operator-ineligible`, `non-boolean-root`, `malformed-declaration`,
  `missing-name`, `invalid-value`, `operation-member-mismatch`, and the
  cause-less `invalid_semantic_graph`.
- **IDs.** No new AC ID collides with `origin/main`: FR-402-AC-5 to AC-9 and
  FR-370-AC-21 are absent there.
- **Slot encoding.** The tokens `__`, `_xHH`, `_s` and single
  alphanumerics form a prefix-free code, and every tag is free of `_`, so
  decoding is unique. HI-15's expected names check out:
  `slot_s_xC3_xA9_sa` for `é` and `arg_sa_sVault_sstep_si`.
- **Ceremony.** No digest, pin, SHA or tracking record was added.

The findings below cover reader-order gaps, untested refusals, and the
sibling body specs that delegate to FR-402.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-402 step 3 (Fairness) does not order its checks: the non-`operation`-target check, FR-370 "Fairness resolution" steps 1 to 3, and the owner-package check. It also gives no locus for FR-370's resolution refusals when they are reached through a Group. For a `protocol_root` target, FR-370's resolution ("belongs to the protocol clause the fairness clause is over") is undefined, so the kind check must come first. HI-12(b) expects `malformed-declaration` (owner package), but if `W`'s type has no `step`, name resolution would give `missing-name` first. FR-402-AC-7 and HI-12(a) state `missing-name` with no locus. Fix: rewrite step 3 as ordered sub-steps: (3a) target kind `operation`, else `invalid_model_binding`/`malformed-declaration`; (3b) FR-370 resolution steps 1 to 3; (3c) the declaring node's `ModelOwner` package equals the Group alias's subject root, else `malformed-declaration`; (3d) profile fit. State that every 3a to 3d refusal is located at `B/members/3/value/members/{g}/members/{m}`. Add that locus to FR-402-AC-7's `missing-name` and to HI-12(a). | spec/objects/interfaces/FR-402-hyperproperty-obligation-identity.md:193-201,221; spec/test-cases/TC-346-hyperproperty-obligation-identity.md:47 | wrong-requirement |
| FND-002 | medium | Step 3's profile-fit check reads "the body wrapper's profile", but step 4 only checks afterwards that `body` targets a `temporal`/`temporal_clause` node. When step 3 runs on a body whose target is not a wrapper, the profile is undefined. Fix: move the Body check (step 4) before Fairness and renumber. Or, if `body` is checked last, state that the profile fit runs only when `body` targets a `temporal_clause` node. | spec/objects/interfaces/FR-402-hyperproperty-obligation-identity.md:198-208 | wrong-requirement |
| FND-003 | medium | Several FR-402 Reader-order refusals have no AC and no TC row: a `subjects` binding that no Group names (`invalid_semantic_graph`); in a finite domain, a `subjects` member, an `alias` binding or a fairness reference (`invalid_semantic_graph`); a body that is not exactly the seven bindings; a fairness target that is not an `operation` (`malformed-declaration`); and a finite-domain `body` that is not Boolean (`ill_typed`/`non-boolean-root`). A reader that skips any of these passes TC-346. Fix: add each to FR-402-AC-6 (subjects), FR-402-AC-7 (shape and fairness kind) and FR-402-AC-8 (finite body), with matching HI rows built from HI-08 and HI-09. | spec/objects/interfaces/FR-402-hyperproperty-obligation-identity.md:178-208,219-222; spec/test-cases/TC-346-hyperproperty-obligation-identity.md:43-49 | correct-requirement-no-evidence |
| FND-004 | medium | FR-476 and FR-477 say their subject is "the node an FR-402 `subjects` binding names" or "as FR-402's `subjects` does", and FR-402 now makes that node the `model`/`model_root` node. But FR-477 still says "the alias's `model` node", which is the ambiguous wording QSL-1093 fixed in FR-402. The two reader steps, FR-477 step 3 and FR-476 step 4, still admit any `model` node, including `model`/`object_type`, which FR-402 step 2 refuses. Fix: in FR-477 Properties `subjects`, write "each referencing the `model`/`model_root` node of the domain package the alias selects, as FR-402's `subjects` does". In FR-477 step 3 and FR-476 step 4, write "a target that is not a `model`/`model_root` node". Add a `model`/`object_type` target case to FR-476-AC-4 and FR-477-AC-4. | spec/objects/interfaces/FR-477-checked-package-step-relation-claim-body.md:41-44,137-142; spec/objects/interfaces/FR-476-checked-package-model-terminal-declaration-body.md:39-41,101 | wrong-requirement |
| FND-005 | medium | This finding is pre-existing and outside the diff, but it is the same defect class QSL-1093 fixes. FR-477's `executions` is an aggregate of `binding(variable, aggregate)`, and each entry holds `binding("parameters", aggregate)`, `binding("preconditions", aggregate)` and `binding("postconditions", aggregate)`. Its `invariants` is an aggregate of `binding(alias, aggregate)`. Under FR-322 "Body grammar", a Tuple holds no `binding` and a Group's `binding` takes a Leaf value only, so a strict reader refuses every step-relation body `malformed_wire`. Fix: give FR-477 its own ticket, a sibling of QSL-1093, to restate `executions` and `invariants` in a grammar-legal shape, for example one execution Group per variable, with each list-valued member moved to a node reached by `reference`. | spec/objects/interfaces/FR-477-checked-package-step-relation-claim-body.md:45-79; spec/objects/interfaces/FR-322-checked-package-artifact.md:1718-1724 | wrong-requirement |
| FND-006 | medium | The body wrapper's exclusion cannot be tested against any named reader behaviour. FR-402 says "a consumer that lists or resolves single-trace temporal claims SHALL exclude it". FR-402-AC-8 and HI-14 check "not among the package's single-trace temporal claims" without naming the listing, and no refusal is stated for an FR-368 request item whose `claim` names the wrapper's `claim` occurrence. Fix: name the QSpec surface that lists single-trace claims (FR-368 item admission), state the FR-272 refusal for an item that claims a body wrapper as a single-trace temporal claim, and add that case to FR-402-AC-8 and HI-14. | spec/objects/interfaces/FR-402-hyperproperty-obligation-identity.md:100-105,220; spec/test-cases/TC-346-hyperproperty-obligation-identity.md:48 | correct-requirement-no-evidence |
| FND-007 | low | FR-402 now puts the trace variable's name in the quantifier Group's `variable` binding and in every slot name, so renaming a trace variable changes the node. "Obligation identity" and FR-402-AC-3 ("differ only in source spelling") do not say whether bound-variable renaming counts as spelling. QSL FR-488 Q-6 stays open on this point. Fix: add one sentence to FR-402 "Obligation identity", such as "A trace variable's name is part of the clause: renaming it changes the node and the obligation identity". Add an HI row for it, so QSL can close Q-6. | spec/objects/interfaces/FR-402-hyperproperty-obligation-identity.md:138-155,217 | wrong-requirement |
| FND-008 | low | FR-322-AC-121 lists the body steps as state-graph, temporal, abstraction, refinement, terminal, step-relation, hybrid, and so on. It leaves out the hyperproperty step that the "Body steps" list now holds, and it also predates the implementation-guarantee and composition steps. Fix: make AC-121's order match the Body steps list, inserting `hyperproperty` between `step-relation` and `hybrid`. | spec/objects/interfaces/FR-322-checked-package-artifact.md:589-596,4491 | wrong-requirement |
| FND-009 | low | HI-12(b) needs HI-10's alias `W` to be selected, but the row starts from HI-08, which has no `W`. The expected `malformed-declaration` also depends on whether `W`'s object type declares `step` (see FND-001). Fix: write it as "HI-08 plus HI-10's alias `W`, with the fairness node's declaring node an object type of `W`'s package that declares an operation `step`". | spec/test-cases/TC-346-hyperproperty-obligation-identity.md:47 | correct-requirement-no-evidence |

## Verdict

The design is sound and grammar-legal, and it resolves the fairness
contradiction between FR-402 and FR-370. FND-001, FND-002 and FND-004 are
wording fixes to reader order and to the sibling specs that delegate to
FR-402. Fix them in this PR. FND-003 and FND-006 are evidence gaps that need
AC and TC additions in this PR. FND-005 is pre-existing in FR-477 and needs
its own ticket. FND-007 to FND-009 are low-severity clarifications. The diff
is not ready to merge until FND-001 to FND-004 and FND-006 are fixed.
