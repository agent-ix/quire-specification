---
id: FR-339
title: "Define the complete native V1 grammar"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-131
    type: references
  - target: ix://agent-ix/quire-specification/AD-003
    type: references
---

# FR-339: Define the complete native V1 grammar

## Description

When a source declares the complete V1 profile, the parser SHALL recognize
exactly the composed EBNF and lexical rules in the complete-V1 grammar contract.

## Inputs

UTF-8 source bytes, selected native edition and complete-V1 profile identity.

## Outputs

A lossless source tree containing every token and trivia occurrence, or located
lexical/syntactic refusals.

## Behavior

The [complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) is
incorporated by this requirement as the normative syntax contract. Its base
grammar and complete-V1 facets form one grammar; a conforming implementation may
factor parser modules but cannot expose backend-specific syntax, omit a facet or
accept an undeclared extension under the complete-V1 identity. Reserved words,
literal spellings, operator precedence and declaration forms are versioned by
the linked language profile. Lossless parsing preserves source bytes and loci
without granting syntactically invalid input semantic meaning.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-339-AC-1 | Every production in the base and complete-V1 facet grammar has positive, boundary and located-negative corpus vectors. | Test (TC-180) |
| FR-339-AC-2 | Parsing and lossless rendering reproduce accepted UTF-8 source bytes exactly, including comments and whitespace. | Test (TC-180) |
| FR-339-AC-3 | Backend installation cannot add, remove or reinterpret syntax under the same language/profile identity. | Test (TC-180) |
| FR-339-AC-4 | An undeclared keyword, operator or extension refuses at its exact source span without producing a checked declaration. | Test (TC-180) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) is the
  incorporated lexical and syntactic authority.
- [Subsystem and interface index](../../subsystems/index.md) owns the parser and
  source-package boundary.
- Every requirement linked in frontmatter is a normative prerequisite.
