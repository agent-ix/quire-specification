---
id: TC-937
title: "ContractPackage reader admits only a complete canonical value"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-330
    type: verifies
---
# TC-937: ContractPackage reader admits only a complete canonical value

## Description

Verify that the single ContractPackage reader enforces the closed wire
contract before returning a typed value to any consumer.

## Test Procedure

Read one independently constructed valid canonical ContractPackage. Repeat
with one mutation per case: wrong FR-330 contract version; a checked-package
version substituted in the outer tag; missing, duplicate or unknown member;
wrong member kind; unresolved or incompatible graph reference;
missing source-map entry; accounting/request mismatch; and stale identity.
For each refusal, capture the typed result before invoking a consumer.

## Expected Results

The valid value returns the existing typed `admitted` envelope. Each mutated
value returns the FR-330 typed `refused` result using FR-322's generic
wire/canonical checks and FR-330's own contract-version rule, with the
applicable FR-322 code/cause mapping and FR-272 diagnostic identifying its
stage and available loci. No consumer receives or reparses that value. No alternate reader accepts a value refused by the owner.
