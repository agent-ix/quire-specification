---
id: Plan-008
title: "Complete V1 scope, architecture and delivery reconciliation"
type: Plan
status: active
relationships:
  - target: ix://agent-ix/quire-specification/StR-002
    type: references
  - target: ix://agent-ix/quire-specification/FR-020
    type: references
  - target: ix://agent-ix/quire-specification/FR-031
    type: references
  - target: ix://agent-ix/quire-specification/FR-032
    type: references
  - target: ix://agent-ix/quire-specification/NFR-001
    type: references
---
# Implementation Plan: Complete V1 scope, architecture and delivery reconciliation

## Outcome

Repair the existing Quire program before more product implementation. Produce
one atomic master inventory, reconcile every capability against existing
specifications, code, epics and tickets, evaluate the information architecture
of specifications facing substantial expansion, design and review the complete
modular architecture, then populate the existing program with every required
implementation ticket. No capability may close through deferral.

This plan belongs to QSpec #1/#63 and updates the existing quire-research #36
roadmap. It does not create a competing implementation program.

## Requirements Summary

- [ ] **StR-002**: preserve one composed native language across state, temporal
  and choreography semantics while expanding it to the complete V1 inventory.
- [ ] **FR-020**: compose independently versioned semantic profiles without
  hiding required features behind a partial release.
- [ ] **FR-031**: report requested capabilities and unsupported consumers
  exactly, without erasing admitted language meaning.
- [ ] **FR-032**: preserve historical definitions while extending the current
  edition and implementation graph explicitly.
- [ ] **NFR-001**: preserve versioned meaning and evidence identity throughout
  scope correction, architecture revision and implementation planning.

## Authoritative Inputs

- QSpec #1, #9 and #63; quire-research #36–#43 and #65–#67.
- The current 51-row capability crosswalk and functional ledger.
- Existing QSpec corpus: 2 StR, 8 US, 73 functional FR, 13 NFR, 11 IT, 90 object-layer FR,
  definitions, 76 TC, 7 Test Matrices and 4 ArchitectureDescriptions.
- Default branches and live tickets across QSpec, QSL, Contract IR, codegen,
  Protocol, Observation, TL, Verification, FCD, Quoin and related Rust owners.
- Current QSL reserved-source inventory and every prose-only exclusion.

## Dependency Graph

- `Task-001 -> Task-002`
  Reason: the governed planning contract precedes the authoritative inventory.
- `Task-002 -> Task-003, Task-005`
  Reason: specification and issue reconciliation require atomic capabilities.
- `Task-003 -> Task-004`
  Reason: organization changes must follow a complete existing-spec coverage map.
- `Task-003 + Task-004 + Task-005 -> Task-006`
  Reason: architecture must reuse sound existing semantics, fit a scalable spec
  topology and retain established component owners.
- `Task-006 -> Task-007 -> Task-008`
  Reason: the comprehensive architecture precedes normative completion and the
  coherent review revision.
- `Task-008 -> Task-009 -> Task-010`
  Reason: implementation tasking derives only from accepted requirements, then
  the final audit proves total coverage before engineering resumes.

### Shared constraints

- Existing complete specification and implementation are reused, not recreated.
- Existing epics are repaired before a new child epic is proposed.
- Closed leaf tickets remain evidence; widening a parent does not invalidate them.
- `Later`, `Deferred`, `Future`, `Research-only`, unowned and unticketed are
  forbidden delivery dispositions.
- A blocked capability remains V1-required with an explicit dependency.
- Unsound claims are atomic integrity constraints, not excuses to omit their
  corresponding sound language or analysis capability.
- Stable semantic IDs, history, tags, links and `resources/native-v1/` remain intact.

## Test Plan

### Planning and inventory controls

- [ ] **TC-020 / inventory census**: every previous capability, reserved source
  construct, prose exclusion, existing spec artifact, public implementation
  surface and live ticket maps to an atomic inventory row.
- [ ] **TC-020 / forbidden disposition**: zero capability rows use a deferral,
  future-only, research-only, unowned or unticketed disposition.
- [ ] **TC-032 / historical preservation**: reorganizing or amending the current
  standard preserves old definition identities and evidence links.

### Specification and architecture controls

- [ ] **TC-030 / existing-spec reuse**: each capability identifies exact existing
  coverage before an amendment or new artifact is proposed.
- [ ] **TC-020 / atomic coverage**: compound, bounded, conflicting and missing
  specification states are distinguished and completely resolved.
- [ ] **TC-040 / extensibility**: adding an operator, type, declaration, semantic
  family, backend and consumer does not require architectural replacement.
- [ ] **TC-049 / temporal composition** and **TC-170 / choreography subjects**:
  the complete architecture retains typed common expressions and independent
  temporal/protocol semantics without duplicate authorities.

### Program and ticket controls

- [ ] **TC-020 / epic ownership**: every capability has exactly one primary
  existing epic or a justified missing child-epic action.
- [ ] **TC-020 / implementation path**: every capability has complete tickets for
  all applicable syntax, checking, runtime, wire, backend, integration and
  qualification stages.
- [ ] **TC-032 / maintained roadmap**: QSpec #1/#9 and the existing
  quire-research plan, JSON manifest and issue graph agree exactly.

## Remaining Work

### Track A: Truth and scope foundation

- **A1 = Task-001** Install Plan-008 — exit: one Quire-valid governed bundle owns
  the reconciliation sequence without starting `/specify`.
- **A2 = Task-002** Build the atomic master inventory — exit: every known feature,
  constraint, artifact, code surface and ticket has one visible row.
- **A3 = Task-003** Audit existing specification coverage — exit: every row has
  evidence-backed reuse, repair, split, amendment or missing-artifact action.

### Track B: Existing structure and ownership

- **B1 = Task-004** Evaluate specification information architecture — exit:
  expansion owners have a preserve/reorganize plan before bulk authoring.
- **B2 = Task-005** Reconcile existing epics and tickets — exit: every row has a
  primary existing owner or a justified missing child.

### Track C: Coherent architecture and specification

- **C1 = Task-006** Design the complete modular architecture — exit: all ten
  subsystems and their typed seams are defined together.
- **C2 = Task-007** Complete `/specify` — exit: reused and amended artifacts cover
  the entire inventory with EARS acceptance behavior.
- **C3 = Task-008** Complete self `/spec-review` — exit: the coherent revision
  passes base plus all seven analyses and supplemental object/architecture checks.

### Track D: Delivery graph and release gate

- **D1 = Task-009** Populate the existing implementation program — exit: all
  epics and implementation tickets exist in the established program spine.
- **D2 = Task-010** Prove campaign coverage — exit: inventory-to-spec,
  inventory-to-epic and inventory-to-ticket audits are all 100%.

## Parallel Execution Summary

```text
A1 -> A2 -> A3 --------> C1 -> C2 -> C3 -> D1 -> D2
            \-> B1 ----/
       \--------> B2 --/
```

The architecture and normative authoring lane is single-writer. Evidence census
may be gathered independently, but no domain agent may make an isolated scope or
architecture decision.

## Task File Mapping

| Task | Track | Owns | Verified by | Status |
| --- | --- | --- | --- | --- |
| Task-001 | A | Plan-008 governance | TC-020 | done |
| Task-002 | A | Atomic inventory | TC-020, TC-032 | done |
| Task-003 | A | Existing-spec coverage | TC-020, TC-030 | done |
| Task-004 | B | Spec information architecture | TC-030, TC-032 | done |
| Task-005 | B | Epic/ticket reconciliation | TC-020 | done |
| Task-006 | C | Complete subsystem architecture | TC-040, TC-049, TC-170 | done |
| Task-007 | C | Normative specification completion | TC-020, TC-030 | done |
| Task-008 | C | Coherent review and remediation | TC-020, TC-032 | in_progress |
| Task-009 | D | Existing delivery program | TC-020, TC-032 | not_started |
| Task-010 | D | Final coverage and tracking gate | TC-020, TC-032 | not_started |

## Coordination Rules

- Work from fresh isolated worktrees based on current default branches.
- Do not touch another agent's branch, files or active worktree.
- Do not begin `/specify` before Tasks 001–006 complete.
- Do not begin product implementation before Task-010 completes.
- Major spec expansion first evaluates and, where necessary, repairs repository
  information architecture, especially in QSpec and Protocol.
- Reorganization preserves semantic IDs and is justified by a demonstrated
  navigation, ownership, duplication or growth problem; no cosmetic churn.
- After each merge update the child ticket, parent epic, ledger, matrix and
  maintained plan immediately.
- Product implementation remains Rust-only, `publish = false`, local-test-only
  with `target-codex-backends`; no hosted CI dispatch or tag/resource mutation.

## Completion Criteria

- Atomic inventory has no omissions or deferral states.
- Existing-spec coverage is verified before new authoring.
- Spec repositories can absorb the expansion without flat or duplicate structure.
- Complete architecture is accepted as one coherent revision.
- Every capability has normative coverage, tests, an epic and implementation tickets.
- Existing program tracking agrees mechanically and textually at every level.
- Only then may the implementation campaign resume.
