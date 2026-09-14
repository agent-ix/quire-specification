---
id: SR-529
title: "EARS review of FS02 protocol identity rulings"
type: SpecReview
analysis: ears-conformance
scope: "Changed normative statements for quire-specification #28"
review_set: subset
evaluated_revision: "task/28 working tree based on 6a50ab6"
review_date: "2026-09-13"
---
# EARS review of FS02 protocol identity rulings

## Summary

PASS. Quire reports all 354 specification documents grammar-clean. Manual review
confirms that each identity trigger names the linker, binder or evaluator and
that every adverse condition produces a concrete refusal, inequality,
incomplete result or duplicate-effect verdict.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Resolved: FR-288's first form split the EARS subject and `SHALL` across lines; the final event-driven statement is one complete `When ..., the linker SHALL ...` sentence. | FR-288 |
| FND-002 | low | Resolved: two criteria used `handle` where the grammar checker read a vague response verb; the final criteria state exact componentwise equality while retaining handle nouns in the object definition. | FR-238-AC-5; FR-288-AC-1 |

## Statement review

| Statement class | Result |
| --- | --- |
| Runtime identity binding | Event/state triggers name the binder; missing, empty, foreign, wrong-kind and conflicting inputs refuse without generation or normalization. |
| Static definition qualification | The `When` trigger names a linked definition used as a qualifier; the linker retains four exact structured components. |
| Occurrence binding | The frontend exposes the bounded schema and the concrete binder refuses absent, surplus, wrong-role and out-of-domain values. |
| Replay and duplication | Conditions distinguish replay of one effect key from a second effect key at one singleton occurrence and name the corresponding one-effect or violation result. |
| Commit and recovery | The evaluator retains separate boundary/occurrence/observation identities and selects recovery through exact local handles without inferring success. |
| Ordering | Every `SHALL NOT` names the prohibited inference: lexicographic key comparison cannot establish source, causal, delivery, retry, timestamp or temporal order. |

Repository-wide validation has zero grammar findings. Its sole structural failure
is the unchanged `Status` versus `Coverage Status` header in
`spec/native-source/tests.md`, outside issue #28.
