---
id: FR-309
title: "Bind non-authoritative analysis and codegen consumers"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
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
consumer options.

## Outputs

Typed findings/generated artifacts with identities, source maps and accounting,
or unsupported/refused/incomplete/failed result.

## Behavior

Consumers cannot admit source, redefine semantics or turn heuristics into proof.
Each finding states its method and confidence/claim kind. Each generated artifact
accounts for every requested source obligation and preserves or reports loss.

## Consumer authority contract

Every consumer request names semantic subject, requested obligation identities,
method/profile, limits and output format. The response accounts for each
obligation as generated, unsupported, requires-bound, invalid or failed and
records any representation loss. Analysis findings are evidence about a subject,
not edits to that subject. Generated source includes correspondence back to
every represented obligation; a consumer that cannot preserve required meaning
emits no artifact for that obligation.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-309-AC-1 | Every finding/artifact identifies exact input package, consumer profile, tool/options and source correspondence. | Test (TC-229) |
| FR-309-AC-2 | Unsupported nodes produce exact accounting and no approximating generated code. | Test (TC-229) |
| FR-309-AC-3 | A heuristic finding remains non-authoritative and cannot satisfy a proof/qualification claim. | Test (TC-229) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
