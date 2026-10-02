---
id: FR-303
title: "Format and diagnose native source incrementally"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-302
    type: references
  - target: ix://agent-ix/quire-specification/FR-047
    type: references
---

# FR-303: Format and diagnose native source incrementally

## Description

When an editor requests formatting or language diagnostics, the tool SHALL
operate on the lossless CST and publish deterministic source edits and typed
incremental diagnostics for the exact document revision.

## Inputs

Lossless CST, checked semantic model, formatting profile, document revision and
incremental change set.

## Outputs

Non-overlapping edits, versioned diagnostics, symbols/navigation/completion or
typed refusal.

## Behavior

Formatting is idempotent and preserves semantic tokens/string spellings under
the selected edition. Diagnostics carry catalog codes and source/runtime loci,
never message-derived identity. Stale revision responses are marked stale and
cannot be applied. Language-server requests use the same typed parser/checker.

## Incremental service contract

Requests bind document identity, source digest and profile. Responses bind the
same tuple and include deterministic diagnostics ordered by primary span, stable
code and related span. Formatting computes edits against exact source bytes,
round-trips through the parser and is accepted only when the checked semantic
digest is unchanged. Cancellation returns incomplete; a superseded revision is
stale and its edits/diagnostics cannot be applied to another revision.

Navigation resolves a source position to the definition span and the reference
spans that the checker bound at that position for the same document revision,
and reports an unbound position as unbound. Completion proposes exactly the
declarations the checker admits at that position under the selected profile,
each with its resolved type and its owning declaration identity. Both read the
checked semantic model; neither derives a result from a lexical match over the
source bytes, and both are stale under a superseded revision on the same terms
as diagnostics.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-303-AC-1 | Formatting twice produces no further edits and rechecking preserves the same semantic package. | Test (TC-223) |
| FR-303-AC-2 | Incremental and full reparse/check produce equivalent diagnostics and symbols for the same bytes. | Test (TC-223) |
| FR-303-AC-3 | A stale document revision or overlapping/semantic-changing edit set refuses application. | Test (TC-223) |
| FR-303-AC-4 | Definition and reference navigation at a bound position returns exactly the checker's binding spans for that revision; an unbound position reports unbound rather than a lexical match, and a superseded revision is stale. | Test (TC-223) |
| FR-303-AC-5 | Completion at a position proposes exactly the declarations admitted there under the selected profile, each with its resolved type and owning declaration identity; a declaration the profile prohibits is not proposed, and a superseded revision is stale. | Test (TC-223) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
