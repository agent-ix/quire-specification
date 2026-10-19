---
id: SR-12893
title: "Base spec review of STD-175 zero-instance hyperproperty items (QSpec PR #281)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@1d130b62a9b40320f97c76d09017b967e1002157; PR #281 diff against origin/main 4f65ed2: FR-395, FR-399, TC-340"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-399
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-395
    type: reviews
---
# SR-12893: Base spec review of STD-175 (QSpec PR #281)

## Summary

Ticket: STD-175. Review set: base, integrity (SR-12894) and
failure-domain (SR-12895). The paired QSL PR #987 is reviewed in SR-12890
to SR-12892.

The PR applies the planner default. A hyper item with zero instances settles
`inconclusive`, `unsettled`, `no-instance` before any state is explored, and
`requires-bound` is classified first. FR-399 states that order as a SHALL,
matching FR-361 Instancing and FR-361-AC-8. FR-331 already carries the wire
cause `no-instance`. FR-399-AC-11 and VS-05 are free ids on main and in open
PRs #273 and #200, and neither of those touches FR-395, FR-399 or TC-340.
`quire validate` exits 0 on the three changed files.

## Verdict

No high findings. There are two low findings here. The medium findings are
in SR-12894 and SR-12895.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The VS-05 row has four cells (a trailing `FR-399-AC-11`) in a three-column table, Vector, Input and Expected result. GFM drops the extra cell, so the AC tag never renders, and no other row carries one. Also, HB-01 already has `behaviours (v: V::Vault)` (TC-337:28), so "HB-01 with `behaviours (v: V::Vault)`" restates it. | spec/test-cases/TC-340-hyper-vacuity-and-stutter.md:30 |
| FND-002 | low | "has one instance per universe key and so none" reads as a contradiction. Say "has one instance per key of the product of its parameters' universes, so here none". | spec/functional/analysis/FR-399-settle-hyperproperty-results.md:86-88 |
