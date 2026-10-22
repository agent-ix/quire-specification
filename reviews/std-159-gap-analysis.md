---
id: SR-13802
title: 'Gap analysis of QSpec PR #326: FR-419 changed and new criteria against TC-372 rows (STD-159)'
type: SpecReview
analysis: gap-analysis
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

Ticket: STD-159. PR: quire-specification#326, head b8cbeabdd. Method: gap-analysis, limited to the changed and new criteria of FR-419 against TC-372's vector rows and the implementation that verifies them. Plan completion: not assessed.

- FR-419-AC-2 (changed) maps to TC-372 RT-03 (fixed priority, changed to the third task, job 0, iterates 6, 10, 13) and RT-04 (EDF, `13/12`, unchanged). The row and the AC agree.
- FR-419-AC-7 (new) maps to TC-372 RT-12 (new) with the same task set and result. The row and the AC agree. AC-7's Verification cell names TC-372.
- TC-372's Expected Results ("Every response time, demand, verdict and evidence check is exactly as stated") still holds; "demand" remains true of the EDF rows.
- The implementation: QSL origin/main 0b839ddc3 `qsl-analyze/tests/it/schedulability.rs` tests `a_third_wcet_of_six_misses_under_fixed_priority_and_edf` and `jitter_and_blocking_move_the_miss_to_b_and_its_evidence_rechecks` assert both vectors exactly and pass (16 of 16 schedulability tests, scratch worktree).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-419-AC-2 | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:129 | examined |
| FR-419-AC-7 | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:134 | examined |
| TC-372 RT-03 | spec/test-cases/TC-372-schedulability.md | examined |
| TC-372 RT-04 | spec/test-cases/TC-372-schedulability.md | context_only |
| TC-372 RT-12 | spec/test-cases/TC-372-schedulability.md | examined |
| QSL schedulability tests | quire-spec-language@0b839ddc3 qsl-analyze/tests/it/schedulability.rs:227-297 | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No QSL test carries a `QSpec-FR-419-AC-2` or `QSpec-FR-419-AC-7` trace tag, although the two QSL tests that assert these exact vectors pass. The AC-2 tag was removed in QSL-584 (QSL `reviews/qsl-584-gap-analysis.md` FND-003) because QSpec's old evidence differed; with this PR the evidence agrees, and quire-spec-language#1184 does not add the tags. Follow-up in QSL: tag those two tests with `QSpec-FR-419-AC-2` and `QSpec-FR-419-AC-7`. | quire-spec-language@0b839ddc3 qsl-analyze/tests/it/schedulability.rs:264-296 |

## Verdict

Not clean, one low cross-repo trace gap. Every changed or new FR-419 criterion has a TC-372 row that states the same input and result, and the implementation already produces exactly those results. Only the QSL trace tags that would bind the two criteria to their tests are missing.

## Dispositions

Round 1, head 4cb925e17. QSL origin/main 48b47ab99 and quire-spec-language#1184 head 692abe7b4 checked by `git grep 'QSpec-FR-419'`.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | Cross-repo: the trace tags belong on QSL tests, which this QSpec-only PR cannot change. Still missing: QSL origin/main 48b47ab99 and #1184 head 692abe7b4 tag only QSpec FR-419-AC-1, AC-3 and AC-4. Needs a QSL follow-up (STD-159 QSL half or a new QSL ticket) tagging `a_third_wcet_of_six_misses_under_fixed_priority_and_edf` with `QSpec-FR-419-AC-2` and `jitter_and_blocking_move_the_miss_to_b_and_its_evidence_rechecks` with `QSpec-FR-419-AC-7`. |

The new criteria have TC-372 rows that state the same inputs and results: FR-419-AC-8 maps to RT-13, AC-9 to RT-05 (now with its miss evidence), and AC-5's new refusals to RT-14. Recomputed independently (scratchpad probe `rv326-d1/probe/rta2.py`): RT-13 job 0 iterate 2 and job 2 refused at job 1 (completion 6, not above 6); RT-05 `<= 11` iterates 5, 9, 12 and `<= 12` proved with 5, 9, 12, 12. QSL `cargo test -p qsl-analyze --test it schedulability` at 48b47ab99: 16 passed, including the tests that assert these vectors.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The new criteria carry no QSL trace tag either: `a_miss_for_a_job_past_the_busy_period_names_the_job_that_closes_it` (FR-419-AC-8's vector) is tagged `FR-248-AC-2` only, `a_response_claim_holds_at_twelve_and_misses_at_eleven` (AC-9) `FR-247-AC-1` only, and `a_task_set_or_claim_that_breaks_a_rule_is_refused_with_its_code` (AC-5's clash and dual-budget refusals) is untagged. The AC-8 test settles only the job-0 evidence; no QSL test runs the analysis on that set to assert job 0, iterate 2. No QSL test refuses a non-positive deadline. Follow-up in QSL alongside FND-001. | quire-spec-language@48b47ab99 qsl-analyze/tests/it/schedulability.rs:169-262,485-500 |
