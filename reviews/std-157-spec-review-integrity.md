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

## Verdict

Not mergeable until FND-001 is resolved. One fix is to amend FR-044's
"sole exception" sentence and FR-149 lines 51 and 74 to name this second
exception. Another is to restate the exception as an expected-type rule (the
other operand of `=`/`!=` supplies the arithmetic node's result type, as
FR-146 already lists for `case`), which avoids adding a conversion at all.
