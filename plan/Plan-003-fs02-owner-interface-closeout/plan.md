---
id: Plan-003
title: "FS02 owner/interface closeout"
type: Plan
status: complete
relationships:
  - target: ix://agent-ix/quire-specification/FR-019
    type: references
  - target: ix://agent-ix/quire-specification/FR-030
    type: references
  - target: ix://agent-ix/quire-specification/FR-035
    type: references
---
# Implementation Plan: FS02 owner/interface closeout

## Requirements Summary

### Functional requirements

- [x] **FR-019**: Preserve the selected source/model profile boundary without
  inventing presence syntax or claiming the planned runtime-value corpus.
- [x] **FR-030**: Bind the native package to exact accepted model,
  configuration, definition and producer/native selections.
- [x] **FR-035**: Keep static subject identity separate from concrete
  assessment roles, populations, snapshots/windows and observations.

## Dependency Graph

- `QS #29 + #28 + #31 + #40 -> Task-001`
  Reason: vocabulary, identity, protocol and temporal/observation authority must
  be accepted before their common owner map can close.
- `FCD #95 / PR #99 -> QSL #93 / PR #94 -> Task-001`
  Reason: the native consumer must select and exercise the accepted immutable
  static producer revision before FS02 can close.
- `Task-001 -> FS03 / FS04 / FS05`
  Reason: those profiles consume FS02's authority and identity decisions.

### The seam

This plan changes the living standard index, owner map, capability ledger and
crosswalk only. It consumes merged FCD/QSL evidence and does not modify Rust,
Cargo, fixtures, a TestMatrix status or any historical artifact.

## Test Plan

### External integration evidence

- [x] **QSL IT-009**: Confirm the manifest/lock select FCD merge `4042882` and
  the real direct adapter suite passes all eight identity/refusal controls.
- [x] **FCD Plan-017**: Confirm its nine tasks, 90 target rows and final
  cross-architecture/static-producer evidence are complete.

### Tracking and specification checks

- [x] Confirm every current FCD #95/QSL #93 “open” statement is removed from
  the living owner map, ledger, crosswalk, master index and TC-007 status.
- [x] Confirm every edited statement preserves FCD #93, assessment production,
  runtime consumers and planned TC rows as open.
- [x] Validate the changed formal artifacts and all review/plan artifacts with
  Quire; record inherited repository-wide matrix-schema failures separately.

## Remaining Work

### Track A: Critical path (serial)

- **A1 = Task-001** Reconcile and close FS02 — Easy; exit: every living tracker
  agrees with accepted revisions and every unimplemented remainder has an owner.
- **Gate = Task-001** PR readiness — measures factual/tracking agreement; pass:
  local qualification gates and PASS `/gap-analysis`, with Rust review correctly
  recorded not applicable because no executable file changed.

## Parallel Execution Summary

`accepted QS/FCD/QSL inputs -> Task-001 reconciliation -> FS03/FS04/FS05`

One shared tracker set makes this deliberately serial. No implementation lane is
delegated.

## Task File Mapping

| Task | Track | Owns (references) | Verified by (verifies) | Status |
| --- | --- | --- | --- | --- |
| Task-001 | A | FR-019, FR-030, FR-035 | QSL IT-009 | done |

## Coordination Rules

- Do not rewrite historical reviews or change planned TestMatrix rows to passed.
- Do not edit FCD, QSL, TL, Contract-IR, protocol or observation code from this
  repository closeout.
- After merge, close FS02 #3 and immediately update QS #1, ledger #9, and the
  prerequisite state of FS03 #4, FS04 #5 and FS05 #6.
