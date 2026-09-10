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

## Expected Results

Every request has a disposition with typed cause/claim identity. Replay and independent state outcomes remain available under their precise scope. Unsupported requests cannot disappear; failed dependents do not execute or inherit another request's success.

