---
id: TC-932
title: "ContractPackage retains the exact checked package identity and lock"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-330
    type: verifies
---
# TC-932: ContractPackage retains the exact checked package identity and lock

## Description

Verify that the output value binds to the exact admitted checked package and
does not reconstruct or alter its package identity or lock.

## Test Procedure

Admit a checked package whose lock has at least two ordered profile selections
and one dependency selection. Lower one requested item. Compare the
ContractPackage `package.source.package_id` and `package.source.lock` byte-for-byte
as typed values with the admitted checked package. Then mutate the
ContractPackage's source package identity, one lock selection, and
`package.lowering_profile` independently;
also choose a lowering profile whose required semantic selection conflicts
with the lock.

## Expected Results

The unmodified ContractPackage retains the exact `source.package_id`, lock
members, selected identities and order, plus the explicit lowering
`DefinitionRef` nested in `package`. Identity or lock mutation is refused as a
typed provenance error. The conflicting lowering profile makes each affected
request `invalid_input` before lowering emits a node. No default profile or
reconstructed lock is admitted.
