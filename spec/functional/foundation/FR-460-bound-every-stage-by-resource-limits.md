---
id: FR-460
title: "Bound every stage by resource limits, never by nesting depth"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-146
    type: references
  - target: ix://agent-ix/quire-specification/FR-272
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-461
    type: references
  - target: ix://agent-ix/quire-specification/NFR-010
    type: references
---
# FR-460: Bound every stage by resource limits, never by nesting depth

## Description

When a stage processes an input, the implementation SHALL bound that stage's
work only by the caller's resource limits, never by nesting depth or by the
host stack.

## Inputs

An input of any nesting depth (source text, a semantic graph, a value, a JSON
document or a wire artifact) and the caller's resource limits for the stage:
input bytes, node and edge counts, occurrence and diagnostic counts, rational
bits, work units and time.

## Outputs

The stage's result on the input's content, or the stage's limit outcome
naming the limit that was reached (FR-461).

## Behavior

This rule applies to every stage of every implementation: parser, reader,
encoder, checker, lowering, generator, evaluator, replay and every wire
format reader and writer.

1. **Depth is not a limit.** Nesting depth is bounded only by the caller's
   resource limits: input bytes, node and edge counts, occurrence and
   diagnostic counts, rational bits, work units and time. No stage has a depth limit kind,
   a caller depth setting, a compile-time depth ceiling or a constant sized
   to a stack. An input that fits the stage's resource limits is admitted or
   refused on its content, whatever its depth.
2. **Stack use does not grow with input.** Every structure walk, including
   the copy, comparison, hashing, rendering and release of a recursive value,
   uses memory that the stage's node, byte or work limits already charge. A
   walk never uses host call-stack frames in proportion to input size or
   depth.
3. **Memory is bounded by the configured limits.** Every working structure a
   walk keeps grows by at most a constant per node, byte or work unit already
   charged, so the configured limits bound the stage's memory.
4. **Reaching a limit is a stated outcome.** A stage limit settles as
   `stage_limit_exceeded` (FR-272), and an execution budget as
   `incomplete { limit_kind, ... }` at the denied charge
   (`quire.value.accounting/v1`). Each names the limit, its value and its
   setting (FR-461). A host stack exhaustion is never an outcome, because no
   stage reaches one.
5. **Every limit has a published default.** A limit the caller does not
   configure takes its published default. Defaults are size and work values
   sized for realistic specifications, never depth values. An input that
   does not fit the defaults runs under limits the caller raises to fit it.

[FR-146](../expressions/FR-146-check-total-pure-functions.md)'s checking and
call-nesting statements are this rule applied to the checker and the
evaluator. [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)
fixes the JSON depth of a checked package by its schema, so its reader
charges bytes, nodes, edges, occurrences, diagnostics and work.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-460-AC-1 | A 100,000-deep sum, `else if` chain and `let` chain each parse, check, lower to a checked package, read back and evaluate under node, token and work limits the caller sets to fit them, with the same result as a shallow input of equal meaning. | Test (TC-425) |
| FR-460-AC-2 | A 100,000-deep JSON array, a 100,000-deep JSON object and a 100,000-long recursive list value are read, encoded and compared under byte and work limits set to fit them, and the encoding equals the RFC 8785 text of the same document. | Test (TC-425) |
| FR-460-AC-3 | Each FR-460-AC-1 and FR-460-AC-2 case runs on a thread whose call stack is smaller than one frame per nesting level would need, and completes without host stack exhaustion. | Test (TC-425) |
| FR-460-AC-4 | No limit outcome, effective-limit record or diagnostic of any stage carries a depth or nesting limit kind, and no stage accepts a depth setting from the library, a replay request or the CLI. | Test (TC-425) |
| FR-460-AC-5 | An input that does not fit a stage's default limits settles `stage_limit_exceeded` or `incomplete` naming a byte, node, edge, occurrence, diagnostic or work limit, and the same input succeeds once that limit is raised to fit it. | Test (TC-426) |

## Dependencies

- [FR-461](FR-461-name-the-limit-value-and-setting.md) owns what a limit
  outcome names and the setting names.
- [FR-272](../../objects/foundation/FR-272-typed-diagnostic-cause.md) owns the
  `stage_limit_exceeded` causes.
- [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md) owns the
  schema-fixed body grammar of the checked package.
- [NFR-010](../../non-functional/NFR-010-bound-composed-processing.md) owns the
  incomplete-before-unestablished-result rule for composed processing.

## References

- Linear STD-143; QSL ADR-030 D-1.
