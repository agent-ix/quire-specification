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

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | "a `holds`-style argument" names nothing in a `graph-predicate`. `holds(...)` is a `temporal-primary` of the temporal clause, not a primary of `expr`, so no state-graph predicate can contain one. Say "an operand of a primary, such as a call argument or a quantifier body" instead. | proposals/quire-v1/shared-grammar.md:737-738 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | The FND-004 rewording doubled a phrase. The sentence now reads "a temporal operator in an `if` condition, a `let` binding, or an operand of an operand of a primary, such as a call argument or a quantifier body". Delete one "an operand of". | proposals/quire-v1/shared-grammar.md:737-738 |
| FND-006 | low | SG-R18's parenthesised chain `(c.a until c.b) until c.c` has two temporal operators, and the vector expects the refusal at the first `until`. FR-390 says only "located at the operator" and states no rule for a predicate with several operators. A checker that refuses at the outer (root) operator, the second `until`, also follows FR-390 and fails the vector. Fix: state in FR-390 that the refusal is located at the first temporal operator in source order, or make the vector accept either locus. | spec/test-cases/TC-330-state-graph-claim-syntax-and-routing.md:44; spec/functional/analysis/FR-390-analyze-state-graph-claims.md:116-119 |

## Dispositions

Round 1, reviewed at fad2cd29132c763e218b918230ed71c48a210a0c. Against
quire-spec-language PR #968 at b82b29bc8bee5d29ca40a5df3bb6e80eb2d4760b.

Flat binary chains: as the QSpec grammar now stands, `c.a until c.b until c.c`
is a syntax error. `graph-relation` admits at most one binary temporal
operator, exactly as the temporal facet's `temporal-relation` does, and a
chain has to be parenthesised: `(c.a until c.b) until c.c`. Keep it a syntax
error. That is consistent with the temporal facet, and admitting a chain
would need an associativity rule that neither facet has. #968's flat
`StateGraphFormula` chain should be aligned to it on QSL-535.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | deferred | Planner ruled QSpec's `graph-predicate` normative; QSL-535 aligns #968 to it. #968 at b82b29b still reads `let`/`if` only through `Expression` (so `if c.a then eventually c.b else c.c` is an S1 error there), chains binary operators flatly, and reads a parenthesised group as `StateGraphFormula`. TC-330 still has no vector for a temporal operator in an `if` branch or for a chained binary operator; adding those two would make the alignment testable from QSpec. |
| FND-003 | fixed | - |

Round 2, reviewed at 3b8d62b923cc38b8a14913efe63671ce1ed6a537. The chain
decision is now stated in the grammar prose (shared-grammar.md:739-741) and in
FR-390, and it matches the EBNF: `graph-relation` takes one optional binary
operator. SG-R17 (`if c.a then eventually c.b else c.c` parses through
`graph-predicate`'s `if` alternative, refused at `eventually`) and SG-R18's
flat chain (the error falls at the second `until`, where `graph-relation` has
ended and `graph-body` expects `}`) both match the grammar. FND-002's round-1
deferral (QSL-535 aligns #968; planner ruling: QSpec's grammar is normative)
stands as recorded, and SG-R17 and SG-R18 now make that alignment testable from
QSpec.

| FND | outcome | reason |
| --- | --- | --- |
| FND-004 | fixed | - |
