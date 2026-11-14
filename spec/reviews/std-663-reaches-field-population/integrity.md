---
id: SR-202611141351002
title: "Integrity review of the STD-663 reaches_field population link"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification; PR #652 at 1a2b4659: FR-322 Population link of a reaches (lines 2668-2714), Reaches over a field (2629-2666, context), FR-322-AC-73 (context), FR-322-AC-398"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-322, type: references }
---

## Summary

Ticket: STD-663. This review checks the amended section against "Reaches over a field" and against
the relationship-end rules it extends. The owner rule and the refusals are consistent. Some wording
inherited from the relationship-end-only text was not rescoped. The new ordering rule ("fails steps
1 to 4 ... before the `population` check") has no criterion.

## Verdict

CONDITIONAL. The rule that `T` is the field's owner agrees with steps 2 and 3. Step 3 forces the
field's reference target to equal its owner, so "owner" and "reference target" name one
declaration, matching QSL FR-380. The dependencies rule, the absent-not-null rule and the
structural-path rule are member-neutral and apply unchanged. The navigate refusal and the
field-projection schema refusal are stated in both the section and AC-398. The schema refusal
requires the `OperationMember` field branch to be conditioned on the operation identity; see base
FND-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Locus ordering for `reaches_field` is under-specified and untested. The retained sentence "The check follows the destination-type check of FR-322-AC-57 in that order" is unscoped, and a `reaches_field` field member has no `destination_type`. The new sentence "whose field member fails steps 1 to 4 refuses ... before the `population` check" attributes steps 1 and 4, which are operand checks, to the field member. FR-322-AC-398 has no case for the ordering, for example a set-typed field edge that carries a valid `population`, which must refuse at the application and not at `member.population`. | spec/objects/interfaces/FR-322-checked-package-artifact.md:2695-2702,4560 |
| FND-002 | low | The first rule is unscoped: "`T` SHALL be the declaration of the type of the end the member's role names". A `field` member has no role. The next sentence overrides it with "`T` SHALL be the declaration of the object type `T` that owns the field", which uses `T` both for the population's model declaration and for the owner. A reader can follow it, but scoping the first rule to `relationship_end` and naming the owner differently would remove the circular wording. | spec/objects/interfaces/FR-322-checked-package-artifact.md:2683-2694 |
