---
id: SR-12824
title: "Scope-boundary review of PR #279: QSpec graph-predicate against QSL S1"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-specification@5193b7d306a98ac4eac8950974af9c188db1c899; PR #279 diff against origin/main 1da546f: shared-grammar.md graph-body/graph-predicate and recognition rules, FR-390 Admission paragraph; context: quire-spec-language origin/main 49bcf9a FR-165 Parsing (S1), FR-165-AC-2, FR-165-AC-4, ADR-022 SG-4/SG-5/GM-7, and quire-spec-language PR #968 qsl-cst/src/grammar.rs (StateGraphClaim, StateGraphBody, StateGraphPredicate, StateGraphFormula, StateGraphTerm)"
review_set: subset
---
# Scope-boundary review of PR #279 (STD-171)

## Summary

Ticket: STD-171. This review looked at two boundaries: QSpec, which owns
the grammar, against QSL's S1, which implements it (merged FR-165 and the
code in open PR #968); and syntax errors against admission refusals.

Clean against merged QSL spec: FR-165 Parsing (S1) names this production. It
says `over` is optional in all three forms, operators may stand at any
operand position with an optional interval, and fairness is admitted at the
start of the body. All three agree with the PR. FR-165-AC-2 and FR-165-AC-4
(`c.versionNumber = 2 and eventually ...`, `not eventually ...`,
`(eventually ...)`, `{ true }` with no `over`) all parse under the new
EBNF. ADR-022 SG-4 ("no temporal operator in its predicates") is enforced at
admission, which is consistent. #968's `Interval` and `Fairness` match the
QSpec productions exactly.

FR-390-AC-2 is reachable from source under the QSpec grammar now. It is
reachable through QSL only once #968 lands, and only for vectors that
write `over` (FND-001).

## Verdict

FND-001 is a divergence that needs action on QSL-535 / #968, not on this PR.
FND-002 needs an owner decision on which side moves. FND-003 is a medium
wording fix in this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | QSL #968 makes `over` mandatory: `x("over"), bound_parameter()` in all three `StateGraphClaim` alternatives. This PR, QSL FR-165 Parsing (S1), FR-165-AC-4 and ADR-022 SG-5 all make it optional. Against #968, SG-S05 (`possible P using v { true }`) and every new over-less vector (SG-R10, SG-R11, SG-R12, SG-R13) are S1 syntax errors, so FR-390-AC-2's operator and fairness refusals are unreachable for them. #968's own review already records this as its FND-003 (medium, owner question). The fix belongs on QSL-535: make `over` optional, as FR-165 already requires. It does not block this PR. | proposals/quire-v1/shared-grammar.md:575-583; quire-spec-language PR #968 qsl-cst/src/grammar.rs StateGraphClaim:1104-1146 |
| FND-002 | medium | Apart from `over`, #968 accepts a different language from `graph-predicate`, and no vector catches it. (a) Temporal operators in `let`/`if` result positions: `if c.a then eventually c.b else c.c` parses under QSpec (`graph-predicate` `'if' expr 'then' graph-predicate`). #968 reads `let`/`if` only inside a plain `Expression`, and `StateGraphFormula` has no `let`/`if`, so it is an S1 syntax error there. (b) Chained binary operators: #968's `StateGraphFormula` is a flat `Term { op [Interval] Term }` with `op` any of `and`, `or`, `implies`, `until`, `release`, `since`, `triggered` chain, so `c.a until c.b until c.c` parses there. QSpec's `graph-relation` admits one binary operator, so it is a syntax error. (c) #968 reads a parenthesised group as `StateGraphFormula` (no `let`/`if`), and QSpec as `graph-predicate`. Each is a source that one side admits and refuses at admission while the other rejects it at parse. Fix: decide which side moves. Then add a TC-330 vector for an `if` with a temporal operator in a branch, and one for a chained binary operator. | proposals/quire-v1/shared-grammar.md:585-600; quire-spec-language PR #968 qsl-cst/src/grammar.rs StateGraphPredicate:1161, StateGraphFormula:1177, StateGraphTerm:1188 |
| FND-003 | medium | The new FR-390 paragraph says both constructs reach admission and are "never a syntax error; the refusals above are the only place a state-graph claim rejects them". The grammar makes several of them syntax errors: fairness after the predicate (SG-R13, which this same PR adds), fairness inside `from(...)`/`to(...)` (author point 1), any fairness on `unique path` (SR-12823 FND-002), and a temporal operator in an `if` condition, a `let` binding or inside a primary's operand such as `exists(x in s: eventually x.ok)` or `size(...)` (author point 3). The FR-390 bullet "at any operand position" and QSL FR-165's "at any position" overstate in the same way. Two implementers reading FR-390 and the EBNF disagree on `if eventually c.a then ...`. Fix: narrow the paragraph to the positions the grammar gives (result positions of a `graph-predicate`, and the start of a `graph-body`), and say that a temporal operator or fairness constraint anywhere else is a syntax error. | spec/functional/analysis/FR-390-analyze-state-graph-claims.md:116-119; spec/functional/analysis/FR-390-analyze-state-graph-claims.md:124-127; spec/test-cases/TC-330-state-graph-claim-syntax-and-routing.md:39 |
