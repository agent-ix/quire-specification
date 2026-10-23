---
id: SR-16260
title: "Post-merge spec-review of checked-package hybrid model nodes (quire-specification PR 400)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@059998c44688d4bcd212273aea097f696c0e5567 (vs parent); spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md; spec/objects/interfaces/FR-322-checked-package-artifact.md; spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md; spec/test-cases/TC-966-checked-package-hybrid-model-nodes.md; proposals/checked-package-v2/schema.json; proposals/checked-package-v2/node-identity-preimage.schema.json; proposals/checked-package-v2/hybrid-node-vectors.json; tests/checked_package_v2.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-481
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-193
    type: references
  - target: ix://agent-ix/quire-specification/TC-966
    type: references
---

# Post-merge spec-review of checked-package hybrid model nodes (quire-specification PR 400)

## Summary

Ticket: STD-385 (spec gap behind QSL-900), QSpec half. The QSL half is
quire-spec-language #1350. The PR was merged by its author without a review.
This is a post-merge review, so findings are fixed forward. Reviewed
`git show 059998c4` against its parent (10 files, +2744/-20). Oracles: FR-322
(body grammar, structural preimage, owner, dependencies, strict wire
validation), FR-370, FR-469, FR-475 to FR-477, FR-331, FR-193, and QSL FR-447
and FR-250 at QSL main 425bab9b.

Gates run by the reviewer at 059998c4:

- `quire validate --scope . 'spec/**/*.md'` gives 86 failure lines, the
  baseline. FR-481 and TC-966 report no failure.
- `cargo test --test checked_package_v2 --test backend_provider_v1` gives
  132 + 23 passed.
- QSL conformance (`cargo test -p qsl-package --lib checked_v2` on QSL main
  425bab9b with `QSPEC_DIR` at 059998c4): 46 passed. `make check-spec-validation` on QSL main with the same `QSPEC_DIR` exits 0.

Open PR interaction: QSpec #398 (STD-395, head 9239a52e) also edits FR-322,
`schema.json` and the preimage schema. `git merge-tree` of main and #398
conflicts textually only in `spec/spec.md`, where both add a row after FR-477
in the relationships list and in the index table. The fix is to keep both
rows. FR-322 and both schemas merge cleanly. #398 adds a "Reference literal
values" paragraph directly after "Scalar literal values", so the fix for
FND-007 lands next to it.

Examined and clean:

- Body grammar: every hybrid body fits FR-322's five strata. The `variables`
  and `resets` values are Tuples of Groups, and `modes`, `rates` and
  `transitions` are Groups. Making terms their own nodes keeps the depth
  fixed (FR-322-AC-40/41).
- Structural preimage: self-typed nodes use `semantic_type: null`. `owner`
  is present exactly when the node is declared (model and mode carry a
  `SourceOwner`; a transition carries none). This matches FR-322 "Structural
  node preimage", the OwnerPresenceRule and FR-469's pattern.
- Dependencies are the body's reference targets. Literal `type`s are not
  listed, which matches the FR-322 dependency rule.
- Transition targets and reset variables are indices, so the graph has no
  cycle.
- Body-step order: the hybrid step comes after step-relation and before
  protocol. This is consistent with FR-475 to FR-477, and the hybrid step's
  inputs (temporal clause placement and the over reference) are checked
  first by the earlier temporal step.
- Hand-checked the vectors against QSL FR-447-AC-1: the variables `v`, `a`
  and `t` are at levels 0 to 2 and typed rational; `Moving`'s rates are
  `negate(a)`, `0` and `1`; `Stopped`'s rates are `0`, `0` and `1`; the
  transition is target 1, guard `v = 0`, reset `(0, 0)`; the `Moving`
  invariant is left-associated; `h` is at level 0 and typed by the model;
  the clause is `always [0,10] holds(v < 30)` with clock `model-time` and
  activation `origin`. All match.
- FR-193's removal of "no hybrid node / no v2 bytes" is complete: no other
  QSpec artifact still says a hybrid item is in-process only.
- Every `quire.op.rational.*` and `quire.op.boolean.*` identity is in the
  schema.

The clock-rate check, which the brief asked about, is judged in FND-006.
It is sound in the direction it checks. The wire cannot tell `1` from
`rational(2, 2)`, so the class is the producer's attestation and cannot be
re-derived. The spec states neither that nor the admitted case.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | [scope-deferral] Decision 7 keeps the hybrid nodes out of `positive-all-families.json` "until QSL's reader admits `hybrid`", a deferral with no ticket or QSL requirement. This also leaves FR-322-AC-13 ("all 14 closed node tags"), TC-233 ("all 14 node tags (FR-322-AC-13)") and `REQUIRED_NODE_TAGS: [&str; 14]` stale, now that the closed union has 15 tags. Fix: add the hybrid nodes to the all-families fixture; make AC-13, TC-233 and the test require 15 tags; land this together with the QSL I2-reader requirement (SR-16261 FND-001). | spec/objects/interfaces/FR-322-checked-package-artifact.md:2000; spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:225; tests/checked_package_v2.rs:61; proposals/checked-package-v2/README.md:48 | correct-requirement-no-evidence |
| FND-002 | high | [soundness] A bare variable cannot be a rate, guard operand root or reset value. "Hybrid terms" says a term node is "an `expression` node whose body root is an application, or a `value`/`literal` node", and reader check 3.2/3.3 refuses a rate or reset value that is not such a node (`ill_typed`/`type-mismatch`). QSL FR-447 lowers `Variable(i)` to a `reference` to the variable's `value`/`parameter` node. So `x' = v` (the textbook position/velocity flow) or a reset `{ x = v }`, both admitted at S3, is emitted as a reference to a parameter node and refused. Fix: admit the model's variable `value`/`parameter` node as a rational term; add a vector with a rate that is a bare variable. | spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:56-72; spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:208-224 | wrong-requirement |
| FND-003 | high | [soundness] One mutation gets two incompatible expected results. AC-4 and HY-03 expect `variables`/`modes` swapped to reach the reader's model-shape check (`invalid_semantic_graph` at the body). AC-5 and HN-05 expect the same edit to fail the published `SemanticNode` schema, because `HybridModelBody` fixes the binding names by `prefixItems`. Strict wire validation refuses a schema failure (`refused`/`malformed_wire`) before any body step, so the AC-4 result cannot be reached. The same holds for class `continuous` (AC-4: `invalid_package`/`invalid-value` in hybrid step 2; HN-06: schema failure) and for the shape halves of checks 1, 3.1 and 3.3. Fix: decide once whether these shapes belong to the schema or to the hybrid step, and give each mutation the one result that order produces. | spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:278; spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:279; spec/test-cases/TC-966-checked-package-hybrid-model-nodes.md:41 | wrong-requirement |
| FND-004 | high | [soundness] HY-10's second case expects the wrong locus. With "every variable classed `clock`", check 5 first refuses `v` at its class literal, because `v`'s rate `-a` is not the rational one. The "no `hybrid` variable" refusal at `variables` comes second and is never reached. So "at `variables`" fails a reader that follows the stated order. Fix: change the edit so that every rate is the literal `1` (for example a one-variable model `t` classed `clock` with rate `1`), or expect the locus at `v`'s class literal. | spec/test-cases/TC-966-checked-package-hybrid-model-nodes.md:48; spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:229-233 | wrong-requirement |
| FND-005 | high | [soundness] FR-193 now gives trace correspondence to "a process provider reading the model's `hybrid` nodes". The process provider is the one that produced the trace, and `invalid_result` is a refusal of a result by whoever reads it. As written, nothing obliges the host that reads a process provider's result entry to check it. An external solver's mismatched trace would then be kept, and this contradicts QSL FR-250, which checks every trace. FR-481 "Hybrid trace correspondence" and AC-7 repeat this. Fix: the reader of the result entry (the producer, which holds the checked model and the emitted nodes) checks every `HybridTrace` whichever provider returned it; AC-7 asserts the host's check. | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:184-195; spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:260-268; spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:281 | wrong-requirement |
| FND-006 | medium | [soundness] The clock-rate check holds in the direction it checks: `clock` implies the literal `1` in every mode. The class, however, is the producer's attestation and cannot be re-derived. `t' = rational(2, 2)` (hybrid by FR-193's syntactic rule) and `t' = 1` (a clock) lower to the same literal-`1` node, so a `hybrid` variable whose every rate is that node must be admitted. That in turn means the reader cannot enforce FR-193-AC-7 against a producer that misclassifies. Check 5 cites FR-193-AC-7 as if it did, and no vector pins the admitted case, so an implementer could add the converse check and refuse valid QSL output. Fix: state that the class is producer-attested and enters identity; add an admitted vector with a `hybrid` variable whose rates are all the literal `1`. | spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:229-233 | correct-requirement-no-evidence |
| FND-007 | medium | [soundness] The spelling of a `rational` literal is defined only inside FR-481, by analogy ("the canonical magnitude spelling FR-322 gives a quantity"). FR-322 "Scalar literal values" is the rule that refuses a non-canonical spelling (`invalid_package`/`invalid-value`), and it does not cover `rational`. These are the first `rational` literals on the wire. So `2/2` or `1/1` is neither refused nor said to be or not to be "the rational one" that check 5 compares against. Fix: add `rational` to FR-322 "Scalar literal values" with its refusal and a vector (this lands next to #398's new paragraph). | spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:58-62; spec/objects/interfaces/FR-322-checked-package-artifact.md:1858-1879 | correct-requirement-no-evidence |
| FND-008 | medium | [soundness] FR-481 does not say which temporal profile a hybrid claim may use. Check 7 requires the integer interval form, but FR-370 admits integer intervals only under infinite-trace and the three bounded profiles. Under `quire.temporal.timed/v1` the temporal step, which runs before the hybrid step, refuses them `operation-member-mismatch`. Neither FR-481 nor QSL FR-447 restricts the profile, so a claim QSL admits under a timed profile is emitted and then refused. Fix: state the admitted profiles (and QSL S3's refusal for the others), or admit the timed form. | spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:246-250; spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md:189-193 | wrong-requirement |
| FND-009 | medium | [coverage] `tc_966_hybrid_model_nodes_meet_the_schema_and_carry_their_own_keys` is tagged FR-481-AC-1 and AC-6 but asserts only part of each. For AC-1 it checks the clock rate but not `Moving`'s `-a` and `0` rates, and not the transition's target, guard or reset. For AC-6 it checks over/model typing and the clock, but not the activation, the formula or the `holds` operand's read of `v`, and none of AC-6's four refusals. `tc_966_a_changed_rate_rekeys...` never checks the keys of the added `-(2 * a)` nodes. Fix: assert the rest of AC-1 and AC-6 and the keys of the extra nodes, or narrow the tags. | tests/checked_package_v2.rs:16366-16486; tests/checked_package_v2.rs:16489-16539 | correct-requirement-no-evidence |
| FND-010 | low | [test-intent] AC-4 states "`Moving` with two rates" and HY-05 tests "a fourth rate". AC-3 and HN-02 use `-(2 * a)` while QSL FR-447-AC-5 and TC-1187 use `-2 * a`, although the vectors claim to be "QSL FR-447's". Fix: use one mutation and one spelling in both repos. | spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:277-278; spec/test-cases/TC-966-checked-package-hybrid-model-nodes.md:43 | wrong-requirement |

## Verdict

Not sound as merged. Fix forward. There are five high findings:

- a scope deferral (FND-001);
- a hole that refuses valid QSL output (FND-002);
- two self-contradictory expected results (FND-003, FND-004);
- a trace check given to the wrong party (FND-005).

The node model itself is sound: tag, owners, keys, preimage, dependencies,
body grammar and step order. The hand-checked vectors match QSL FR-447. The
clock-rate check is sound one way, but it rests on a producer-attested class
that the spec should say is producer-attested (FND-006). The
`quire validate`, cargo and QSL conformance (46 passed) gates pass.
