---
id: IT-060
title: "Integrate target output mappers through the common package boundary"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-120
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-121
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-125
    type: verifies
---
## Objective

Verify that independent qcir OCL, SysML/KerML and FRETish mappers consume the
same strict request and emit the same FR-298/FR-299 envelope rules while retaining
separate target correspondence and implementation ownership.

## Target Integration

The integration joins strict FS05 packages and accepted native/model/temporal/
observation semantics to qcir #55/#56/#57 through
`quire.output.mapping/v1`. No target mapper invokes another target,
reads target text as native source or depends on an external runtime.

## Preconditions

FR-120–125, FR-269, FR-297–299 and NFR-060/061 are selected by exact identity;
each target mapper declares its exact supported capabilities and generator
identity; the common and target limits are known before dispatch.

## Inputs

One common package containing eligible scalar/state obligations, one exact
operation pre/post obligation, one sequence/relationship control, one temporal
obligation, one incomplete source result and one unsupported construct. Repeat
under each target profile with exact-limit and just-over-limit variants.

## Test Procedure

1. Dispatch the same ordered obligation set independently to each exact target profile.
   - IT-060-SC-01: every mapper returns one record per obligation through the common envelope.
2. Compare eligible and unsupported cases against the selected target profile.
   - IT-060-SC-02: each classification and condition/cause is target-specific, explicit and source-bound.
3. Mutate one source/profile/model/clock/generator/limit member at a time.
   - IT-060-SC-03: the affected identity changes or the request refuses; equal text cannot repair it.
4. Omit, duplicate and cross-wire records and output regions.
   - IT-060-SC-04: package assembly refuses with no partial package.
5. Attach an optional parser/tool observation, then remove, fail and version-mutate it.
   - IT-060-SC-05: the immutable package and mapping classifications remain unchanged.
6. Attempt to feed generated target text back as native source or into another mapper.
   - IT-060-SC-06: intake refuses; no round-trip or cross-target authority exists.

## Expected Results

All target mappers share intake, loss, identity, resource and package behavior
while retaining separate mapping rules. Unsupported constructs never receive
plausible substitutes. Structural target observations remain downstream evidence.
This integration remains Planned until all three owner implementations exist.
