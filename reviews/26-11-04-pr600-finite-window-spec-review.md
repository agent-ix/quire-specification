---
id: SR-20261104200600
title: "Spec review of quire-specification PR #600 (timestamped finite-window, QSL-1500)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; FR-091, FR-094, FR-161, FR-301, FR-242, ADR-004, TC-221, TC-980, proposals/finite-window, tests/finite_window.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-091
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-301
    type: reviews
---

## Summary

Ticket QSL-1500, PR #600 head 188e8559. The FR-091 and FR-094 text mirrors QSL FR-501 (main 096eace05) rule for rule, including the QSL-1568 repeated-key rule, FR-285's ten-category catalog and exit 11. 42 vectors were re-derived by hand against the rules (every changed one) and all agree. finite_window, acceptance_criterion_ids and qualification_vectors pass with --locked.

## Verdict

PASS with low findings. Mergeable.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The counter vectors FWV-39 to FWV-44 and FWV-70 to FWV-72 carry no W or completeness, and the test asserts the verdict only, never the reason StraddlesBound, so establishment of those rows is not checked. | tests/finite_window.rs; proposals/finite-window/finite-window-vectors.json |
| FND-002 | low | FWV-57 and FWV-58 are checked as stamps.is_empty(); no check reads canonical order for a decreasing listing, so FR-091-AC-22's second clause cannot fail. | tests/finite_window.rs |
| FND-003 | low | FR-091 states refusals as refuse and AC-16 names the position, but not the QSL code invalid_runtime_input/invalid-value or the stage (admit versus evaluate) that QSL FR-501 AC-7 and AC-17 fix. | FR-091-AC-16; FR-091-AC-22 |

## Coverage

Examined: FR-091 behavior and AC-10 to AC-26, FR-094-AC-11, FR-301 table, order and AC-2, ADR-004, FR-242, TC-221 CL-02, TC-980, FWV-01 to FWV-75.
