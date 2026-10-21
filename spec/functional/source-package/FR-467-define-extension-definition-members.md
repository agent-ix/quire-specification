---
id: FR-467
title: "Define an extension definition's grammar and typed-node schemas"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-133
    type: references
  - target: ix://agent-ix/quire-specification/FR-131
    type: references
  - target: ix://agent-ix/quire-specification/FR-339
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---

# FR-467: Define an extension definition's grammar and typed-node schemas

## Description

An extension definition that adds a declaration form SHALL carry two members besides its identity and dependency edges: a grammar schema and a typed-node schema. The members make an extension's forms readable by any consumer from the definition alone, so adding an extension adds definition data and no consumer code.

## Inputs

An extension definition, the core grammar ([FR-339](FR-339-define-complete-native-grammar.md)), the core node kinds of the checked package ([FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)) and the other extension definitions a subject selects.

## Outputs

A validated extension definition that [FR-133](FR-133-extend-language-uniformly.md) can make selectable, or a refusal naming the definitions involved.

## Behavior

### Grammar schema

The grammar schema lists one or more declaration-form productions over the declarative token vocabulary: the identifiers, quoted strings, numeric literals and punctuation of the [complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) lexical contract, and the core `type-ref` production. A production introduces no token class of its own.

Each production begins with one keyword, its *leading keyword*. A leading keyword is recognised as a keyword only as the first token of a declaration. Anywhere else the same spelling remains an ordinary identifier, so selecting an extension changes the meaning of no source that already parses.

### Typed-node schema

The typed-node schema gives each production exactly one node kind and one typed subnode per clause of the production. A subnode names the checked type or the node kind its clause must check against. A node kind is unique among the core node kinds and the node kinds of every extension a subject selects.

### Validation

The members are validated before the extension is selectable, in this order, and the first violation is reported:

1. `missing-member`: the definition has no grammar schema, no typed-node schema, a production the typed-node schema does not cover, or a production with no node kind.
2. `conflicting-definition`: a leading keyword equals a core keyword or another selected extension's leading keyword; or a node kind equals a core node kind or another selected extension's node kind.
3. `definition-cycle`: the extension's dependency edges, together with those of the extensions a subject selects, contain a cycle.

Each refusal names every definition involved, the offending member and, for a collision, the keyword or node kind. A consumer reads the two members and nothing else of the extension to admit its forms; it does not infer a member from the definition's bytes, name or position.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-467-AC-1 | An extension definition with a grammar schema whose one production begins with the keyword `retention` and has clauses `subject: Text` and `days: Int[1, 3650]`, and a typed-node schema giving it a node kind not held by the core, validates and is selectable. | Test (TC-949) |
| FR-467-AC-2 | A definition missing its grammar schema, its typed-node schema, the node-kind entry of a production, or a subnode type for a clause refuses `missing-member` naming the definition and the missing member. | Test (TC-949) |
| FR-467-AC-3 | A production whose leading keyword is `function`, or equals another selected extension's leading keyword, refuses `conflicting-definition` naming both definitions and the keyword. | Test (TC-949) |
| FR-467-AC-4 | A node kind equal to a core node kind, or to another selected extension's node kind, refuses `conflicting-definition` naming both definitions and the node kind. | Test (TC-949) |
| FR-467-AC-5 | Two extensions that depend on each other refuse `definition-cycle` naming both in path order. | Test (TC-949) |
| FR-467-AC-6 | A definition violating both a missing member and a keyword collision reports `missing-member` only, and one violating both a collision and a cycle reports `conflicting-definition` only. | Test (TC-949) |
| FR-467-AC-7 | Source that uses the word `retention` as an identifier outside a declaration's first token parses identically with and without the extension selected. | Test (TC-949) |

## Dependencies

- [FR-133](FR-133-extend-language-uniformly.md) owns what an extension may change; this requirement owns the definition members that carry its forms.
- [FR-339](FR-339-define-complete-native-grammar.md) owns the core keywords and token vocabulary.
- [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md) owns the core node kinds.
- [FR-131](FR-131-compose-complete-profile-bundle.md) owns the bundle in which the definitions are selected.
