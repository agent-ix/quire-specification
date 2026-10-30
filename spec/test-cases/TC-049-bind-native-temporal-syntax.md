---
id: TC-049
title: "Retain typed temporal syntax and activation bindings"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-048
    type: verifies
---
## Description

Planned checks for the native temporal declaration and its explicitly scoped
Boolean leaves. These cases do not claim parser or temporal implementation.

## Test Procedure

Place the shared grammar's timed-refund fragment in a complete fixture unit
with exact synthetic model/profile definitions. Compare independently authored
expected syntax/binding trees for all future/past forms, grouping and query
leaves. Mutate one interval, name scope, capture dependency, leaf type or clock
binding at a time. Bind concurrent charged triggers with equal amounts, then a
duplicate receipt, a changed later amount, a wrong-order refund and an inclusive
deadline refund. Independently compare outer true with holds(true) across a
one-position false-extension boundary. Attempt current TL lowering for the
timestamped and past-time forms.

Bind the three exact [temporal root definitions](../../proposals/quire-v1/definitions/README.md)
independently. The common temporal facet is not a selectable source root.
Replace only the root identity while keeping its clock input: an
incompatible clock/boundary selection refuses instead of reusing an earlier
result. Confirm that shared query/graph expressions keep their selected meaning
inside `holds`, while temporal profile selection cannot enable state declarations
or an ambient operation/pre-state scope.

## Expected Results

Each accepted tree retains its original spans and exact imported type/profile
references. Mutation controls fail at their actual syntax/scope/type/binding
stage. Captures and current inputs remain distinct; missing population or clock
authority cannot produce success. The two constant-boundary forms produce the
specified different truths. Unsupported lowering preserves the source-valid
native subject without emitting a replacement-profile formula.
