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

Runs state and temporal clauses through the CLI `run` command and reads the
`native-run-result/2` records back, covering every basis, the witness member,
the temporal disposition and the identifier refusal. Planned.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| NR-01 | `invariant forall(item in self.items: item < self.n)` over an observation where the third item fails | One `native-run-result/2` record with FR-352's base members, `basis` `decisive-counterexample` and a `witness` naming the quantifier's occurrence key, the third item, `index` 2 and the value path to `self.items`; the strict reader admits it | FR-352-AC-1, FR-352-AC-3, FR-352-AC-6 |
| NR-02 | `exists(item in self.items: item > 5)` true at the second item; `not forall(...)` true | `basis` `decisive-witness` with a `witness` for each | FR-352-AC-6 |
| NR-03 | `forall(...) == q` true; `forall(...)` true over every element | `basis` `closed-scope` and no `witness` | FR-352-AC-3, FR-352-AC-6 |
| NR-04 | A refused, an undefined, a budget-exhausted, a cancelled and an unsupported clause run | `basis` `unavailable` and no `witness` for each | FR-352-AC-6 |
| NR-05 | NR-01's record with its identifier changed to `quire.run-result/2`, and with the identifier removed | Each refuses before consumption with FR-014 `unsupported-version` | FR-352-AC-2 |
| NR-06 | NR-01's record with: `basis` `maybe`; `witness` removed; a `witness` added to NR-03's record; `index` removed; an extra `witness` member; `trace_position` written as `null` | Each refuses | FR-352-AC-7 |
| NR-07 | NR-01's record with a required `usage` member removed, an unknown top-level member added, an unknown disposition member added, and a `temporal` `{"kind": "holds"}` added to its disposition, each independently | Each refuses with a located wire diagnostic; the unchanged record reads | FR-352-AC-8, FR-352-AC-12 |
| NR-08 | A clause run whose evaluation refuses with an FR-096 record containing `code`, `cause`, `fields`, `locus` and `location`; then independently remove each assigned member or replace the refused outcome with only its `kind` | The original reads with every assigned field intact; each edit refuses with a located wire diagnostic | FR-352-AC-8 |
| NR-09 | A clause run ending in an FR-100 `InternalFault` with stage and invariant in `disposition.internal_fault`; then remove either identifier or add a fabricated `outcome` | The original reads as internal failure with its identifiers; each edit refuses with a located wire diagnostic | FR-352-AC-8 |
| NR-10 | QSL FR-115's forbidden-parent-change Frame report, followed by edits independently removing its `disposition.refusal.frame_witness`, deleting a required frame-witness member, and adding that member to a non-Frame result | The original reads with the complete typed FR-115 witness, `basis: closed-scope` and no top-level `witness`; each edit refuses with a located wire diagnostic | FR-352-AC-9 |
| NR-11 | QSL TC-1037 row 2's Frame report (`Orders::Pump::rehome`, `modifies []`, `p1`-`t1` of `Pump-housing-Tank` removed) and row 4's (`p3`-`t1` added); then independently, from row 2's report, remove each `link` member of the change, remove `population` from `source`, set `difference` to `"moved"` and to `null`, and give `source` a bare key; then TC-1037 rows 1, 10 and 11 (relationship modified, end at a deleted object, relationship not complete in both snapshots) | Each original reads with `code` `frame_violation`, `cause` `unauthorized-change`, top-level `population` `Q`, a `link` change naming `Pump-housing-Tank`, `removed` (then `added`), `source` and `target` references and `modifies`, `basis: closed-scope` and no top-level `witness`; each edit refuses with a located wire diagnostic; the rows 1, 10 and 11 reports are `success` and carry no `frame_witness` | FR-352-AC-9 |
| NR-12 | QSL TC-1160's temporal clause runs: `Reaches` over a lasso with an empty prefix and loop 0, 1 (`violated`); its `pending` and `holds` runs; a `work_units` stop (`incomplete`); a division by zero at position 2 (`undefined`); a missing weak fairness premise (`missing-fairness-premise`); an empty loop and an empty finite trace (`trace-refused`, `variant` `empty-loop` and `empty-trace`); a refused `holds` condition (`condition-refused`); `Windowed` with `min_count 5` (`insufficient-data`); a `Windowed` report holding `WindowLimitReached{max_window_values, 2, monitor.max_window_values}` (`window-limit-reached`); and a `trace-refused` report with `variant` `tuple-arity`, `expected` 2, `found` 1 | Each is written at stage `evaluate` with `disposition.temporal` of its kind, exactly that row's members, `category`, `truth`, `basis` and `outcome`, no top-level `witness` and no `refusal`, and reads back to an equal disposition, the two `trace-refused` variants and the `tuple-arity` payload included | FR-352-AC-10 |
| NR-13 | NR-12's `pending`, `undefined`, `insufficient-data` and `window-limit-reached` records; then NR-12's `undefined` record with its outcome reason set to `insufficient-data`, its `incomplete` record with its outcome limit set to `max_window_values`, NR-01's state clause run given an `undefined` outcome with reason `insufficient-data`, and one given an `incomplete` outcome with limit `max_window_values` | `pending` reads `inconclusive` with no `truth` and `basis` `unsettled`; `undefined` reads `violation`, `truth` `false`, `basis` `closed-scope`; `insufficient-data` reads with reason `insufficient-data` and `window-limit-reached` with limit `max_window_values`; each of the four edited records refuses | FR-352-AC-11 |
| NR-14 | Independently: `temporal` `{"kind": "holds"}` added to a stage `admit` disposition of a temporal selection; NR-12's `violated` record with a top-level `witness`, an `internal_fault` or a `refusal` added, `kind` set to `held`, `counterexample` removed, `position` set to `null`, an extra `temporal` member, and the counterexample's `trace_position` set to the unsigned integer 1, which differs from the record's `position` 0; NR-12's empty-loop record with `variant` `empty-hoop`; NR-12's `pending` record with `basis` `unavailable` and with `truth` `true` | Each refuses with a located wire diagnostic naming the member | FR-352-AC-12 |
| NR-15 | A `local-projection` and a `realizability` analysis over QSL TC-1413's protocols, each settling `proved`; a projection with a divergent action at the third event of `path_a` and an unrealizable hidden-choice protocol, each `refuted`; an `inconclusive` result of each FR-331 protocol-analysis cause and of `bound-reached{depth}`; an `unsupported` result; a `max_states` stop; a declined item and an engine failure | Each analysis result is written as a protocol-analysis record with `format`, `analysis` (`kind`, `key`, `label`, `execution`, `truth`, `settlement`), `basis` and, for the two refutations only, `witness`, and reads back equal; the projection witness has no `quantifier` or `index`, the divergent node's canonical node-id text, a `value_path` rooted at the request's `assessed_root` with empty `steps`, and `trace_position` `"2"`; the stop is FR-360 `failed`, `resource-incomplete`, `settlement` `incomplete` `limit-reached`; the declined item and the engine failure write no protocol-analysis record | FR-352-AC-14 |
| NR-16 | NR-15's projection refutation record, independently: `disposition` added; `analysis` removed; `usage` added; `kind` `refinement`; the key's `role` removed; `label` `proved` with `truth` `pending`; `settlement.result` `inconclusive`; `witness.index` 0; `deciding_element` with its node-id members reordered; a `member` step added to `value_path`; `trace_position` `"02"` | Each refuses with a located wire diagnostic naming the member; the removed `role` is refused `invalid_runtime_input`/`missing-member` | FR-352-AC-15 |

## Expected Results

Every clause run carries exactly one basis label, a witness travels exactly
with a decisive basis in its closed shape, and only `native-run-result/2`
reads as a native run result.
