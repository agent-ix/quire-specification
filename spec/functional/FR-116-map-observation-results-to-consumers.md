---
id: FR-116
title: "Map observation outcomes to executable and output consumers"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-009
    type: implements
---
## Description

When delivering an observation assessment to an executable or exported consumer, the consumer adapter SHALL retain the requested subject, capability, source/binding revision, per-obligation disposition, completeness reasons, and mapping-loss classification.

## Behavior

The adapter SHALL classify each mapped obligation as preserved, conditional, unrepresented, or refused; incomplete, pending, and refused outcomes are not Boolean success. It SHALL consume A-owned interchange identities and B-owned result semantics; codegen and export mappings do not become clause authorities.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-116-AC-1 | A consumer receives source/binding/progress provenance with each per-obligation disposition. | Test (TC-146) |
| FR-116-AC-2 | An unsupported or lossy mapping is visible as conditional, unrepresented, or refused. | Test (TC-146) |
| FR-116-AC-3 | A consumer cannot convert pending, incomplete, or refused into a passed Boolean result. | Test (TC-146) |

## Dependencies

- [Shared foundation](../../proposals/quire-v1/shared-foundation.md).
- A owns interchange/versioning; B owns result semantics; D/E own their mapping meanings.
