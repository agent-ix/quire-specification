---
id: SR-9004
title: "Integrity review of QSpec PR #247 (resource_exhausted causes)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@5017b5cf6b6d1c731e103b6cb2452145af856d41; spec/objects/foundation/FR-272-typed-diagnostic-cause.md, proposals/quire-v1/definitions/native-diagnostics.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, spec/complete-v1/tests.md"
review_set: subset
---

## Summary

Ticket: QSL-745 item (c). Consistency between the FR-272 and
native-diagnostics `resource_exhausted` and `cancelled` rows, judged both by
reading and by the repo's own TC-233 row parsers
(`diagnostic_row_causes` / `diagnostic_row_payload_keys` in
tests/checked_package_v2.rs). By reading, both rows list the same four
`resource_exhausted` causes and `caller-cancelled`; the `allocation-failed`
and `reservation-size-overflow` field text is a faithful copy; removing
"effective limit" from `cancelled` is right. Mechanically, the
native-diagnostics row does not follow the row grammar the existing TC-233
tests parse, and the two `cancelled` rows retain different fields.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The native-diagnostics `resource_exhausted` row ends its cause list with `.` instead of `;` and writes "retain `requested` as" instead of "`requested`:". Under the repo's TC-233 parsers it yields causes {insufficient-next-charge, allocation-failed, reservation-size-overflow, intake-limit-exceeded, requested, `<unit>:<amount>`, bytes:<amount>, additional-elements:<amount>} and keys {upstream, setting, limit, actual} (no `requested`), so FR-272-AC-16's "list exactly" cannot pass the way AC-10/AC-11 are checked. Fix: `…or `intake-limit-exceeded`; for `insufficient-next-charge`, retain…` and "`requested`: the exact upstream…". | proposals/quire-v1/definitions/native-diagnostics.md:82, FR-272-AC-16, tests/checked_package_v2.rs:6031-6089 | wrong-requirement |
| FND-002 | medium | The two `cancelled` rows retain different fields: native-diagnostics "the cancellation source" only, FR-272 "the actual stage, accounting contract, cancellation source, prior usage and the work item that could not proceed". FR-272-AC-16 closes only the cause, and TC-233's "same causes and payload keys" holds only vacuously because neither row names a key; `insufficient-next-charge` likewise names no keys although `stage`, `limit` and `consumed` are canonical. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:78, proposals/quire-v1/definitions/native-diagnostics.md:83, FR-272-AC-16 | wrong-requirement |
