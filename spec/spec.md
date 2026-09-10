---
type: master-requirements
name: quire-specification
org: agent-ix
component_type: language-standard
implementation_language: null
standards_alignment:
  - iso-iec-ieee-29148
  - ieee-828
relationships:
  - target: ix://agent-ix/quire-specification/FR-019
    type: contains
  - target: ix://agent-ix/quire-specification/FR-001
    type: contains
  - target: ix://agent-ix/quire-specification/FR-002
    type: contains
  - target: ix://agent-ix/quire-specification/FR-003
    type: contains
  - target: ix://agent-ix/quire-specification/FR-004
    type: contains
  - target: ix://agent-ix/quire-specification/FR-005
    type: contains
  - target: ix://agent-ix/quire-specification/FR-006
    type: contains
  - target: ix://agent-ix/quire-specification/FR-007
    type: contains
  - target: ix://agent-ix/quire-specification/FR-008
    type: contains
  - target: ix://agent-ix/quire-specification/FR-009
    type: contains
  - target: ix://agent-ix/quire-specification/FR-010
    type: contains
  - target: ix://agent-ix/quire-specification/FR-011
    type: contains
  - target: ix://agent-ix/quire-specification/FR-012
    type: contains
  - target: ix://agent-ix/quire-specification/FR-013
    type: contains
  - target: ix://agent-ix/quire-specification/FR-014
    type: contains
  - target: ix://agent-ix/quire-specification/FR-015
    type: contains
  - target: ix://agent-ix/quire-specification/FR-016
    type: contains
  - target: ix://agent-ix/quire-specification/FR-017
    type: contains
  - target: ix://agent-ix/quire-specification/FR-018
    type: contains
  - target: ix://agent-ix/quire-specification/IT-003
    type: contains
  - target: ix://agent-ix/quire-specification/US-001
    type: contains
  - target: ix://agent-ix/quire-specification/US-002
    type: contains
  - target: ix://agent-ix/quire-specification/US-003
    type: contains
  - target: ix://agent-ix/quire-specification/US-004
    type: contains
  - target: ix://agent-ix/quire-specification/StR-001
    type: contains
  - target: ix://agent-ix/quire-specification/NFR-001
    type: contains
  - target: ix://agent-ix/quire-specification/NFR-002
    type: contains
  - target: ix://agent-ix/quire-specification/NFR-003
    type: contains
  - target: ix://agent-ix/quire-specification/NFR-004
    type: contains
  - target: ix://agent-ix/quire-specification/IT-001
    type: contains
  - target: ix://agent-ix/quire-specification/IT-002
    type: contains
---
# Master Requirements Specification

## Native finite-state language standard

## 1. Purpose

This root indexes the discrete requirements for Agent A's assigned native finite-state work. It corrects the earlier proposal-only authoring by using the requested /specify catalog contract. All artifacts remain draft with conditional /spec-review findings; retrospective documentation does not retroactively approve implementation.

## 2. Scope

### 2.1 In Scope

FS01 admitted grammar/profile; FS02 shared model/source and artifact identity; FS03 finite-state semantics; FS05 strict interchange, canonical domains and migration.

### 2.2 Out of Scope

Temporal/protocol/message semantics, B's portable method/plan/result schemas, C's existing-repository adapter implementation, new model authority, a second executable binder, public publication and standard-wide license selection are outside this repository's implementation ownership. These exclusions do not remove the required coordinated interfaces or the full supported-state workflow.

## 3. System Overview

### 3.1 System Description

The language defines an explicit finite-state profile. The native pipeline preserves source identity through parsing, model linking, runtime validation, reference evaluation and qualified backend projection. A schema package is distinct from a runtime population. Existing shared-model and executable-IR owners remain authoritative.

### 3.2 Intended Users

Specification authors, model producers, verification operators and toolchain integrators.

## 4. Requirements Architecture

Each stakeholder requirement, story, functional requirement, quality constraint and integration specification is a separate catalog-typed file. Proposal documents remain detailed design inputs; this index does not replace those requirements.

## 5. Requirement Classes

StR states the operational need. US records user intent. FR defines observable behavior. NFR records measurable quality constraints. IT specifies a real external process/API/file boundary. TC and matrix coverage are assessed by the review workflow.

## 6. Requirement Identification

IDs are stable within this repository. Cross-repository identities use ix://agent-ix/quire-specification/ID. Acceptance criteria use the parent ID plus -AC-N; integration steps use -SC-NN.

## 7. Requirement Quality Policy

Requirements use atomic observable outcomes. Unknown support, absent evidence and unimplemented prerequisites remain explicit. Validation of Markdown structure cannot establish semantic correctness or implementation completion.

## 8. State and Execution Model

### 8.1 State Semantics

Runtime inputs are immutable typed finite populations with explicit closure assumptions. Object identity differs from record equality. Optional absence, invalid input and unavailable observation retain distinct meanings.

### 8.2 Transition Semantics

Operation observations belong to one invocation with an authored frame and exact object delta. This first profile does not define a protocol state machine or temporal clock semantics.

### 8.3 Determinism Guarantees

Guard and collection evaluation order, source activation and selected canonical domains remain explicit. Resource exhaustion yields incompleteness, not a logical violation.

## 9. Events and Signals

### 9.1 Event Model

The pipeline may report observed implication activation. It introduces no domain event bus or observation protocol.

### 9.2 Event Guarantees

Only events actually observed during evaluation are reported. Missing backend probes remain unavailable coverage.

## 10. Error and Failure Model

### 10.1 Error Classification

Invalid syntax/input and unsupported capabilities refuse their stage. Missing observations, cancellation and exhausted resources remain incomplete.

### 10.2 Failure Handling Guarantees

No failed prerequisite yields a completed Boolean. No unsupported obligation is silently erased.

## 11. Traceability

Frontmatter relates StR, US, FR/NFR and IT artifacts. Acceptance-criterion-to-TC coverage and implementation trace tags require the review workflow; existing Rust tests do not automatically create formal TC identities.

## 12. Verification Strategy

Run Quire over this exact repository scope. Use real existing producers/consumers for integration evidence. The end-state acceptance remains a supported healthy, violating and refused/incomplete native state workflow. Existing syntax tests and authored semantic examples are partial evidence only.

## 13. Change Management

The user requested /specify and /spec-review for all work. Existing implementation is being specified retrospectively and remains subject to findings. Further dependent implementation waits for the required review and accepted shared contracts. Historical fixture and profile bytes remain immutable under their selected bindings.

## 14. Lifecycle Status

The 2026-09-09 ruling selects native Quire as the sole editable formal-clause
language. [FR-019](functional/FR-019-admit-native-source-profile.md) and the
[versioned amendment](../proposals/native-source-1-draft/README.md) carry forward
its admission rules. The replacement candidate is not enabled by this document;
the adopted historical definitions and their implementation remain distinguishable.

The owner adopted e897f81 for internal implementation. Accepted IR ADR-0054
(PR61, 690bde7) subsequently removed the universal Filament reader prerequisite;
#54 is closed. Generic native compilation can use the existing IR API.
Independent consumer qualification and full FS02/FS03/FS05 acceptance remain
open. FR-017/018 define the versioned amendment resolving B's representational
findings; its tests do not claim B's consumer implementation or adoption.

## 15. Governance Notes

Use private owning issues and isolated worktrees. A owns the native producer and these requirements; B owns portable verification contracts; C owns coordinated existing-repository consumers. Do not spawn additional agents under the current assignment. Preserve original dependency/template grants. Standard-wide publication terms remain unresolved.

## 16. References

The full task is tmp/formalization-agent-a-language-core.md in the workspace. The current authoring pack selected agent-ix from the Git remote. Authoring used Quoin 0.23.1, Quire 0.31.0 and the installed spec-artifacts-iso catalog shapes.

## Requirements

| Artifact | Type | Status |
| --- | --- | --- |
| [FR-001](functional/FR-001-select-native-profile.md) | FR | Draft |
| [FR-002](functional/FR-002-resolve-owned-declarations.md) | FR | Draft |
| [FR-003](functional/FR-003-bind-source-correspondence.md) | FR | Draft |
| [FR-004](functional/FR-004-separate-artifact-identities.md) | FR | Draft |
| [FR-005](functional/FR-005-guard-optional-values.md) | FR | Draft |
| [FR-006](functional/FR-006-preserve-control-evaluation.md) | FR | Draft |
| [FR-007](functional/FR-007-prove-integer-definedness.md) | FR | Draft |
| [FR-008](functional/FR-008-preserve-sequence-semantics.md) | FR | Draft |
| [FR-009](functional/FR-009-preserve-object-identity.md) | FR | Draft |
| [FR-010](functional/FR-010-evaluate-exact-reachability.md) | FR | Draft |
| [FR-011](functional/FR-011-validate-populations.md) | FR | Draft |
| [FR-012](functional/FR-012-bind-operation-observations.md) | FR | Draft |
| [FR-013](functional/FR-013-validate-operation-frames.md) | FR | Draft |
| [FR-014](functional/FR-014-read-exact-wire.md) | FR | Draft |
| [FR-015](functional/FR-015-preserve-canonical-domains.md) | FR | Draft |
| [FR-016](functional/FR-016-migrate-explicitly.md) | FR | Draft |
| [FR-017](functional/FR-017-select-verification-artifact-roles.md) | FR | Amendment |
| [FR-018](functional/FR-018-select-verification-jcs-domain.md) | FR | Amendment |
| [FR-019](functional/FR-019-admit-native-source-profile.md) | FR | Native-source definition candidate; implementation incomplete |
| [US-001](usecase/US-001-understand-native-meaning.md) | US | Draft |
| [US-002](usecase/US-002-exchange-exact-subjects.md) | US | Draft |
| [US-003](usecase/US-003-assess-finite-state.md) | US | Draft |
| [US-004](usecase/US-004-retain-evidence-meaning.md) | US | Draft |
| [StR-001](stakeholder/StR-001-native-assessment-trust.md) | StR | Draft |
| [NFR-001](non-functional/NFR-001-preserve-versioned-meaning.md) | NFR | Draft |
| [NFR-002](non-functional/NFR-002-classify-resource-incompleteness.md) | NFR | Draft |
| [NFR-003](non-functional/NFR-003-qualify-reference-bindings.md) | NFR | Draft |
| [NFR-004](non-functional/NFR-004-retain-review-and-rights.md) | NFR | Draft |
| [IT-001](integration/IT-001-legacy-canonical-reader.md) | IT | Draft |
| [IT-002](integration/IT-002-independent-reference-consumer.md) | IT | Draft |
| [IT-003](integration/IT-003-verification-reference-amendment.md) | IT | Planned |
