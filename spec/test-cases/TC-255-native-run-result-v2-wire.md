---
id: TC-255
title: "Produce and read native-run-result/2 with its basis and witness"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-014
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-352
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-243
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-351
    type: verifies
---

# TC-255: Produce and read native-run-result/2 with its basis and witness

## Description

Runs state clauses through the CLI `run` command and reads the
`native-run-result/2` records back, covering every basis, the witness member
and the identifier refusal. Planned.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| NR-01 | `invariant forall(item in self.items: item < self.n)` over an observation where the third item fails | One `native-run-result/2` record with FR-352's base members, `basis` `decisive-counterexample` and a `witness` naming the quantifier's occurrence key, the third item, `index` 2 and the value path to `self.items`; the strict reader admits it | FR-352-AC-1, FR-352-AC-3, FR-352-AC-6 |
| NR-02 | `exists(item in self.items: item > 5)` true at the second item; `not forall(...)` true | `basis` `decisive-witness` with a `witness` for each | FR-352-AC-6 |
| NR-03 | `forall(...) == q` true; `forall(...)` true over every element | `basis` `closed-scope` and no `witness` | FR-352-AC-3, FR-352-AC-6 |
| NR-04 | A refused, an undefined, a budget-exhausted, a cancelled and an unsupported clause run | `basis` `unavailable` and no `witness` for each | FR-352-AC-6 |
| NR-05 | NR-01's record with its identifier changed to `quire.run-result/2`, and with the identifier removed | Each refuses before consumption with FR-014 `unsupported-version` | FR-352-AC-2 |
| NR-06 | NR-01's record with: `basis` `maybe`; `witness` removed; a `witness` added to NR-03's record; `index` removed; an extra `witness` member; `trace_position` written as `null` | Each refuses | FR-352-AC-7 |
| NR-07 | NR-01's record with a required `usage` member removed, an unknown top-level member added, an unknown disposition member added, and an absent `package_id` encoded as `null`, each independently | Each refuses with a located wire diagnostic; the unchanged record reads | FR-352-AC-8 |

## Expected Results

Every clause run carries exactly one basis label, a witness travels exactly
with a decisive basis in its closed shape, and only `native-run-result/2`
reads as a native run result.
