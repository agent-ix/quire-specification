---
id: TC-427
title: "The v2 body grammar fixes package depth by schema"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-427: The v2 body grammar fixes package depth by schema

## Description

Checks that every published v2 body is in the five-stratum grammar, that a
nested inline term is a malformed wire, that a deep expression yields a
package of fixed JSON depth, and that read takes no digest evidence. Rows
BG-01 and BG-02 run in this repository's qualification crate; BG-03 and BG-04
run against the producer and the IR reader.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| BG-01 | Every node body of every published v2 package fixture, and every application node-identity preimage of `node-identity-vectors.json` | Each is in the grammar, and each preimage fits the `ApplicationNode` schema | FR-322-AC-39 |
| BG-02 | Each `body_grammar_mutations` entry of `fixtures/adverse.json` applied to `positive-all-families.json`: an application inside an application's arguments, an application in an aggregate's members, an application as a binding's value, an aggregate inside a Group, and a `binding` as a body root; then each with its `flattened` replacement, which names the composite subterm by `reference` | Each mutation fails the V2 schema and is outside the grammar (`refused:malformed_wire`); each flattened form passes the schema and is in the grammar | FR-322-AC-40 |
| BG-03 | A package holding a 100,000-deep sum, and one holding the one-level sum `x + x` | Both have the same maximum JSON nesting depth; the deep package is emitted, read back and admitted under node, edge and byte limits sized for its node count; no reader limit or effective-limit record names a depth | FR-322-AC-41 |
| BG-04 | A package whose lock names source and definition digests, read with no source or definition bytes and no digest evidence | The package is admitted | FR-322-AC-42 |
| BG-05 | An `abstraction_relation` body with one object entry carrying one field, one population entry and one frame entry carrying one parameter; then the same body with a further level nested in an object entry's field | The first fits the schema's `AbstractionRelationBody`, is a Body production and has JSON depth 5, fixed by the entry shapes whatever the number of entries; the second fails the schema | FR-322-AC-39 |

## Expected Results

The grammar holds for every published body and every application preimage,
every nested inline composite is a malformed wire, a deep expression changes
the node count and not the JSON depth, and read verifies no source or definition
digest against caller evidence.
