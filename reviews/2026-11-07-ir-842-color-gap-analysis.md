---
id: SR-20261106202641
title: "IR-842 Color fixture scoped gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification; PR 635 at 5039e654289dbf377b483e8be4519b31e49df3e0; FR-154 variant identity, TC-234 I34, proposals/checked-package-v2/domain-package-acme-shapes.json, tests/model_variant_fixture.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
  - target: ix://agent-ix/quire-specification/TC-234
    type: references
---

## Summary

Ticket: IR-842. This change-scoped review checks the fixture identity regression and its new producer-data guard. No whole-repository assurance, successful compiler admission or runtime enum-agreement claim is made.

## Verdict

**PASS for the scoped fixture correction** — no new in-scope implementation or trace defect found. The broad native intake criterion is not fully discharged by this producer fixture test.

## Coverage

Plan completion: not assessed

Reconciliation: quire matrix --scope . --format json, exit 0; static axis only, no run-evidence store read. The repository-wide output reports 447 tagged, 2447 untagged and 312 method-without-symbol criteria. Those existing repository-wide gaps are outside the explicitly assigned fixture change and are not being treated as a repository PASS.

FR-154-AC-11 is tagged by shapes_variant_case_identities_preserve_declared_case in tests/model_variant_fixture.rs:14. This is a real artifact-value regression for seven minted identities, original payloads and origins. A tag establishes a binding; it does not make the test evidence for every behavior in this broad consumer criterion. Malformed-case refusal attribution and successful runtime enum comparison remain consumer obligations. No FR-014-AC-2 tag or claim was added.

## In-scope Inspection

The shared owner schema requires the canonicalization descriptor with algorithm rfc8785-v1. This descriptor names a serialization algorithm; it is not a content hash, digest, pin or tracking manifest. All seven Shape/Color/List case identities now preserve their existing declared case. The JSON diff preserves case names, payload references, origins, type identities and declaration order. No QSL rule was relaxed and no fixture data was pruned.

The new dependency-free Rust target parses the actual published JSON, checks the descriptor, requires exactly seven variant entries and finds each expected owner by identity rather than displayName. Its independent golden table verifies every case identity and payload presence/absence; original origins are checked against their owning declarations. Missing or incorrect cases fail unconditional assertions. The expected identity strings are the authored-data contract, not a replacement minting or intake implementation. The trace has exactly TC-234 and FR-154-AC-11, in that order. No ignored branch, mock, unsafe code, local schema copy, dependency change or public API was introduced.

Reviewed focused evidence: the new target passes 1/1, strict Clippy for that target passes, and the changed Rust target formatting/diff checks pass (/tmp/ir842-color-isolated-focused.log). Four existing TC-940 fixture/projection controls pass (/tmp/ir842-color-existing-vector-checks.log). Seven authored documents validate structurally. The actual QSL CLI comparison retains the original malformed-case refusal and shows the corrected exact fixture reaching a separate assembly ill_typed/type-mismatch refusal; it does not prove successful complete compilation, enum evaluation or IR-842/FR-014-AC-2 agreement.

Gate limitation: repository-wide formatting reports drift in 14 untouched files, and all-target Clippy reports 18 existing lints in tests/checked_package_v2.rs. That file is byte-unchanged by this PR. These are not new in-scope source defects, and this review neither suppresses them nor claims the required aggregate gate passed. The lead must run and report the unchanged repository gate honestly before any merge decision. No Cargo or aggregate gate was run by this reviewer.


## Assurance Context

No AssuranceProfile is declared in this repository's spec tree. Direct change-scoped review is authorized by the lead; the repository-wide matrix is disclosed without a whole-repository acceptance claim.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
