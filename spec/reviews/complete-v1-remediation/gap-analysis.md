---
id: SR-603
title: "Gap analysis of complete V1 requirement grammar and framing"
type: SpecReview
analysis: gap-analysis
scope: "requirement statement grammar and current-state framing across spec/"
review_set: subset
evaluated_revision: "fc5434b on task/63-full-v1-specification"
review_date: "2026-09-17"
---

## Summary

Swept every normative document for requirement grammar findings and for text
that narrates a design the system does not have. Grammar findings sit at the
pre-existing baseline of five. One framing cluster is recorded rather than
edited, because the rewrite touches accepted architecture text.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-641 | medium | Open: FR-001, FR-019 and AD-003 narrate a superseded draft state profile candidate, and FR-020 describes an earlier bundling of its own obligations. Current-state framing would state only the selected contract. | FR-001; FR-019; FR-020; AD-003 |
| FND-642 | low | Resolved: profile-scoped criteria added during this pass were rewritten to state the admitted behavior rather than the absence of a refusal. | FR-008; FR-007 |
| FND-645 | low | Resolved: ADR-003 narrated a superseded design, stating that the Filament reader and universal typed-model adapter "are superseded". It now states what `quire-spec-language/FR-005` says is true: exactly one Contract IR binder, no general Filament model authority, and no such reader or adapter. | ADR-003 |
| FND-643 | low | Open: five grammar findings persist at the baseline — two ears:missing-subject, two ears:unclassifiable and one quality:ambiguous-term. A requirement subject and its `shall` must stay on one physical line for the grammar reader to classify the statement. | spec/ |
| FND-644 | low | Open: nine documents fail structural validation. Eight are Test Matrices carrying a `Coverage Status` column where the archetype asserts `Status`; the ninth, `spec/reviews/complete-v1/architecture-evaluation.md`, carries an `analysis` value outside the SpecReview enum. Both are pre-existing and unchanged by this pass. | spec/; spec/reviews/complete-v1/architecture-evaluation.md |
