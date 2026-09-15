---
id: FR-311
title: "Qualify the complete V1 capability corpus"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/AD-011
    type: references
  - target: ix://agent-ix/quire-specification/FR-020
    type: references
---

# FR-311: Qualify the complete V1 capability corpus

## Description

When evaluating complete-V1 conformance, the qualification runner SHALL execute
positive, boundary and refusal vectors for every one of the 176 atomic inventory
capabilities under one exact ecosystem lock.

## Inputs

Independent conformance corpus, qualification profile, ecosystem lock and
selected producer/runtime/backend/tool subjects.

## Outputs

Per-capability results, coverage/accounting report and scoped qualification
disposition.

## Behavior

Qualification profiles distinguish language, runtime, monitor, analyzer/backend,
tool and consuming-system claims. Every inventory ID appears exactly once as a
primary row and may have multiple vectors. Missing, unsupported, failed or
unexecuted vectors remain gaps. Only an attributed decision record promotes a
qualification result to release acceptance.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-311-AC-1 | The corpus maps all 176 inventory IDs exactly once and includes positive, boundary and refusal behavior where applicable. | Test (TC-231) |
| FR-311-AC-2 | A missing/unexecuted/unsupported vector prevents only its containing qualification scope from claiming complete success. | Test (TC-231) |
| FR-311-AC-3 | Exact same corpus and ecosystem lock reproduce canonical per-capability results; a changed pin creates a distinct run. | Test (TC-231) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
