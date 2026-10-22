---
id: SR-16080
title: "Spec review — vacuous-match means no matched fair universal tuple established (FR-399)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; spec/functional/analysis/FR-399-settle-hyperproperty-results.md, spec/test-cases/TC-340-hyper-vacuity-and-stutter.md at f817fb9"
review_set: subset
evaluated_revision: "f817fb981610c14ef81af4e26dc26aefc7e257e6"
relationships:
  - target: ix://agent-ix/quire-specification/FR-399
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-340
    type: reviews
---

## Summary

Ticket: STD-313. PR quire-specification#379 changes FR-399's `vacuous-match`
row to mean "no matched fair universal tuple established for a non-empty
`μ_U`". That covers a vacuous clause and an inductive HP-2 proof for an
instance with no matched tuple established. The inductive-proof row now proves
only with an empty `μ_U` or an established matched tuple. The PR also amends
FR-399-AC-5 and TC-340 VS-01. Checked against QSL main 39823fe: FR-182 (V-3,
V-6, the ordered inductive rule, AC-7, AC-10), FR-183, and ADR-023 HM-7, HV-4
and QS-5 as merged.

## Verdict

Mergeable after the major finding is addressed. The meaning matches QSL. HV-4
now defines `VacuousMatch` as "no matched fair universal tuple has been
established for a non-empty `μ_U`", which covers the HM-7 case and an EN-3
proof with no accepted witness. FR-182 V-3 and V-6 split the inductive outcome
the same way. The wire token is unchanged, and ADR-023 QS-5, a mapping row,
needs no change. The empty-`μ_U` case proves without a witness in both specs.
No ids are added, and no other open QSpec PR touches FR-399 or TC-340. Gates:
`quire validate` gives 86 failure lines with no FR-399 or TC-340 diagnostics,
and `cargo test --test checked_package_v2 --test backend_provider_v1` passes
(23 + 119).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | "Established" is the deciding condition in both amended rows, but QSpec never says what establishes a matched fair universal tuple. QSL fixes it as an `Accepted` `MatchWitnessCheck` of a fair `μ_U`-matched lasso tuple for the outcome's instance that replays (FR-182, FR-183), and treats a `Refused` check or a witness for another instance as none. Without this in FR-399, two engines can read "established" differently, and an engine could settle `proved` from an inductive proof with no checkable evidence. | spec/functional/analysis/FR-399-settle-hyperproperty-results.md:77, spec/functional/analysis/FR-399-settle-hyperproperty-results.md:82 |
| FND-002 | medium | VS-01's second and third runs do not give the input that separates them. "with no matched fair universal tuple of the instance established" and "with a matched tuple of that instance established" name no witness tuple and no way to supply or check one. So the `proved` run cannot be built from the vector, and nothing can fail on a wrong witness. QSL FR-182-AC-10 uses FR-183-AC-8's matched and unmatched witnesses and adds an Accepted witness for another instance; VS-01 has no such case. | spec/test-cases/TC-340-hyper-vacuity-and-stutter.md:26 |
| FND-003 | low | The amended inductive-proof row lists its preconditions (`μ_U` empty or a matched tuple established) but leaves out "every subject has an initial state". It also states no precedence against the `no-initial-state` row, while QSL FR-182 checks `NoInitialState` first "whatever witness comes with it". As written, an empty-`μ_U` inductive proof over a subject with no initial state matches both the `proved` row and the `no-initial-state` row. | spec/functional/analysis/FR-399-settle-hyperproperty-results.md:77, spec/functional/analysis/FR-399-settle-hyperproperty-results.md:83 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | VS-01 (b)-(e) run on the secure vault, which TC-337 declares with population universe `{v}` and the single operation `step`. The vectors use instances `v1` and `v2`, which that universe does not have, and (c) uses a lasso step `mix`, an operation only TC-344's busy/mix vault declares. As written, (b)-(e) cannot be built. They need a secure-vault variant with a two-key universe (as QSL FR-182-AC-10 uses `{v, w}`), and (c) needs a mismatch the secure vault can produce, for example a second lasso whose `step` argument `i` differs from the first's. | spec/test-cases/TC-340-hyper-vacuity-and-stutter.md:26, spec/test-cases/TC-337-hyperproperty-syntax-and-binding.md:22 |

## Dispositions

Round 1, against head aedc1063f6057846f31e408a81981720653ce851.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |

Round 2, against head 32352ee4b9f5e6a047e58d2e7768c89ccf44f064.

| FND | outcome | reason |
| --- | --- | --- |
| FND-004 | fixed | - |
