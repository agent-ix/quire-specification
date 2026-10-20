---
id: SR-13500
title: "base review of STD-220 typed as-bindings in the TC-376/380/381/384 protocol fixtures (PR #297)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@fae947fd4d5b25849cbe4166f99623c22babeb7a; spec/test-cases/TC-376-two-branch-join-worked-example.md, spec/test-cases/TC-380-compensation-worked-example.md, spec/test-cases/TC-381-per-trigger-activation-and-instance-budget.md, spec/test-cases/TC-384-scheduler-fairness-and-adversarial-scheduling.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/TC-376
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-380
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-381
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-384
    type: reviews
---
# SR-13500: base review of STD-220 (PR #297)

## Summary

Ticket: STD-220. Base checklist over
`git diff refs/remotes/origin/main...fae947fd4` (4 files, +19/-11). The diff
types every `as (x)` binder in the TC-376, TC-380, TC-381 and TC-384 protocol
fixtures, renames TC-384's attempt node `Set` to `Raise`, and adds to each TC
a step that parses the fixture at S1. Spec-only diff, so code-review and
gap-analysis do not apply. No requirement statement changed, so no EARS
sub-analysis ran.

Checks made by hand, with my own probes (scratch, uncommitted):

- S1. A `qsl_cst::parse` probe (QSL main 469f8b49c) parsed every fenced
  protocol fixture under `spec/test-cases/`, each under a header
  `profile p = "quire.value.complete/v1";`. That is five fixtures: TC-310,
  TC-376, TC-380, TC-381 and TC-384. On main, TC-376/380/381/384 refuse
  `invalid_syntax`/unexpected token and TC-310 is admissible. At the PR head
  all five are admissible with no diagnostic. A control, the TC-384 head
  fixture with `Raise` put back to `Set`, refuses at `Set`, so the rename is
  needed.
- Types. Each binder takes the fixture's own model object type: `M::Cell`
  (TC-376), `M::Acct` (TC-380), `M::Gate` (TC-381) and `M::Flag` (TC-384).
  This is the type of `over (k: …)` and of the attempts' receiver, and every
  one is declared in the TC's model prose. It follows the TC-310 precedent
  (`as (a: R::Ring)`). Binders that were typed already (`M::ChargeEffect`,
  `M::Failure`, `M::Ping` and the compensation binders) are unchanged.
- Behaviour. No vector's expected result changed except SF-06's node name.
  Universes are single-object, so an object-typed binder holds one value and
  no state count or step set moves (FL-01, CP-01, IB-01..07, SF-01..09).
- Rename sweep. `grep` over the repository finds no other `quiet::Set` or
  `Starve::…::Set`. TC-387 RP-08 cites TC-384 by vector id only. One prose
  use of `Set` was missed (FND-001).
- Gates. `quire validate` on the four touched files exits 0, the same as on
  main, with no warning on a touched file. `cargo test --test
  checked_package_v2 --test backend_provider_v1` passes (11 + 109).
  `quire matrix --format tsv` is byte-identical to main (2,301 rows). Tools:
  quire 0.36.2 (engine 0.50.2), quoin 0.28.3.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-384 SF-01 still names the renamed node: "the duplicate/lose loop starves `quiet` while `Set` stays enabled and is unfair". After the rename there is no node `Set`, and the procedure prose and SF-06 say `Raise`. The rename is incomplete, and the vector now names a node the fixture does not declare. Change it to `Raise`. | spec/test-cases/TC-384-scheduler-fairness-and-adversarial-scheduling.md:51 |
| FND-002 | low | The added step ends "(every protocol fixture of the test cases parses)". That claim is not true of the corpus, and one TC cannot verify it. TC-946's run fragment keeps untyped binders (`as (sent)`, `as (got)`, `as (charge)`, `as (closed)`), and so does TC-947 vector FN1 (`finish Closed as (closed)`). Drop the parenthetical, or narrow it to this TC's fixture. | spec/test-cases/TC-376-two-branch-join-worked-example.md:46; spec/test-cases/TC-380-compensation-worked-example.md:55; spec/test-cases/TC-381-per-trigger-activation-and-instance-budget.md:41; spec/test-cases/TC-384-scheduler-fairness-and-adversarial-scheduling.md:45 |
| FND-003 | low | TC-380's model still says "The effect and finish record types have one value each", but the finish binder is now `M::Acct`, the object type, not a finish record type. The choreography surface types finish and attempt binders with their own record types (`M::OrderClosed`, `M::ChargeAttempt`). Under a single-object universe `M::Acct` has one value, so CP-01's nine states hold. Read as a record of Acct's fields, though, it has two (`bal` 0/1), and t5 and t8 would split. Either declare a one-value finish record type in the model, or reword the sentence to name `M::Acct`. | spec/test-cases/TC-380-compensation-worked-example.md:29; spec/test-cases/TC-380-compensation-worked-example.md:48 |

## Verdict

Not mergeable as is: one medium finding (FND-001) and two low ones. The
binder typing is correct. All four fixtures parse at S1, where every one
refused on main. The types are the fixtures' own model object types, and no
vector's meaning changed. FND-001 is a one-word fix. FND-002 and FND-003 are
one-line wording fixes, or can be deferred.

Examined and clean: the four rewritten fixtures, TC-384 procedure prose and
SF-06, and the added step's parse instruction itself. Also clean: TC-310's
fixture, which parses on main and at the head.
