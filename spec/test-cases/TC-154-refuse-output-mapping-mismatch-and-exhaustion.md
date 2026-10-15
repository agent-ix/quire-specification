---
id: TC-154
title: "Refuse output-mapping mismatch and exhaustion"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-120
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-121
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-125
    type: verifies
  - target: ix://agent-ix/quire-specification/NFR-060
    type: verifies
---
## Description

Accepted FS06 portable adverse/resource controls..

## Test Procedure

Mutate or omit each source/profile/standard/model/semantic/generator identity;
duplicate obligations/records; cross-wire source regions and target regions;
overflow aggregate arithmetic; exercise zero, exact and just-over request,
obligation, node, work, record and output-byte limits; inject cancellation
at validation, mapping and assembly.

## Expected Results

Each invalid case refuses with its affected axis and no substitute. Resource or
cancellation failure exposes no partial package, output bytes or smaller success.
Equal target text never repairs an identity/record failure.
