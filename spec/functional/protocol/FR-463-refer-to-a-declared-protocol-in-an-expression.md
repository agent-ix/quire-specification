---
id: FR-463
title: "Refer to a declared protocol in an expression"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-330
    type: references
---

# FR-463: Refer to a declared protocol in an expression

## Description

The checker SHALL accept `protocol` and a protocol's name as an expression that
refers to a protocol declared in the same unit, give the reference the target's
semantic type and bind it to the target's typed node key, never to the name.

## Inputs

A unit that selects `quire.protocol.complete/v1` and holds an expression
containing a protocol reference.

## Outputs

A checked reference bound to the target protocol's typed node key, or a
refusal.

## Surface syntax

The complete-V1 `primary` production admits one more alternative:

```ebnf
primary            = … | protocol-reference ;
protocol-reference = 'protocol', qualified-name ;
```

`protocol-reference` is admitted wherever the grammar admits a `primary`.
`protocol` leads a reference only in expression position, where no declaration
can begin, so it never changes how a `protocol-clause` declaration parses. The
target is a `qualified-name`; the expression carries no package selector,
alias or key. A target that is not a `qualified-name` refuses
`invalid_syntax`.

## Behavior

**Resolution.** The checker SHALL resolve the written name once, in the unit's
own declaration scope, to a protocol declaration of the unit, and SHALL bind
the checked reference to that declaration's `protocol`/`protocol_clause` node
key. A name offered only by an import does not resolve: a reference binds a
local protocol only.

**Type and value.** The checked type of the reference is the target's semantic
type (today the Boolean type node the protocol clause carries). The value of the
reference is the value of the target protocol clause: a pure reference denotes
the clause it names and is evaluated exactly as that clause is.

**Carrier.** The checked package carries the reference as one
`protocol`/`protocol_reference` node whose body is exactly
`{term: "reference", target}`, as FR-322-AC-66 and FR-330-AC-4 specify. The
node's key follows its bound target. Occurrences are not part of the key, so
several references to one protocol share one node carrying one `expression`
occurrence, with its own source-map entry, for each reference.

**Refusals.** A name that resolves to no local declaration, or only to an
imported one, refuses `missing_declaration`/`missing-name` at the reference. A
name that resolves to a declaration that is not a protocol refuses
`ill_typed`/`operator-ineligible` at the reference, naming the resolved kind.
No refusal produces a package.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-463-AC-1 | `protocol Fill` and `protocol M::Fill` parse as `protocol-reference` wherever a `primary` is admitted, each with the written `qualified-name` as its target and no package selector. A unit that holds a `protocol-clause` declaration and a reference parses both, and the declaration parses as without the reference. | Test (TC-943) |
| FR-463-AC-2 | `protocol`, `protocol 3` and `protocol (Fill)` refuse `invalid_syntax` at the token that is not a `qualified-name`. | Test (TC-943) |
| FR-463-AC-3 | In a unit that declares `protocol Fill`, `protocol Fill` checks to a reference bound to the typed node key of `Fill`'s `protocol`/`protocol_clause` node, typed with that node's semantic type. | Test (TC-943) |
| FR-463-AC-4 | `protocol Missing` refuses `missing_declaration`/`missing-name` at the reference; a name offered only through an import alias refuses the same way; a name that resolves to a declared function refuses `ill_typed`/`operator-ineligible`. | Test (TC-943) |
| FR-463-AC-5 | The checked package for a unit holding `protocol Fill` carries one `protocol`/`protocol_reference` node whose body is exactly `{term: "reference", target: <Fill's key>}`, as FR-322-AC-66 specifies, and the reference's key changes when `Fill`'s key changes. | Test (TC-943) |

## Dependencies

- [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)
  (FR-322-AC-66) owns the carrier and its admission rules.
- [FR-330](../../objects/interfaces/FR-330-contract-package.md) (FR-330-AC-4)
  owns the package's single semantic node collection.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns
  the `primary` production this requirement extends.

## References

- Tickets: Linear QSL-770; paired QSL-764.
