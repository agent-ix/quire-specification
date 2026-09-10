---
id: FR-020
title: "Select and compose exact native semantic profiles"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-005
    type: traces_to
  - target: ix://agent-ix/quire-specification/FR-001
    type: references
  - target: ix://agent-ix/quire-specification/FR-004
    type: references
---
## Description

When a native package selects its edition and semantic profile bundle, the frontend shall bind every clause and cross-profile reference to those exact definitions.

## Inputs

Exact native source identities/revisions/bytes/spans; one language and edition;
an explicit set of profile definition references; authored requirement/clause
identities; exact model dependency closure. A definition reference includes name,
revision and byte digest. A source unit may declare a default selected facet;
every clause's effective facet is explicit in the linked package. There is no
default inferred from installed backends or whichever parser accepts the text.

## Outputs

A linked package with typed state, temporal and protocol clauses and their
profile dependencies, or a located typed refusal. A requested backend capability
is selected separately after language admission. Static package identity excludes
runtime populations, observations and mutable monitoring state.

## Behavior

The standard shall include state, temporal and protocol/choreography families in the Quire v1 release capability set.
The frontend shall use native Quire as the sole editable authority for each clause.
The frontend shall reject an unknown edition, profile, definition revision, definition digest or required feature before executing the affected clause.
The frontend shall reject conflicting definitions selected under the same profile identity.
The frontend shall retain each clause's family and selected semantic definition in the linked package.
The linker shall resolve cross-profile references through typed native declaration identities.
The linker shall reject a cyclic semantic dependency between composed clauses.
The linker shall bind every imported state predicate to its declared model, anchor, parameters and immutable capture environment.
The type checker shall reject a pending temporal or protocol result used as an ordinary Boolean predicate.
The frontend shall retain source-valid clauses when a requested backend cannot execute their selected semantics.
The capability selector shall report the unsupported requested method with its clause and profile identity.
The package reporter shall retain a disposition for every selected clause, including unsupported or incomplete clauses.
The migrator shall preserve historical profile definitions and source identities when producing a new semantic subject.

The complete v1 bundle is a release claim over the selected facets, not permission
to coerce one facet into another. Profile-specific releases may identify a
smaller implementation milestone without claiming full Quire v1. A later
profile or method does not change previously selected meanings. Generated
TL/OCL/SysML/FRETish documents are derived artifacts, never additional authored
clauses or fallback source definitions.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-020-AC-1 | One native package binds state, temporal and choreography clauses to exact profile definitions and one shared model closure. | Test |
| FR-020-AC-2 | Unknown, mismatched and conflicting profile definitions refuse with the offending clause/selection and stable typed cause, without fallback. | Test |
| FR-020-AC-3 | Cross-profile predicate references retain declaration, source, model, anchor and capture identity; cycles and status-as-Boolean uses refuse. | Test |
| FR-020-AC-4 | A source-valid protocol remains available for replay when local projection is unsupported; the report retains both requested capability dispositions. | Test |
| FR-020-AC-5 | A package containing a supported state clause and an unsupported requested temporal lowering cannot report complete package success or omit the temporal clause. | Test |
| FR-020-AC-6 | Old definition bytes and identities remain unchanged; recompiling under a new facet creates a distinct semantic subject without restamping historical results. | Test |
| FR-020-AC-7 | A release capability report missing any required v1 family cannot claim full Quire v1 conformance. | Test |

## Dependencies

- [Architecture](../assurance/AD-001-composed-native-language.md).
- [V1 capability decisions](../../proposals/quire-v1/capabilities.md).
- [Exact profile selection](FR-001-select-native-profile.md).
- [Artifact identity dimensions](FR-004-separate-artifact-identities.md).
