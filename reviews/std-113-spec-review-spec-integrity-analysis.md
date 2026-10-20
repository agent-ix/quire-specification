---
id: SR-13420
title: "STD-113 quire-specification PR 296 integrity review of the Quantity type reference"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@f78981550 vs refs/remotes/origin/main 7c66d67; spec/functional/type-model/FR-142-evaluate-quantities-and-units.md, spec/test-cases/TC-187-quantity-and-unit-conversion.md, spec/test-cases/TC-350-probabilistic-declarations.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-142
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-187
    type: reviews
---

## Summary

Ticket: STD-113. Integrity review: consistency of the new rule with the rest of the corpus,
traceability of AC-11/AC-12 to TC-187 U30/U31, and system-not-build-log content. AC-11 maps
to U30 and AC-12 to U31, each row naming its AC. Both ACs cite `Test (TC-187)`, and TC-187
already `verifies` FR-142. A repository-wide search for `Quantity<` finds one other use, TC-350,
and it now contradicts FR-142.

## Examined scope

- TC-187 U30 (examined): Check `d: Quantity<ms>` with the literals `3 ms` and `3 s`, then `d: Quantity<T>` and `d: Quantity<nothing>` (FR-142-AC-11)
- TC-187 U31 (examined): Check `f(d: Quantity<ms>): Quantity<ms>` returning `d`, then returning `d` converted to `s` (FR-142-AC-12)
- TC-187 Test Procedure (examined): defines `T` with canonical root `s` and no other time unit
- TC-350 PD-01, PD-03 (examined): `... on a Quantity<Time> parameter`
- FR-142#quantity-type-reference (examined)

## Verdict

**Changes needed: one medium and two low findings.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-350 PD-01 and PD-03 still type a parameter `Quantity<Time>`, a type over the dimension `Time`. FR-142's new bullet says `Quantity<D>` over a dimension "is not a type form", and AC-11 refuses it `missing_declaration`/`missing-name`. The repo now holds two contradictory vectors. Update PD-01/PD-03 to declare a unit of `Time` and write `Quantity<ms>`. | spec/test-cases/TC-350-probabilistic-declarations.md:26,28; spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:179-181 |
| FND-002 | low | TC-187's Test Procedure declares time dimension `T` with root `s` only, yet U30 and U31 use `ms`. AC-11 declares `ms` (target `s`, scale `1/1000`) but the TC does not, so a test author has to copy it in from the FR. Add `ms` to the procedure's unit list. | spec/test-cases/TC-187-quantity-and-unit-conversion.md:19-33,69-70 |
| FND-003 | low | Normative FR text carries build-log and provenance phrasing. "no checked-package wire form, schema or vector changes" describes this PR, not the system. "the prose `Quantity<Time>` of ADR-024 §7.2" cites a planner's prose through an unqualified ADR id, but ADR-024 is QSL's: QSpec has no ADR-024, and other QSpec files say "QSL ADR-024". Describe the behaviour only, for example "its key is the unit's declaration key", and qualify or drop the citation. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:164-166,179-181 |
