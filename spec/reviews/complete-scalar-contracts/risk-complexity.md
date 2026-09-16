---
id: SR-593
title: "Risk and complexity review of complete scalar contracts"
type: SpecReview
analysis: risk-complexity
scope: "QSpec #68 scalar semantic volatility, resource and conformance risks"
review_set: all
evaluated_revision: "cd4c0a0 plus QSpec #68 working-tree remediation"
review_date: "2026-09-15"
relationships:
  - target: ix://agent-ix/quire-specification/issues/68
    type: reviews
---

## Summary

PASS with implementation risks made explicit. Unicode normalization, exact
decimal/quantity arithmetic, IEEE bit/flag behavior, nominal identity and
representation-independent recursive accounting remain high-complexity areas,
but each now has a closed authority and conformance oracle.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-5931 | medium | Unicode table drift is contained by exact official artifact hashes and the full normalization corpus. | value-text-unicode-17.md; TC-186 |
| FND-5932 | medium | Host numeric shortcuts are excluded by rational DecimalLoss, exact affine composition, IEEE bit vectors and strict `exact`. | FR-140; FR-142; FR-148 |
| FND-5933 | medium | DAG memoization and backend cost models cannot change normative resource outcomes. | value-accounting.md; FR-149; TC-194 |
| FND-5934 | low | Consumer divergence is checked through shared I04 vectors and production-reader TC-217, not duplicated nominal types. | FR-322; TC-217 |
| FND-5935 | medium | IEEE exact-real implementations may use rational, symbolic-root or hardware techniques, but comparison/exceptional/finite charge applicability and fixed-width semantic allowances prevent that choice from changing accounting. | value-accounting.md; TC-193 F17–F19 |
| FND-5936 | high | Closed-wire compatibility risk is contained by preserving V1 and requiring explicit V2 selection/refusal; downstream consumers must add V2 deliberately rather than accepting unknown fields. | checked-package-v1; checked-package-v2; FR-322-AC-11 |
| FND-5937 | medium | Migration can rekey nominal nodes even when many existing nodes remain identical. A total ordinal correspondence with explicit `relinked-identical`/`relinked-rekeyed` basis prevents consumers from assuming graph-key stability across package versions. F3 re-review: the canonical `nominal-relinked-rekeyed` vector exercises rekeyed enum, dimension and unit nodes, and the basis is fixed by digest equality. | migration-correspondence.schema.json; migration-correspondence-vectors.json; FR-322-AC-12; TC-217 |
