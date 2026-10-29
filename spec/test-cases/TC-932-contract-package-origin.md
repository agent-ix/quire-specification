---
id: TC-932
title: "ContractPackage retains the exact checked package identity and lock"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-330
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-330-AC-5
    type: verifies
---
# TC-932: ContractPackage retains the exact checked package identity and lock

## Description

Verify that the output value binds to the exact admitted checked package and
does not reconstruct or alter its package identity or lock.

## Test Procedure

Admit a checked package whose lock has at least two ordered profile selections
and one dependency selection. Lower one requested item. Compare the
ContractPackage `package.source.identity` and `package.source.lock` byte-for-byte
as typed values with the admitted checked package. Then mutate the
ContractPackage's source package identity, one lock selection, and
`package.lowering_profile` independently.

## Expected Results

The unmodified ContractPackage retains the exact `source.identity`, lock
members, selected identities and order, plus the explicit lowering
profile reference in closed `{authority, identity}` form nested in `package`.
No default profile or reconstructed lock is admitted.
