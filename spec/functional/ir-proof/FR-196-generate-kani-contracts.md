---
id: FR-196
title: "Generate exact Kani contracts and harnesses"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/AD-010
    type: references
  - target: ix://agent-ix/quire-specification/FR-195
    type: references
---

# FR-196: Generate exact Kani contracts and harnesses

## Description

When Kani generation is requested, the backend SHALL derive contract
preconditions, symbolic domains and harness bounds from the checked model and
record the exact Kani/Rust/options pins.

## Inputs

Target-neutral items, model domains, requested proof claims, Kani capability
profile and toolchain/options lock.

## Outputs

Rust contracts/harnesses with complete accounting, requires-bound/unsupported
dispositions, or generation failure.

## Behavior

Boolean, numeric, state, collection, graph, temporal and protocol items are
accepted only where the selected Kani profile encodes their exact semantics.
Finite bounds come from model domains or explicit proof requests. Unsupported
constructs emit no approximate harness. Generated crates are Rust-only,
`publish = false`, and pin the exact Kani version/options.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-196-AC-1 | Generated preconditions and symbolic ranges equal the model's inclusive domains, including boundary values. | Test (TC-218) |
| FR-196-AC-2 | Missing bounds or unsupported exact semantics produce requires-bound/unsupported and no harness for that item. | Test (TC-218) |
| FR-196-AC-3 | Evidence records Kani, Rust, dependency and option pins plus every requested obligation disposition. | Inspection |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
