---
id: SR-13664
title: "EARS review of QSpec PR #320 (STD-118) replay selected item"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@b1c0365; spec/objects/interfaces/FR-323-native-runtime-envelope.md, spec/test-cases/TC-278-replay-envelope-members.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-278
    type: reviews
---

## Summary

Ticket: STD-118. EARS conformance of the two requirement statements the diff adds to FR-323: the selected-item paragraph and the executor refusal paragraph. AC rows are criteria, not EARS statements, and are out of this lens except as context.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-323 | spec/objects/interfaces/FR-323-native-runtime-envelope.md | examined |
| FR-323-executor | spec/objects/interfaces/FR-323-native-runtime-envelope.md | examined |
| FR-323-AC-18 | spec/objects/interfaces/FR-323-native-runtime-envelope.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The selected-item paragraph states the producer's obligation (exactly one member, chosen by kind, the other absent; the frame occurrence key; the argument keys) as a definition with no actor and no SHALL. Write it as "When the producer builds a replay request, it SHALL carry exactly one of ...". | spec/objects/interfaces/FR-323-native-runtime-envelope.md:124-135 |
| FND-002 | low | The SHALL covers only the both/neither/mismatch refusal; the resolution and the `missing_declaration`/`missing-name` refusal ("It resolves ..., refusing ...") have none. Give them their own statement: "If the selected item names no declaration of the recompiled package, then the executor SHALL refuse ...". | spec/objects/interfaces/FR-323-native-runtime-envelope.md:139-142 |

## Verdict

0 high and 0 medium finding(s) in this lens; minor wording only; no defect in this lens blocks merge.

## Dispositions

Round 1, head c0543ce (3470fee, c0543ce).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
