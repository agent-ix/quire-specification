---
id: SR-9135
title: "Integrity review of the TL-46 rulings: remaining incomplete labels and stale FR-341 links"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification; PR #650 diff origin/main...0403e284 plus remaining `incomplete` uses: FR-385, FR-411, FR-193, FR-483, FR-488, FR-380, FR-360, TC-315, TC-983 (tickets QSL-1869, QSL-1873)"
review_set: subset
review_date: "2026-11-12"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-360, type: references }
  - { target: ix://agent-ix/quire-specification/FR-385, type: references }
  - { target: ix://agent-ix/quire-specification/FR-488, type: references }
  - { target: ix://agent-ix/quire-specification/FR-489, type: references }
---

## Summary

REVISE. Tickets: QSL-1869, QSL-1873. Ruling QSL-1869-2 says `incomplete` is
an FR-331 result kind and never an FR-360 label. Inside the diff it is applied
consistently. Outside it, two sibling verdict tables still put `incomplete` in
an FR-360 (or stale FR-341) label column, one of them the direct twin of the
FR-489 table this PR amended. FR-411, FR-193 and FR-483 use `incomplete` in an
FR-331 result column and are correct under the ruling. TC-315 case 9(g) reads
`incomplete`, `resource-incomplete`, which is the FR-331 kind plus FR-241 value
and stays consistent.

## Review set

`grep` over `spec/` for `` `incomplete` `` in table cells and for every table
whose header names an FR-360 or FR-341 label column (FR-363, FR-379, FR-380,
FR-385, FR-391, FR-399, FR-488, FR-489).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-488's Settlement table, headed "FR-360 label", still has "Work or resource bound reached \| `incomplete` \| `unavailable`", and FR-488-AC-10 says an exhausted bound "settles `incomplete`". This is the same row the PR rewrote in FR-489 and contradicts ruling QSL-1869-2. Fix: amend the row and AC-10 to FR-360 `failed`, FR-241 `resource-incomplete` (FR-331 result `incomplete`), as FR-489. | spec/functional/ir-proof/FR-488-attach-a-temporal-guarantee-to-implementation-rust.md:131; spec/functional/ir-proof/FR-488-attach-a-temporal-guarantee-to-implementation-rust.md:169 |
| FND-002 | medium | FR-385's reduced-verdict table (header "FR-341 label", a stale id for FR-360) has the budget row label `incomplete`, and FR-385-AC-9 says the run "settles `incomplete` ... and never `failed`", which under the ruling is wrong: the FR-360 label is `failed`. Fix: repoint FR-385's FR-341 links to FR-360, change the row to `failed` with execution `resource-incomplete` (FR-331 `incomplete`), and reword AC-9 to "never execution `failed`" as FR-377-AC-13 now does. | spec/functional/analysis/FR-385-report-reduced-verdicts.md:81; spec/functional/analysis/FR-385-report-reduced-verdicts.md:91; spec/functional/analysis/FR-385-report-reduced-verdicts.md:127 |
| FND-003 | low | FR-360's Description and Scope cover infinite-trace, state-graph and model-subject hyperproperty results only, yet FR-177, FR-379, FR-385, FR-488 and FR-489 read their outcomes through it. The strict-reader SHALL does not formally bind refinement, reduction or composition results. Fix: extend FR-360 Scope to name refinement (FR-379, FR-177), reduction (FR-385) and composition (FR-488, FR-489) results. | spec/objects/temporal/FR-360-infinite-trace-result-disposition.md:38 |
| FND-004 | low | FR-380's settlement table header and links still name "FR-341 label" with path `objects/temporal/FR-341-infinite-trace-result-disposition.md`, which does not exist (FR-341 is the checked-package state-clause body). Same stale link the PR fixed in FR-379. Fix: repoint to FR-360. | spec/functional/analysis/FR-380-transfer-claims-through-refinement.md:104 |
| FND-005 | low | TC-983 step 5 expected result reads bare `incomplete` and "five distinct outcomes", while the amended FR-489-AC-9 says FR-360 `failed`, FR-241 `resource-incomplete`, and "four distinct outcomes". Fix: state the stopped outcome in FR-360/FR-241 terms and reconcile the count. | spec/test-cases/TC-983-composition-carries-liveness-and-fairness-premises.md:48 |

## Verdict

REVISE. The ruling itself is sound; the sweep is incomplete. FND-001 and
FND-002 leave the spec saying both "`incomplete` is not an FR-360 label" and
"FR-360 label `incomplete`". These can be fixed in this PR or deferred to a
follow-up ticket; FND-001 is the stronger case for this PR because quire-protocol
FR-202 reads FR-488/FR-489 composition labels through FR-360.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | FR-488 table row and FR-488-AC-10 now read FR-360 `failed`, FR-241 `resource-incomplete` (FR-331 result `incomplete`) (head 0fed3afc). |
| FND-002 | fixed | FR-385 links and table header repointed to FR-360; budget row reads `failed` (FR-331 result `incomplete`); AC-9 reads FR-360 `failed` and "never execution `failed`". |
| FND-003 | fixed | FR-360 Scope now names refinement (FR-379, FR-177), reduced-run (FR-385) and composition and guarantee (FR-488, FR-489) results. |
| FND-004 | fixed | FR-380 relationship, prose link, table header and dependency link repointed from FR-341 to FR-360. |
| FND-005 | fixed | TC-983 step 5 expects FR-360 `failed` with FR-241 `resource-incomplete` (FR-331 `incomplete`) naming the limit, and drops the "five" count for "the outcomes stay distinct". |
