---
id: SR-20261106202640
title: "IR-842 Color case identity failure-domain review"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification; PR 635 at fe5ac46c; spec/functional/type-model/FR-154-admit-domain-package-model.md; spec/test-cases/TC-234-domain-package-model-intake.md; FR-154 variant identity, FR-154-AC-11, TC-234 I34"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
  - target: ix://agent-ix/quire-specification/TC-234
    type: references
---

## Summary

Ticket: IR-842. Identity confusion, case sensitivity, refusal attribution and unchanged collision boundary. This is the pre-implementation review at the frozen specification-only candidate.

## Verdict

**PASS** — The isolated lowercased-identity mutations retain case names and origins, so refusal cannot be satisfied by silently case-folding the input. Positive nullary cases retain their original identities. Duplicate-case, duplicate-identity and payload policies remain the existing FR-154 rules.

## Scope Examined

- FR-154 variant identity bullet at lines 186–194, including the Color/Red/Green example.
- FR-154-AC-11 identity-refusal obligation, with existing origin and duplicate-identity precedence as context.
- TC-234 I34 positive nullary cases and isolated lowercase identity mutations.
- Producer context: the public extraction frontend slug contract preserves ASCII letter case; existing QSL intake uses that owner function.

## Assurance Context

No AssuranceProfile is declared in this repository's spec tree. Direct subset review applies. No enforced review run was started, and no human acceptance or final gate is claimed.

## Evidence and Limits

The scoped Quire check passed for both changed artifacts: 2/2 grammar-clean, zero grammar findings. Git diff checking passed. No fixture or Rust implementation has changed yet, and no Cargo command ran. The current published fixture is intentionally awaiting this same PR's implementation stage. This review does not claim native model admission or FR-014-AC-2 runtime enum-comparison evidence. Implementation and focused test evidence will be reviewed separately in the same PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Implementation-stage review

Candidate: 5039e654289dbf377b483e8be4519b31e49df3e0.

**PASS for the in-scope change** — All lowercased case identities in the one fixture are corrected, including payload-bearing and nullary cases, so fixing only the first reported Green error cannot hide the same defect in Shape or List. Explicit payload absence and origin assertions prevent lossy repair. The separate downstream assembly refusal remains explicit.

The shared owner schema requires the canonicalization descriptor with algorithm rfc8785-v1. This descriptor names a serialization algorithm; it is not a content hash, digest, pin or tracking manifest. All seven Shape/Color/List case identities now preserve their existing declared case. The JSON diff preserves case names, payload references, origins, type identities and declaration order. No QSL rule was relaxed and no fixture data was pruned.

The new dependency-free Rust target parses the actual published JSON, checks the descriptor, requires exactly seven variant entries and finds each expected owner by identity rather than displayName. Its independent golden table verifies every case identity and payload presence/absence; original origins are checked against their owning declarations. Missing or incorrect cases fail unconditional assertions. The expected identity strings are the authored-data contract, not a replacement minting or intake implementation. The trace has exactly TC-234 and FR-154-AC-11, in that order. No ignored branch, mock, unsafe code, local schema copy, dependency change or public API was introduced.

Reviewed focused evidence: the new target passes 1/1, strict Clippy for that target passes, and the changed Rust target formatting/diff checks pass (/tmp/ir842-color-isolated-focused.log). Four existing TC-940 fixture/projection controls pass (/tmp/ir842-color-existing-vector-checks.log). Seven authored documents validate structurally. The actual QSL CLI comparison retains the original malformed-case refusal and shows the corrected exact fixture reaching a separate assembly ill_typed/type-mismatch refusal; it does not prove successful complete compilation, enum evaluation or IR-842/FR-014-AC-2 agreement.

Gate limitation: repository-wide formatting reports drift in 14 untouched files, and all-target Clippy reports 18 existing lints in tests/checked_package_v2.rs. That file is byte-unchanged by this PR. These are not new in-scope source defects, and this review neither suppresses them nor claims the required aggregate gate passed. The lead must run and report the unchanged repository gate honestly before any merge decision. No Cargo or aggregate gate was run by this reviewer.
