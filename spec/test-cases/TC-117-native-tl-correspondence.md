---
id: TC-117
title: "Prove or refuse native-to-TL temporal correspondence"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-090
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-092
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-095
    type: verifies
---
## Description

Planned independent differential and identity-mutation controls for the native
temporal bridge.

## Test Procedure

Over the supported event-position/fixed-sample bounded-future domain, generate
formulas, total Boolean valuations, closed traces and open prefixes and compare
an independent native oracle with the exact selected TL evaluator profiles.
Include the one-position `always[0,1] p` closure discriminator. Request
timestamped finite-window and each past operator from current TL. Independently
mutate every native/TL profile, source/span, model/predicate, clock, capture,
interval, closure/history and bridge identity. Offer numeric, nullable,
unavailable and pending proposition inputs.

## Expected Results

Every supported vector agrees and retains full correspondence. Finite-window,
timestamped and current past-time mappings return typed unsupported without a
substitute formula. Each identity mutation fails on its own axis, and only total
native Boolean values enter the TL proposition population.
