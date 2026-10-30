---
id: TC-971
title: "A present option is an identity-bound value with a closed payload reference"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-971: A present option is an identity-bound value with a closed payload reference

## Description

Verify the closed body, structural application-node identity, and reader refusal
shapes for the V2 `value`/`option_value` node.

## Test Procedure

Read `positive-integer-literal-references.json`. Resolve each OV-01/OV-02
`element_type_selector` and `payload_selector` by the stated node tag, form,
literal fields and ordinal. Build the node id as the structural `{package, path, ordinal}`
whose `path` is the application's node tag, form and the selected option type
and payload node ids. Admit the package through the strict reader.
Confirm the node id differs when either the selected option type or
payload reference changes.

Apply each OV-SR1..OV-SR4 mutation to a valid body (empty members, wrong
binding name, inline literal instead of a reference, and multiple bindings)
and confirm it fails `$defs.OptionValueBody`.

For each OV-R1..OV-R5 reader refusal, mutate the valid option-value node as
specified and run the package through the strict reader. Assert its
`expected_code`, `expected_cause` and exact absolute JSON-pointer `locus`:

- OV-R1: a non-composite semantic type is `invalid_semantic_graph`, has no cause,
  and is refused at `/semantic_graph/nodes/{n}/semantic_type`.
- OV-R2: a non-option composite semantic type is `invalid_semantic_graph`, has
  no cause, and is refused at `/semantic_graph/nodes/{n}/semantic_type`.
- OV-R3: a Boolean value used as the Integer payload is
  `ill_typed`/`type-mismatch` at
  `/semantic_graph/nodes/{n}/body/members/0/value`.
- OV-R4: an option-type node used as the payload instead of a checked value is
  `ill_typed`/`type-mismatch` at
  `/semantic_graph/nodes/{n}/body/members/0/value`.
- OV-R5: a dependency list with an extra target is
  `invalid_semantic_graph`, has no cause, and is refused at
  `/semantic_graph/nodes/{n}/dependencies`.

Tag the tests `#[trace("TC-971", "FR-322-AC-244")]`.

## Expected Results

Both fixture-selected positive values are admitted, their node ids
include the option type and payload node ids. All four malformed bodies fail the closed schema.
The five reader mutations produce exactly the specified code, cause (or no
cause), and node-member locus. Present-option construction has no operator
identity.
