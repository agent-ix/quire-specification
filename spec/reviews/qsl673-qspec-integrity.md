---
id: SR-4943
title: "integrity review of QSL-673 QSpec formatter output budget contract"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@7775952ddfec73f0663f608c8d4475740d7a4f52; FR-461, TC-930, spec/complete-v1/tests.md"
review_set: subset
---

## Summary

FR-461-AC-7 is atomic and falsifiable through TC-930, and the new TC is linked
back to FR-461. The maintained complete-V1 traceability matrix does not carry
the new AC row, leaving the criterion-to-test inventory incomplete.

## Verdict

CONDITIONAL — add the missing matrix row; no runtime/full-gate acceptance is
claimed.

## Examined Scope

```yaml
scope:
  - {id: FR-461-AC-7, path: spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md, role: examined, excerpt: "A byte-emitting lifecycle operation with an output_bytes limit reports the operation's *.output_bytes setting when the next emitted byte would exceed the configured bound, returns no partial output, succeeds at the exact output length including its terminating newline, and accepts the same bound through its typed limits field and the CLI settings operation."}
  - {id: TC-930, path: spec/test-cases/TC-930-byte-emitting-operation-raises-its-output-budget.md, role: examined, excerpt: "Every bound below the emitted length returns the operation's typed byte-limit outcome naming format.output_bytes and returns no partial output. The exact bound succeeds and includes the terminating newline."}
  - {id: FR-461, path: spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md, role: examined, excerpt: "The same name raises the same limit at every entry point."}
  - {id: TM-FR-461, path: spec/complete-v1/tests.md, role: examined, excerpt: "The complete-V1 matrix has rows for FR-461-AC-1 through AC-6 and TC-426."}
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The hand-maintained complete-V1 matrix omits FR-461-AC-7, so the new criterion is present in FR-461 and TC-930 but absent from the matrix used to audit criterion coverage. Add `FR-461 | FR-461-AC-7 | TC-930`. | spec/complete-v1/tests.md:1527-1532,1897; spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md:99 |

## Evidence and Limits

Targeted Quire validation passed for all three changed QSpec documents. The
computed matrix completed with exit 0 but reports the repository's many
existing untagged criteria; this review records the new hand-authored matrix
omission specifically and makes no full-repository coverage claim.

## Dispositions

Round 1 disposition reviewed at `69ad57b14312517f777a1a899f7cdcdb53899af5`.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 69ad57b14312517f777a1a899f7cdcdb53899af5 |

The complete-V1 matrix now contains `FR-461 | FR-461-AC-7 | TC-930`; the traceability omission is resolved.
