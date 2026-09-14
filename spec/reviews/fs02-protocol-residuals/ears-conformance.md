---
id: SR-533
title: "EARS review of FS02 protocol residual rulings"
type: SpecReview
analysis: ears-conformance
scope: "Changed normative statements for quire-specification #31"
review_set: subset
evaluated_revision: "task/31 working tree based on dcc22bd"
review_date: "2026-09-13"
---
# EARS review of FS02 protocol residual rulings

## Summary

PASS. Quire reports all 357 specification documents grammar-clean. Manual
review confirms each changed trigger names the linker, binder, evaluator or
correlation service and each adverse condition produces a concrete refusal,
incomplete result, contradiction or non-success outcome.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Resolved: the first FR-111 edit split the actor from `SHALL` across Markdown lines, so the EARS scanner could not classify two statements. Each actor and modal now occur in one complete sentence. | FR-111 |
| FND-002 | low | Resolved: FR-054 initially used the vague response phrase “supports stable equality.” It now requires a result type with defined stable-equality semantics. | FR-054 |

## Statement review

| Statement class | Result |
| --- | --- |
| Channel admission | Event trigger names the protocol linker; missing/unknown/conflicting members and malformed FIFO/interval inputs refuse. |
| Graph identity | Mutations name the required different FR-288; same-key/different-content produces identity contradiction/refusal. |
| Runtime binding | Static role, role instance, participant and component substitutions are explicit refusal cases; internality absence/unknown never defaults. |
| Compensation | Registration, count and commit conditions name exact retained state and refuse rather than approximating from replay, order or absence. |
| Correlation | Association names the correlation service; missing is incomplete, conflicts/ambiguity refuse, and unknown producer/declaration input is admission refusal. |
| Prohibitions | `SHALL NOT` clauses identify forbidden inference from key, timestamp, arrival, trace/provider, another premise or old wire limitations. |

Repository-wide validation has zero grammar findings. Its sole structural failure
is the unchanged `Status` versus `Coverage Status` header in
`spec/native-source/tests.md`, outside issue #31.
