---
id: SR-8714
title: "ears-conformance review of PR #244 phase-2 qualify list"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@2989920af9570f8858b8dee479aa8b35729005cc; proposals/quire-v1/definitions/model-complete.md (phase-2 qualify row, new paragraph after the normalization rules table)"
review_set: subset
---

## Summary

Ticket: QSL-743. An ambiguity and wording check of the added prose. The text
is a declarative definition, written in present tense like the surrounding
model-complete.md, and is not an EARS SHALL statement. That matches the file's
convention, so it is not a finding. One ambiguity was found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | "A variant type declares no supertype" is stated as a fact, but no rule says what happens when a variant type's IR node carries a non-empty `supertypes` list: the Supertypes row refuses this only for persistence interfaces and namespaces, and QSL's variant reader ignores the member | model-complete.md:295-296; model-complete.md:154 |

## Verdict

The text is otherwise clear. "keyed by the case node's own declaration key,
never the `variant` export key" and "changes no effective identity, entry or
position" are precise and match FR-056 word for word.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 17ffad9 |
