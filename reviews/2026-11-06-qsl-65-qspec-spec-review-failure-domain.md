---
id: SR-20261104200623
title: "QSL-65 nullable corpus identity failure-domain review"
type: SpecReview
analysis: failure-domain
review_set: subset
scope: "agent-ix/quire-specification; QSL-65; PR 628 frozen 3b20dbae; FR-336-AC-12, TC-970 owning-model procedure, corpus ObligationIdentity schema and replay-request fixture"
---

# QSL-65 owning-model failure-domain review

## Summary

PASS for the specification phase. Reviewed the distinction between an absent owning model and a missing wire member, preservation of present model identity, rejection of malformed and surplus values, and the boundary between corpus validation and the unchanged FR-071 public request. No applicable authored AssuranceProfile was found. No source implementation, build, or final gate was run.

## Examined scope

| Unit | Role | Excerpt or constraint |
| --- | --- | --- |
| FR-336-AC-12 | examined | The required `obligation_identity.model` member admits `null` and maps it to no owning model (`None`), or admits a non-empty string and maps it unchanged to the present owning model (`Some`). A missing member, an empty string, another JSON type or an unknown obligation-identity member is refused; no model identity is inferred or fabricated. |
| FR-336 no inference | examined | In particular, the adapter SHALL NOT infer a model from the package, selected function, dependency or another identity. |
| TC-970 malformed owning models | examined | Independently remove `model`, replace it with an empty string, Boolean, number, array or object, and add an unknown obligation-identity member; each mutated request is refused by the closed schema. |
| Schema required/type/closure | examined | `model` is still required; only null and nonempty string satisfy its schema; unknown identity members remain prohibited. |
| Corpus fixture | examined | Its function obligation uses explicit null; the fixture still makes no executable qualification claim. |
| FR-201 absence | context_only | An absent component compares only to absence and is never defaulted. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Null cannot silently become a selected package or fabricated model identity. A missing member remains a schema refusal even though a permissive `Option<String>` deserializer by itself could map it to None; the specified order requires validating the closed request. Nonempty present strings remain identity values, including their original spelling, rather than names inferred from another request field. Wrong-type values and surplus members are adverse cases, not fallback paths.

The changed schema definition is referenced only by the corpus ReplayRequest obligation member; it does not relax protocol activation identity requirements or alter the public request's product wire. No callback side effects, cycles, traversal termination, state transitions, resource accounting, or cross-owner admission capability is introduced. The updated fixture is deliberately only a wire example; real positive replay qualification remains the QSL consumer's obligation. The forthcoming Rust tests must exercise both positive mappings and each stated refusal partition before merge.
