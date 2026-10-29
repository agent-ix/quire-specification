---
id: TC-972
title: "Check arm: outcome mapping, evidence, refusals and registration"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-483
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-290
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-331
    type: verifies
---

# TC-972: Check arm: outcome mapping, evidence, refusals and registration

## Description

Check the shared check-arm contract over both kinds: negotiation dispositions, the four arm outcomes and their FR-331 results, proof evidence, counterexample kinds, refusals, limits and registration.

## Test Procedure

Read `proposals/backend-check-arm-v1/vectors.json`. For each case, derive each `$node:<label>` NodeId from its label as the file's `rekey` member states, recompute the FR-290 disposition from the case's registry and item, validate the case's outcome stage against `proposals/backend-provider-v1/schema.json`, apply the arm's own refusal gate, and compare the arm outcome with the result. The harness is `tests/backend_check_arm_v1.rs`; its pinned case list fails when a vector is removed.

The harness recomputes the proof basis and certification of every `proved` entry from the case's `method` (enumeration `exhaustive`/`uncertified`, Kani `bounded-proof`/`certified`, SMT `bounded-complete` or `inductive` with depth 0 for a value item and 1 for a contract item, plugin `trusted`), the effective `max_cases` (the item's `check.limits.max_cases`, else the descriptor default), the schema refusal of a zero or non-count `max_cases`, the population domain rule and the independence of items that share a claim.

## Vectors

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| SH-01 | unbounded item, bounded-only candidate, no bound available: kind `value-validity`, form `clause` | `unsupported`, `unbounded-extent`; no result entry | FR-483-AC-6 |
| SH-02 | unbounded item, unbounded candidate: SMT inductive proof, no certificate: kind `value-validity`, form `clause` | `supported`; arm outcome `supported`, result `proved`, basis `inductive`, `uncertified` | FR-483-AC-2, FR-483-AC-6 |
| SH-03 | bounded item, SMT proof with a verified certificate: kind `value-validity`, form `clause` | `supported`; arm outcome `supported`, result `proved`, basis `bounded-complete`, `certified` | FR-483-AC-2 |
| SH-04 | operation-contract SMT proof is one step deep: kind `operation-contract`, form `invariant` | `supported`; arm outcome `supported`, result `proved`, basis `bounded-complete`, `uncertified` | FR-483-AC-2 |
| SH-05 | Kani harness with positive SUCCESS count: kind `value-validity`, form `clause` | `supported`; arm outcome `supported`, result `proved`, basis `bounded-proof`, `certified` | FR-483-AC-2 |
| SH-06 | plugin proof is trusted: kind `value-validity`, form `clause` | `supported`; arm outcome `supported`, result `proved`, basis `exhaustive`, `trusted` | FR-483-AC-2 |
| SH-07 | engine unknown: kind `value-validity`, form `clause` | `supported`; arm outcome `unknown`, result `inconclusive`, cause `solver-inconclusive` | FR-483-AC-4 |
| SH-08 | rejected certificate: kind `value-validity`, form `clause` | `supported`; arm outcome `unknown`, result `inconclusive`, cause `certificate-rejected` | FR-483-AC-4 |
| SH-09 | provider time limit: kind `value-validity`, form `clause` | `supported`; arm outcome `unknown`, result `incomplete`, cause `timed-out` | FR-483-AC-4 |
| SH-10 | caller deadline: kind `value-validity`, form `clause` | `supported`; arm outcome `unknown`, result `incomplete`, cause `cancelled` | FR-483-AC-4 |
| SH-11 | item max_cases overrides the descriptor default: kind `value-validity`, form `clause` | `supported`; arm outcome `supported`, result `proved`, basis `exhaustive`, `uncertified` | FR-483-AC-4 |
| SH-12 | descriptor default max_cases applies when the item has none: kind `value-validity`, form `clause` | `supported`; arm outcome `unknown`, result `incomplete`, cause `limit-reached` | FR-483-AC-4 |
| SH-13 | candidate counterexample does not replay: kind `value-validity`, form `clause` | `supported`; arm outcome `unknown`, result `inconclusive`, cause `replay-parity` | FR-483-AC-3 |
| SH-14 | no provider advertises value-validity: kind `value-validity`, form `clause` | `unsupported`, `unsupported-requested-capability`; no result entry | FR-483-AC-7 |
| SH-15 | no provider advertises operation-contract: kind `operation-contract`, form `invariant` | `unsupported`, `unsupported-requested-capability`; no result entry | FR-483-AC-7 |
| SH-16 | candidate lacks the population domain kind: kind `value-validity`, form `clause` | `unsupported`, `unsupported-requested-capability`; no result entry | FR-483-AC-8 |
| SH-17 | candidate lists population: the item reaches the arm: kind `value-validity`, form `clause` | `supported`; arm outcome `supported`, result `proved`, basis `exhaustive`, `uncertified` | FR-483-AC-8 |
| SH-18 | operation-contract: candidate lacks population: kind `operation-contract`, form `invariant` | `unsupported`, `unsupported-requested-capability`; no result entry | FR-483-AC-8 |
| SH-19 | package lacks the node the form requires: kind `operation-contract`, form `clause` | `supported`; arm outcome none (declined), result `declined`, cause `incomplete-input` | FR-483-AC-5 |
| SH-20 | one claim, two kinds, and one claim requested twice: four terminal records: kind `value-validity`, form `clause` | `supported`; item 0 value-validity `proved`, item 1 operation-contract `refuted`, item 2 value-validity `proved`, item 3 value-validity `incomplete` | FR-483-AC-9 |

## Expected Results

Every shared vector SH-01 to SH-20 settles as its row states. A `proved` entry carries the basis of FR-483's proof-evidence table. A `limit-reached` cause names `max_cases`, the effective value and the setting `value_check.max_cases` or `contract_check.max_cases`. A `max_cases` that is zero, negative, a string or accompanied by another member is refused by the `Check` definition. A vector with a candidate counterexample the replay does not reproduce settles `inconclusive` `replay-parity` and records no counterexample. A value-validity or operation-contract item whose extent holds a `population` domain is `unsupported` under a candidate that lacks `population` and `supported` under one that lists it.

The harness also checks the descriptor rule: a descriptor that advertises `value-validity` or `operation-contract` publishes `bounds.max_cases` as a positive count, and the schema refuses a descriptor with no `bounds`, with `bounds` lacking `max_cases`, or with `max_cases` zero or not a count (`descriptor_cases` in the vector file). No item runs without a case budget.
