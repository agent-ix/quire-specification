---
id: TC-426
title: "Limit outcomes name the limit, its value and its setting"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-460
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-461
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-323
    type: verifies
---

# TC-426: Limit outcomes name the limit, its value and its setting

## Description

For each stage, drives an input past a default limit, checks what the outcome
names, and reruns it with that limit raised through each entry point.
Planned.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| LS-01 | For each stage limit, an input one unit over its published default | `stage_limit_exceeded` naming the stage, the counter, the configured bound, the count the denied charge would have reached, the locus where known and the setting name; the rendered diagnostic states each of them | FR-460-AC-5, FR-461-AC-1 |
| LS-02 | LS-01's input rerun with that limit raised to fit it through the library setter, through a replay request's `stage_limits` entry and through `--limit <name>=<value>`, each under the setting name LS-01 reported | Each rerun succeeds with the same result | FR-461-AC-2 |
| LS-03 | A replay request whose `stage_limits` raises `s1.tokens` and `s3.nodes` above their defaults, over a source that needs both raised | The replay recompiles under the raised values and reproduces the proving run's verdict | FR-461-AC-3, FR-323-AC-8 |
| LS-04 | A replay request naming no `stage_limits` entry for `s3.nodes` | The recompile runs S3 at the published default of `s3.nodes` | FR-323-AC-8 |
| LS-05 | `--limit s9.widgets=5`, `--limit s3.depth=5` and a replay `stage_limits` entry `s3.depth` | Each refuses before any stage runs, naming the argument | FR-461-AC-4, FR-323-AC-8 |
| LS-06 | A function call that exhausts its `ScalarLimitsV1` `work_units` budget; and a request carrying `ScalarLimitsV1` and `PopulationAdmissionLimitsV1` whose population admission exhausts its own `work_units` | The call settles `incomplete` naming the `work_units` counter and the setting `accounting.work_units`, and rerun with `accounting.work_units` raised through each entry point it completes; the admission stop names `admission.work_units`, and raising only `accounting.work_units` leaves it stopped | FR-461-AC-5 |
| LS-07 | A model-check item over a state graph one state larger than its `model_check.max_states` limit | The item settles `incomplete` naming the state-count limit, its value and the setting `model_check.max_states`; rerun with `model_check.max_states` raised through the library setter, a replay request and `--limit`, the item finishes with the same verdict each time | FR-461-AC-6 |
| LS-08 | Backend-provider positive vector `limit-reached-accounting-setting` (an exact probabilistic item stopped by its evaluation meter: a `LimitReached` cause with `limit` `work_units` and `setting` `accounting.work_units`) and its adverse vector `limit-reached-bare-accounting-counter` (`setting` the bare `work_units`) | The positive validates; the adverse fails the provider schema at `/results/0/cause/setting` | FR-461-AC-8 |
| LS-09 | The ordered charge-point table of `quire.value.accounting/v1`, its amount table and the published `value-accounting-charge-point-vectors.json`; then the vectors' undeclared spellings; then a kernel's `ChargePoint::ALL` spellings | The table's spellings, in table order, equal the published list and equal the amount table's points, with no count written in the list; each undeclared spelling is absent from the ordered table (and therefore the amount table); this repository's `tests/value_accounting_vocabulary.rs` enforces that half. A conforming kernel test SHALL compare its `ChargePoint::ALL` spellings with `VALUE_ACCOUNTING_CHARGE_POINT_VECTORS` so one side adding or dropping a point fails | FR-461-AC-9, FR-461-AC-10 |
| LS-10 | The `ScalarLimitsV1` Limit object of `quire.value.accounting/v1` and the published `value-accounting-limit-kind-vectors.json`; then the vectors' undeclared spellings; then a kernel's `LimitKind::ALL` spellings | The Limit object's counter spellings, in field order, equal the published list (the `ScalarLimitsV1` counters only, with no count written), and each undeclared spelling is absent from it and from every `limit_kind` value the definition names; this repository's `tests/value_accounting_vocabulary.rs` enforces that half. A conforming kernel test SHALL compare its `LimitKind::ALL` spellings with `VALUE_ACCOUNTING_LIMIT_KIND_VECTORS` so one side adding or dropping a counter fails | FR-461-AC-11 |

## Expected Results

Every limit outcome tells the caller which setting to raise and by how much,
the same name raises the same limit at every entry point, and an unknown or
depth setting name never reaches a stage.
