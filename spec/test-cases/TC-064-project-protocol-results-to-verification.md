---
id: TC-064
title: "Project protocol results to portable verification findings"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-062
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-249
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-283
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-285
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
complete-global-conformance closure state and one non-global `not-required`
state while retaining their exact premise sets only in the canonical source.
For participation, include FR-285 `known` facts with an empty required set and
with all, some and none of a non-empty required set participating, plus an
`unknown` fact carrying known participants, missing dependencies and typed
reasons. Mutate absent required sets, mixed variants, duplicate references and
each E02 target label in the source fact.
Project each result under one exact E02 obligation, plan, task, run, profile, binding
and adapter identity, then assess the emitted
`TechniqueResult` with the quire-verification strict reader.

For each case, mutate the canonical result key,
target obligation, execution binding, adapter version and one source state in
turn. Separately preserve a
strict-reader-accepted source result while corrupting only the target-context or
derived-projection identity. Permute JSON
object-member presentation while preserving the semantic `bindingRefs` order.
Cross-wire protocol adequacy into E02 adequacy, remove the retained
canonical-result artifact, label a shared oracle contribution independent,
substitute complete-global-conformance closure for an E02 finding axis,
select an unknown target version or required feature, and add one otherwise
valid future source state that the selected mapping does not represent. Repeat
an unchanged projection and compare its derived identity and canonical source
bytes.

## Expected Results

Every declared source row emits its exact target state, and the simultaneous
case produces every FR-007 gap independently of the compatibility summary. E02
adequacy equals observation adequacy only; protocol adequacy remains unchanged
in the canonical source artifact. Complete-global-conformance closure and its
premises also remain unchanged only in that source artifact and cannot become an
E02 finding or sufficiency decision. The derived identity is distinct and stable
for identical inputs, changes for every
identity/version mutation, is unchanged across object-member permutations, preserves binding order,
and retains the exact URI and media
type of unchanged canonical bytes. Cross-wiring, invented oracle independence,
missing source retention, unknown revisions/features and unrepresented source
states are refused with the specified stable code and without a
`TechniqueResult`. An invalid source is always
`invalid-source-result`, including when another source defect is present; it is
never reported as `identity-mismatch`. `identity-mismatch` is emitted only after
the source result is valid and a target-context or derived-projection identity
does not match its normative tuple.
Known participation derives `not-required`, `complete`, `partial` or
`not-started` from the exact required and participating sets; unknown derives
`unknown` while retaining every known participant, missing dependency and typed
reason. Absent sets, mixed variants, duplicates and target labels supplied as
source values are refused rather than defaulted or copied back.
