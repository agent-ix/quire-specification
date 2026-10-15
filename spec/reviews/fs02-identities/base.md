---
id: SR-527
title: "Base review of FS02 protocol identity rulings"
type: SpecReview
analysis: base
scope: "quire-specification #28; FR-035, FR-049, FR-050, FR-051, FR-057, FR-058 and protocol identity objects FR-210 through FR-238, FR-287 and FR-288 changed by the ticket"
review_set: subset
review_date: "2026-09-13"
---
# Base review of FS02 protocol identity rulings

## Summary

PASS. The packet answers every identity question in issue #28 with exact key,
equality, absence and non-causal ordering rules, and it keeps relationship facts
outside runtime opaque keys. Every new criterion maps to a planned mutation case;
the packet claims no executable cross-repository coverage.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Resolved: the first draft used “qualified protocol-definition identity” without defining its components; FR-288 now fixes the exact source-document key, admitted declaration handle, formal owner/clause and semantic selection closure. | FR-288; FR-049; FR-050 |
| FND-002 | medium | Resolved: the first draft named a global typed recovery-predicate identity that current native syntax does not provide; the final text uses the actual declaration-local compensation and typed recovery-expression handles and retains FR-015's source-free identity limitation. | FR-057; FR-238; FR-015 |
| FND-003 | low | Resolved: FR-210 no longer describes a definition as untyped, and FR-223 makes the related send a relationship outside the complete FR-221 key. | FR-210; FR-223 |

## Ruling disposition

| Issue #28 question | Authoritative disposition |
| --- | --- |
| Role, channel and static definition | FR-288 qualifies the linked protocol definition. Role and channel append their exact admitted declaration-local handles. A package-local index alone, display name or span cannot identify either. |
| Workflow, role instance and message | Each carries an FR-287 authority-qualified opaque identity of its own kind. Definition, role, workflow, channel, obligation and configuration links are separately checked relationships. |
| Delivery | The producer supplies a distinct FR-287 `delivery` key. No semantic total order over deliveries is introduced. Receipt remains a distinct subject and relationship. |
| Forward and compensation attempts | Each attempt has its own FR-287 key. Retry is an explicit relationship between distinct attempt keys; a maximum or earlier/later binder does not mint identity or order. |
| Business and compensation effects | Each effect has its own FR-287 key. The establishing attempt is mandatory relationship evidence, not a key member. Replaying the same effect key is one effect; a second effect key for one singleton occurrence is a detectable duplicate. |
| Commit boundary | Static boundary, concrete node occurrence and concrete commit observation retain separate keys; `never` is an explicit value with no commit identity. Which dispositions forbid remains #31. |
| Recovery relation | The key is FR-288 plus the exact compensation handle and typed recovery-expression handle. Target/achieved values are operands. No unsupported source-free expression hash is claimed. |
| Node occurrence | The portable key is FR-288, workflow key, control handle, role/structural owner and complete outer-to-inner repeat ordinals. Outside repeats uses a present empty ordinal sequence. |

## Coverage and refusal review

TC-035, TC-050, TC-051, TC-057, TC-058 and TC-171 independently mutate key
members, kinds, authorities and relationship bindings; cover missing/empty,
foreign, wrong-kind, replay and duplicate cases; and verify that lexical key
ordering creates no causal, delivery or retry order. TM-003 and TM-004 keep every
new row `Planned`. Unsupported or incomplete identity inputs refuse explicitly;
no name, trace, span, payload, timestamp, table slot or provider is accepted as
an approximation.

The packet does not decide the FIFO key scope, graph key, observation key,
activation/completeness, registration key, retry counting or forbidding-commit
questions retained by #31.
