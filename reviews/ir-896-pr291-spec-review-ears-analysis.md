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
