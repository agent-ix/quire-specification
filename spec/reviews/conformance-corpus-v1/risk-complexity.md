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

PASS. The format keeps its first version small: typed structural alternatives
are closed, operation-specific values remain operation-owned, and the fixture
does not overstate qualification coverage.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No unresolved risk/complexity issue found: the schema avoids duplicating operation result schemas while still fixing the expected value/refusal discriminator and refuses future silent extensions. | FR-336; schema.json; fixture |
