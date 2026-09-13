---
id: TC-050
title: "Bind exact protocol roles and instances"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-050
    type: verifies
---
## Description

Verify exact role, role-instance and workflow-instance binding for internal and external participants.

## Test Procedure

Link O1 and O2 concurrently through shared provider P, including two role slots
with equal display text or one compatible participant/provider. Compare static
role/channel handles and authority-qualified workflow, role-instance and message
identities. Independently substitute an endpoint/channel for a role, a component
for a role instance, remove a concrete runtime identity, use the binding-
requirement identity as the runtime identity, change authority/kind, reuse one
complete key across incompatible definition/role/workflow/configuration/
obligation/channel facts, and permute key order.

## Expected Results

The valid package retains distinct static and runtime identities and canonical
key order without creating causal or delivery order. Every absent, placeholder,
foreign, colliding or incompatibly rebound identity refuses at its source
location; one invalid dependency does not erase an independent valid definition.
