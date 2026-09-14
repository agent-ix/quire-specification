---
id: FR-123
title: "Generate bounded SysML 2.0 and KerML 1.0 output"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-121
    type: references
  - target: ix://agent-ix/quire-specification/FR-035
    type: references
---
## Description

When `quire.output.sysml2-kerml1/v1` revision `1-draft.1` is selected, the
SysML/KerML mapper SHALL generate only the admitted requirement/constraint
expression fragment over exact authoritative model bindings.

## Inputs

An admitted common request, total checked native expressions and exact SysML
2.0/KerML 1.0 type/unit/attribute/library correspondences.

## Outputs

Deterministic SysML requirement/constraint text regions and complete FR-298
records.

## Behavior

The mapper SHALL retain native identity in the package manifest and treat any
textual short name as display only. Generated local attributes SHALL reference
or correspond exactly to authoritative model declarations and SHALL NOT create
a second model authority.

The mapper SHALL preserve assumption/required-constraint separation only under
an exact selected structural correspondence. It SHALL classify unsupported
pre/post anchors, collections, relationships, temporal/history, protocol,
parts/ports/actions/states/allocations and model-interchange semantics without
generating plausible replacement structures.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-123-AC-1 | Selected total Boolean/scalar constraints generate a requirement definition with exact type/unit/attribute correspondence. | Test (TC-152) |
| FR-123-AC-2 | Native identity remains in the manifest and cannot be reconstructed from or replaced by a target short name. | Test (TC-152) |
| FR-123-AC-3 | Assumption/requirement splitting occurs only under an exact selected correspondence; otherwise source implication remains intact or unrepresented. | Test (TC-152) |
| FR-123-AC-4 | Unsupported model, anchor, collection, graph, temporal and protocol constructs receive explicit non-preserved records with no substitute model. | Test (TC-152) |
| FR-123-AC-5 | Parser/typechecker or pilot-tool acceptance cannot establish semantic preservation or target-wide conformance. | Test (TC-155) |

## Dependencies

- [SysML/KerML profile](../../../proposals/quire-v1/output-mappings/sysml-2.0-kerml-1.0.md).
- [FR-121](FR-121-account-for-every-output-obligation.md).
- Qcir #56 owns implementation; model producers retain declaration authority.

## Status

Accepted by FS06; executable exporter and correspondence evidence remain qcir #56.
