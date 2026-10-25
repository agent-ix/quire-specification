---
id: SR-12049
title: "QSL-819 QSpec PR #268 code review: the reaches population-link reader check and its vector test"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@29987ff9816cead0237f607bb5fd0c1d02a449f4; tests/checked_package_v2.rs"
review_set: subset
---

## Summary

Ticket: QSL-819. PR quire-specification#268, reviewed at 29987ff9816cead0237f607bb5fd0c1d02a449f4, by reading only, with no build. In scope: `application_dependencies` (now including the member `population`), `OperationOracle::population_link_holds` and its place in `classify_in`'s admission tuple, `append_population_chain`, and `tc_233_a_reaches_population_link_names_a_population_over_its_role_end_type` (`#[trace("TC-233", "FR-322-AC-73")]`).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS. The mutation check was done by reasoning. If `population_link_holds` were forced to return `true`, all three refusal cases would be admitted and the test would fail:

- `integer_range`: the node exists, the recomputed `dependencies` list it, and no other step reads the population.
- Absent node: graph shape only compares the list, so the dangling `fff…` digest passes every earlier step.
- `model_population` over `Invoice`: the appended `Invoice` model node passes the owner join, and the chain passes the derived-shape checks.

Each would reach `Ok(())` against an expected `Err(Operation(OperatorIneligible))`, so the test is mutation-sensitive for the check as a whole. The two admitted cases (`max` `3` and `0`) show that the setup itself admits, so each refusal comes from the link check and not from the fixture. The `T == E` comparison is also killed on its own, by the `Invoice` case. The `operator == "reaches"` guard has no killing vector, because there is no navigate case. The planner accepted that as the author's choice. The tag/form guard is only partly killed: the `integer_range` case still refuses through the `Set` lookup. Neither of these is a defect in the stated vectors. Dependencies: `application_dependencies` and the graph-shape comparison share one function, so `without-population` is a `GraphShape` refusal before any operation check. That matches the `invalid_semantic_graph` the vector records.
