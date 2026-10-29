---
id: FR-273
title: "Producer stage"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-047
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-272
    type: references
---
# FR-273: Producer stage

## Description

When a wrapper re-reports a failure it observed elsewhere, the wrapper SHALL retain the originating producer stage rather than substituting its own display stage.

FR-047 Outputs require "a stable catalogued classification, failure stage and
structured cause details with exact available loci", and FR-047-AC-5 requires
that "per-request resource/unsupported failures keep their actual stage and
prerequisites rather than becoming another clause's Boolean result". The
proposed catalog states the retention rule: "Every diagnostic retains its
observing producer stage and its catalogued code. ... a wrapper retains the
originating stage and cause instead of replacing them with its own display
stage. This catalog does not rename historical stage labels or create a
competing pipeline enum"
([native diagnostics catalog](../../../proposals/quire-v1/definitions/native-diagnostics.md)).

This artifact types the stage as a **value**, not as one ecosystem-wide
enumeration. Its value is the pair of the exact selected producer-interface
reference and one exact label from that interface's closed stage vocabulary.
Different producer interfaces may publish different closed vocabularies without
creating aliases. A reader that does not support the selected interface or its
stage vocabulary retains the exact supplied interface reference and stage label
with the original bytes and available provenance, then refuses interpretation;
it never chooses another list by label or rewrites the pair.

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Producer-interface reference | FR-047 and native catalog | Exact identity of the interface that owns stage membership. |
| Stage label | Selected producer interface | One exact member of that interface's closed stage vocabulary. |
| Originating-stage retention | Proposed catalog; FR-047-AC-5 | A wrapper "retains the originating stage and cause instead of replacing them with its own display stage"; a failure keeps "their actual stage". |
| Selected producer interface | Proposed catalog | The only membership authority for the paired stage label; no global first-wins registry participates. |

## Identity and comparison

- **Identity kind** — *value*. A producer stage has no identity of its own and is
  compared componentwise.
- **Key** — the exact selected producer-interface reference followed by the
  exact stage label it defines. Equality is componentwise over exactly these and
  nothing else.
  Rendering is excluded: FR-047-AC-1 requires that "changing/localizing
  human-readable messages leaves the same typed failure's stage, code and cause
  details unchanged", and FR-047 states "Diagnostic formatting/localization
  cannot change classification". A wrapper's **display stage** is by the same
  rule outside the key: the catalog requires retaining "the originating stage
  and cause instead of replacing them with its own display stage".
- **Key order** — producer-interface reference order followed by lexicographic
  label order, for collection comparison only.
  It carries no causal meaning and expresses **no pipeline, precedence or
  severity ordering between stages**. The stage names in either list below
  appear in a prose sequence; the prose establishes no order over them, and the
  catalog explicitly declines to "create a competing pipeline enum".
- **Causal order** — none. A stage is not a causal predecessor of another stage,
  and an upstream diagnostic observed at another stage is a referenced identity,
  not an ordering: "Related declarations and upstream diagnostics retain their
  own exact source/catalog identities rather than being flattened into a string"
  (proposed catalog).
- **Absence** — a diagnostic whose stage is not available has an absent stage,
  and an absent stage compares only to absence. It is never defaulted to a
  wrapper's own stage. "Unavailable context is explicitly unavailable; byte
  zero, an empty path or a name-search match cannot masquerade as a located
  failure" (proposed catalog).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-273-AC-1 | A wrapper re-reporting a failure retains the originating stage label rather than its own display stage. | Analysis |
| FR-273-AC-2 | Localizing or reformatting a diagnostic message leaves its stage label unchanged. | Analysis |
| FR-273-AC-3 | A diagnostic with no available stage compares only to absence and receives no default stage. | Analysis |
| FR-273-AC-4 | A stage label admits only when it is a member of the exact selected producer interface's closed vocabulary; an unknown interface/member or a label copied from another interface is retained exactly and refuses without fallback or rewrite. | Analysis |

## Resolved membership ruling

The two earlier lists do not define one global axis:

- **FR-047 formerly named six failure concerns:** "A failure at source recognition,
  identity/linking, type/profile checking, input binding, capability selection
  or resource-limited execution, with its available source/subject context."
  These are requirement coverage categories, not wire members.
- **The diagnostic candidate formerly named eleven generic stages:** "Stage is
  the actual source,
  lexical, parsing, profile, linking, checking, package, input-validation,
  evaluation, lowering or host boundary named by the selected producer
  interface." That generic list is superseded as membership. The selected
  producer interface owns its exact closed labels.

The merged native compiler demonstrates why this boundary is necessary. Its
native diagnostic interface currently distinguishes `source`, `lex`, `parse`,
`profile`, `format`, `source_map`, `link`, `check`, `validate` and `evaluate`;
its command envelope owns additional file/request/output/package/lower/input and
extraction labels. Those are separate interfaces, not permissions to extend this
standard by observing an installed enum. Their exact selected interface
identities remain the membership authority.

## Dependencies

- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md) establishes the
  stage-retention and producer-interface selection obligations.
- [Native diagnostics catalog](../../../proposals/quire-v1/definitions/native-diagnostics.md)
  is the proposed, non-adopted source of the eleven-stage list.
- [FR-272](FR-272-typed-diagnostic-cause.md) supplies the typed cause retained
  alongside the stage.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Status

Accepted FS02 membership ruling. Producer interfaces still require their own
closed vocabularies and consumer implementation evidence.
