---
id: SR-12894
title: "Integrity review of STD-175 zero-instance hyperproperty items (QSpec PR #281)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@1d130b62a9b40320f97c76d09017b967e1002157; PR #281 diff against origin/main 4f65ed2: FR-395, FR-399, TC-340; cross-checked against FR-361, FR-391, FR-396, FR-331, FR-363, TC-337 and QSL PR #987"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-399
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-395
    type: reviews
---
# SR-12894: Integrity review of STD-175 (QSpec PR #281)

## Summary

Ticket: STD-175. This pass checks consistency across QSpec and with QSL
PR #987.

The outcome matches QSL FR-182 V-6 and FR-182-AC-9. The FR-399 verdict-table
row is the only hyper enumeration of `no-initial-state`, and it gains
`no-instance`. HP-5 items settle by FR-396 through FR-391's rows, which
already carry `no-instance`. FR-363, FR-331 and FR-391 already list the
cause. No enumeration was missed. FR-395 states the per-key instance and the
product over aliases (any empty factor gives none) in the same words as QSL
FR-172.

## Verdict

One medium finding: the product case is stated but not tested. There is one
low finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-395 now says a clause over several aliases has zero instances when any one parameter ranges over an empty universe. That is the product rule. FR-399-AC-11 and VS-05 use only the single-alias HB-01. An analyzer that takes the first parameter's universe, or the union of the universes, passes VS-05 and still explores HB-02 (`d: S::Device, e: N::Device`) with `N`'s universe empty. Add a two-alias case (HB-02 with one empty factor) to AC-11 and VS-05. | spec/functional/analysis/FR-395-analyze-hyperproperties-over-behaviours.md:68-75; spec/functional/analysis/FR-399-settle-hyperproperty-results.md:178; spec/test-cases/TC-340-hyper-vacuity-and-stutter.md:30 |
| FND-002 | low | The new FR-399 paragraph links to FR-361 Instancing as normative, but FR-399's `relationships` has no FR-361 edge (FR-395 has one). | spec/functional/analysis/FR-399-settle-hyperproperty-results.md:89-90 |

## Dispositions

Round 1, reviewed at 7ab523498b59f4e8fb5a5fade21a1c46b9a8bb82. Hand check of
the new VS-05 two-alias rows (HB-02, TC-337:29): with `S::Device` empty and
`N::Device` of 3 keys, an engine that reads only the first parameter finds
no instance and passes. With `N::Device` empty and `S::Device` of 2 keys, the
same engine finds 2 instances, explores, and fails. An engine that takes the
union of the universes fails both rows.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
