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
| NR-08 | A clause run whose evaluation refuses with an FR-096 record containing `code`, `cause`, `fields`, `locus` and `location`; then independently remove each assigned member or replace the refused outcome with only its `kind` | The original reads with every assigned field intact; each edit refuses with a located wire diagnostic | FR-352-AC-8 |
| NR-09 | A clause run ending in an FR-100 `InternalFault` with stage and invariant in `disposition.internal_fault`; then remove either identifier or add a fabricated `outcome` | The original reads as internal failure with its identifiers; each edit refuses with a located wire diagnostic | FR-352-AC-8 |
| NR-10 | QSL FR-115's forbidden-parent-change Frame report, followed by edits independently removing its `disposition.refusal.frame_witness`, deleting a required frame-witness member, and adding that member to a non-Frame result | The original reads with the complete typed FR-115 witness, `basis: closed-scope` and no top-level `witness`; each edit refuses with a located wire diagnostic | FR-352-AC-9 |
| NR-11 | QSL TC-1037 row 2's Frame report (`Orders::Pump::rehome`, `modifies []`, `p1`-`t1` of `Pump-housing-Tank` removed) and row 4's (`p3`-`t1` added); then independently, from row 2's report, remove each `link` member of the change, remove `population` from `source`, set `difference` to `"moved"` and to `null`, and give `source` a bare key; then TC-1037 rows 1, 10 and 11 (relationship modified, end at a deleted object, relationship not complete in both snapshots) | Each original reads with `code` `frame_violation`, `cause` `unauthorized-change`, top-level `population` `Q`, a `link` change naming `Pump-housing-Tank`, `removed` (then `added`), `source` and `target` references and `modifies`, `basis: closed-scope` and no top-level `witness`; each edit refuses with a located wire diagnostic; the rows 1, 10 and 11 reports are `success` and carry no `frame_witness` | FR-352-AC-9 |

## Expected Results

Every clause run carries exactly one basis label, a witness travels exactly
with a decisive basis in its closed shape, and only `native-run-result/2`
reads as a native run result.
