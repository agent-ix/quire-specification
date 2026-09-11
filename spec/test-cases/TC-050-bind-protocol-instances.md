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

Link O1 and O2 concurrently through shared provider P. Independently substitute an endpoint/channel for a role, a component for a role instance, and duplicate or remove each required binding.

## Expected Results

The valid package retains distinct static and runtime identities. Every adverse substitution refuses at its source location; one invalid dependency does not erase an independent valid definition.
