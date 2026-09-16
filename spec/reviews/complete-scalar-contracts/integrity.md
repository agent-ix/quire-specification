---
id: SR-590
title: "Integrity review of complete scalar contracts"
type: SpecReview
analysis: integrity
scope: "QSpec #68 consistency, atomicity, definition closure and matrix traceability"
review_set: all
evaluated_revision: "cd4c0a0 plus QSpec #68 working-tree remediation"
review_date: "2026-09-15"
relationships:
  - target: ix://agent-ix/quire-specification/issues/68
    type: reviews
---

## Summary

PASS after remediation. Grammar, architecture, requirements, definitions, I04
identity rules and tests now select one consistent scalar model. Every added
criterion has a planned matrix owner and no definition dependency cycle remains.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-5901 | high | Resolved: Decimal and IEEE grammar now permit the documented omitted strict-`exact` spelling. QSL #118 amendment: `Decimal[lo, hi; smin, smax; mode]` bounds bound the membership scale `s* = max(s, smin)` and membership coefficient `c* = v × 10^s*`, so membership depends only on the mathematical value (unlike normalized `Rational` components, because decimal scale is representational); `smax` is the target scale, and reversed or non-`u32` bounds refuse with `ill_typed` (TC-185 D10–D12). PR #71 re-review: FR-140 and AC-3 limit rounded-value-plus-loss to a rounding step under a non-`exact` mode, range nonmembership refuses under every mode, and membership is decided without materializing `c*` (TC-185 D19). | shared-grammar.md; FR-140; FR-148 |
| FND-5902 | high | Resolved: accounting, Unicode, IEEE and division profiles are acyclic leaves selected by the complete-value root/closure. | value-complete.md; value-accounting.md; value-*.md |
| FND-5903 | medium | Resolved: canonical rational loss records, unit offsets, preimage hashes and Unicode artifact digests are exact and independently checked. | FR-140; TC-185; TC-187; value-text-unicode-17.md; checked_package_v2.rs |
| FND-5904 | low | Added AC6/AC7 rows and FR-201 AC5 plus FR-322 AC4–AC12 mappings remain planned rather than falsely promoted. PR #70 re-review: qualification-crate tests no longer trace reader, linker, lowerer or evaluator criteria (TC-217, TC-187, FR-322-AC-7 Inspection); they trace the new producer-side FR-142-AC-8, FR-201-AC-7 and FR-322-AC-13–AC-16 through TC-233 with planned rows, leaving TC-217/TC-187 rows unbacked for their real owners. | spec/complete-v1/tests.md; TC-233; FR-142; FR-201; FR-322 |
| FND-5905 | high | Resolved: the authored definition lock closes every catalog artifact by identity, revision, digest domain and raw-byte digest (FR-201-AC-6, TC-233). F1 re-review: catalog membership no longer implies selection; each catalog role occupies exactly one always, conditional or exactly-one slot under the closed trigger set. F5 re-review: rule-lock tests carry machine-read `#[trace("TC-233", "FR-201-AC-6")]` ownership and the matrix row exists; PR #70 re-review moved it from TC-231 so the crate no longer backs FR-311 corpus rows. | complete-value-lock.json; value-complete.md; FR-201; spec/complete-v1/tests.md; checked_package_v2.rs |
| FND-5906 | high | Resolved: nominal node preimages are joined to the semantic type, body and exact dependency set, closing the remaining split-brain path between an intact key and contradictory node meaning. | checked-package-v2/schema.json; FR-141; FR-142; FR-322; checked_package_v2.rs |
| FND-5907 | high | Resolved: the closed-wire addition is not mislabeled as a compatible V1 amendment. Contract, graph, identity-preimage and package-identity versions advance together while semantic node-key domains remain independently versioned. | interface-004; FR-322; checked-package-v2/schema.json |
| FND-5908 | high | Resolved: FR-201 catalogs both package domains, FR-195 selects V2 as the active complete-V1 lowerer input, the historical I04 review is marked V1-only, and migration correspondence closes NFR-074 without mutating either package identity. | FR-201; FR-195; NFR-074; i04-checked-package/dependency.md |
