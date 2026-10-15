---
id: SR-9125
title: "Code review of quire-specification PR #254: protocol_reference occurrence schema"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@80cf2fb983ea4ec8108656d57c59d3b5689bcae5; proposals/checked-package-v2/schema.json, tests/checked_package_v2.rs"
review_set: subset
---
# Code review of quire-specification PR #254

## Summary

Ticket: QSL-777. The code review covers only the schema.json edit and the
Rust test that reads that schema.

Examined:
- The schema.json change touches one arm, BodyBindingRules for
  `protocol`/`protocol_reference`. `maxItems: 1` is removed, while
  `minItems: 1`, the `expression` role const and the
  declaration/owner/nominal/recursion exclusion stay. Nothing else changed
  (clean).
- Behaviour, checked with jsonschema over positive-all-families plus
  carrier R: two `expression` occurrences validate; zero occurrences and a
  non-`expression` role fail. A repeated ordinal is not a schema concern
  (clean).
- tests/checked_package_v2.rs has no change in this PR, and no test refers to
  `protocol_reference`. The coder's log shows `cargo test --test
  checked_package_v2` with 98 passed and 0 failed. No existing test depends
  on the removed cap (clean).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

**PASS.** The schema edit is the minimal change and is correct, and the test
suite that reads the schema stays green.
