---
id: SR-830
title: "Spec review — PR #172 STD-142/STD-115/STD-123 unions, case and pre-call precondition"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@b1699de7804bb1fecd4c686fd237670e8df79912; spec/objects/interfaces/FR-440, FR-441, FR-322; spec/functional/type-model/FR-143, FR-144; spec/functional/expressions/FR-146; spec/functional/foundation/FR-046; spec/objects/foundation/FR-208; spec/assurance/AD-015; proposals/quire-v1/definitions/value-accounting.md; proposals/quire-v1/state-contract.md; TC-262..265, TC-395..401; spec/complete-v1/tests.md; spec/composed-foundation/tests.md; spec/spec.md"
review_set: subset
---
# Spec review — PR #172 STD-142/STD-115/STD-123 unions, case and pre-call precondition

## Summary

Ticket: STD-142 (with STD-115 and STD-123). Spec-only PR; the review covers
`git diff origin/main...HEAD` at b1699de. Integrity, EARS and object checks are
folded into this one pass. It was checked against the Wave B/C owner rulings,
against AD-015, and against QSL-383's FRs on QSL branch spec/383-sum-types
(e6e93c7f).

What holds:

- The FR-441 member-key vectors recompute: `40a8492e…d692` (U1 `Circle`),
  `83c93448…296b` (U1 `Empty`), `611ebd3f…e75c` (U2 `Circle`), from JCS +
  SHA-256 of the stated preimage.
- `occ(Shape::Rect(2, 3)) = 3` and nullary `occ = 1` agree across FR-143,
  value-accounting.md and TC-399.
- The FR-144 union key (member identifier bytes, then payload keys) matches
  the enum row. OrderedSet keeps first-occurrence order (TC-397 Y04), and the
  IEEE-payload refusal uses the existing `ill_typed`/`operator-ineligible`.
- There are no caps, no depth limit, no pins, ledgers or compat paths, and no
  ticket ids in normative prose. The PR deletes the lock-scoping sentences
  from TC-262..265.
- Undefined, refused or incomplete scrutinee outcomes propagate, consistent
  with the QSL-366 ruling. QSL's engines map undefined to REFUTED.
- The FR-046/FR-208 pre-call observation matches STD-123 and QSL FR-106
  PreCall (pre snapshot, self, parameters; no post, result, delta or frame).
- Every new AC has a behaviour TC, and the index rows are present.
- `quire validate` on the changed spec files reports only failures that
  origin/main already has (two tests.md column asserts, FR-208, FR-322).
- `cargo test --locked --no-fail-fast` fails only the digest/lock tests
  (tc_195 x2, tc_233 lock catalog on `value-accounting.md` bytes). #174
  deletes them, so they are a merge item and not a finding.

## Verdict

**Not mergeable as it stands.** One high finding: the published V2 schema and
catalog are not updated. There are also three medium spec gaps.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-440 "Published schema" says the V2 schemas and catalog enumerate `union`, `union_value`, `case`, `arm_body`, `union_arms` and `quire.op.control.case`, but the PR does not touch `proposals/checked-package-v2/schema.json` (CompositeTypeNode/ValueNode/ExpressionNode enums lack them) or the node-identity schema, and adds no `union-nodes` fixture. FR-322-AC-17 (closed catalog) is false at HEAD, and TC-395's precondition cannot be met. Precedent #163 updated schemas, fixtures and vectors in the same change. | spec/objects/interfaces/FR-440-checked-package-union-and-case-nodes.md:94; proposals/checked-package-v2/schema.json:119-122 |
| FND-002 | medium | Arm binder levels start "one above the highest level bound at the `case` (the enclosing declaration's parameters and every enclosing binder)". QSpec defines binder levels only for state-clause binders (FR-341); function parameters and `let`/quantifier binders have none. So binder node ids, and the `case` and package ids, are not reproducible. TC-395 N03's "`s` at level 0" has no FR backing. | spec/objects/interfaces/FR-440-checked-package-union-and-case-nodes.md:71-78; spec/test-cases/TC-395-checked-package-union-and-case-nodes.md:43 |
| FND-003 | medium | "Case result type" does not say whether an arm-body type refusal precedes or follows the FR-146 exhaustiveness obligations. It checks "every arm body", including arms whose head is `unknown-member` or `arm-arity`, whose binders have no type. A `case` with an ill-typed body and a missing arm can report either `ill_typed` or `undefined_expression`. QSL FR-318 chose body-first and skips unresolved arms, and defers to QSpec where the two differ. | spec/functional/expressions/FR-146-check-total-pure-functions.md:102-126 |
| FND-004 | medium | FR-046-AC-7's refusals ("whose snapshot is a post snapshot", "omit a declared parameter") name no code or cause. FR-046 does not say how a snapshot is identified as post. TC-401 Q04/Q05 add `wrong_snapshot`/`wrong-observation` and `invalid_runtime_input`/`missing-member`, which no FR states. | spec/functional/foundation/FR-046-validate-state-invocation-inputs.md:37; spec/test-cases/TC-401-pre-call-precondition-observation.md:31-32 |
| FND-005 | low | FR-441-AC-3's "and selects no `case` arm" can never fail: the refused input never reaches a `case`. | spec/objects/interfaces/FR-441-union-member-key.md:61 |
| FND-006 | low | TC-397's Description and Expected Results say visiting order never depends on insertion order, but Y04 correctly expects OrderedSet first-occurrence (insertion) order. | spec/test-cases/TC-397-union-canonical-key.md:15-16,39 |

## Dispositions

Round 1, reviewed at 36fb68cd5eee056c8e4ea74d1e5b2302c7c1e333. No new findings.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 36fb68c — schema.json enumerates union/union_value/case, the case operator class and quire.op.control.case, and binds UnionTypeBody/UnionValueBody/CaseBody; node-identity-preimage.schema.json gains the case class, the identity and UnionMemberPreimage; fixture positive-union-nodes.json and union_member_vectors added; tc_395 x2 and tc_396 pass. The external catalog entry (quire-verification-contracts) is an overlap merge item. |
| FND-002 | fixed | 36fb68c — FR-440 "Binder levels" defines levels for function parameters, state-clause values, let, collection and arm binders, with a worked vector; FR-440-AC-5 and TC-395 N03/N08 cite it. |
| FND-003 | fixed | 36fb68c — FR-146 orders scrutinee, then resolved arm bodies, then exhaustiveness, and skips bodies of unknown-member/arm-arity arms; FR-146-AC-14 and TC-398 R08..R11 added. |
| FND-004 | fixed | 36fb68c — FR-046 defines a snapshot's role by its observation member and names wrong_snapshot/wrong-observation and invalid_runtime_input/missing-member; FR-046-AC-7 states both. |
| FND-005 | fixed | 36fb68c — FR-441-AC-3 now asserts the key changes with the member's case or the union; TC-396 K05 matches. |
| FND-006 | fixed | 36fb68c — TC-397 Description and Expected Results scope order independence to set and bag; ordered set keeps first-occurrence order. |

Round 2, reviewed at dbcb52705982f7f24cba35e52a56718131a0ab11, stacked on #170 at 650f034. No finding is still open, so this round adds no rows. New since round 1: the squash b135e53, the tc_233 closure test admitting `arm_body` (1c7e00b), and the AD-016 "Union family" section (dbcb527). The AD-016 section matches FR-143, FR-144, FR-146, FR-440 and FR-441. Every AC and TC it cites exists and says what the row says. The owner split gives the catalog entries to quire-verification-contracts, as qvc#11 implements. FR-322 on the stacked head lists `union` in `any_value` and `structural_kind` and `temporal` in `any_term`. `cargo test --locked --no-fail-fast` with an uncommitted path `[patch]` to qvc#11 at 02cb972 exits 0 (lib 0, model 4, v2 59, member 10, shared 7). `quire validate` on the changed spec files reports only the object-file schema failures main already has. No new findings.
