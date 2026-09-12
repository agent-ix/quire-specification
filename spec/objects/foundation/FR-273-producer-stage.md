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

This artifact types the stage as a **value**, not as an enumeration, because no
adopted artifact fixes its membership. The catalog delegates membership: the
stage is the one "named by the selected producer interface". The two stage lists
that do exist diverge; see the open gap below.

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Stage label | Proposed catalog, "Common structured context" | The stage "named by the selected producer interface". The catalog "does not rename historical stage labels or create a competing pipeline enum". |
| Originating-stage retention | Proposed catalog; FR-047-AC-5 | A wrapper "retains the originating stage and cause instead of replacing them with its own display stage"; a failure keeps "their actual stage". |
| Selected producer interface | Proposed catalog | The interface that names the stage. It is the membership authority; this artifact names no members. |

## Identity and comparison

- **Identity kind** — *value*. A stage label has no identity of its own; it is
  compared componentwise as an exact string.
- **Key** — the stage label, taken together with the selected producer interface
  that names it. Equality is componentwise over exactly these and nothing else.
  Rendering is excluded: FR-047-AC-1 requires that "changing/localizing
  human-readable messages leaves the same typed failure's stage, code and cause
  details unchanged", and FR-047 states "Diagnostic formatting/localization
  cannot change classification". A wrapper's **display stage** is by the same
  rule outside the key: the catalog requires retaining "the originating stage
  and cause instead of replacing them with its own display stage".
- **Key order** — lexicographic over the label, for collection comparison only.
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
- **Digest domain** — a stage label carries no digest. The catalog that carries
  it is selected by identity, revision and byte digest, whose domain rule is
  [FR-201](FR-201-canonical-identity-domain.md).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-273-AC-1 | A wrapper re-reporting a failure retains the originating stage label rather than its own display stage. | Analysis |
| FR-273-AC-2 | Localizing or reformatting a diagnostic message leaves its stage label unchanged. | Analysis |
| FR-273-AC-3 | A diagnostic with no available stage compares only to absence and receives no default stage. | Analysis |
| FR-273-AC-4 | No member list for this value is asserted by this artifact while the two recorded stage enumerations remain unreconciled. | Inspection |

## Open gap: two divergent stage enumerations

Two sources enumerate the same axis and do not agree on its members. Both are
quoted verbatim and neither is preferred.

- **FR-047 Inputs names six stages:** "A failure at source recognition,
  identity/linking, type/profile checking, input binding, capability selection
  or resource-limited execution, with its available source/subject context."
- **The proposed catalog names eleven stages:** "Stage is the actual source,
  lexical, parsing, profile, linking, checking, package, input-validation,
  evaluation, lowering or host boundary named by the selected producer
  interface; a wrapper retains the originating stage and cause instead of
  replacing them with its own display stage."

Side by side:

| FR-047 (six) | Proposed catalog (eleven) |
| --- | --- |
| source recognition | source |
| identity/linking | lexical |
| type/profile checking | parsing |
| input binding | profile |
| capability selection | linking |
| resource-limited execution | checking |
| — | package |
| — | input-validation |
| — | evaluation |
| — | lowering |
| — | host boundary |

The columns are **two divergent enumerations of one axis**, not a row-by-row
correspondence; the pairing above is positional listing only and asserts no
mapping. No artifact reconciles them, and neither list is stated to be a
refinement, a superset or a display view of the other. Because membership is not
established, this artifact declares no enumeration, picks neither list, and
synthesises no merged list. Selecting between them is an owner ruling.

## Dependencies

- [FR-047](../../functional/FR-047-emit-typed-located-causes.md) establishes the
  stage-retention obligation and the six-stage input list.
- [Native diagnostics catalog](../../../proposals/quire-v1/definitions/native-diagnostics.md)
  is the proposed, non-adopted source of the eleven-stage list.
- [FR-272](FR-272-typed-diagnostic-cause.md) supplies the typed cause retained
  alongside the stage.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
