---
id: SR-9003
title: "Cross-reference review of QSpec PR #247 (resource_exhausted causes)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@5017b5cf6b6d1c731e103b6cb2452145af856d41; spec/objects/foundation/FR-272-typed-diagnostic-cause.md, proposals/quire-v1/definitions/native-diagnostics.md; against agent-ix/quire-spec-language origin/main FR-255, FR-260 and qsl-foundation LimitExceeded"
review_set: subset
---

## Summary

Ticket: QSL-745 item (c). Cross-repository lens: checks the new
`intake-limit-exceeded` cause and its payload names against the QSL
requirements and code that produce it (QSL FR-260 item 5 / FR-260-AC-4,
FR-255 Behavior 1 / FR-255-AC-1, `qsl-foundation` `LimitExceeded::catalog_fields`,
`qsl-semantics` `ModelRefusalCause::IntakeLimitExceeded`). The cause name,
code and setting `intake.input_bytes` agree with QSL. The payload names do
not: QSL FR-255 requires `kind`, `bound`, `actual`, `setting` for exactly this
cause, and QSpec now pins `setting`, `limit`, `actual`. The FR-272 row cites
QSL FR-260 item 5 / FR-260-AC-4 correctly (it exists on QSL main); the
native-diagnostics row carries no QSL source for the new cause.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | `intake-limit-exceeded` payload is pinned as `setting`, `limit`, `actual`, but QSL FR-255 Behavior 1 and FR-255-AC-1 require intake's `resource_exhausted`/`intake-limit-exceeded` catalog record to carry `kind`, `bound`, `actual` and `setting`, and QSL's `LimitExceeded::catalog_fields` emits `kind`/`bound`/`actual`/`setting`; QSL-745's AC "QSpec catalog and QSL spec agree" fails. Planner must pick one spelling: adopt `kind`+`bound` in QSpec (add to the canonical list, matching shipped QSL code and every FR-255 row) or rename QSL's `bound`→`limit` and drop/justify `kind` across QSL FR-255 and code. | FR-272-AC-16, spec/objects/foundation/FR-272-typed-diagnostic-cause.md:77, proposals/quire-v1/definitions/native-diagnostics.md:82, QSL spec/functional/FR-255:141-153,210 | wrong-requirement |
| FND-002 | low | The native-diagnostics `resource_exhausted` row adds `intake-limit-exceeded` with no QSL source citation, while the FR-272 row now cites QSL FR-260 item 5 / FR-260-AC-4 and the row's only citation (QSL FR-082-AC-10..11) is about allocation. | proposals/quire-v1/definitions/native-diagnostics.md:82 | wrong-requirement |

## New findings (disposition pass 1)

Reviewed at agent-ix/quire-specification@9c5d7871bb6f7e22aa7d7489703b2ed30cbd1b78; unchanged by a66831b7893692135acb60a65e0074f2510f5cc4.

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-003 | high | The new `insufficient-next-charge` keys `stage`, `definition`, `limit`, `consumed`, `subject` disagree with QSpec's own charge record (FR-150-AC-11 `incomplete(limit_kind, limit, consumed, next_charge, charge_point)`) and with the record QSL emits on #811 (`stage`, `accounting_contract`, `consumed`, `next_charge`, `charge_point`, qsl-semantics/src/model/intake/unit.rs and observation.rs; QSL FR-255 Behavior 1): the accounting contract is `definition` instead of `accounting_contract`, the work item is `subject` instead of `charge_point`, and `next_charge` is dropped. The `cancelled` row copies the same choice. | FR-272-AC-16, spec/objects/foundation/FR-272-typed-diagnostic-cause.md:77-78, proposals/quire-v1/definitions/native-diagnostics.md:82-83, spec/functional/type-model/FR-150-preserve-model-normalization-provenance.md:140 | wrong-requirement |
| FND-004 | high | FR-272-AC-16 closes `resource_exhausted` to exactly four causes, but QSL specifies and emits `resource_exhausted`/`input-bytes-exceeded` for the formatter's `format.output_bytes` ceiling (QSL FR-255 Behavior 1 table and FR-255-AC-1, qsl-cst/src/format.rs `OutputBudgetExhausted`, on QSL main and #811). QSpec has no such cause, so the two catalogs still disagree on this code. | FR-272-AC-16, proposals/quire-v1/definitions/native-diagnostics.md:82 | missing-requirement |

## New findings (disposition pass 2)

Reviewed at agent-ix/quire-specification@88271576e42fbf1b20b53301fbc996a83902e386.

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-005 | low | QSL emits `resource_exhausted`/`population-identity-limit-exceeded` (qsl-semantics/src/model/refusal.rs, on QSL main, impl/business-model-meanings and #811), a cause neither QSL's spec nor QSpec catalogs; QSL FR-255's own table puts `identity.input_bytes` under `stage_limit_exceeded`. Under FR-272-AC-16's closed five it is non-conforming. QSL-side: route to QSL-745 (a)/(b) or a QSL ticket; no QSpec change. (`ancestor-steps`/`family-steps` are the already-ruled QSL-745 follow-ups.) | FR-272-AC-16, QSL qsl-semantics/src/model/refusal.rs:1333,2034 | implementation-bug-despite-evidence |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 9c5d787 |
| FND-002 | fixed | 9c5d787 |
| FND-003 | fixed | 8827157 |
| FND-004 | fixed | 8827157 |
| FND-005 | deferred | QSL-side: routed to the QSL-745 work in agent-ix/quire-spec-language#811; no QSpec change in #247 |
