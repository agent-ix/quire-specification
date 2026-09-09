---
id: Plan-001
title: "Qualify and hand off the verification reference amendment"
type: Plan
status: active
relationships:
  - target: ix://agent-ix/quire-specification/FR-017
    type: references
  - target: ix://agent-ix/quire-specification/FR-018
    type: references
---
# Plan-001: Verification reference amendment

## Scope

Produce the versioned schema/vector handoff requested by B on PR8. This plan
does not include B's consumer implementation or A's later native compiler work.
Readiness is established by SR-009–016 at f446a64 over specification 9244dcc.

## Dependency Graph

The requirement DAG is recorded once in the
[dependency review](../../spec/reviews/verification-reference/dependency.md).
Task frontmatter owns the execution edges. One serial A track is permitted;
no additional agents are spawned. The critical path is bounded fixture/Rust
qualification followed by the reviewed PR handoff. The first is the larger
task; there is no wall-clock delivery guarantee.

## Test Plan

[TM-001](../../spec/verification-reference/tests.md) is the single test inventory.
IT-003-SC-01/02/04 can be observed now; SC-03 belongs to B's future qualification.
No source tags or matrix completion may equate those partial stages with full
consumer acceptance. Preserve the existing adopted state-core bytes exactly.

## Execution and Gates

Task-001: authored fixture bytes and a Rust qualification-only test crate calling
B's pinned public JCS API plus a Rust JSON Schema validator. Gate: exact golden
bytes, actual schema positive/negative controls, bounded input failures, local
Rust formatting/Clippy/tests and retained dependency rights.

Task-002: code/Rust review, scoped gap record, final validation, exact commit push
and private B handoff. Gate: the retained artifacts accurately distinguish
producer checks from planned independent reader/report observations. Then make
the PR reviewable and land it if its actual merge prerequisites are satisfied.

## Limits

The test crate is not a production shared-reference reader or report assembler.
Its dependency pin is immutable; a Cargo git checkout at the exact B revision
satisfies IT-003's selected clean source requirement without editing B's workspace.
No new executable language or hosted workflow is introduced.
