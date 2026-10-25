---
id: SR-12105
title: "Integrity review of QSpec PR #273: FR-149 arithmetic operand against a bounded integer"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@09bf86dc79fec4c70b36f57e18bbe6b92599e3ec; PR #273 diff against origin/main: spec/functional/type-model/FR-149-apply-complete-equality-matrix.md, spec/test-cases/TC-194-complete-equality-matrix.md; consistency against FR-044, FR-206 and FR-149's unchanged Behavior text"
review_set: subset
---
# Integrity review of QSpec PR #273 (STD-157)

## Summary

Ticket: STD-157. This review checks whether the new FR-149 section is
consistent with the requirements it lives alongside. The section adds a second
implicit numeric conversion, but three statements that say no such conversion
exists were left unchanged.

Checked and clean: AC-19..21 are atomic and each names one observable outcome.
TC-194 E33/E34 trace to AC-19..21 through TC-194's existing `verifies`
relation on FR-149. The equality-matrix row edited at line 60 links to the
new section.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The PR adds an implicit `Integer`-to-`Int[lo,hi]` coercion under `=`/`!=` but leaves three statements that forbid it. FR-149 Behavior says "No ... implicit conversion ... creates a common type" (line 51). FR-149 says "Operands of different types are comparable only after an explicit conversion declares a common type" (line 74). FR-044 says "general implicit conversion ... inadmissible" and calls FR-149's rational `n/1` promotion "the sole operation-scoped exception"; FR-206 repeats that operands and results share a named type. An implementer following FR-044 or FR-149 line 74 refuses what the new section admits. | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md:51,74,172-199; spec/functional/foundation/FR-044-check-composed-numeric-definedness.md:31; spec/objects/foundation/FR-206-numeric-domain.md:20-22 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | FR-044 Behavior now names two exceptions, but its ACs were not updated. AC-3 still says that under complete value "only FR-149's expected-rational `/` promotion and FR-147's admitted integer operators apply", and AC-9 refuses "general implicit conversion outside FR-149's complete-value rational `/` promotion". A test written from AC-3 refuses `later.tally = n + 1` (an `Integer` operand in an `Int[0,3]`-typed node) as a wrong named type, which the new rule admits with an obligation. | spec/functional/foundation/FR-044-check-composed-numeric-definedness.md:65,71 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-149 lines 51 and 78, FR-044 Behavior line 31 and FR-206 now name the second exception. FR-044-AC-3 and AC-9 still list only the rational promotion (new FND-002). |

## Verdict

Not mergeable until FND-001 is resolved. One fix is to amend FR-044's
"sole exception" sentence and FR-149 lines 51 and 74 to name this second
exception. Another is to restate the exception as an expected-type rule (the
other operand of `=`/`!=` supplies the arithmetic node's result type, as
FR-146 already lists for `case`), which avoids adding a conversion at all.

## Verdict (disposition pass 1)

FND-001 is fixed at a291b4d. Not mergeable until new FND-002 is fixed:
FR-044-AC-3 and AC-9 need to name the FR-149 expected-type rule alongside the
rational `/` promotion.
