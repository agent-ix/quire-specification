---
id: SR-85910
title: "Spec review of quire-specification PR #455 (STD-504): FR-154 unit bounds form a bounded quantity type"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@5606957fd17a1011982356e2ee6c203793c497e6; PR #455 diff against main b30710e3; FR-154 Unit bounds, FR-154-AC-28, FR-154-AC-31, FR-142 Quantity type reference, FR-322 step 4 unit row, TC-234 I22 and I24; checked against FR-146, FR-272, TC-280 and quire-spec-language FR-230-AC-26 and TC-685 step 25"
review_set: subset
---
# Spec review of quire-specification PR #455

## Summary

Ticket: STD-504. The PR makes a `min`/`max` unit bound form the bounded quantity type `Quantity<u>[min, max]`, a subtype of `Quantity<u>`; an out-of-range quantity literal refuses `ill_typed`/`type-mismatch`; the runtime invariant stays. It amends FR-154 "Unit bounds", adds FR-154-AC-31, rewords FR-154-AC-28, adds TC-234 I24, adjusts I22, and reconciles one sentence each in FR-142 and FR-322.

Examined:
- FR-154 "Unit bounds" (examined)
- FR-154-AC-28 (examined)
- FR-154-AC-29 (examined)
- FR-154-AC-31 (examined)
- FR-142 Quantity type reference (examined)
- FR-322 step 4 unit row (examined)
- TC-234 I22 and I24 (examined)
- FR-154 unit member shape (context_only)
- FR-146 quantity divisor and negation text (context_only)
- QSL FR-230-AC-26 (context_only)

Checked and clean: FR-142, FR-154 and FR-322 now agree on the type, the subtype direction, and that the checked package adds no node; no criterion id was renumbered (AC-31 is the next free id, AC-28 and AC-29 keep theirs); the AC-28 reword keeps every earlier assertion (initial-state refusals, absent optional, sequence elements, recorded successor, conversion, nonzero proof) and the "forced" claim is true, because a constant `7/2 ms` assigned to a `[1, 3]` member now refuses statically; QSL FR-230-AC-26 already states the same ruling. `quire validate` on FR-142, FR-154 and TC-234 passes; FR-322 fails identically on main (missing `id`), so the PR did not cause it. TC-234 fixtures are prose rows with no hashes or identities, so nothing was hand-fudged.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-154-AC-31 and I24 list "a field default" as a position where `0 ms`..`10 ms` check and `11 ms` refuses. FR-154 "Unit bounds" intake rule says a member of a unit type has `defaultKind` `none` and no `defaultValue` (I19 refuses a semantic default on a unit field). The position cannot be built at intake, so an implementer cannot write that vector; either drop it or say which default (QSL source default) is meant. | spec/functional/type-model/FR-154-admit-domain-package-model.md:216; FR-154-AC-31; TC-234 I24 |
| FND-002 | medium | FR-154-AC-31 says `-1 ms` refuses "at the literal". FR-146 says `-1 s` is the unary minus applied to the literal `1 s`, never a literal. The refusal location for the folded negation is undefined (the literal `1 ms`, which satisfies `min` 0, or the negation expression). Name the position, or say the constant expression refuses at the negation. | FR-154-AC-31; spec/functional/expressions/FR-146-check-total-pure-functions.md:330 |
| FND-003 | low | FR-154-AC-28 says the postcondition sets `A/d` to `self.d + 2 ms` (`7/2 ms`) but names no pre-state; `7/2 ms` only follows from `3/2 ms`, which only I22 states. The AC is not self-contained. | FR-154-AC-28 |
| FND-004 | low | The new text types "a bounded member" `Quantity<u>[min, max]` but does not say the type of a sequence, map or optional member whose element is bounded (`Sequence<Quantity<ms>[1, 3]>`?), nor whether a literal element of such a collection is checked statically. The runtime clause still lists elements; FR-322 TC-280 MU-02 types `Set<Quantity<ms>>` with no bound. No AC-31 vector covers a collection element. | FR-154 "Unit bounds"; TC-280 MU-02 |
| FND-005 | low | Several new rules have no vector: a constant expression that folds (`7 ms + 4 ms`), an open bound (`min` only or `max` only), a comparison operand (`self.d < 11 ms` must not refuse), an unsatisfiable conjunction refusing every literal, and bounded-to-bounded flow between two different bounded types (admitted? the text only says plain-to-bounded). A rule with no failing vector can regress unseen. | FR-154-AC-31; TC-234 I24 |

## Verdict

Approve in substance; changes requested for FND-001 and FND-002 (two vectors in AC-31 and I24 that are unbuildable or ambiguous) before merge. The ruling is implemented consistently across FR-154, FR-142, FR-322 and QSL FR-230-AC-26. FND-003 to FND-005 are low.

## Dispositions

Round 1, reviewed at 22349c14ed60ab5c44da818923f1894f82a0e1bc.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 22349c14: AC-31 and I24 drop the field-default position and state that a unit member has no default |
| FND-002 | fixed | 22349c14: FR-154 and AC-31 define the refusal site as the outermost folded constant expression; `-1 ms` refuses at the negation, citing FR-142-AC-25 and "Quantity negation" |
| FND-003 | fixed | 22349c14: AC-28 and I22 name the pre-state `3/2 ms` |
| FND-004 | fixed | 22349c14: FR-154 types Sequence, Set, Map and Option members with bounded elements and checks literal elements and payloads; AC-31 and I24 cover `sequence[5 ms, 11 ms]` and `Some(11 ms)` |
| FND-005 | fixed | 22349c14: I24 adds folding, min-only and max-only bounds, comparison operands, the unsatisfiable pair and bounded-to-wider-bounded flow |

Round 1 checks of the new text found no new defect. The sentence on "equal or looser" bounds is awkwardly worded but consistent: a bounded value that flows to a narrower bounded position is still admitted as a plain `Quantity<u>` and left to the run-time invariant.
