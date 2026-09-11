---
id: TC-140
title: "Bind typed observations without implicit values"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-110
    type: verifies
---
## Description

Planned OB01 controls for [FR-110](../functional/FR-110-bind-typed-observations.md). No executable test or completed coverage is claimed.

## Test Procedure

Bind a valid provider effect, then independently substitute its entity, signal,
unit, trigger, and schema revision; remove a required field; and change
internal/external visibility. Change the selected binding-contract definition,
revision, or byte digest, then separately change only the concrete mapping and
admitted-record set under the original selected definition and revision.

## Expected Results

Only the valid record is admitted with exact identities. Each substitution is
typed refusal, missing value is incomplete/refused rather than false, and
visibility remains retained. A changed binding-contract definition, revision, or
byte digest requires a newly linked subject; changed concrete mapping or records
retain the same static subject and produce a distinct identified assessment input.
