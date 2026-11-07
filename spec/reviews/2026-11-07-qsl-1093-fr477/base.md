---
id: SR-20261107109302
title: "Base review of the QSL-1093 grammar-legal step-relation body"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification branch spec/qsl-1093-fr477, diff origin/main...HEAD: FR-477 (Properties body, Code claim request, Reader order, pointers, AC-1, AC-2, AC-4 to AC-6), TC-962 RN-01, RN-02, RN-07, RN-11, RN-14, RN-15; read against FR-322 Body grammar, FR-402 (QSL-1093 fix f96cb5c1), FR-397 and FR-485"
review_set: base
relationships:
  - target: ix://agent-ix/quire-specification/FR-477
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-962
    type: reviews
---

## Summary

Ticket QSL-1093, closing FND-005 of the FR-402 review. This review covers
only the committed diff `origin/main...HEAD` of branch
`spec/qsl-1093-fr477`. It uses the base checklist and runs no analysis
lenses.

What holds:

- **Grammar.** Every body member is a FR-322 Member.
  - `subjects` is a binding to a Group of `binding(alias, reference)`.
  - `executions` is a binding to a Tuple of execution Groups, whose members
    are `binding`s of Leaves: `variable` and `alias` are literals, and
    `operation` and `implementation` are references.
  - `parameters`, `preconditions`, `postconditions` and `invariants` are
    bindings to Tuples of Groups of `reference` Leaves. An empty
    precondition Group is an empty aggregate, which is a legal Tuple member.
  - `body` is a binding of a reference.
  - The deepest chain is Member binding, Tuple, Group, binding, Leaf, which
    is the limit FR-322 allows. This mirrors FR-402's quantifier Groups.
- **Pointers.** The body members are indexed 0 to 6 as `subjects`,
  `executions`, `parameters`, `preconditions`, `postconditions`,
  `invariants` and `body`. So `B/members/1/value/members/{k}`,
  `B/members/2..4/value/members/{k}` and `B/members/5/value/members/{j}` are
  correct. AC-5 and RN-14's `B/members/2/value/members/1` is `y`'s
  `parameters` Group, which is correct.
- **Old shape refused.** An execution Group that nests a `parameters`
  aggregate holds a `binding` whose value is not a Leaf. It is then not a
  Group, so the enclosing Tuple is malformed, and AC-6 and RN-15 correctly
  expect `malformed_wire`.
- **Consumers.** FR-485 ("every execution has an implementation binding")
  and FR-397 "Code claim" agree with the optional `implementation` binding.
  No other spec reads the old six-binding `executions` entry.
- **Ceremony.** No digest, pin, SHA or tracking record was added.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | `implementation` is now optional ("absent when no relation binds the operation"), but no reader check ties its absence to the package. A producer, or a forged package, can omit `implementation` from an execution Group while the package holds a `correspondence`/`abstraction_relation` node with a `frames` entry keyed by that operation. The reader admits it, and "Code claim request" then drops the code claim without a refusal. The old AC-4 case ("an `executions` entry missing `implementation` refuses") was removed and nothing replaced it. Fix: add to step 4 (Premises) that an execution Group holds `implementation` exactly when the package holds an abstraction relation whose `frames` hold an entry keyed by its operation, else `invalid_model_binding`/`malformed-declaration` at the execution Group. Add that case to FR-477-AC-4 and a TC-962 row (RN-01 with `x`'s `implementation` binding removed). | spec/objects/interfaces/FR-477-checked-package-step-relation-claim-body.md:61-65,120-124,164-170,188; spec/test-cases/TC-962-checked-package-step-relation-node.md:39 | correct-requirement-no-evidence |
| FND-002 | low | For the count checks, step 1 gives no exact locus. When `parameters`, `preconditions` or `postconditions` hold a different number of Groups than `executions`, or `invariants` a different number than `subjects`, the refusal is "at the first member that breaks the layout". With a missing Group there is no member at the missing index. RN-07 says "at `parameters`". Fix: state that a count mismatch refuses at the Tuple's binding, `B/members/{i}`. Give that pointer in RN-07's second case. | spec/objects/interfaces/FR-477-checked-package-step-relation-claim-body.md:144-151; spec/test-cases/TC-962-checked-package-step-relation-node.md:39 | wrong-requirement |
| FND-003 | low | Step 1 leaves two parts of the body unconstrained. It does not require each `subjects` member to be a `binding(alias, reference)`: a Group also admits a bare Leaf, and step 2 assumes bindings. It also does not check the new `variable` binding: two execution Groups with one `variable`, or a non-text `variable` literal, pass every step. Fix: add to step 1 that `subjects` members are each `binding(alias, reference)` and that each `variable` is a text literal distinct from every other execution Group's, else `invalid_semantic_graph`. Add one AC-4 case and one RN row. | spec/objects/interfaces/FR-477-checked-package-step-relation-claim-body.md:46-55,144-151 | wrong-requirement |

## Verdict

The new body is grammar-legal under FR-322 and consistent with FR-402's
QSL-1093 shape. The member order, reader order, JSON pointers, the ACs and
the TC-962 vectors agree. FND-001 is a real detection gap, opened by making
`implementation` optional. Fix it in this PR. FND-002 and FND-003 are
low-severity tightenings.
