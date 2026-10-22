---
id: SR-16186
title: "Spec review of quire-specification PR #386: STD-332, claim extent of per-step refinement query items"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@0d1128e45; PR #386 (git diff origin/main...0d1128e45): spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md, spec/objects/protocol/FR-290-protocol-claim-kind.md, spec/test-cases/TC-950-per-step-refinement-queries-and-certificates.md"
review_set: subset
evaluated_revision: "0d1128e45"
relationships:
  - target: ix://agent-ix/quire-specification/FR-468
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-290
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-950
    type: reviews
---
# SR-16186: Spec review of quire-specification PR #386: STD-332, claim extent of per-step refinement query items

## Summary

Ticket: STD-332. Reviewed `git diff origin/main...HEAD` at 0d1128e45 against QSL FR-146 at main (PR #1263's Claim extent bullets and FR-146-AC-10, TC-551 step 11), FR-290's advertised-mode table and FR-290-AC-13's domain-kind check, and FR-331's extent and `cardinality` vocabulary. The roots (receiver, result and parameter types; `_` argument domains; the context population; populations a `reaches` walks), the initial query's roots, the no-domain rules, the ambiguous-context refusal and the bounded/unbounded classification match QSL FR-146 in behaviour. 'A population a proof bound bounds adds none' correctly replaces QSL's 'bounded by FR-380' (QSpec FR-380 is another requirement). FR-290's added paragraph only says the per-step obligation takes the existing table. Ids FR-468-AC-8 and FR-290-AC-14 are unique across main and every open PR. Gates at the head: validate 86 failure lines (main 86); cargo 23 + 119 passed; no test-parsed file changes. origin/main is an ancestor; no textual conflict with the other six PRs. Lead defaults are flagged.

Method: quoin spec-review (base), with integrity, EARS, criterion-strength (by hand: no Jev client) and failure-domain checks folded in; code-review of the `tests/` change where present is folded in here. Gap-analysis does not apply (spec, vectors and trace tags only).

Units examined:

- `FR-468 Claim extent` (examined): spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md
- `FR-468 negotiation paragraph` (examined): spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md
- `FR-468-AC-8` (examined): spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md
- `FR-290 per-step paragraph` (examined): spec/objects/protocol/FR-290-protocol-claim-kind.md
- `FR-290-AC-14` (examined): spec/objects/protocol/FR-290-protocol-claim-kind.md
- `TC-950 step 8` (examined): spec/test-cases/TC-950-per-step-refinement-queries-and-certificates.md
- `FR-290-AC-13` (context_only): spec/objects/protocol/FR-290-protocol-claim-kind.md

## Verdict

**MERGEABLE after a fix**: one medium finding, one low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The negotiation vectors omit two inputs that decide the expected disposition. By FR-290's table an `unbounded` item on a bounded-only candidate settles `requires-bound` only 'when a finite bound is available', otherwise `unsupported`/`unbounded-extent`; and by FR-290-AC-13 the domain-kind check runs first, so a candidate whose `domains` lacks `population` (or, for FR-468-AC-8's `peek` variants, `integer`) settles `unsupported-requested-capability`. FR-290-AC-14, FR-468-AC-8 ('each unbounded query item settles requires-bound') and TC-950 step 8 state neither `finite_bound_available` nor the candidate's `domains`, so a correct implementation can return `unsupported` and fail the vector. State 'with a finite bound available, on a candidate whose domains include population' (and say whether the `peek` variants are negotiated). | spec/objects/protocol/FR-290-protocol-claim-kind.md:398, spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md:115, spec/test-cases/TC-950-per-step-refinement-queries-and-certificates.md:27 |
| FND-002 | low | FR-468-AC-8 has two `peek` variants, one taking `n: Integer` and one returning `Integer`, each adding one `integer` domain; TC-950 step 8's expected result ('peek's step query gains one integer domain for the parameter and one for the result') reads as one variant with two extra domains. Say 'in each variant, one integer domain', matching the AC and QSL TC-551 step 11. | spec/test-cases/TC-950-per-step-refinement-queries-and-certificates.md:31 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | see after_excerpt |
| FND-002 | fixed | see after_excerpt |

Disposition pass 1 verdict (head 8836c790): **MERGEABLE.** FND-001 and FND-002 fixed in 8836c790. Checked the new peek vectors against FR-290-AC-13: on a bounded-only candidate an unbounded item's boundable `integer` domain outside `domains` settles `unsupported-requested-capability`, and with [`population`, `integer`] and a finite bound `requires-bound`, as stated. Merges cleanly onto current main 6f24ddd4.
