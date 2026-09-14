---
id: TC-151
title: "Generate bounded OCL 2.4 output"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-122
    type: verifies
---
## Description

Accepted FS06 portable OCL profile controls. TM-008 remains Planned.

## Test Procedure

Map total Boolean/scalar invariants, ConfigVersion bounds, exact operation
pre/post reads and bounded Sequence queries. Independently exercise optional/
invalid values, non-integral rationals, index, unsupported collection operations,
relationship navigation without exact mapping, temporal and protocol constructs.
Run ParentPrecedes once with a complete relationship mapping and once without it.

## Expected Results

Only the admitted fragment generates source-correspondent OCL. Native bounds,
`@pre`, order and multiplicity remain exact. Every other case is conditional,
unrepresented or refused with no dropped relationship or approximate expression.
