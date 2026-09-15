---
id: FR-309
title: "Bind non-authoritative analysis and codegen consumers"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-195
    type: references
  - target: ix://agent-ix/quire-specification/AD-004
    type: references
---

# FR-309: Bind non-authoritative analysis and codegen consumers

## Description

When an analysis or code-generation consumer runs, it SHALL consume an exact
checked/IR package and emit source-bound typed findings or artifacts with
complete loss and capability accounting.

## Inputs

Checked or target-neutral package, consumer profile, requested outputs and
toolchain/options lock.

## Outputs

Typed findings/generated artifacts with identities, source maps and accounting,
or unsupported/refused/incomplete/failed result.

## Behavior

Consumers cannot admit source, redefine semantics or turn heuristics into proof.
Each finding states its method and confidence/claim kind. Each generated artifact
accounts for every requested source obligation and preserves or reports loss.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-309-AC-1 | Every finding/artifact identifies exact input package, consumer profile, tool/options and source correspondence. | Test (TC-229) |
| FR-309-AC-2 | Unsupported nodes produce exact accounting and no approximating generated code. | Test (TC-229) |
| FR-309-AC-3 | A heuristic finding remains non-authoritative and cannot satisfy a proof/qualification claim. | Test (TC-229) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
