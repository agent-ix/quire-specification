---
id: SR-1293
title: "Integrity review of PR #188: division checks only the exposed member"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@f0002b92095dee386759dd17530db0f757f285d6; PR #188 diff against origin/main 57e51d6c: FR-147, FR-007, FR-271, FR-272, TC-192, spec/complete-v1/tests.md, proposals/quire-v1/definitions/native-diagnostics.md, proposals/quire-v1/definitions/value-accounting.md, proposals/state-core/state-semantics.md, proposals/state-core/profile.md (context)"
review_set: subset
---
# Integrity review of PR #188 (IR-601)

## Summary

Ticket: IR-601. I checked that the edited documents agree with each other and
with the documents they cite.

- FR-147, FR-007, FR-271, FR-272, native-diagnostics.md and value-accounting.md
  agree on the code, the two causes, the four charge points in order, and the
  amounts (`value_occurrences=1`, `result_units += 1`).
- value-accounting.md drops the "paired quotient/remainder costs two result
  units" sentence. The refusal still follows `integer-division.domain` and
  precedes result retention.
- FR-147-AC-10 has a row in `spec/complete-v1/tests.md`, and TC-192 already
  `verifies` FR-147.
- `tests/checked_package_v2.rs` VALUE_REFUSAL_ROWS matches the FR-272 and
  native-diagnostics rows, so TC-233 keeps both rows in sync.

## Verdict

Changes needed: one medium finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new state-semantics.md text says "Only the member the operator exposes is checked against the declared bounds ... signed minimum `rem` -1 is `0`". The state-core profile it elaborates still says "Every intermediate result must fit its declared type" and "Zero divisor and minimum-signed-value divided by `-1` require refusal unless excluded by bounds/guards" (profile.md:117-123). Under profile.md, MIN `rem` -1 refuses, because its quotient `2^63` is an intermediate outside signed-64. state-semantics.md also says of itself that a later semantic amendment cannot silently cover the profile. The two state-core documents now disagree. Make them agree: change profile.md:118-123 the same way, or drop the state-semantics.md hunk and leave the historical state-core profile as it was (FR-147 governs complete-V1). | proposals/state-core/state-semantics.md:172-175; proposals/state-core/profile.md:117-123 |
