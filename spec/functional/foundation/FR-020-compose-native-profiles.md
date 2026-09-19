---
id: FR-020
title: "Report the actual composed language capability scope"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-005
    type: implements
  - target: ix://agent-ix/quire-specification/FR-030
    type: references
  - target: ix://agent-ix/quire-specification/FR-031
    type: references
---
## Description

When reporting a language release's capability scope, the publisher SHALL limit
its full Quire v1 claim to the required capabilities supported under the exact
selected definitions and corresponding conformance evidence.

## Inputs

The versioned v1 capability inventory, exact semantic definition selections,
implementation/backend support declarations and scoped conformance references.

## Outputs

An explicit capability report identifying supported scope and remaining gaps;
a partial profile/package release remains distinguishable from full Quire v1.

## Behavior

The required scope is the complete 200-row atomic V1 inventory, including state,
types/models, bounded and unbounded temporal meaning, dynamic choreography,
runtime/workflow, analysis and synthesis, exact backends, tooling/mappings and
qualification across systems, related workflow instances and declared
populations. One implemented construct in each family is not the entire
capability set.
Missing, unsupported, unimplemented or unqualified required capabilities remain
explicit gaps. Planned cases or historical-profile tests alone do not establish
the new definition's conformance. A smaller release may identify its own exact
supported scope without claiming full Quire v1.

The standard defines this claim boundary; existing publication/conformance
consumers own its implementation. No new centralized release or evidence store
is introduced. Dependency order, provider limitation or a failed experiment
cannot remove a capability from the required V1 inventory.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-020-AC-7 | A report missing any required v1 family cannot claim full Quire v1 conformance. | Test (TC-020) |
| FR-020-AC-8 | A report with all three families but a missing required related-instance or population capability retains that gap and cannot claim full v1. | Test (TC-020) |
| FR-020-AC-9 | Each supported capability names its exact selected definition, implementation scope and corresponding evidence; planned or old-profile evidence alone cannot discharge it. | Test (TC-020) |
| FR-020-AC-10 | A partial-profile release identifies its supported scope accurately without representing itself as full V1; every missing inventory capability remains open implementation or qualification work. | Test (TC-020) |
| FR-020-AC-11 | A capability report accounts for all 200 inventory IDs exactly once and has no deferred, later, optional-product-scope or silently omitted disposition. | Test (TC-180) |

## Dependencies

- [Complete V1 capability inventory](../../../docs/v1-capability-inventory.md).
- [Exact definition binding](FR-030-bind-composed-definitions.md).
- [Per-request capability results](FR-031-report-requested-capabilities.md).
- [Planned matrix](../../composed-foundation/tests.md).

The earlier **unreviewed** FR-020 bundled unrelated obligations. Its AC-1/2
are now owned by FR-030; AC-3 by FR-033/034/040; AC-4/5 by FR-031; and AC-6 by
FR-032. Those old criterion IDs are not reused for different behavior. AC-7
retains its original release-scope meaning. No historical implemented profile
or accepted review is changed by decomposing this draft.
