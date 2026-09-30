---
id: SR-520
title: "Dependency review of FS01 language charter ratification"
type: SpecReview
analysis: dependency
scope: "AD-003, profile definition set, ADR-0053 handoff and capability ledger"
review_set: subset
evaluated_revision: "task/2 working tree based on 96fc29d"
review_date: "2026-09-13"
---
# Dependency review of FS01 language charter ratification

## Summary

PASS. FS01 now closes only the language/profile identity and subsystem boundary;
it exposes, rather than absorbs, the remaining producer, semantic-family,
backend, Kani, observation and protocol dependencies.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No unresolved dependency is incorrectly represented as an FS01 blocker or completed implementation. | AD-003; capability ledger |

| Dependency | Current FS01 treatment |
| --- | --- |
| Contract-IR ADR-0053 | Completed by PR #72; the local handoff is historical correspondence. |
| Historical compiler profile | `ix:native` / `0-draft` / `state-finite/0-draft` stays immutable and remains the released bounded implementation basis. |
| Composed state definitions | Identity roles/dependency direction accepted; FS02/FS03 freeze semantic revisions and producer correspondence. |
| Original ConfigVersion model | Still requires an explicit finite maximum and relationship correspondence; the bounded worked example does not discharge it. |
| Temporal/TL work | Native temporal profile roles are accepted; exact temporal correspondence and TL implementation remain with their named owners. No TL-owned file is changed. |
| Protocol/result work | Native protocol profile role is accepted; protocol consumer and result work remains with Agent B. |
| Oracle/Kani work | The ledger records the merged scalar slice and qcir #69/#70/#71 remainder without changing source admission. |
| Output mappings | FS06 retains Rust mapping implementations and loss evidence; foreign runtimes are not qualification dependencies. |

The profile definition graph is monotone: edition to state core to query to
graph, with separately selected temporal/protocol/observation/package roles.
Unknown, conflicting, cyclic or unavailable selections refuse rather than select
an installed substitute.
