---
id: SR-5502
title: "base review of QSL-673 QSpec formatter output budget contract"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@7775952ddfec73f0663f608c8d4475740d7a4f52; spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md, spec/test-cases/TC-930-byte-emitting-operation-raises-its-output-budget.md, spec/complete-v1/tests.md"
review_set: subset
---

## Summary

The QSpec half adds a concrete `format.output_bytes` instance to FR-461 and a
boundary test case. IDs, links, criteria shape and targeted Quire validation are
clean; one hand-maintained complete-V1 traceability row was omitted for the new
criterion.

## Verdict

CONDITIONAL — the specification text is usable after the traceability table is
updated. This is a spec-only review; no implementation or runtime acceptance is
claimed.

## Examined Scope

```yaml
scope:
  - {id: FR-461-AC-7, path: spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md, role: examined, excerpt: "A byte-emitting lifecycle operation with an output_bytes limit reports the operation's *.output_bytes setting when the next emitted byte would exceed the configured bound, returns no partial output, succeeds at the exact output length including its terminating newline, and accepts the same bound through its typed limits field and the CLI settings operation."}
  - {id: TC-930, path: spec/test-cases/TC-930-byte-emitting-operation-raises-its-output-budget.md, role: examined, excerpt: "Exercise the caller-selected output-byte budget at the formatter boundary, including the setting name, exact boundary and absence of partial output."}
  - {id: FR-461, path: spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md, role: examined, excerpt: "A byte-emitting lifecycle operation names its output-byte budget with the operation name and the output_bytes counter."}
  - {id: TM-FR-461, path: spec/complete-v1/tests.md, role: examined, excerpt: "The complete-V1 matrix lists FR-461-AC-1 through FR-461-AC-6 and TC-426."}
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The complete-V1 criterion-to-test matrix lists FR-461-AC-1 through AC-6 but has no FR-461-AC-7 row, even though TC-930 is added to the test catalog. Add the AC-7 to TC-930 mapping so the new obligation is represented in the maintained matrix. | spec/complete-v1/tests.md:1527-1532,1897; spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md:99 |

## Evidence and Limits

Targeted validation passed: 3/3 changed QSpec documents grammar-clean and 7/7
criteria property-extractable. Full-scope validation remains blocked by 137
pre-existing structural failures in the frozen repository. `quire matrix`
completed with exit 0; the repository-wide matrix has many inherited untagged
criteria, so no full-coverage claim is made.

## Dispositions

Round 1 disposition reviewed at `69ad57b14312517f777a1a899f7cdcdb53899af5`.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 69ad57b14312517f777a1a899f7cdcdb53899af5 |

The complete-V1 matrix now contains `FR-461 | FR-461-AC-7 | TC-930`; the traceability omission is resolved.

### Fixed excerpts

- `spec/complete-v1/tests.md`: `| FR-461 | FR-461-AC-7 | TC-930 |  |`
