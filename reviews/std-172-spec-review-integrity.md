---
id: SR-12851
title: "Integrity review of PR #278: FR-153 population named by qualified name"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@eb852d41d0c5ee5510b19e373df41555210a0105; PR #278 diff against origin/main 1da546f: FR-153, FR-383-AC-13, shared-grammar.md; cross-checked against merged quire-spec-language main 49bcf9a FR-391, TC-1076, FR-155, FR-104, FR-106, FR-107, FR-380, and QSpec FR-046, FR-263, model-complete.md, native-diagnostics.md"
review_set: subset
---
# Integrity review of PR #278 (STD-172)

## Summary

Ticket: STD-172. This pass checks that the new FR-153 rule agrees with the
rest of QSpec and with the merged QSL FR-391.

Most of it agrees. The observation anchors (current, pre, post, and pre under
pre(e)) match QSL FR-391 Typing and QSpec FR-046. The binding comes from
observation input, so it does not contradict the no-ambient-default
statements in FR-263, FR-112 and the shared grammar (line 450), or the FR-153
Description's request-bound environment. Under the respelling, FR-383-AC-13
keeps its footprint exactly: AnyField{Counter, v} and AnyMembership{counters}.
That matches QSL FR-155-AC-2. The refusal codes match FR-391-AC-2, AC-3 and
AC-5.

Making AC-12 a subset of FR-391 is right for four of the omitted items: the
requirement-record domain (FR-104 is a QSL checker output), the footprint
specifics (FR-383-AC-13 already names AnyMembership{P} per population), the
S4 interim refusal and the IR lowering (QSL implementation state). Two
omitted items are QSpec semantics; they are in SR-12852. Two typing terms
in the new paragraph are undefined in QSpec; they are below.

## Verdict

No high findings. Three medium findings, all of which leave a QSpec-only
implementer unable to derive the same type QSL derives. There is one low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new paragraph and AC-12 say the operand types as Population<U>[N] "when the declaration states a maximum". A QSpec population declaration has no maximum: model-complete.md:173 lists only member types and extent, and FR-153's maximum belongs to the binding. QSL FR-380 says this outright ("A domain-package population declaration has no maximum (QSpec FR-153)") and gets N from its own bound declaration. So the [N] branch has no source in QSpec. One reader always types Population<U>; another looks for a maximum that does not exist; and QSL's Population<Counter>[3] from a bound (FR-391-AC-4) is not derivable from QSpec. Say where N comes from (a QSL bound carried in the checked package, or none in QSpec), or state that in QSpec the operand is Population<U>. | spec/functional/type-model/FR-153-query-closed-model-environments.md:129-131; spec/functional/type-model/FR-153-query-closed-model-environments.md:164; proposals/quire-v1/definitions/model-complete.md:173 |
| FND-002 | medium | "U is the population's member object type" assumes one member type, but a population declaration lists member types. TC-198's PA has [A, B] and L14 expects Population<M::A> without saying why. QSL FR-391 Outputs defines U as the canonical member type: the least of the declared member types in ascending DeclarationKey order (QSL FR-104 Requirements). QSpec has no such rule, so a QSpec implementer could pick B, the first listed type, or refuse. State the rule. | spec/functional/type-model/FR-153-query-closed-model-environments.md:129-131; spec/test-cases/TC-198-closed-model-lookup.md:72 |
| FND-003 | medium | FR-383-AC-13 writes allInstances<Counter>(M::counters) with the type argument unqualified. QSL FR-155-AC-2 and FR-391-AC-1 write allInstances<M::Counter>(M::counters), and FR-153 requires T to resolve from a qualified name. The two specifications this PR is meant to align now spell the same example differently, and the QSpec spelling does not resolve under QSpec's own rule. | spec/functional/analysis/FR-383-explore-under-state-space-reduction.md:307 |
| FND-004 | low | Shared-grammar.md:408-412 still says static resolution classifies a bare qualified reference "as an enum member or other declared value". A population operand is now a third outcome, and it is not a value. The new sentence at line 450 does not update that classification. | proposals/quire-v1/shared-grammar.md:408-412; proposals/quire-v1/shared-grammar.md:450 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | FR-153 now takes N from "the maximum the unit authors for P with a bound declaration". QSpec defines no such declaration: shared-grammar.md has bound only as a reserved word and in verification-step [ 'bound', uint ], and no production `bound <label> on M::P [N]`. FR-153's own Dependencies say the complete-V1 grammar "owns every source form used by this requirement". The form is QSL FR-380's (S1 parses it there). So a QSpec-only reader cannot find the form that supplies N. Cite QSL FR-380 as the source of the authored maximum, or add the production. | spec/functional/type-model/FR-153-query-closed-model-environments.md:140-143; spec/functional/type-model/FR-153-query-closed-model-environments.md:193-194; proposals/quire-v1/shared-grammar.md:602 |
| FND-006 | low | L14 justifies U = A by "4531dd9e precedes c5264509". Those are effective type identities, but FR-153 orders by declaration key (package, then node, as UTF-8 bytes), here ix://test/orders/A before ix://test/orders/B. The result is the same, but the stated reason teaches the wrong key. | spec/test-cases/TC-198-closed-model-lookup.md:72 |

## Dispositions

Round 1, reviewed at 561f23fcf64f179b9632072134dd9d7574abdfb0. The canonical
member type rule matches QSL FR-104 Requirements: the least declared member
type in ascending DeclarationKey order, package then node, as UTF-8 bytes.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |
| FND-005 | fixed | - |
| FND-006 | fixed | - |

Round 2, reviewed at e8958629d320a4f782260e6c521e1fb1baf212be. The FND-005 and
FND-006 rows above are round 2's. FR-153 now cites quire-spec-language FR-380
for the authored maximum, and the Dependencies bullet makes the exception.
L14 orders by declaration key.
