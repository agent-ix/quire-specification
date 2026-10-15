---
id: Plan-006
title: "FS04 temporal and observation acceptance"
type: Plan
status: complete
relationships:
  - target: ix://agent-ix/quire-specification/FR-090
    type: references
  - target: ix://agent-ix/quire-specification/FR-095
    type: references
  - target: ix://agent-ix/quire-specification/FR-110
    type: references
  - target: ix://agent-ix/quire-specification/FR-116
    type: references
---
# Implementation Plan: FS04 temporal and observation acceptance

## Requirements Summary

### Functional requirements

- [x] **FR-090–095**: select explicit clocks, preserve bounded future/past,
  activation, progress and exact native/TL correspondence meanings.
- [x] **FR-110–116**: bind typed observations, explicit population/completeness,
  replay/incremental semantics, adequacy and generic consumer-result mapping.

### Non-functional requirements

- [x] **NFR-040**: preserve checked temporal work and retention boundaries.
- [x] **NFR-050–052**: bound observation retention, preserve reproducibility and
  contain executable observation behavior.

## Dependency Graph

- `FS01 + FS02 + QSL temporal admission -> Task-001`
  Reason: identity, shared authorities and compiler selection precede semantic
  acceptance.
- `QObs PR #19 + TL owner APIs + qcir PRs #67/#68/#77 -> Task-001`
  Reason: accepted meanings need grounded owner boundaries, not complete
  downstream integration.
- `Task-001 -> qcir #71 -> Plan-002 Task-007`
  Reason: semantic acceptance stabilizes the remaining correspondence work;
  only execution may promote the Planned conformance rows.

QObs #18/#13 retain declaration-keyed subject/topology adoption. Qcir #71
retains native/TL composition. FS06 owns target-specific result mappings.

### The seam

This plan accepts standard meanings and portable controls. It changes no Rust,
Cargo, schema, historical fixture, released tag or executable evidence status.
The temporal/observation integration tests remain Planned.

## Test Plan

### Specification and portable-control checks

- [x] Validate FR-090–095, FR-110–116, NFR-040 and NFR-050–052 with the
  installed Quire catalog.
- [x] Confirm unsupported clocks/operators, missing history/progress and
  incomplete observation populations remain explicit non-values.
- [x] Run scoped base, dependency and EARS self-review and resolve findings.

### Repository qualification

- [x] Run formatting, unit tests and strict offline Clippy with
  `--target-dir target-codex-backends`.
- [x] Validate changed formal artifacts and confirm Planned matrix statuses.
- [x] Run targeted `/gap-analysis`; Rust review is not applicable unless an
  executable Rust/Cargo/workflow file enters the final diff.

## Remaining Work

### Track A: Critical path (serial)

- **A1 = Task-001** Accept and reconcile FS04 — exit: temporal/observation
  meanings are accepted, tracking matches merged reality and qcir #71 has a
  stable normative input without overstating executable evidence.

## Parallel Execution Summary

`FS01 + FS02 + owner implementations -> Task-001 -> qcir #71 -> conformance gate`

One standard closure and one tracker set make this intentionally serial. Agent B
continues owning TL-* libraries; this plan changes no TL repository.

## Task File Mapping

| Task | Track | Owns (references) | Verified by (verifies) | Status |
| --- | --- | --- | --- | --- |
| Task-001 | A | FR-090–095, FR-110–116, NFR-040/050–052 | TC-110–118, TC-140–147 | done |

## Coordination Rules

- Do not mark integration tests passed from portable semantic
  controls or external component evidence.
- Do not close qcir #71 or QObs #18/#13 through FS04.
- After merge, close FS04 #5 and immediately update QS #1, ledger #9, qcir #71
  and the stale QObs dependency descriptions with the accepted revision.
