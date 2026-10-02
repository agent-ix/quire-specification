---
id: TC-060
title: "Separate conformance and advanced analysis claims"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-060
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-290
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-291
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-292
    type: verifies
---
## Description

Verify that monitorability, conformance, projection, realizability, refinement and composition have independent premises and dispositions.

## Test Procedure

Use an unprojectable but globally monitorable protocol, a satisfiable trace
without a causal implementation strategy and local contracts with circular
assumptions. Request every claim from a backend supporting only
`global-conformance`, then make one selected advanced backend return a typed
failure while an independent replay result remains valid. Enumerate every FR-290
protocol-family member, and pair each non-protocol member with a protocol
fragment and backend to confirm it refuses as mismatched. Independently mutate
or omit claim kind, fragment identity and construct membership, and backend
identity and capability. Pair otherwise valid fragments and backends with
disjoint claim capabilities and attempt display-name selection and fallback.
Request a claim whose required capability no registered backend advertises.

## Expected Results

Global results remain available. Each advanced claim is refused, unsupported, failed or separately decided with its exact fragment and premises; no fallback runs, and satisfiability or circular assumptions establish no stronger success. Only exact compatible claim/fragment/backend selections dispatch. Unknown, absent, mismatched or display-derived selections expose no successful result. A claim whose required capability no registered backend advertises settles `unsupported` with a warning naming that capability, and every independent result stays available.
