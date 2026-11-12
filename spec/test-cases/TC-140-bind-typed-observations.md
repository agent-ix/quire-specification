---
id: TC-140
title: "Bind typed observations without implicit values"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-110
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-296
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-260
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-261
    type: verifies
---
## Description

Accepted FS04 portable OB01 controls for [FR-110](../functional/observation/FR-110-bind-typed-observations.md). No executable standard coverage is claimed.

## Test Procedure

Bind a valid provider effect, then independently substitute its entity, signal,
unit, trigger, and schema identity; remove a required field; and change
internal/external visibility. Change the selected binding-contract definition
identity, then separately change only the concrete mapping and admitted-record
set under the original selected definition.
Permute object-member presentation, mutate each FR-296 key member one at a time
and then each attribute member, remove a key member, and pair one key with
unequal observation members. Admit three records for one binding identity and
subject in a known progress order, then a record with the same binding identity
and another subject, and one with another binding identity and the same subject.
Offer a caller-supplied node, instance and ordinal in place of the derivation.
Admit a record whose binding identity resolves to no declared role mapping.
Change a record's trace ID, signal name and timestamp outside progress position.
Supply a correction to one of the three records, then supply a correction with the exact immutable
predecessor and then with missing, self-referential and unknown predecessors.

## Expected Results

Only the valid record is admitted with exact identities. Each substitution is
typed refusal, missing value is incomplete/refused rather than false, and
visibility remains retained. A changed binding-contract definition identity
requires a newly linked subject; changed concrete mapping or records
retain the same static subject and produce a distinct identified assessment input.
Presentation permutation preserves the observation key; every key-member
mutation changes it and an attribute mutation does not. Missing keys, the same
key on unequal members and invalid predecessor relations refuse without a
partial admitted observation. A valid correction preserves predecessor members
and carries a new source ordinal.
The three records are occurrences one to three of one node and instance by
progress position then source ordinal; the other two are a different instance and
a different node. Caller attribution is refused. A binding identity resolving to
nothing is refused. The trace ID, signal and timestamp changes leave the derived
node, instance and occurrence unchanged. A correction replaces its predecessor
and adds no occurrence.
