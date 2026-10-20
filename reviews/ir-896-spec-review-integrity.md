---
id: SR-13361
title: "Integrity review of QSpec PR #290 (IR-896): surface lists, verification methods and stage vocabulary"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@595b874; proposals/quire-v1/definitions/native-diagnostics.md, spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md, spec/objects/foundation/FR-272-typed-diagnostic-cause.md, spec/objects/interfaces/FR-330-contract-package.md, spec/test-cases/TC-937-contract-package-reader.md, spec/objects/foundation/FR-273-producer-stage.md; ticket IR-896"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-271
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-272
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-273
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-330
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-937
    type: reviews
---

## Summary

Ticket: IR-896. PR: quire-specification#290 at 595b874, scoped to `git diff refs/remotes/origin/main...595b874`. Cross-document consistency of every list of `stage_limit_exceeded` surfaces in QSpec, and in the QSL spec at main 891825c (ADR-013 T-4, ADR-014 §1 and line 350, FR-096), of the AC verification methods against TC `verifies` edges, and of the stage label against FR-273. QSpec lists: FR-271 row (updated), FR-272 row (updated), native-diagnostics row (updated), native-diagnostics prose at line 174 (updated) and the native-diagnostics QC-28 paragraph at lines 180-183 (I2 only, consistent). QSL lists naming the surfaces: ADR-013 T-4, ADR-014 lines 80 and 350, and FR-096 "row names these surfaces". None names the ContractPackage reader, which is consistent only if the reader is not a T-4 producer (FND-001). All four new AC ids are unique, are cited by TC-937 `verifies` edges, and have targets that resolve.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| native-diagnostics-T4-prose | proposals/quire-v1/definitions/native-diagnostics.md | examined |
| native-diagnostics-row-T4-sentence | proposals/quire-v1/definitions/native-diagnostics.md | examined |
| FR-271-stage_limit_exceeded-row | spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md | examined |
| FR-271-AC-7 | spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md | examined |
| TC-937-relationships | spec/test-cases/TC-937-contract-package-reader.md | examined |
| FR-330-reader-results | spec/objects/interfaces/FR-330-contract-package.md | examined |
| FR-273-AC-4 | spec/objects/foundation/FR-273-producer-stage.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The native-diagnostics prose now says `stage_limit_exceeded` "covers T-4's `LimitExceeded` limit kind, raised from ... the FR-330 ContractPackage reader". The ContractPackage reader is Contract IR's: it returns `ContractPackageReadResult::Incomplete(CheckedPackageIncomplete)`, not a QSL ADR-013 T-4 `StageFailure::Limit(LimitExceeded)`. The same file's row still says the code "is a `LimitExceeded` limit kind T-4 defines for" a list without that reader, so the file now disagrees with itself, and QSL ADR-013 T-4, ADR-014 lines 80 and 350, and FR-096 also omit it. Keep the reader out of the T-4 sentence (state it as a separate, non-T-4 surface, as the row's own new sentence does), or update the QSL lists with it. | proposals/quire-v1/definitions/native-diagnostics.md:172-175 |
| FND-002 | low | FR-271-AC-7's verification method is Inspection, but TC-937 declares it `verifies` FR-271-AC-7 and its vectors require a test to show that "none resolves to an FR-271 row". Make the method `Test (TC-937)` as on the three sibling ACs, or drop the TC-937 edge. | spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md:177 |
| FND-003 | low | FR-273 types a stage as the pair (producer-interface reference, label from that interface's closed vocabulary), and AC-4 refuses a label copied from another interface. `package_read` is a member of the checked-package v2 `Diagnostic.stage` enum (`proposals/checked-package-v2/schema.json`), whereas the ContractPackage reader is the `quire.contract-package/v1` interface, which states no stage vocabulary. Name the producer interface that `package_read` belongs to for these diagnostics. | spec/objects/interfaces/FR-330-contract-package.md:445 |

## Verdict

Findings: one medium, two low. The medium is that the native-diagnostics file contradicts itself on whether the ContractPackage reader is a T-4 `LimitExceeded` producer. Every other surface list is consistent; the new ids are unique and traced; TC-937's targets resolve.
