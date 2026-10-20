---
id: SR-13419
title: "STD-113 quire-specification PR 296 EARS review of the Quantity type reference"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@f78981550 vs refs/remotes/origin/main 7c66d67; spec/functional/type-model/FR-142-evaluate-quantities-and-units.md, spec/objects/interfaces/FR-322-checked-package-artifact.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-142
    type: reviews
---

## Summary

Ticket: STD-113. EARS requirement-grammar review of the new FR-142 "Quantity type reference"
statements and the FR-322 forwarding sentence. The missing-name bullet is a well-formed
unwanted-behaviour statement (`If ..., then the checker SHALL refuse ...`). The other SHALL
bullets are ubiquitous, but their subject is a type, a literal or a parameter rather than the
checker. `quire validate` reports no grammar warning on FR-142 at head.

## Examined scope

- FR-142#quantity-type-reference bullet 1 (examined): The type `Quantity<u>` SHALL denote the quantities whose unit is `u` ...
- FR-142#quantity-type-reference bullet 2 (examined): A quantity literal `n u` SHALL have type `Quantity<u>`, and an expression of type `Quantity<u>` SHALL be accepted where `Quantity<u>` is required. An expression of any other type ... SHALL refuse `ill_typed`/`type-mismatch` at the expression ...
- FR-142#quantity-type-reference bullet 3 (examined): If the qualified name of `Quantity<u>` names no unit visible to the unit of compilation (a dimension name included), then the checker SHALL refuse `missing_declaration`/`missing-name` at the name.
- FR-142#quantity-type-reference bullet 4 (examined): A parameter, field or result of type `Quantity<u>` SHALL admit exactly the values above and no bound on the magnitude ...
- FR-322#derived-type-forms-and-forwarding (examined): A `Quantity<u>` annotation likewise introduces no node ... (descriptive, no SHALL)

## Verdict

**PASS with one low finding.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Bullets 1, 2 and 4 put the obligation on a type, a literal, an expression or a parameter ("The type `Quantity<u>` SHALL denote", "A quantity literal ... SHALL have type", "An expression ... SHALL refuse", "A parameter ... SHALL admit"), not on the system that must act. Bullet 2 also joins three obligations. Recast them on the checker, as bullet 3 already does, for example "The checker SHALL type a quantity literal `n u` as `Quantity<u>`" and "When an expression whose type is not `Quantity<u>` stands where `Quantity<u>` is required, the checker SHALL refuse `ill_typed`/`type-mismatch` at the expression". | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:161-178 |
