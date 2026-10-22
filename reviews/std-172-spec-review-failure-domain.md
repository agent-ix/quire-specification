---
id: SR-12852
title: "Failure-domain review of PR #278: FR-153 population named by qualified name"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@eb852d41d0c5ee5510b19e373df41555210a0105; PR #278 diff against origin/main 1da546f: FR-153 Behavior paragraph (lines 122-140) and FR-153-AC-12; cross-checked against merged quire-spec-language main 49bcf9a FR-391 Resolution and Evaluation"
review_set: subset
---
# Failure-domain review of PR #278 (STD-172)

## Summary

Ticket: STD-172. This pass looks for identity confusion and unstated failure
modes in the name-to-binding rule. It judges which of the items QSL FR-391
states but AC-12 leaves out are QSpec system behavior.

The value-use refusal, unknown-name refusal and open-extent outcome are
stated. Two resolution failure modes that FR-391 states are missing from
QSpec. Both are about what M::P means in a given place, and QSpec owns static
resolution of qualified references (shared-grammar.md:408-412), so they are
not QSL implementation details.

## Verdict

No high findings. Two medium findings and one low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The paragraph says how a state clause names a population, but not what M::P means outside one. QSL FR-391 Resolution and AC-3 say that in a function body it names no population and refuses missing_declaration/missing-name. QSpec only says that a function names one through a parameter. A QSpec implementer could resolve allInstances<M::A>(M::PA) in a function body, which has no observation, so it would need an ambient binding, which FR-153 forbids; or it could refuse with a different code. Two engines then disagree on a function body that QSL refuses. State the outside-a-state-clause outcome, and add a TC-198 L14 case for it. | spec/functional/type-model/FR-153-query-closed-model-environments.md:122-126; spec/functional/type-model/FR-153-query-closed-model-environments.md:164 |
| FND-002 | medium | Identity confusion: M::P can resolve both as a population and as an enum member (an enum named M with a member P, beside a model alias M). QSL FR-391 refuses this ambiguous_declaration/ambiguous-name, naming both. QSpec's resolution rule (shared-grammar.md:408-412) classifies a bare qualified reference as an enum member first, so a QSpec engine could silently take the enum value and then refuse operator-ineligible, or type-mismatch, at the allInstances call instead of reporting the ambiguity. State the ambiguity refusal in FR-153, or give the precedence rule. | spec/functional/type-model/FR-153-query-closed-model-environments.md:124-127; proposals/quire-v1/shared-grammar.md:408-412 |
| FND-003 | low | The phrase "a population that is not admitted as closed gives incomplete_population/incomplete-scope" can be read as covering every admission failure. The rules above make conflicting-identity and foreign-type refusals, and subtype non-closure unclosed-subtypes. The paragraph also does not say what happens when the observation has no row for P at all; the catalog offers incomplete_population/missing-required-artifact, and FR-046-AC-4 calls it unavailable. Say "whose extent is not closed", and name the missing-row outcome. | spec/functional/type-model/FR-153-query-closed-model-environments.md:135-137; proposals/quire-v1/definitions/native-diagnostics.md:78 |

## Dispositions

Round 1, reviewed at 561f23fcf64f179b9632072134dd9d7574abdfb0. The new causes
exist in native-diagnostics.md:78: incomplete_population with
missing-required-artifact, incomplete-scope or unclosed-subtypes.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |

Round 2, reviewed at e8958629d320a4f782260e6c521e1fb1baf212be. No finding was
open, so there are no round-2 rows and no new findings in this lens.

Round 3 (final record pass), reviewed at 2b006cd52014ce6dc331bf6bc429eaeb9a182a35. No finding was open in this lens, so there are no new rows.
