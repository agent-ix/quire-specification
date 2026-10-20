---
id: SR-13369
title: "EARS analysis of QSpec PR #291 (IR-896): FR-322 reader-code cause statement"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@866c90d; spec/objects/interfaces/FR-322-checked-package-artifact.md; ticket IR-896"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
---

## Summary

Ticket: IR-896. PR: quire-specification#291 at 866c90d. EARS check of the two units the PR adds: the FR-322 statement at :613-620 and FR-322-AC-78. `quire validate` reports the same EARS warnings on main and on the head (only the line-74 statement, pre-existing); no new EARS diagnostic.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322-reader-code-cause-statement | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-78 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new normative clause 'so a refusal record pairing any cause with one of them is unpaired' has no system subject and no response. It matches no EARS pattern: it should be a ubiquitous or unwanted-behaviour form such as 'If a nested refusal record pairs a cause with one of these codes, the ContractPackage reader shall refuse it as `invalid_package`/`invalid-value` at its `cause` pointer.' The pairing sentence is a plain constraint over the cause catalogue, which is acceptable in this FR's declarative style. `quire validate` flags neither sentence; its EARS warnings are the pre-existing line-74 ones. AC-78 is a direct assertion of the outcome, the canonical AC shape. | spec/objects/interfaces/FR-322-checked-package-artifact.md:613-617 |

## Verdict

One low finding. AC-78 is well formed. The 'is unpaired' clause in the statement names no actor and no response; recast it as an unwanted-behaviour requirement on the ContractPackage reader (see SR-13368 FND-002).

## Dispositions

Against head f82969b (fix commit f82969b on top of 866c90d). `cargo test --test checked_package_v2 --test backend_provider_v1` passes (9, 108). `quire validate` on FR-322 and TC-233 gives the same output as origin/main 9d7268c once line numbers are ignored. `quire matrix`: the only change from main is the new row FR-322-AC-78, `tagged` at tests/checked_package_v2.rs:10358 and :10401. Value check, run as mutations on main and on the head: on origin/main, widening or deleting the schema's `unsupported-wire`/`unsupported-feature` pairing, and removing `unsupported-wire` from FR-272's `unknown_wire` row, each pass the whole checked_package_v2 suite. On the head, the first two fail the extended AC-47 test and the third fails the new test. Listing `unsupported-wire` in FR-272's `unknown_contract_version` row fails both the new test and the existing FR-272-AC-22 test, so that half was already guarded. So the PR protects something real but small: the schema pairing of two causes and their FR-272 owner rows, which no test guarded before. It does not touch the Contract IR divergence, which Contract IR's own tests must catch.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | fixed in f82969b: A V2 reader SHALL pair the cause `unsupported-wire` only with `unknown_wire` and `unsupported-feature` only with `unknown_required_feature`, as FR-272 and the `Diagnostic` schema's `DiagnosticCausePairing` do; `unknown_contract_version` and `unknown_required_capability` carry no cause (FR-272), so t |
