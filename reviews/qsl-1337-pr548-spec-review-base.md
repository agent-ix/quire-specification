---
id: SR-1337004
title: "Spec review of quire-specification PR #548 (QSL-1337): FR-330 graph order follows FR-322"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@aaf1c48a (PR #548, fr330-graph-order-follows-fr322) vs origin/main; FR-330 semantic_graph bullet (line 54), FR-330-AC-4, FR-330-AC-16, FR-330 feature_accounting payloads (172, 174), source_map ordering (334), TC-931, TC-937, FR-463, TC-217"
review_set: base
date: "2026-10-31"
---

## Summary

Ticket: QSL-1337. The PR changes FR-330 line 54 and FR-330-AC-4 from
"ascending node-id order" to FR-322's producer-chosen graph order
(FR-322-AC-95). This matches FR-322's Graph order paragraph (a reader neither
sorts the nodes nor requires a sort order) and FR-330-AC-16 (reordering the
output graph without changing any key is equal).

No other FR-330 or ContractPackage text, and no TC verifying FR-330-AC-4,
still requires an ascending graph order. TC-931 already expects "the
producer's graph order (FR-322)". TC-937 and FR-463 carry no graph-order
requirement. FR-330 line 68 already says "Order is the producer's choice".
The remaining "ascending node-id order" uses are unaffected. Lines 172 and 174
pick the first payload node by FR-322's componentwise node-id comparison.
FR-322 line 197-199 says that comparison "orders no array of the package".
Line 334 orders the source_map collections, not the graph. TC-217's ascending
rule is for an application node's `dependencies` list (FR-322-AC-10), a sorted
member list. FR-464 and FR-272 use the same comparison for sets and diagnostics.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. Mergeable. Examined and clean: FR-330 line 54, FR-330-AC-4,
FR-330-AC-16, FR-330 lines 68, 172, 174 and 334, TC-931 Expected Results,
TC-937, FR-463 line 63, and TC-217 line 34-36.
