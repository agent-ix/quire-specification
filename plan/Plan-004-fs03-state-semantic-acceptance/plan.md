---
id: Plan-004
title: "FS03 state semantic acceptance"
type: Plan
status: complete
relationships:
  - target: ix://agent-ix/quire-specification/FR-033
    type: references
  - target: ix://agent-ix/quire-specification/FR-040
    type: references
  - target: ix://agent-ix/quire-specification/FR-041
    type: references
  - target: ix://agent-ix/quire-specification/FR-043
    type: references
  - target: ix://agent-ix/quire-specification/FR-044
    type: references
  - target: ix://agent-ix/quire-specification/NFR-010
    type: references
---
# Implementation Plan: FS03 state semantic acceptance

## Requirements Summary

### Functional requirements

- [x] **FR-033/034/038**: accept typed acyclic predicates, exact declaration
  identity, explicit bindings and lexical scope.
- [x] **FR-039/044**: accept exact rational normalization/division and checked
  numeric definedness without approximation.
- [x] **FR-040**: accept the ordered state-core, query and graph definition
  closure without capability unioning.
- [x] **FR-041**: accept bounded ordered duplicate-preserving query semantics.
- [x] **FR-042/045/046**: accept exact anchors, aliases, captures, guard facts,
  invocation inputs and non-Boolean unavailable/refused outcomes.
- [x] **FR-043**: accept typed finite identity/reference/reachability semantics.

### Non-functional requirement

- [x] **NFR-010**: stop state work at declared limits without fabricating a
  Boolean, empty collection or unreachable result.

## Dependency Graph

- `FS01 + FS02 -> Task-001`
  Reason: source/profile identity and model/static authority precede state
  semantic adoption.
- `state-core -> state-queries -> state-graph`
  Reason: each extension retains the exact semantics and refusals of its parent.
- `Task-001 -> qcir #69`
  Reason: the expanded bounded Kani profile consumes accepted meaning and cannot
  supply its own.

Concrete observation/protocol consumers are independent downstream work. They are not prerequisites and are not claimed complete.

### The seam

This plan accepts existing standard rules and portable expected vectors. It does
not modify Rust, Cargo, historical profile bytes or executable fixture status.

## Test Plan

### Specification and vector checks

- [x] Validate FR-033/034/038-046, NFR-010, TC-033/034/038-046/048 and the three
  state definitions with the installed Quire catalog.
- [x] Confirm every accepted vector has healthy/adverse/refused/incomplete
  expectations and that no matrix row is promoted.
- [x] Run base, dependency and EARS self-review and resolve every finding.

### Repository qualification

- [x] Run formatting, unit tests and strict offline Clippy with
  `--target-dir target-codex-backends`.
- [x] Validate changed formal artifacts and record the inherited TestMatrix
  header mismatch separately.
- [x] Run targeted `/gap-analysis`; Rust review is not applicable unless an
  executable Rust/Cargo/workflow file enters the final diff.

## Remaining Work

### Track A: Critical path (serial)

- **A1 = Task-001** Accept and reconcile FS03 — exit: the state closure and
  portable vectors are accepted, every implementation limit remains explicit,
  and qcir #69 has a stable normative input.

## Parallel Execution Summary

`FS01 + FS02 -> Task-001 -> qcir #69`

One standard definition closure and one tracker set make this intentionally
serial. No implementation lane is delegated.

## Task File Mapping

| Task | Track | Owns (references) | Verified by (verifies) | Status |
| --- | --- | --- | --- | --- |
| Task-001 | A | FR-033/034/038-046, NFR-010 | TC-033/034/038-046/048 | done |

## Coordination Rules

- Do not edit the unrelated active Plan-002 native temporal bundle; FS03 #4's
  instruction to update it applies only where relevant, and it is not relevant.
- Do not mark planned conformance rows passed from QSL's partial implementation.
- After merge, close FS03 #4 and immediately update QS #1, ledger #9 and qcir #69
  with the open implementation boundary.
