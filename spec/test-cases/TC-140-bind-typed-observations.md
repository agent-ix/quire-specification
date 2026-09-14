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

Planned OB01 controls for [FR-110](../functional/observation/FR-110-bind-typed-observations.md). No executable test or completed coverage is claimed.

## Test Procedure

Bind a valid provider effect, then independently substitute its entity, signal,
unit, trigger, and schema revision; remove a required field; and change
internal/external visibility. Change the selected binding-contract definition,
revision, or byte digest, then separately change only the concrete mapping and
admitted-record set under the original selected definition and revision.
Independently recompute the FR-296 identity; permute object-member presentation,
mutate every semantic/provenance member, remove or stale the identity, change
its digest domain, and pair one identity with unequal canonical bytes. Supply a
correction with the exact immutable predecessor and then with missing,
self-referential and digest-mismatched predecessors.

## Expected Results

Only the valid record is admitted with exact identities. Each substitution is
typed refusal, missing value is incomplete/refused rather than false, and
visibility remains retained. A changed binding-contract definition, revision, or
byte digest requires a newly linked subject; changed concrete mapping or records
retain the same static subject and produce a distinct identified assessment input.
Presentation permutation preserves the derived observation identity; every
semantic/provenance mutation changes it. Missing/stale/cross-domain identities,
same identity on unequal bytes and invalid predecessor relations refuse without
a partial admitted observation. A valid correction preserves predecessor bytes
and obtains a new identity.
