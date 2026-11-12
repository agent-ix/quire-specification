---
id: TC-050
title: "Bind exact protocol roles and instances"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-050
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-210
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-211
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-212
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-213
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-214
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-215
    type: verifies
---
## Description

Verify exact role, role-instance and workflow-instance binding for internal and external participants.

## Test Procedure

Link O1 and O2 concurrently through shared provider P, including two role slots
with equal display text or one compatible participant/provider. Compare static
role/channel handles and authority-qualified workflow, role-instance,
participant and message identities. Bind one participant compatibly to two
distinct role instances. Independently substitute an endpoint/channel for a
role, a participant or component for a role instance, or a role instance,
component or static binding-slot identity for a participant. Remove a concrete
runtime identity, use the binding-requirement identity as the runtime identity,
change authority/kind, reuse one
complete key across incompatible definition/role/workflow/configuration/
obligation/channel facts, and permute key order. For every static role the binding supplies, exercise
exact `internal`, exact `external`, absence, an unknown/differently cased value
and inference from model/provider authority.

## Expected Results

The valid package retains distinct static and runtime identities and canonical
key order without creating causal or delivery order. Every absent, placeholder,
foreign, colliding or incompatibly rebound identity refuses at its source
location; one shared participant never merges its role instances. Internality
is a binding fact that admits exactly one closed member per bound role and establishes no adequacy by itself; the checked package role carries none and is never defaulted to one.
One invalid dependency does not erase an independent valid definition.
