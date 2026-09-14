---
id: FR-121
title: "Account for every output obligation"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-116
    type: references
  - target: ix://agent-ix/quire-specification/FR-269
    type: references
  - target: ix://agent-ix/quire-specification/FR-298
    type: references
---
## Description

When mapping an admitted request, the selected mapper SHALL return exactly one
FR-298 record for every requested obligation and classify it as `preserved`,
`conditional`, `unrepresented` or `refused` without omission or approximation.

## Inputs

One admitted request and the exact common/target mapping rules.

## Outputs

One complete ordered record set suitable for FR-299 package assembly, or a typed
whole-operation failure with no partial package.

## Behavior

Each record SHALL retain source identity/revision/digest/span, required semantic
and model dependencies, target profile, classification, exact conditions/causes
and generated regions. `Preserved` requires every necessary source fact to be
represented without a semantic condition. `Conditional` retains nonempty
conditions; `unrepresented` and `refused` retain causes and claim no output region.

The mapper SHALL retain pending, incomplete and refused source results as their
exact source facts rather than preserved Boolean output.

The mapper SHALL retain observation adequacy and protocol adequacy under their
separate FR-245/FR-246 domains.

The mapper SHALL NOT accept or emit an unqualified `adequacy` field.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-121-AC-1 | Every requested obligation has exactly one source-bound FR-298 record and no unrequested record exists. | Test (TC-150) |
| FR-121-AC-2 | Preserved/conditional/unrepresented/refused records satisfy their exact output/condition/cause invariants. | Test (TC-150) |
| FR-121-AC-3 | Omitting, duplicating or cross-wiring a record/dependency/classification refuses assembly rather than defaulting to preservation. | Test (TC-154) |
| FR-121-AC-4 | Pending, incomplete or refused source outcomes and parser-accepted text cannot become preserved or Boolean success. | Test (TC-155) |
| FR-121-AC-5 | Observation and protocol adequacy remain independently typed, and bare adequacy refuses. | Test (TC-150) |

## Dependencies

- [FR-116](../observation/FR-116-map-observation-results-to-consumers.md).
- [FR-269](../../objects/observation/FR-269-consumer-mapping-loss.md).
- [FR-298](../../objects/output-mapping/FR-298-per-obligation-output-mapping-record.md).

## Status

Accepted by FS06 as the common loss-accounting rule.
