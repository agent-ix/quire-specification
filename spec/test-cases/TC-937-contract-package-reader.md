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
with one mutation per case: wrong contract version; missing, duplicate or
unknown member; wrong member kind; unresolved or incompatible graph reference;
missing source-map entry; accounting/request mismatch; and stale identity.
For each refusal, capture the typed result before invoking a consumer.

## Expected Results

The valid value returns the existing typed `admitted` envelope. Each mutated
value returns a typed FR-322/FR-272 `refused` result naming its applicable
stage, code, cause and available loci, and no consumer receives or reparses
that value. No alternate reader accepts a value refused by the owner.
