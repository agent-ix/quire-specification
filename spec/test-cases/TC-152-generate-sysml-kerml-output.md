---
id: TC-152
title: "Generate bounded SysML and KerML output"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-123
    type: verifies
---
## Description

Accepted FS06 portable SysML/KerML profile controls. TM-008 remains Planned.

## Test Procedure

Map total Boolean/scalar constraints with exact authoritative type/unit/attribute
bindings. Test native implication with and without exact assumption/requirement
correspondence. Exercise pre/post, collection, relationship, temporal, protocol,
part/port/action/state/allocation and model-interchange constructs. Mutate the
target short name independently from native identity.

## Expected Results

Eligible constraints produce the admitted requirement fragment. Local target
attributes do not become model authority. Unsupported structures are explicitly
non-preserved. Display names never replace native identity.
