---
id: SR-15770
title: "spec review of STD-269/STD-285 uuid, timestamp and population in the FR-181 typed canonical form (QSpec PR #372)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@d00b318a3e8520d5f1896a72cb4d6e90fd4d6f76; spec/functional/runtime/FR-181-simulate-finite-executions.md, spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md, spec/test-cases/TC-189-collection-kind-algebra.md, spec/test-cases/TC-210-finite-simulation-reproducibility.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-181
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-144
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-189
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-210
    type: reviews
---
# SR-15770: spec review of STD-269/STD-285 uuid, timestamp and population in the FR-181 typed canonical form (QSpec PR #372)

## Summary

Tickets: STD-269 and STD-285 (QSL-823, QSL-831). The diff is `origin/main...d00b318a3` (origin/main 5ee0071fc is an ancestor; `git ls-remote` confirms the head). Four files, spec Markdown only. FR-181 gains the `uuid` and `timestamp` forms, lists both as canonical in "Canonical element encoding", states that a population is not a value (a state member holding its objects is `Set<Reference<T>>`), and adds AC-19 to AC-21. FR-144 gains the `UUID`, `Timestamp` canonical-key row, reads supplied UUID/Timestamp collections by the new forms, keeps `string` and population element types refused, and rewrites AC-22. TC-189 C24 and a new TC-210 section carry the vectors.

These claims hold:

- Every JCS byte vector in FR-181-AC-19, AC-20 and TC-189 C24 was recomputed (sorted keys, no whitespace, set elements ordered by UTF-8 bytes) and matches exactly: uuid, Timestamp `-1`, decimal `1.50` (coefficient 150, scale 2), the quantity with `unit`, enum, record with an absent field as `null`, tuple, nullary union, the set-element normalization of `1.50` to coefficient 15, scale 1, `Set<Timestamp>` `{10, 2}` ordered `10` before `2`, and C24's `{0, 1}` order.
- The UUID and Timestamp forms agree with QSL FR-056's K2 canonical payloads: 32 lowercase hex digits `8-4-4-4-12`, and the signed-128 nanosecond count since the POSIX epoch spelled `0` or optional `-` then a nonzero digit. The member name `nanoseconds` is K2's own; `+1`, `01` and uppercase hex are K2-noncanonical, as AC-21 and AC-22 say.
- The new FR-144 key row (canonical payload ASCII bytes, unsigned bytewise, proper prefix first) agrees with FR-149-AC-16 (`10` before `2`, `-1` before `-2`) and with FR-056's statement that the key is a representation order, not chronological. FR-144 already says a key is not an ordering operator.
- The FR-144 supplied-collection paragraph and AC-22 are consistent with the FR-181 forms: a payload outside the form falls in the existing `invalid-value` row; `string` (FR-322 names it, FR-181 has no form) stays `unsupported-feature` before any element is read. No other QSpec file still says uuid or timestamp has no FR-181 form.
- Timestamp default: the PR takes STD-285's lead default (one `nanoseconds` member, the FR-149 key's own text) over STD-269's `seconds`/`nanos` split. That is the only choice consistent with K2 FR-056 and FR-149 without a second grammar. The PR body labels it a lead default, not an owner answer.
- No test-parsed fixture or vector changes. QSL tests read from QSPEC_DIR only `proposals/**` JSON fixtures and vectors, `proposals/quire-v1/definitions/native-diagnostics.md` and `spec/objects/interfaces/FR-322-checked-package-artifact.md` (qsl-package checked_v2, emit/tests; qsl-semantics node_key tests, complete_value_lock, quantities, qspec_diagnostics). None is in this diff, so no QSL conformance run is needed.
- FR-181-AC-19, AC-20 and AC-21 appear on no other open QSpec PR head and not on main. No status or changelog wording; the "lifted by QSL-831" ticket wording is removed. No compat layer.
- Gates in a detached worktree at the head: `quire validate --scope . 'spec/**/*.md'` gives 86 failure lines (baseline), none on a touched file; `cargo test --test checked_package_v2 --test backend_provider_v1` passes 23 and 119.

STD-285 item 3 (the QSL Decimal element form) is correctly not decided here. The PR body says the lead default is "carried into the QSL FR-106 PR for STD-293", but QSL #1255 leaves it undecided too and records it only in its PR body's "Ungrounded" section. The decision is recorded nowhere durable, which is a planner action, not a defect of this diff.

The `code-review` method was skipped, because the diff contains no code.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | major: FR-181-AC-21 and its TC-210 step say two states that "differ only in the order their population objects were created" have equal state keys. No requirement says how the simulator gives a created object its identity. If identity follows creation order, as a counter would make it, the two states hold different reference triples, the keys differ, and the AC is false for that implementation. <br>Fix: state the premise as data: two states whose population members are the same reference identities, inserted in two orders, have equal state keys. Or state that the identities do not depend on creation order. | spec/functional/runtime/FR-181-simulate-finite-executions.md:302, spec/test-cases/TC-210-finite-simulation-reproducibility.md:73-76 |
| FND-002 | low | minor: the Timestamp form states the signed 128-bit domain, but no vector exercises it. AC-21, AC-22 and C24 refuse only `+1`, `01` and uppercase hex. A reader that accepts `170141183460469231731687303715884105728` (2^127) passes every listed case. <br>Fix: add that payload, or `-170141183460469231731687303715884105729`, to FR-144-AC-22 and C24 as `invalid_runtime_input`/`invalid-value`, and keep 2^127 - 1 admitted. | spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md:314, spec/test-cases/TC-189-collection-kind-algebra.md:56 |
| FND-003 | low | minor: layout. "canonical as / defined above. Two / kinds" is broken across three short lines, and the new population sentence runs into one long line with "A unit-bearing quantity ...". <br>Fix: reflow both paragraphs. | spec/functional/runtime/FR-181-simulate-finite-executions.md:131-134, spec/functional/runtime/FR-181-simulate-finite-executions.md:147-150 |

## Dispositions

Disposition pass 1, head 8f6263d74 (fix commit; origin/main is an ancestor). `quire validate --scope . 'spec/**/*.md'` gives 86 failure lines, none on a touched file; `cargo test --test checked_package_v2 --test backend_provider_v1` passes 23 and 119. The 2^127, -2^127 - 1 and 2^127 - 1 decimal strings were recomputed and are exact.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-181-AC-21 now reads "two states whose population member holds the same reference identities, inserted into the set in two different orders, have equal state keys", and the TC-210 step encodes two such states. The AC no longer depends on how created objects get identities. |
| FND-002 | fixed | FR-181-AC-21 lists 2^127 and -2^127 - 1 as outside the form and 2^127 - 1 inside it; FR-144-AC-22 and TC-189 C24 refuse 2^127 as `invalid-value` at `/elements/0` and admit 2^127 - 1. |
| FND-003 | fixed | The "canonical as defined above. Two kinds" paragraph is reflowed, and the unit-bearing quantity sentence is its own paragraph. |

## Verdict

Disposition pass 1: FND-001 to FND-003 are fixed and there is no new finding. Mergeable; it merges before QSL #1255.

Review pass: Mergeable after FND-001 is fixed, a one-sentence change to AC-21 and the TC-210 step. FND-002 and FND-003 are minor and cheap to fix in the same round. The byte vectors, the K2 payload agreement, the FR-144 key row, the AC-22 rewrite, the nanoseconds default, the absence of test-parsed fixture edits, id uniqueness and both gates are verified. Owner or planner decisions: confirm the `nanoseconds` Timestamp form over STD-269's `seconds`/`nanos` default, and record STD-285 item 3 (QSL Decimal element form) on a Linear ticket, because neither PR decides it.
