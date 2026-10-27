---
id: SR-498002
title: 'Code review of quire-specification PR #456 (STD-498): tests/checked_package_v2.rs helper and count changes'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@fb92162a; tests/checked_package_v2.rs diff against main b30710e3 (clause_digest helper, tc_233 model selection count, TM-23 locus)'
review_set: subset
date: '2026-10-27'
---

## Summary

Ticket: STD-498. Rust lane folded in: the PR changes 27 lines of tests/checked_package_v2.rs. `clause_digest("parameter")` now selects the `self` binder by its name member, `clause_digest("object_type")` selects ConfigVersion by name through `published_digest`, the TC-233 model-selection count moves 6 to 8 (four published packages select the domain document twice each), and the TM-23 locus moves from node 51 to node 52. Examined: those three hunks (examined). `cargo test --test checked_package_v2` with #457 applied gives 150 passed and 3 failed on both #456 and main; the three are K15 (no vector), the model-root nominal vector and execution-point-initialization.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| clause_digest helper | tests/checked_package_v2.rs:4630 | examined |
| tc_233 model selection count | tests/checked_package_v2.rs:14170 | examined |
| tc_367 TM-23 locus | tests/checked_package_v2.rs:18625 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The count of 8 and node 52 match the fixtures (clause-operations selects acme/orders, the tick fixture has 52 nodes). The pre-existing failures are identical on main.
