---
id: Plan-005
title: "FS05 interchange and migration acceptance"
type: Plan
status: complete
relationships:
  - target: ix://agent-ix/quire-specification/FR-014
    type: references
  - target: ix://agent-ix/quire-specification/FR-016
    type: references
  - target: ix://agent-ix/quire-specification/FR-018
    type: references
  - target: ix://agent-ix/quire-specification/FR-032
    type: references
  - target: ix://agent-ix/quire-specification/FR-130
    type: references
---
# Implementation Plan: FS05 interchange and migration acceptance

## Requirements Summary

### Functional requirements

- [x] **FR-014/017**: select closed versioned envelopes, exact roles and strict
  refusal before semantic use.
- [x] **FR-015/018**: preserve independent canonical identity domains and exact
  verification-JCS derivation.
- [x] **FR-016/032**: preserve historical selections and require explicit,
  separately identified migration for changed meaning.
- [x] **FR-130**: retain outer composed-package identity separately from each
  inner semantic profile and payload wire.

### Non-functional requirements

- [x] **NFR-001–003**: zero historical byte drift, no inferred equivalence,
  incomplete on resource exhaustion and refusal of unqualified identity reuse.

## Dependency Graph

- `FS01 + FS02 + FS03 -> Task-001`
  Reason: identity, static authority and accepted state meaning precede standard
  interchange acceptance.
- `QSpec PR8 + QSL strict producer/reader + protocol PR52 -> Task-001`
  Reason: the selected packet and at least one independent strict consumer must
  exist; every consumer campaign need not be complete.
- `Task-001 -> qcir #69 K0`
  Reason: the bounded Kani profile may consume accepted standard meanings but
  may never invent or approximate them.

Protocol #11 remains the version-1 regression lane. IT-002/003 and public
registration remain independent qualification/publication work.

### The seam

This plan accepts exact standard meanings and portable controls. It changes no
Rust, Cargo, schema, historical fixture, released tag or executable evidence
status. TM-001 and TM-003 rows remain Planned.

## Test Plan

### Specification and portable-control checks

- [x] Validate FR-014–018, FR-032, FR-130, NFR-001–003 and their selected TC
  artifacts with the installed Quire catalog.
- [x] Confirm historical schema/vector bytes are untouched and every explicit
  refusal/incomplete boundary remains present.
- [x] Run scoped base, dependency and EARS self-review and resolve findings.

### Repository qualification

- [x] Run formatting, unit tests and strict offline Clippy with
  `--target-dir target-codex-backends`.
- [x] Validate changed formal artifacts and confirm Planned matrix statuses.
- [x] Run targeted `/gap-analysis`; Rust review is not applicable unless an
  executable Rust/Cargo/workflow file enters the final diff.

## Remaining Work

### Track A: Critical path (serial)

- **A1 = Task-001** Accept and reconcile FS05 — exit: exact version/canonical/
  migration behavior is accepted, tracking matches merged reality and qcir #69
  has a stable normative input.

## Parallel Execution Summary

`FS01 + FS02 + FS03 + strict /2 evidence -> Task-001 -> qcir #69 K0`

One standard closure and one tracker set make this intentionally serial. Agent B
continues owning TL-* libraries; no implementation lane is delegated here.

## Task File Mapping

| Task | Track | Owns (references) | Verified by (verifies) | Status |
| --- | --- | --- | --- | --- |
| Task-001 | A | FR-014–018, FR-032, FR-130, NFR-001–003 | TC-001–004, TC-030, TC-032 | done |

## Coordination Rules

- Do not mark TM-001/TM-003 passed from partial or external implementation
  evidence; portable semantic controls and execution status are separate.
- Do not close protocol #11, IT-002/003 or public-registration work through FS05.
- After merge, close FS05 #6 and immediately update QS #1, ledger #9 and qcir
  #69 with the accepted revision and exact remaining boundaries.
