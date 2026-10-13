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

## New findings (disposition pass 1)

Reviewed at agent-ix/quire-specification@9c5d7871bb6f7e22aa7d7489703b2ed30cbd1b78 (FND-003) and a66831b7893692135acb60a65e0074f2510f5cc4 (FND-004).

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-003 | medium | The new `kind` definition ("a limit's kind as QSL FR-255's setting table lists it (for example input bytes, node count…)") gives the value as the table's prose ("input bytes"), while QSL's `LimitExceeded::catalog_fields` emits `kind` as the cause tag (`input-bytes-exceeded`). FR-272-AC-13 requires each payload value to be one exact ASCII string, and no spelling is pinned, so two implementers write different values. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:190-191, FR-272-AC-13 | wrong-requirement |
| FND-004 | low | The new `conflicting_declaration` rows define `subject` only as "the operation identity or `DeclarationKey` that both declarations resolve to", but FR-271's new row (and QSL FR-246) also apply the code to two tasks with one name, which is neither. The task case has no `subject` definition (reviewed at a66831b). | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:80, proposals/quire-v1/definitions/native-diagnostics.md:85, spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md:83 | wrong-requirement |

## New findings (disposition pass 2)

Reviewed at agent-ix/quire-specification@88271576e42fbf1b20b53301fbc996a83902e386.

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-005 | medium | The `insufficient-next-charge` record drops `limit_kind`, and both rows say "no second key carries its name". That is not one thing under two names: the record now cannot say which counter was denied. A charge point can be denied by different counters (TC-185 D13: `decimal.arithmetic` under `integer_bits` vs `work_units`), and `limit` alone does not identify one. Recommend retaining `limit_kind` (FR-150-AC-11 / FR-330 vocabulary) as a canonical key on the charge record. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:77, proposals/quire-v1/definitions/native-diagnostics.md:82, FR-150-AC-11, spec/test-cases/TC-185-exact-decimal-semantics.md:39 | wrong-requirement |
| FND-006 | medium | The new `input-bytes-exceeded` cause is scoped to "the format and intake byte budgets", but intake's byte budget already carries `intake-limit-exceeded` (FR-272 row; QSL FR-260-AC-4; QSL FR-255 Behavior 1 table gives format → `input-bytes-exceeded`, intake → `intake-limit-exceeded`). An over-long intake document now has two catalogued causes. Recommend scoping `input-bytes-exceeded` to the format output budget (non-intake byte budgets). | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:77, proposals/quire-v1/definitions/native-diagnostics.md:82 | wrong-requirement |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 9c5d787 |
| FND-002 | fixed | 9c5d787 |
| FND-003 | fixed | 8827157 |
| FND-004 | fixed | 8827157 |
