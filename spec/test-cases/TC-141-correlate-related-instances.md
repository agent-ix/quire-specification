---
id: TC-141
title: "Correlate related instances without heuristics"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-111
    type: verifies
---
## Description

Accepted FS04 portable OB01 correlation controls. TM-007 remains Planned; no
executable standard coverage is claimed.

## Test Procedure

Create concurrent O1/O2 with a shared provider. Type their relationships by
relationship declarations in the checked package's domain package.
Link a refund to O1's effect, offer it for O2, remove/link-conflict the relation,
replay one exact relationship and offer a second distinct relationship for the
same required slot. Reverse its endpoints and independently mutate each FR-287
endpoint member, relationship kind/declaration key, domain package identity, digest
domain and digest. Offer a relationship declaration key that is not in the
checked package. Repeat a delivery receipt and substitute its message,
delivery or receive identity in each other position.

## Expected Results

O1 is selected only by its declared relation. Missing is incomplete;
conflicting and multiple distinct matches refuse; exact replay is idempotent.
A declaration outside the checked package, endpoint reversal/substitution, repeated
delivery and every declaration-selection/domain mismatch refuse without
manufacturing an effect or reconstructed relationship. Receipt, delivery,
receive and message remain distinct.
