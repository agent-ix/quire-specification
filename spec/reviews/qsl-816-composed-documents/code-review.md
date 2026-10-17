---
id: SR-11761
title: "QSL-816 PR 267 code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@ed81fddbc60c401f849a11dd73d0c3c9d862873f; tests/composed_documents.rs, proposals/checked-package-v2/composed-document-vectors.json"
review_set: subset
---
# QSL-816 PR 267 code review

## Summary

Ticket: QSL-816. Rust lane folded in. Reviewed `tests/composed_documents.rs`
and its vector file. No build run (brief: author's focused run, 4 passed).
The four tests follow the repo's oracle-over-published-vectors convention,
carry `#[trace("TC-945", "FR-154-AC-14")]`, and have no panics outside
fixture-shape `expect`s.

Examined:
- `a_composed_document_declares_no_node_and_an_object_type_declares_one` (examined).
- `no_document_of_an_artifact_type_is_lifted_and_no_lifted_name_has_a_hyphen` (examined).
- `a_reference_to_a_composed_document_is_a_missing_declaration` (examined).
- `a_node_named_by_an_artifact_type_id_is_malformed` (examined).

## Verdict

Changes needed: one medium finding. The document and node tests check the
vectors against the rule as stated, which is the repo convention. The
reference test does not connect to the documents at all.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `a_reference_to_a_composed_document_is_a_missing_declaration` restates set membership over a hand-written `package_nodes: ["ix://test/orders/Order"]` instead of the nodes lifted from `documents`. It passes whatever the lift does: if `lifted_identity` emitted FR-204, the reference cases would still refuse missing-name. The list also disagrees with the documents, which lift both `Order` and `Money`. So the C05 claim ("a reference to a composed document refuses because it is not lifted") is never computed. Derive the reference cases' node set from `lifted_identity` over `documents` and drop `package_nodes`. | tests/composed_documents.rs:93-111; proposals/checked-package-v2/composed-document-vectors.json:50-67 |
