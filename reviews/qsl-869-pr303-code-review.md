---
id: SR-13601
title: 'Code review of QSpec PR #303: FR-322 population reference tests'
type: SpecReview
analysis: code-review
scope: 'agent-ix/quire-specification@ab6ff70; git diff origin/main...ab6ff70 (PR #303, branch qspec-fr322-popref): tests/checked_package_v2.rs, tests/model_member_types.rs; ticket QSL-869'
review_set: subset
date: '2026-10-20'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
---

## Summary

Ticket: QSL-869. PR: quire-specification#303, head ab6ff70. Code review with the rust-review lane, over the test-only Rust changes: two new tests, plus oracle and helper edits in tests/checked_package_v2.rs and tests/model_member_types.rs. The OperationOracle edit (line 1145) gives family `population` only to `relation`/`population` targets, where main gave it to every `relation` node. That matches the spec's family rule. `is_model_declaration`, `document_declares_owner` and `owner_declares` gain the population arm. The full-reader test builds each case over a projection with `populations`, rekeys through the existing helpers, and checks the closed preimage, the key digest, application dependencies and the typed refusal codes. The TC-280 test covers the five closed-preimage mutations, the owner-identity key change and PR-01..PR-06. The ownerless branch checks only that the V2 schema refuses, which is what AC-82 states. Bindings checked: both tests' `#[trace]` tags match the criteria they assert. Gates: the three requested test targets pass (112/11/23). `cargo clippy --test checked_package_v2 --test model_member_types` gives no warnings. rustfmt drift in model_member_types.rs is the same three pre-existing hunks as main.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| tc_233_a_population_reference_names_the_declared_population_it_reads | tests/checked_package_v2.rs | examined |
| tc_233-one-node-assertion | tests/checked_package_v2.rs | examined |
| tc_280_a_population_reference_node_is_keyed_by_its_declaration_key_alone | tests/model_member_types.rs | examined |
| OperationOracle-relation-family | tests/checked_package_v2.rs | examined |
| FR-322-AC-83 | spec/objects/interfaces/FR-322-checked-package-artifact.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The assertion 'one population is one node, whichever operation reads it' compares the population node ids of two separately built packages. One holds only the all_instances read of `counters`, the other only the lookup read. So it checks key determinism, not AC-83's 'one population read by both operations is one node'. No package with both reads is ever built or admitted. Fix: add a case that points both the all_instances and the lookup operand 0 at one appended `counters` node, and assert that the package is admitted with exactly one `relation`/`population` node. | tests/checked_package_v2.rs:13147-13156 |

## Verdict

Mergeable from the code side. One low test-intent finding: AC-83's 'one node for both operations' is only shown by key determinism across two packages (FND-001).
