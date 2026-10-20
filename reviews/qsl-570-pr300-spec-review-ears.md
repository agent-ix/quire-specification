---
id: SR-13551
title: 'Spec review of QSpec PR #300: the FR-408 tail rules that meet 2^-64, EARS'
type: SpecReview
analysis: ears-conformance
scope: 'agent-ix/quire-specification@7a61910f0; git diff refs/remotes/origin/main...7a61910f0 (PR #300, branch std-408-bounds-qspec, one commit, +14/-3): spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md; read against agent-ix/quire-spec-language origin/qsl-570-impl a9a693ac0 (PR #1079 head) qsl-analyze/src/statistical/bounds.rs and statistical.rs, QSL FR-189 and FR-192, QSpec FR-407 and TC-355; ticket QSL-570'
review_set: subset
date: '2026-10-20'
relationships:
- target: ix://agent-ix/quire-specification/FR-408
  type: reviews
---

## Summary

Ticket: QSL-570. PR: quire-specification#300 (private), head 7a61910f0, scoped to `git diff refs/remotes/origin/main...7a61910f0`: one file, FR-408's 'Exact rational bounds' method list. Spec-only. It states the tail rules of terra's EN-4 engine (bounds.rs on qsl-570-impl, PR #1079 head a9a693ac0): the `ln 2` series stops at tail `2^-(71 + b)`, `b` the bit length of `|k|`; the quantile bisects to `2^-65`; the `Φ`, `π` and `1/√(2π)` precision is `2^-(72 + ⌈3u²/4⌉)` capped at `2^-328`; a quantile above 16 is refused as beyond precision. EARS review of the changed text: the `ln x` bullet's new sentences and the normal-quantile bullet. Both sit in FR-408's method list, descriptive prose under the FR statement's SHALL, as on main, so their tail rules are not separate requirements. The new rationale ('a fixed `2^-70` tail meets `2^-64` only where `φ` is large') explains why the tail grows. It describes the system, not this change, and no status or build-log wording ('now', 'previously', 'fixed') was added. The exception is the refusal sentence, which is a behaviour. Gates: `quire validate --scope .` on FR-408 exits 0 on the head and on main with the same output (module duplicate-archetype notices and one `semantic.inline-data-schema` notice, none in FR-408). `cargo test --locked` on the head: 158 passed, 0 failed (backend_provider_v1 11, checked_package_model 4, checked_package_v2 111, model_member_types 22, record_conformance 3, shared_reference 7); no test reads FR-408's bounds. No FR-408 vector changes: AC-2's N values, AC-3's thresholds and AC-12's `n = 11632` use bounds that already sit far inside `2^-64` and TC-355 SC-07 states only the `2^-64` claim, which the new rule meets. quoin 0.28.3, quire 0.36.2 (engine 0.50.2). Methods: spec-review/base (SR-13550) and spec-review/ears (SR-13551). code-review and gap-analysis were not run, because no code or test file changed; the engine was read only to check the spec against it.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-408-bounds-ln | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md | examined |
| FR-408-bounds-quantile | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md | examined |
| FR-408-admission | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | 'A `δ` whose quantile lies above 16 ... has no bound and is refused as beyond precision' is a behaviour in passive voice inside the method list, with no actor, no SHALL and no trigger form. Every other refusal in FR-408 is in Admission as 'the provider settles the item `unsupported` with <cause>'. Fix with SR-13550 FND-003: move it there in that form, for example 'If a regenerative request's `min(α', β')` has no quantile bound (the lower `Φ` bound at 16 is below `1 − min(α', β')`), the provider settles the item `unsupported` with <cause> at negotiation'. | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:200-202 |

## Verdict

One low EARS finding (the refusal sentence); the rest of the change is method prose and needs no EARS form. No status wording. The substantive findings are in SR-13550.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 at 9e6be51ec: At negotiation, before any sample is drawn, the provider settles the item `unsupported` with: - `missing-confidence` when the claim states no confidence parameters; - `every-scheduler` when the claim's scheduler clause is `under every scheduler`, with or without a fairness set; - `exact-only-form... |

Round 1: FND-001 is fixed. The refusal is now an Admission bullet in the same 'the provider settles the item `unsupported` with <cause> when ...' form as its siblings. No new EARS finding. Gates on 9e6be51ec: quire validate --scope . on FR-408 exits 0 with output identical to main (module notices only); both SR files under reviews/ validate and are byte-identical to the reviewer's round-0 files; cargo test --locked 158 passed, 0 failed.
