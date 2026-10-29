---
id: FR-461
title: "Name the limit, its value and its setting in every limit outcome"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-460
    type: references
  - target: ix://agent-ix/quire-specification/FR-272
    type: references
  - target: ix://agent-ix/quire-specification/FR-300
    type: references
  - target: ix://agent-ix/quire-specification/FR-301
    type: references
  - target: ix://agent-ix/quire-specification/FR-323
    type: references
---
# FR-461: Name the limit, its value and its setting in every limit outcome

## Description

When a stage or an analysis engine reaches a configured limit, the
implementation SHALL report the limit, its value and the one setting name that
raises the limit at every entry point.

## Inputs

A stage's or an engine's configured limits, given through the library limits
type, a replay request's `stage_limits` or the CLI's `--limit <name>=<value>`
option, and the charge that a limit denied.

## Outputs

A `stage_limit_exceeded` refusal (FR-272), an `incomplete` execution outcome
or an `incomplete` engine item that names the limit, its value and its
setting.

## Behavior

1. **The outcome names three things.**
   - The limit: its stage or engine and its counter, for example the S3 node
     count or the model checker's state count.
   - Its value: the configured bound in force, and the count the denied
     charge would have reached.
   - Its setting: the name by which a caller raises it.

   The locus is the one FR-272 retains for the cause. This requirement
   defines the count; an engine's stop record exposes it where the owning
   requirement says so (FR-399 for the model checker's hyper limits), and the
   settled `limit-reached` cause names the limit, its value and its setting.
2. **One setting name per limit.** Every configurable stage limit has one
   stable dotted name: the stage, a dot and the counter, for example
   `s1.tokens`, `s3.nodes`, `s3.work_units` or `intake.input_bytes`. The
   counter names a resource (bytes, tokens, nodes, edges, occurrences,
   diagnostics, work units or rational bits), never a depth (FR-460).
   A byte-emitting lifecycle operation names its output-byte budget with the
   operation name and the `output_bytes` counter, for example
   `format.output_bytes`. The budget counts every byte the operation would
   return, including formatting whitespace, comments and a terminating
   newline where the operation emits one.
3. **One name at every entry point.** The same name raises the same limit:
   - in the library, through the limits type's setter for that limit
     (FR-300);
   - in a replay request, as the `stage_limits` entry of that name
     (FR-323);
   - on the CLI, as `--limit <name>=<value>` (FR-301).
4. **Every stage limit can be raised at every entry point.** A replay
   request carries every stage limit and passes each one to its stage. The
   CLI accepts every setting name. An unknown setting name refuses before any
   stage runs and names the argument (FR-301).
5. **Engine limits use the same rule.** Every member of an analysis engine's
   limits type is a limit under this requirement: the model checker, the
   statistical and exact probabilistic engines, the zone checker, the
   certificate checkers, the scheduler, and the window, sample-step and time
   budgets. Its setting name is the engine's name, a dot and the member, for
   example `model_check.max_states`, `statistical.max_samples` or
   `zone_check.max_symbolic_states`. The list of counters is open: each engine's limits
   type defines its members, and each member has exactly one setting name. An
   engine item that stops at a limit settles `incomplete` and names the limit,
   its value and its setting, and the same name raises it at every entry
   point.
6. **Execution budgets name their counter.** An `incomplete` execution
   outcome names its `quire.value.accounting/v1` counter, for example
   `work_units`, as its limit. A counter of the evaluation meter, one of the
   ten `ScalarLimitsV1` counters (`integer_bits`, `decimal_digits`,
   `scale_expansion`, `text_input_bytes`, `text_scalars`,
   `normalized_scalars`, `unit_edges`, `value_occurrences`, `work_units` and
   `result_units`), has the setting `accounting`, a dot and the counter name,
   for example `accounting.work_units`, and that one name raises the budget
   at every entry point. The counters of the other accounting limit objects
   are stage limits and take their stage's dotted name under Behavior 2:
   `PopulationAdmissionLimitsV1`'s as `admission.<counter>` and
   `ModelNormalizationLimitsV1`'s as `model.<counter>`, for example
   `admission.work_units` and `model.work_units`. So two independent counters
   that share a counter name have two setting names, and every setting name,
   of a stage, an engine or an execution budget, is dotted. The backend-provider
   `LimitReached` cause carries the counter as `limit` and the dotted name
   as `setting`, and its `SettingName` pattern admits no bare counter name.
7. **Rendering.** A rendered limit diagnostic states the limit, the
   configured value, the count reached, the locus and the setting, for
   example: S3 node limit 100000 reached (100001) at `f`; raise it with
   `--limit s3.nodes=<n>` or the request's `stage_limits` entry `s3.nodes`.
   The typed cause, not the text, carries the classification.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-461-AC-1 | A `stage_limit_exceeded` outcome carries the stage, the counter, the configured bound, the count the denied charge would have reached, the locus where known and the setting name, and its rendering states each of them. | Test (TC-426) |
| FR-461-AC-2 | For each stage limit, an input that exceeds its default is rerun with that limit raised through the library setter, through the replay request's `stage_limits` entry and through `--limit <name>=<value>`, each under the setting name the outcome reported, and each rerun succeeds with the same result. | Test (TC-426) |
| FR-461-AC-3 | A replay request whose `stage_limits` raises any stage limit runs that stage under the raised value, not under its default. | Test (TC-426) |
| FR-461-AC-4 | An unknown setting name, or a setting name naming a depth, given to `--limit` or to a replay request's `stage_limits` refuses before any stage runs and names the argument. | Test (TC-426) |
| FR-461-AC-5 | An `incomplete` execution outcome names its `quire.value.accounting/v1` counter as its limit and, for a `ScalarLimitsV1` counter, `accounting.<counter>` as its setting, such as `accounting.work_units`, and raising that setting through each entry point lets the same run complete; a request carrying both `ScalarLimitsV1` and `PopulationAdmissionLimitsV1` that exhausts population admission's `work_units` names the setting `admission.work_units`, and raising `accounting.work_units` does not let it complete. | Test (TC-426) |
| FR-461-AC-6 | An engine item stopped at an engine limit, such as the model checker's state count, settles `incomplete` naming the limit, its value and its dotted setting name, such as `model_check.max_states`, and raising that setting through the library setter, the replay request and `--limit` lets the same item finish. | Test (TC-426) |
| FR-461-AC-7 | A byte-emitting lifecycle operation with an `output_bytes` limit reports the operation's `*.output_bytes` setting when the next emitted byte would exceed the configured bound, returns no partial output, succeeds at the exact output length including its terminating newline, and accepts the same bound through its typed limits field and the CLI settings operation. | Test (TC-930) |
| FR-461-AC-8 | A backend-provider `LimitReached` cause for an exhausted execution budget with `limit` `work_units` and `setting` `accounting.work_units` validates against the provider schema; the same cause with `setting` `work_units` fails it. | Test (TC-426) |
| FR-461-AC-9 | The spellings the `quire.value.accounting/v1` ordered charge-point table declares equal, in table order, the spellings of the published list `value-accounting-charge-point-vectors.json`, and every spelling has an amount row; the set includes `equality.plan-form`, `frame.write`, `frame.redefines-link`, `declaration.check` and `temporal.visit`. | Test (TC-426) |
| FR-461-AC-10 | A spelling the table does not declare, such as `temporal.visits`, is absent from the table and the published list, and a one-sided change to either fails the comparison. A kernel's `ChargePoint::ALL` spellings equal the list; that comparison lives in quire-exact's own tests, which compare the `as_str()` set with `VALUE_ACCOUNTING_CHARGE_POINT_VECTORS` and require each undeclared entry to fail `from_code`. | Test (TC-426) |

## Dependencies

- [FR-460](FR-460-bound-every-stage-by-resource-limits.md) owns the rule that
  every limit is a resource limit with a published default.
- [FR-272](../../objects/foundation/FR-272-typed-diagnostic-cause.md) owns the
  `stage_limit_exceeded` causes and payload names, including `setting`.
- [FR-300](../tooling/FR-300-expose-rust-lifecycle-api.md),
  [FR-301](../tooling/FR-301-expose-complete-cli.md) and
  [FR-323](../../objects/interfaces/FR-323-native-runtime-envelope.md) own the
  three entry points.

- QSL-673 specifies the formatter instance of this contract as
  `format.output_bytes` and its typed `FormatLimits.output_bytes` field.

## References

- Linear STD-143; QSL ADR-030 D-3.
