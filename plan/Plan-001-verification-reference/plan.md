---
id: Plan-001
title: "Qualify and hand off the verification reference amendment"
type: Plan
status: complete
relationships:
  - target: ix://agent-ix/quire-specification/FR-017
    type: references
  - target: ix://agent-ix/quire-specification/FR-018
    type: references
---
# Plan-001: Verification reference amendment

> Superseded in part: this completed plan instructs stored digests, the verification JCS identity domain and digest-keyed vectors. Those were removed (artifact-ref/3-draft, shared-reference/2-draft; FR-018-AC-1, -2, -3, -5, -6, TC-004 and IT-003-SC-02 withdrawn). Treat those instructions as historical.

## Scope

Produce the versioned schema/vector handoff requested by B on PR8. This plan
does not include B's consumer implementation or A's later native compiler work.
Readiness is established by SR-009–016.

## Dependency Graph

The requirement DAG is recorded once in the
[dependency review](../../spec/reviews/verification-reference/dependency.md).
Task frontmatter owns the execution edges. One serial A track is permitted;
no additional agents are spawned. The critical path is bounded fixture/Rust
qualification followed by the reviewed PR handoff. The first is the larger
task; there is no wall-clock delivery guarantee.

## Test Plan

IT-003-SC-01/02/04 can be observed now; SC-03 belongs to B's future qualification.
No source tags or matrix completion may equate those partial stages with full
consumer acceptance. Do not modify the adopted state-core inputs.

## Execution and Gates

Task-001: authored fixture bytes and a Rust qualification-only test crate calling
B's public JCS API plus a Rust JSON Schema validator. Gate: canonical output
conforming to JCS on the authored vectors, actual schema positive/negative controls, bounded input failures, local
Rust formatting/Clippy/tests and retained dependency rights.

Task-002: code/Rust review, scoped gap record, final validation, commit push
and private B handoff. Gate: the retained artifacts accurately distinguish
producer checks from planned independent reader/report observations.
Specification PR #8 merged. Independent consumer adoption remains
outside this plan exactly as declared in Scope and Test Plan.

## Limits

The test crate is not a production shared-reference reader or report assembler.
Its dependency on B is a Cargo git dependency, recorded only in Cargo.toml and
Cargo.lock, which satisfies IT-003's selected clean source requirement without editing B's workspace.
No new executable language or hosted workflow is introduced.
