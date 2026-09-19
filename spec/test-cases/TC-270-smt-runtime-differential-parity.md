---
id: TC-270
title: "SMT/native runtime differential parity corpus"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-355
    type: verifies
---

# TC-270: SMT/native runtime differential parity corpus

## Description

Evaluate the shared differential corpus through the registered SMT backend
and through native `runtime::execute`, and compare verdicts member by member.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-355. Execute its
positive case, each declared boundary, and each explicit refusal/non-conclusive
mutation through the real local Rust boundary; compare the complete typed
result and artifact accounting with the criterion.

## Vectors

| Vector | Mutation | Expected disposition |
| --- | --- | --- |
| DP-01 | Configure the shared differential corpus with zero members | Refuses as an empty corpus rather than reporting a vacuous parity report; the corpus is required non-empty and to include at least one member per obligation form in the FR-349 descriptor's declared obligation-form set |
| DP-02 | Run a member whose SMT verdict is `inconclusive` and whose native runtime reaches no decisive holds/counterexample verdict over the same bounded domain, then mutate the native side to a decisive verdict | The matched `inconclusive`/no-decisive-verdict pair records agreement; the mutated pair records an explicit parity failure naming the member and both verdicts |
| DP-03 | Declare quantification and induction in the SMT descriptor's obligation-form set, then remove the only corpus member for induction | Refuses the corpus naming `induction` as the uncovered declared form; no parity report is produced |

## Expected Results

Every positive and boundary result matches FR-355; every corpus member is a
bounded instance evaluated by both the SMT backend and the native runtime
over that same bounded domain, under the same package, domain and claim
identity; matching verdicts are recorded as agreement under the defined
SMT-proved/refuted/inconclusive correspondence to the native runtime's
verdict; a decode, domain or verdict mismatch outside that correspondence is
recorded as an explicit parity failure naming the member and both verdicts,
never silently resolved toward either side; the aggregate parity report
accounts for every corpus member exactly once; the corpus is non-empty and
covers every obligation form the FR-349 backend discharges; each mutation returns its exact
typed disposition with source/provenance and preserves independent sibling
results.
