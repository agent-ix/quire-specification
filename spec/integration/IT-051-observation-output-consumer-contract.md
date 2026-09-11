---
id: IT-051
title: "Integrate observation result mapping with executable and export consumers"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-115
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-116
    type: verifies
---
## Objective

Verify the planned consumer seam from the B-owned result contract to codegen and
export consumers while preserving assessment execution, independent decision-scope
progress and closure, surrounding-execution progress and closure, settlement
basis, exact decision support, completeness, provenance, immutable definition
dependencies, mapping loss, and B's fifth typed complete-global-conformance
closure record.

## Target Integration

The integration joins a structured per-obligation observation result through the [output-mapping contract](../../proposals/quire-v1/observation-output-mapping-contract.md) to the existing codegen/output mapping boundary. It does not alter native clause authority, IR producer types, or a consumer-owned evidence store.

## Preconditions

The selected A/B result and interchange interfaces have named revisions; in
particular, the result input conforms to [B's FR-061](ix://agent-ix/quire-specification/FR-061)
and [B's FR-007](ix://agent-ix/quire-verification/FR-007). Every consumed
definition/profile dependency has an immutable identity, revision, and digest.
The consumer declares supported capabilities and mapping profile.

## Inputs

One preserved result plus pending, incomplete, unsupported, and lossy obligation
controls, each carrying source/binding/definition identities, independently
identified decision-scope progress and closure and surrounding-execution progress
and closure, settlement basis and exact decision support; and target text that
is structurally accepted while its source-bound mapping remains conditional,
unrepresented, or refused. Include a current unqualified codegen bound-coverage
observation as a retained input fact rather than an assessment/result substitute.
Include decisive-witness satisfied and decisive-counterexample violated results
with both scopes open, a settled closed-decision-scope result whose surrounding
execution remains open, global-conformance results with the fifth closure record
in each of `closed`, `open`, `incomplete` and `contradicted` and their exact
required closure-premise identities, and a non-global result with `not-required`
and no global premise set.

## Test Procedure

1. Deliver each structured disposition to the consumer mapping boundary.
   - IT-051-SC-01: identity and per-obligation disposition survive delivery.
2. Exercise conditional, unrepresented, and refused mapping controls.
   - IT-051-SC-02: the consumer emits the corresponding loss classification.
3. Attempt to coerce non-success dispositions into a Boolean pass.
   - IT-051-SC-03: coercion is refused or remains non-success.
4. Observe parsing or serialization of the selected target text where that check is available.
   - IT-051-SC-04: structural acceptance is retained as an observation and does not change the source-bound mapping outcome or establish semantic preservation.
5. Deliver a current codegen bound-coverage observation beside an OB03 result.
   - IT-051-SC-05: its unqualified provenance remains visible and it cannot establish completeness, execution, adequacy, or obligation discharge.
6. Deliver a late contradiction and independently mutate one definition/profile
   identity, revision, or digest.
   - IT-051-SC-06: prior result bytes remain historical, only a linked superseding
     or invalidating result names the corrected input, and definition drift refuses.
7. Deliver the settled-open results through the selected consumer adapter; vary
   surrounding-execution progress while keeping decision-scope facts, settlement
   basis and deciding support fixed. Repeat with the closed-decision-scope result
   under compatible open and closed surrounding-execution states.
   - IT-051-SC-07: the actual consumer output preserves the four progress/closure
     axes independently and retains the same truth, basis and exact deciding
     support; decision-scope closure cannot relabel surrounding-execution closure.
8. Independently omit or duplicate each required decision-scope progress, decision-scope closure,
   surrounding-execution progress and surrounding-execution closure axis. For each
   axis, independently substitute its counterpart from the other scope, then the
   same axis from O2 in an O1 result.
   - IT-051-SC-08: every omitted, duplicated or cross-wired axis refuses the inconsistent
     contribution; another progress or closure fact cannot repair it. An explicitly
     retained open/incomplete surrounding execution remains distinguishable from an
     omitted axis and preserves independently supported settled truth.
9. Deliver each global-conformance result, then independently omit or duplicate
   the fifth closure record and each required execution, branch or workflow
   premise. Cross-wire each record/premise to another axis/scope; separately
   substitute only decision-scope or surrounding-execution closure.
   - IT-051-SC-09: the consumer preserves the typed record, state and exact premise
     identities. Only `closed` with all required premises permits complete global
     success; invalid records/premises refuse, while independently supported
     per-obligation results remain retained.
10. Deliver the non-global `not-required` record with no global premise set, then
    independently add a global premise, mark a global claim `not-required`, and
    replace each of the other four progress/closure axes with `not-required`.
    - IT-051-SC-10: the valid non-global record survives delivery unchanged;
      every invalid premise or `not-required` substitution refuses.

## Expected Results

Consumers retain requested subject, capability, assessment execution,
decision-scope progress, decision-scope closure, surrounding-execution progress,
surrounding-execution closure, complete-global-conformance closure, truth,
settlement basis, decision support,
provenance, completeness, immutable definition dependencies, and loss state.
The fifth closure record retains B's claim-specific state and required premise
rules; per-obligation settlement and `not-required` cannot replace another axis.
Structural
target-text acceptance does not establish semantic correspondence. This planned
integration does not claim codegen CLI, backend qualification, or external mapping
conformance.
