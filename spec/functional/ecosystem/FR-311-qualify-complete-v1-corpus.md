---
id: FR-311
title: "Qualify the complete V1 capability corpus"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-011
    type: references
  - target: ix://agent-ix/quire-specification/FR-020
    type: references
---

# FR-311: Qualify the complete V1 capability corpus

## Description

When evaluating complete-V1 conformance, the qualification runner SHALL execute
positive, boundary and refusal vectors for every atomic inventory capability.

## Inputs

Independent conformance corpus, qualification profile and selected
producer/runtime/backend/tool subjects.

## Outputs

Per-capability results, a coverage report and a scoped qualification
disposition.

## Behavior

Qualification profiles distinguish language, runtime, monitor, analyzer/backend,
tool and consuming-system claims. Every inventory ID appears exactly once as a
primary row and may have multiple vectors, separately for grammar, semantic
kernel, runtime/monitor, analysis, backend, tooling and consuming integration
where applicable. Each vector result is `passed`, `failed`, `unsupported` or
`unexecuted`. A missing, unsupported, failed or unexecuted vector remains a gap
in the scope that contains it. A scope claims complete success only when every
required vector in it passed.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-311-AC-1 | The corpus maps every inventory ID exactly once and includes positive, boundary and refusal behavior where applicable. | Test (TC-231) |
| FR-311-AC-2 | A missing/unexecuted/unsupported vector prevents only its containing qualification scope from claiming complete success. | Test (TC-231) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
