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

A validated extension definition that [FR-133](FR-133-extend-language-uniformly.md) can make selectable, or an `invalid_package` refusal naming the definitions involved.

## Behavior

### Production

A *production* is one declaration form of an extension. Every production has exactly this shape over the [complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) lexical contract:

```ebnf
extension-decl = leading-keyword, ident, '{', [ clause, { ',', clause } ], '}' ;
clause         = ident, ':', expr ;
```

The `ident` after the leading keyword is the declared name of the form. A clause binds its `ident` to an `expr` of the common expression grammar, whose value is checked against the clause's type. Clauses are separated by `,` with no trailing separator, and appear in the order the production lists them. A production introduces no token class and no other punctuation. `retention Audit { subject: "audit", days: 30 }` is a form of the production with leading keyword `retention` and clauses `subject` and `days`.

A source form that does not match its production is refused at the source level, not as an extension-definition refusal: a form missing the declared name, a brace, a separator or a clause value, or carrying a trailing separator, refuses `invalid_syntax`/`unexpected-token` or `unexpected-end` naming the actual token and the expected category; a clause out of the production's order, an unknown clause name or a repeated clause refuses `invalid_syntax`/`unexpected-token` naming the clause token and the clause the production expects there.

A leading keyword is recognised as a keyword only as the first token of a top-level `declaration` of the unit. It is not recognised as a keyword in a member declaration inside a model, record, state or protocol body or in an expression. Anywhere else the same spelling remains an ordinary identifier, so selecting an extension changes the meaning of no source that already parses.

### Members

The definition's exact document holds the two members as JSON values:

- `grammar_schema`: `{"productions": [ {"keyword": K, "clauses": [N, ...]} , ... ]}`, with `K` the leading keyword and each `N` a clause name, in source order, at least one production.
- `typed_node_schema`: `{"nodes": [ {"keyword": K, "node_kind": D, "clauses": [ {"name": N, "type": T} , ... ]} , ... ]}`, one node entry per production, matched to it by `K`, with `D` the node kind and `T` the spelling of a core `type-ref` or of a node kind the clause's value must check against. The entry lists the production's clause names in the production's order.

### Validation

When an extension definition is validated, the definition reader SHALL report the first violated rule below and SHALL report no later one, each as `invalid_package` with the stated cause:

1. `missing-member`: the definition has no `grammar_schema`, no `typed_node_schema`, no production, a production with no node entry, a node entry with no `node_kind`, a production clause with no entry in its node entry, a clause entry with no `type`, a node-entry clause the production does not list, or a node entry whose `keyword` matches no production (the missing member is that production in `grammar_schema`). The refusal names the definition and the missing member; for a clause it names the production keyword and the clause.
2. `conflicting-definition`: a leading keyword equals a *core keyword*, another production's leading keyword in the same definition or in another selected extension; or a node kind equals a core node kind, another node entry's node kind in the same definition or in another selected extension. A *core keyword* is a reserved or contextual keyword of the grammar edition in force for the unit, as the grammar's lexical contract selects it. The refusal names the keyword or node kind, the definition, and the other definition or, for a core keyword, the core grammar; for a collision within one definition it names that definition and both production indices.
3. `definition-cycle`: the dependency edges of the extension, together with those of the extensions the subject selects, contain a cycle. The refusal names every definition of the cycle in path order.

The definition reader SHALL read the two members and the dependency edges of the extension and nothing else; it SHALL NOT infer a member from the definition's bytes, name or position.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-467-AC-1 | An extension definition whose `grammar_schema` has one production with leading keyword `retention` and clauses `subject` and `days`, and whose `typed_node_schema` gives it node kind `retention` with `subject: Text` and `days: Int[1, 3650]`, validates and is selectable, and the unit `retention Audit { subject: "audit", days: 30 }` parses as one form of that production. | Test (TC-949) |
| FR-467-AC-2 | A definition missing `grammar_schema`, missing `typed_node_schema`, a production's node entry, a node entry's `node_kind`, a clause's entry or a clause's `type`, or a node entry listing a clause the production does not, refuses `invalid_package`/`missing-member` naming the definition and the missing member. | Test (TC-949) |
| FR-467-AC-3 | A production whose leading keyword is the core keyword `function` refuses `invalid_package`/`conflicting-definition` naming the extension, the core grammar and `function`; a production whose leading keyword equals another selected extension's refuses the same naming both definitions and the keyword. | Test (TC-949) |
| FR-467-AC-4 | A node kind equal to a core node kind, or to another selected extension's node kind, refuses `invalid_package`/`conflicting-definition` naming the definition or definitions and the node kind. | Test (TC-949) |
| FR-467-AC-5 | Two extensions that depend on each other refuse `invalid_package`/`definition-cycle` naming both in path order. | Test (TC-949) |
| FR-467-AC-6 | A definition violating both a missing member and a keyword collision reports `missing-member` only, and one violating both a collision and a cycle reports `conflicting-definition` only. | Test (TC-949) |
| FR-467-AC-7 | Source that uses the word `retention` as an identifier outside the first token of a top-level declaration, including as a member declaration name inside a model body, parses identically with and without the extension selected. | Test (TC-949) |
| FR-467-AC-8 | Two productions of one definition with the same leading keyword, or two node entries with the same node kind, refuse `invalid_package`/`conflicting-definition` naming the definition and both production indices. | Test (TC-949) |
| FR-467-AC-9 | `retention Audit { subject: "audit", days: 30 }` is a form of the production, while `retention { subject: "audit" }` (no declared name), `retention Audit subject: "audit"` (no braces) and `retention Audit { subject: "audit", }` (trailing separator) each refuse `invalid_syntax`/`unexpected-token` (or `unexpected-end`) naming the actual token and the expected category; `retention Audit { days: 30, subject: "audit" }` (out of order), an unknown clause name and a repeated clause refuse `invalid_syntax`/`unexpected-token` naming the clause token and the expected clause. | Test (TC-949) |
| FR-467-AC-10 | A node entry whose `keyword` matches no production refuses `invalid_package`/`missing-member` naming the definition and the keyword; two node entries of one definition with one `keyword` refuse `invalid_package`/`conflicting-definition` naming the definition and both entry indices. | Test (TC-949) |

## Dependencies

- [FR-133](FR-133-extend-language-uniformly.md) owns what an extension may change; this requirement owns the definition members that carry its forms.
- [FR-339](FR-339-define-complete-native-grammar.md) owns the core keywords and token vocabulary.
- [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md) owns the core node kinds. The checked-package carrier for an extension's node kind is not defined by this requirement; it is tracked as STD-236.
- [FR-131](FR-131-compose-complete-profile-bundle.md) owns the bundle in which the definitions are selected.
