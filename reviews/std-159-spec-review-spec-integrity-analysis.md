---
id: SR-13800
title: 'Integrity review of QSpec PR #326: FR-419 fixed-priority miss evidence as job iterates (STD-159)'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification@b8cbeabdd; git diff origin/main...b8cbeabdd (PR #326, branch std-159-spec): spec/functional/analysis/FR-419-analyze-task-set-schedulability.md, spec/test-cases/TC-372-schedulability.md; ticket STD-159'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-419
  type: reviews
- target: ix://agent-ix/quire-specification/TC-372
  type: reviews
---

## Summary

Ticket: STD-159 (QSpec half; the QSL half is quire-spec-language#1184). PR: quire-specification#326, head b8cbeabdd. Method: spec-review, spec-integrity-analysis. The PR defines the job-`q` recurrence of the level-`i` busy period in FR-419's fixed-priority section, replaces the fixed-priority miss evidence (scheduling points and demands) with the task, job `q` and that job's recurrence iterates, rewrites FR-419-AC-2 and TC-372 RT-03 to job 0 with iterates 6, 10, 13, and adds FR-419-AC-7 and TC-372 RT-12 for a jitter-and-blocking set with iterates 5, 7.

Evidence:

- Hand and script recomputation (scratchpad probe `rta.py`, exact rationals). AC-2 third task `(6, 12)` under `(1, 4)` and `(2, 6)`: `6`, then `6 + ⌈6/4⌉ + 2⌈6/6⌉ = 10`, then `6 + ⌈10/4⌉ + 2⌈10/6⌉ = 13 > 12`. The first two tasks meet with fixpoints 1 and 3, so the third task is the miss. AC-7: `b` starts at `C + B = 5` (`5 + 1 = 6`, not above 6), then `5 + ⌈5/4⌉ · 1 = 7`, and `7 + 1 = 8 > 6`. Task `a` meets at 1. Both match the ACs.
- QSL origin/main 0b839ddc3, scratch worktree: `cargo test -p qsl-analyze --test it schedulability`, 16 passed. `a_third_wcet_of_six_misses_under_fixed_priority_and_edf` asserts `Miss{c, job 0, [6, 10, 13]}` and `Utilization(13/12)`. `jitter_and_blocking_move_the_miss_to_b_and_its_evidence_rechecks` asserts `Miss{b, job 0, [5, 7]}` over exactly AC-7's set and that `recompute` agrees. `qsl-analyze/src/schedulability/busy_period.rs` is the recurrence the new text states: start `(q + 1) · C + B`, response `w + J − q · T`, stop at the first iterate above the bound.
- The new text matches QSL ADR-026 RT-7 and FR-247 nearly word for word, and FR-247-AC-2 carries the same two vectors.
- `quire validate --scope .` over the two touched files exits 0; over `spec/**/*.md` 134 documents fail, the main count, and none is FR-419 or TC-372. `cargo test --test checked_package_v2 --test backend_provider_v1`: 117 and 14 passed. The PR touches no `docs/`, schema or vector file. `proposals/backend-provider-v1/schema.json` leaves FR-419 evidence untyped (line 872), so it is unaffected.
- No other statement under `spec/` outside reviews names scheduling points or the old demands. FR-331 refers to FR-419's evidence by reference only.

Soundness of the alignment. The old evidence was the time-demand test: at every scheduling point `t <= D_i`, the demand `W(t) > t` (for AC-2, 9 > 4, 10 > 6, 12 > 8, 13 > 12). That test is exact only for `D_i <= T_i` with no jitter or blocking, and it is a different analysis from the response-time recurrence FR-419 states as its decision procedure, so evidence and decision disagreed in kind. With jitter, the scheduling points shift to `k · T_j − J_j`, and with `D_i > T_i` it needs the busy period anyway. The job iterates are the decision procedure's own trace: each follows from the previous by the stated recurrence, the first is the stated start, and monotonic iteration toward the least fixpoint makes the first iterate above the bound a proof of the miss. They cover jitter, blocking and `D_i > T_i`, and they are shorter. Aligning QSpec to the implemented QSL baseline is sound.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-419 Fixed priority section | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:72-86 | examined |
| FR-419 Verdicts and evidence table, Miss and Schedulable rows | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:105-109 | examined |
| FR-419-AC-2 | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:129 | examined |
| FR-419-AC-7 | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:134 | examined |
| TC-372 RT-03, RT-12 | spec/test-cases/TC-372-schedulability.md | examined |
| FR-419-AC-1, AC-4, TC-372 RT-05 | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:128,131; spec/test-cases/TC-372-schedulability.md | context_only |
| FR-331 closed-form evidence | spec/objects/interfaces/FR-331-backend-provider-envelope.md:69,264 | context_only |
| QSL ADR-026 RT-4, RT-7; FR-247; FR-248 | quire-spec-language@0b839ddc3 spec/decisions/ADR-026-dense-time.md:370-381; spec/functional/FR-247-analyse-a-task-set-in-closed-form.md:53-82; spec/functional/FR-248-settle-a-closed-form-verdict-after-recomputing-its-evidence.md:66-101 | context_only |
| qsl-analyze busy_period, fixed_priority | quire-spec-language@0b839ddc3 qsl-analyze/src/schedulability/busy_period.rs, fixed_priority.rs | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new text defines a miss for job `q > 0` and requires every earlier job to keep the busy period open, but no FR-419 AC or TC-372 row exercises a job past job 0 or that earlier-job condition. Both new vectors are job 0 with `D = T`. QSL FR-248-AC-2 already has such a vector (`a (1, 2)`, `b (C = 1, T = 3, D = 1, B = 1)`; evidence naming job 2 is refused naming job 1). A provider that only ever examines job 0 satisfies every FR-419 AC. | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:79-84,126-134 |
| FND-002 | medium | "every earlier job keeps the busy period open" does not say what open means. QSL FR-248 fixes it as `w_p > (p + 1) · T_i − J_i`, strict, with `w_p` that job's fixpoint. Without it a second provider could use `>=`, or omit `J_i`, and accept or refuse different `q > 0` evidence, which `check_closed_form` must recompute. | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:83-84 |
| FND-003 | low | The miss and its evidence are stated only against `D_i`. A `response N on S.τ <= d` claim misses against `d`: TC-372 RT-05 (`<= 11 ms`) is refuted with iterates 5, 9, 12, where 12 is not above `D = 12`. QSL compares with `subject.bound(task)`. The Miss row and the new sentence should say the bound is `d` for a response claim. | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:83-85,108 |
| FND-004 | low | The Schedulable row still gives "Each task's fixpoint sequence", while the new busy-period text gives a task with `D_i > T_i` one sequence per job of its busy period. QSL's evidence is per task, per job (`TaskFixpoint { task, jobs: Vec<JobIterates> }`). The row should say each job's fixpoint sequence. | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:107 |

## Verdict

Not clean, but the changed numbers are right. The AC-2 and AC-7 iterates match an independent recomputation and QSL's passing tests, the new text matches QSL ADR-026 RT-7 and FR-247, ids are unique (AC-7, RT-12 are new), the wording describes behaviour with no changelog or status language, and no stale statement elsewhere in `spec/` contradicts it. Moving FR-419 from scheduling-point demands to the recurrence's own iterates is sound: the old evidence came from a different test and does not extend to jitter, blocking or `D > T`. FND-001 and FND-002 are the gaps to close: the `q > 0` branch the PR introduces has no AC, and its open condition is not stated. FND-003 and FND-004 are small.

## Dispositions

Round 1, head 4cb925e17 (fix commit 4cb925e on b8cbeabdd; base 014f0ce, two commits behind origin/main 98db47c, which touch neither FR-419 nor TC-372 and merge cleanly). Each fix was checked against the head text, an independent recomputation of the recurrence (scratchpad probe `rv326-d1/probe/rta2.py`, exact rationals) and QSL origin/main 48b47ab99 (`qsl-replay/src/certificate/closed_form/task_set.rs`, `qsl-analyze/src/schedulability/busy_period.rs`; `cargo test -p qsl-analyze --test it schedulability`, 16 passed).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 4cb925e adds FR-419-AC-8 and TC-372 RT-13: "Tasks `a` (`C = 1`, `T = D = 2`, priority 1) and `b` (`C = 1`, `T = 3`, `D = 1`, blocking 1, priority 2) under fixed priority settle `refuted` with task `b`, job 0 and iterate 2. Evidence naming `b`'s job 2 with iterates 4, 6, 7, 8 settles `inconclusive`, `replay-parity`, because job 1's completion 6 does not exceed `2 · 3`." Recomputed: job 0 starts at 2 > 1; job fixpoints 4 > 3 (open), 6 not > 6 (closed); job 2 iterates 4, 6, 7, 8. QSL `a_miss_for_a_job_past_the_busy_period_names_the_job_that_closes_it` asserts the same. |
| FND-002 | fixed | 4cb925e: "For `q > 0`, every earlier job `p < q` keeps the busy period open: its completion `w_p`, the fixpoint of the same recurrence for job `p`, satisfies `w_p > (p + 1) · T_i − J_i` (strictly); otherwise job `q` lies past the busy period and is not a miss." Matches QSL `busy_period.rs` (`window > (job + 1) * period - jitter`). |
| FND-003 | fixed | 4cb925e: "A miss is the task and the job `q` whose iterates reach a response time above the bound, which is `D_i` for a `schedulable` claim and `d` for a `response N on S.τ <= d` claim." The Miss row now ends "exceeds the bound"; FR-419-AC-9 and RT-05 give `<= 11 ms` refuted with job 0 and iterates 5, 9, 12. Recomputed; QSL `a_response_claim_holds_at_twelve_and_misses_at_eleven` asserts the same. |
| FND-004 | fixed | 4cb925e Schedulable row: "Each job's fixpoint sequence of each task (fixed priority, AMC; one job when `D_i <= T_i`); the busy-period length and QPA's check sequence (EDF)". Matches QSL `TaskFixpoint { task, jobs }`. |

The fix commit also restates the task-set refusals: equal priorities refuse a `fixed_priority`, `amc` or `response` claim naming both tasks; non-positive `D` and a `LO` task with dual budgets refuse; `edf` reads no priority. Each matches QSL `TaskSet::new`, `check_task` and `ScheduleSubject::new` (`PriorityClash { first, second }`, `NotPositive { Deadline }`, `LoDualBudgets`), and AC-5 and RT-14 carry the new cases. `quire validate` over `spec/**/*.md`: 134 failing documents on both head and origin/main, the same set, none FR-419 or TC-372; the touched files and the three `reviews/std-159-*.md` exit 0. `cargo test --test checked_package_v2 --test backend_provider_v1`: 117 and 14 passed. No `docs/`, `tests/`, `proposals/` or vector file is touched, and no statement elsewhere under `spec/` or `proposals/` contradicts the new text.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | The job-0 stop rule still reads "The iteration stops at the fixpoint or as soon as `R + J_i` exceeds `D_i`", while the miss and the Miss row now use "the bound", `d` for a response claim. For AC-9's `<= 11 ms` the stated stop rule would not stop at 12 (12 does not exceed `D = 12`), although the miss evidence stops there; QSL stops at `subject.bound(task)`. Replace `D_i` with "the bound" in that sentence. | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:79-80 |
| FND-006 | low | The behaviour refuses equal priorities for `fixed_priority`, `amc` and `response` claims, but FR-419-AC-5 and TC-372 RT-14 exercise the clash only under `fixed_priority` (and `edf` admission). The `amc` and `response` branches have no criterion, so a provider that checks priorities only for `fixed_priority` satisfies every AC. Add the clash under an `amc` claim and under `T3` to AC-5 and RT-14. | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:54-59,139; spec/test-cases/TC-372-schedulability.md:51 |
