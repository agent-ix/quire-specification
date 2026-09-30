---
id: Plan-007
title: "FS06 output-mapping acceptance"
type: Plan
status: complete
relationships:
  - target: ix://agent-ix/quire-specification/FR-120
    type: references
  - target: ix://agent-ix/quire-specification/FR-125
    type: references
  - target: ix://agent-ix/quire-specification/FR-297
    type: references
  - target: ix://agent-ix/quire-specification/FR-299
    type: references
---
# Implementation Plan: FS06 output-mapping acceptance

## Requirements Summary

### Common subsystem

- [x] **AD-004, FR-120/121/125 and FR-297–299**: select one exact target,
  account for every source obligation without approximation and assemble one
  deterministic immutable output package.
- [x] **FR-269 refinement**: retain `preserved`, `conditional`,
  `unrepresented` and `refused` with exact target/mapping identity and separate
  observation/protocol adequacy.

### Target profiles

- [x] **FR-122**: bounded output-only OCL 2.4 correspondence.
- [x] **FR-123**: bounded output-only SysML 2.0/KerML 1.0 correspondence.
- [x] **FR-124**: bounded output-only FRETish 3.1 correspondence.

### Non-functional requirements

- [x] **NFR-060**: checked aggregate bounds, no partial success and repeatable
  bytes/identity.
- [x] **NFR-061**: Rust-only first-party paths and optional downstream observers.

## Dependency Graph

- `FS01 + FS02 + FS03 + FS04 + FS05 -> Task-001`
  Reason: source/profile authority and accepted scalar, graph, temporal,
  observation and interchange meanings precede derived-output classification.
- `Task-001 -> Task-002`
  Reason: all target profiles consume one loss, identity and package boundary.
- `Task-001 + Task-002 -> Task-003 -> qcir #55/#56/#57`
  Reason: executable exporters must bind to accepted target profiles.

Qcir #55, #56 and #57 are independent implementation lanes after the shared
boundary. No target mapper depends on another target mapper. Agent B's TL-* lane
is untouched; FRETish structural output is not the FR-095 native/TL bridge.

### The seam

This plan accepts QSpec contracts, bounded correspondences and portable
controls. It changes no Rust, Cargo, workflow, schema, executable fixture,
historical native-v1 resource or release tag. IT-060 remains Planned.

## Test Plan

### Specification and portable-control checks

- [x] Validate the common and target requirement/object/NFR/integration/test
  artifacts with the installed Quire catalog.
- [x] Exercise portable controls for missing identity, unsupported meaning,
  target-boundary confusion, resource exhaustion and observer overclaim.
- [x] Run scoped base, boundary, object, dependency, risk and EARS self-review
  and resolve every in-scope finding.

### Repository qualification

- [x] Run formatting, unit tests and strict offline Clippy with
  `--target-dir target-codex-backends`.
- [x] Run repository-wide Quire validation and confirm no Planned evidence is
  promoted and no new coverage-status lie is introduced.
- [x] Run targeted `/gap-analysis`; Rust review is not applicable unless a
  Rust/Cargo/workflow/executable-fixture file enters the final diff.

## Remaining Work

### Track A: Common contract (serial foundation)

- **A1 = Task-001** Define the shared mapping subsystem — exit: exact profile,
  record, loss, package, resource and observer boundaries are accepted.

### Track B: Target profiles (parallel-capable after A1)

- **B1 = Task-002** Define all three bounded profiles coherently — exit: each
  selected construct and every unsupported construct has an exact disposition.

### Track C: Control and handoff

- **C1 = Task-003** Review, validate and route — exit: QSpec tracking and qcir
  implementation tickets name the accepted revision without claiming execution.

## Parallel Execution Summary

```text
accepted FS01–FS05
        |
 common subsystem
   /      |       \
 OCL   SysML/KerML FRETish
   \      |       /
 review + validation + exact handoff
   /      |       \
qcir55  qcir56   qcir57
```

The target implementation lanes may run independently after this PR merges.
The common QSpec boundary is the integration point, not shared target code.

## Task File Mapping

| Task | Track | Owns (references) | Verified by (verifies) | Status |
| --- | --- | --- | --- | --- |
| Task-001 | A | AD-004, FR-120/121/125, FR-269, FR-297–299 | TC-150, TC-154, TC-155 | done |
| Task-002 | B | FR-122–124, NFR-060/061 | TC-151–155 | done |
| Task-003 | C | IT-060 and tracker handoff | Quire validation, SR-552–SR-558 | done |

## Coordination Rules

- Never infer preservation from plausible text or parser/tool acceptance.
- Do not mark IT-060 passed from specification-only controls.
- Do not close qcir #55/#56/#57 through FS06.
- After merge, close FS06 #7 and immediately update QS #1, ledger #9 and each
  qcir implementation ticket with the exact accepted merge revision.
