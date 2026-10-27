---
id: SR-815
title: "Risk and complexity review of conformance corpus v1 wire contract"
type: SpecReview
analysis: risk-complexity
scope: "QSpec FR-336 schema and fixture; QSL consumer behavior"
review_set: subset
---
# Risk and complexity review of conformance corpus v1 wire contract

## Summary

PASS. The replay mapping adds many nested typed values, creating conversion risk at the adapter boundary. The exhaustive member table, closed schema and focused TC-970 mutations make that conversion auditable without changing product wires.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Primary residual risk is implementation fidelity for integer/range and byte-path conversions; test each mapped variant and refusal before the runner consumes broad corpus data. | FR-336-AC-7/8; TC-970; FR-350 |
