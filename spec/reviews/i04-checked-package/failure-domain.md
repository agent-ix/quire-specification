---
id: SR-581
title: "Failure-domain review of I04 CheckedPackage V1"
type: SpecReview
analysis: failure-domain
scope: "interface_004, FR-322, TC-217, checked-package-v1"
review_set: all
---
## Summary

The contract now distinguishes refusal from incomplete resource outcomes and
separates all digest domains. The production reader must still reject duplicate
decoded JSON keys before serde-style object construction.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | A schema validator sees a parsed object and cannot detect duplicate decoded keys; #100's raw reader must reject them before schema validation. | FR-322-AC-4; TC-217 |
