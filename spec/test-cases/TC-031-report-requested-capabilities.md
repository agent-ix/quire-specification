---
id: TC-031
title: "Report each requested assessment capability independently"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-031
    type: verifies
---
## Description

Planned acceptance controls for [FR-031](../functional/FR-031-report-requested-capabilities.md).
No executable test or completed coverage is claimed by this artifact.

## Test Procedure

Request replay and local projection for the same admitted protocol with a backend supporting only replay. Separately request independent state evaluation and unsupported temporal lowering. Inject one failed prerequisite shared by two requests while retaining a third independent request.

Select the exact state-graph, timestamped temporal and finite-global protocol
roots in the [definition catalog](../../proposals/quire-v1/definitions/README.md).
Evaluate a total state predicate, request native timestamped monitoring and ask
the current TL bridge to lower that same temporal subject. Retain the separately
requested global protocol conformance result. A native meaning cannot be switched
to event-position semantics merely to make its TL request supported.

On an admitted open prefix, let a required temporal existential receive its
decisive witness while the surrounding workflow remains open. The temporal
truth/witness stays settled under FR-091; complete global conformance stays
unavailable under FR-059. The composed result mapping must retain both scopes
without treating the witness as workflow closure. This is the concrete unresolved
B/E mapping control, not a claim that FR-061 or a reader already represents it.

## Expected Results

Every request has a disposition with typed cause/claim identity. Replay and independent state outcomes remain available under their precise scope. Unsupported requests cannot disappear; failed dependents do not execute or inherit another request's success.
