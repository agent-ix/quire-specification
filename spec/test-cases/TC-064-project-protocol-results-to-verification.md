---
id: TC-064
title: "Project protocol results to portable verification findings"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
    type: verifies
---
## Description

Verify the total, source-bound projection from one canonical protocol result to
`quire-verification/e02-draft-2` without losing an FR-007 finding or conflating
observation adequacy, protocol adequacy and evidence sufficiency.

## Test Procedure

Build one strict-reader-accepted protocol result for every row of the normative
projection table and every valid participation derivation. Include all four
assumption states, all four observation-adequacy states, all four protocol-
adequacy states, and a result carrying simultaneous partial participation,
`not-met` assumption, inadequate observations and violated truth. Project each
result under one exact E02 obligation, plan, task, run, profile, binding,
dependency closure and adapter identity, then assess the emitted
`TechniqueResult` with the quire-verification strict reader.

For each case, mutate the canonical result identity, canonical byte digest,
target obligation, execution binding, adapter version and one source state in
turn. Cross-wire protocol adequacy into E02 adequacy, remove the retained
canonical-result artifact, label a shared oracle contribution independent,
select an unknown target version or required feature, and add one otherwise
valid future source state that the selected mapping does not represent. Repeat
an unchanged projection and compare its derived identity and canonical source
bytes.

## Expected Results

Every declared source row emits its exact target state, and the simultaneous
case produces every FR-007 gap independently of the compatibility summary. E02
adequacy equals observation adequacy only; protocol adequacy remains unchanged
in the canonical source artifact, and no sufficiency decision is emitted. The
derived identity is distinct and stable for identical inputs, changes for every
identity/version/digest mutation, and retains the exact URI, digest and media
type of unchanged canonical bytes. Cross-wiring, invented oracle independence,
missing source retention, unknown revisions/features and unrepresented source
states are refused without a `TechniqueResult`.
