---
id: SR-202611101500011
title: "Failure-domain review of the STD-660 operation_bodies carrier"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification; PR #651 at 6ff1281f: FR-322 Operation-body carrier reader joins, FR-322-AC-397, operation-body-vectors.json; context QSL FR-576 and FR-094 on quire-spec-language main"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-322, type: references }
---

## Summary

Ticket: STD-660. The reader checks that `body.node` exists, is tagged `function` and is not named by
two rows. It never checks that the node is the body of the row's own operation. A package that
gives an operation the body of another operation is admitted. So is one that gives it a clause of
the same operation other than its body.

## Verdict

FAIL on FND-001. The defect was reproduced with a probe vector, applied locally to the review checkout and
then reverted: swapping the body nodes of OBC-04 rows 0 (`acme/billing` Ledger.settle) and 2
(`acme/orders` Order.settle) makes `read_operation_bodies` return `Ok(())`. Each node is a
`function`, and each is named once. Because neither operation is a dispatch candidate, the candidate
check does not fire. QSL FR-576 forbids substituting the body of another operation, and its AC-9
requires that the node of a row yields that operation's body. Under QSL FR-094, every clause
function carries the `ModelOwner` of its operation member as `owner` and a `clause` binding of
`"body"`, and FR-322 carries `owner` on `function` nodes. The join can therefore be made from node
data, without the path convention that the owner's decision (3) excludes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The reader admits a `body.node` that is not the body of the row's operation. It checks only that the node exists, is tagged `function` and is not shared. Three cases pass: two rows with swapped bodies, a row that names its own operation's precondition clause function (same owner, ordinal 0), and a body node in another document's package. Reproduced: swapping the OBC-04 row 0 and row 2 nodes reads `Ok`. Fix: require that the node's `owner` is the `ModelOwner` of the row's operation member (`document`, member `<declaring owner node>/<operation>`, as `operation_effects` `provenance.member` states it) and that its `clause` binding is `"body"`, then add OBR vectors for a swap, a precondition-clause node and a foreign-package node. | spec/objects/interfaces/FR-322-checked-package-artifact.md:3964-3968; tests/checked_package_v2.rs:22597-22608 |
| FND-002 | low | A `no_body` row whose operation is the `selected_operation` of a dispatch candidate has no `body.node`, so "its `operation_body` equals `body.node`" does not say what happens. The Rust reader refuses `invalid_package`/`invalid-value` at `body` (`agrees = has_body && ...`), but the prose leaves that implicit and no vector covers it. Fix: state that a candidate for a `no_body` operation refuses at `body`, and add the vector. | spec/objects/interfaces/FR-322-checked-package-artifact.md:3967-3970; tests/checked_package_v2.rs:22609-22617 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | No vector exercises the `owner.identity` clause of the new own-body rule. A mutation that removes the clause `entry["owner"]["identity"] != row["document"]` from `read_operation_bodies` still passes all 20 reader refusals. OBR-19, the other-package vector, is refused by the `owner.node` clause instead, because the node IRI `ix://<document>/...` already encodes the document. Fix: either add a vector whose owner `node` matches and whose `identity` does not, or state that `node` determines `identity` and drop the clause. | spec/objects/interfaces/FR-322-checked-package-artifact.md:3966-3970; tests/checked_package_v2.rs:22644-22647 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At 498a3593, `body.node` must have an `owner` that is the `ModelOwner` with `identity` equal to the row `document` and `node` equal to `<declaring owner node>/<operation>`, and a clause of `body`. Otherwise it refuses `invalid_package`/`invalid-value` at `body/node`. New vectors OBR-17 (another operation's body), OBR-18 (a precondition clause) and OBR-19 (another package's node) cover the cases. My swap probes, applied locally and then reverted, now all refuse at the expected locus: the cross-document swap (OBC-04 rows 0 and 2), a same-type swap (Order.settle and Order.touch), one node shared within a type, and one node shared across documents. Removing the `owner.node` clause or the `clause` clause each fails the test. |
| FND-002 | fixed | The section now states that a candidate's operation needs a `function` row whose node equals `operation_body`, so a `no_body` row for that operation refuses at `body`. OBR-20 covers it: a candidate is added for Invoice.settle, which has no body. A mutation that lets `no_body` pass the candidate check fails on OBR-20. |
