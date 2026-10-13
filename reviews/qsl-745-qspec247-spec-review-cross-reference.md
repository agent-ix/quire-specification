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
