---
id: FR-122
title: "Generate bounded OCL 2.4 output"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-121
    type: references
  - target: ix://agent-ix/quire-specification/FR-033
    type: references
  - target: ix://agent-ix/quire-specification/FR-042
    type: references
---
## Description

When `quire.output.ocl24/v1` revision `1-draft.1` is selected, the OCL mapper SHALL
generate only the admitted OCL 2.4 invariant/pre/post fragment.

## Inputs

An admitted common request, total checked native expressions and exact
authoritative context/type/operation/field/collection correspondences.

## Outputs

Deterministic OCL 2.4 text regions and FR-298 records, including explicit
conditional/unrepresented/refused records for unsupported meaning.

## Behavior

The mapper SHALL follow the selected fragment in the FS06 OCL profile. Native
pre-state field reads use `@pre` only under an exact operation/receiver/field
mapping. Ordered duplicate-preserving inputs map only to OCL `Sequence`.

The mapper SHALL classify every source construct outside the admitted fragment
without emitting an approximate substitute.

The mapper SHALL NOT equate OCL null/invalid with native absence/unavailability,
enlarge bounded integer claims, change arithmetic overflow behavior, convert
one-based indexing, discard a relationship, or map temporal/protocol behavior
as a state invariant. Tool/library acceptance is not semantic preservation.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-122-AC-1 | Total Boolean/scalar invariants and exact pre/post field comparisons generate the selected OCL fragment with source and model correspondence. | Test (TC-151) |
| FR-122-AC-2 | Native bounded integer results agree throughout the admitted domain; the target's wider domain cannot enlarge the native claim. | Test (TC-151) |
| FR-122-AC-3 | Optional/invalid/rational/index/unsupported collection/graph/temporal/protocol cases are conditional, unrepresented or refused exactly as the profile states. | Test (TC-151) |
| FR-122-AC-4 | ConfigVersion scalar and pre/post examples preserve their exact bounds/anchors; ParentPrecedes is either exactly mapped or wholly unrepresented/refused. | Test (TC-151) |
| FR-122-AC-5 | Generated OCL cannot be read back as native source or gain preservation from parser/typechecker acceptance. | Test (TC-155) |

## Dependencies

- [OCL 2.4 profile](../../../proposals/quire-v1/output-mappings/ocl-2.4.md).
- [FR-121](FR-121-account-for-every-output-obligation.md).
- Qcir #55 owns implementation.

## Status

Accepted by FS06; executable exporter and correspondence evidence remain qcir #55.
