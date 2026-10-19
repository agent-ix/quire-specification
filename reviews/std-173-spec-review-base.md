---
id: SR-12822
title: "Base review of the FR-331 no-instance wire cause"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; PR #280 (STD-173) at d9780f2: spec/objects/interfaces/FR-331-backend-provider-envelope.md, spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md, spec/functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md, spec/functional/analysis/FR-391-settle-state-graph-claims.md, spec/test-cases/TC-294-verdict-strength-and-stopped-runs.md, spec/test-cases/TC-334-state-graph-partial-runs-and-verdict-map.md, proposals/backend-provider-v1/schema.json, proposals/backend-provider-v1/vectors.json"
review_set: subset
review_date: "2026-10-19"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-331, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-361, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-363, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-391, type: reviews }
  - { target: ix://agent-ix/quire-specification/TC-294, type: reviews }
  - { target: ix://agent-ix/quire-specification/TC-334, type: reviews }
  - { target: ix://agent-ix/quire-specification/TC-431, type: reviews }
---
## Summary

Ticket: STD-173. The PR adds the `no-instance` inconclusive wire cause: the
FR-331 closed-cause row now lists `no-initial-state` and `no-instance`, the
schema `InconclusiveCause` enum gains `no-instance`, the `temporal-outcome`
vector gains result index 5, and FR-361, FR-363, FR-391, TC-294 and TC-334
carry the zero-instance case. The spelling and meaning match the merged QSL
side (qsl-lane2 main 49bcf9a): FR-125 Instancing, FR-126 `Undecided(NoInstance)`,
FR-127 V-6 and FR-169 V-6. The cause is reported before any state is explored,
settles `inconclusive`/`unsettled`, and is never read as holding. Kebab-case
`no-instance` sits beside the existing `no-initial-state`.

Checks run:
- `cargo test --test backend_provider_v1`: 9/9 pass, so the vectors are valid
  against the schema.
- Indexes and accounting for vector index 5 are consistent: dispositions,
  results and accounting each have six entries (0..5), and the owners map
  `/results/5/cause` points to FR-363.
- `quire validate` (0.36.2) on every changed `.md`: only FR-331 fails. The same
  two errors (`[value_object] required 'id'`, `semantic.record-invalid ...
  "fields"`) occur on `origin/main`'s FR-331, so the failure is pre-existing,
  as the author said.
- `quoin matrix` reports FR-331-AC-24 as untagged.

The repo-wide grep for `no-initial-state` and `undecided-successor` found one
cause list without `no-instance`: FR-399's hyperproperty row. That row is
covered in the scope-boundary review. There are no id collisions with #273,
#278 or #279. Draft #200 also allocates FR-331-AC-24.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-331-AC-24 names TC-431 as its verification, but TC-431 has no procedure row that cites FR-331-AC-24, and no `#[trace("TC-431", "FR-331-AC-24")]` exists in tests/backend_provider_v1.rs. quoin matrix reports AC-24 untagged. The precedent (#199, FR-331-AC-23) added a BP row and a trace tag in the same PR. The vectors also cover only half of AC-24: no positive vector carries a `no-initial-state` result, and no adverse vector refuses a misspelling. Failure scenario: someone drops `no-initial-state` from the schema enum, or admits `no-instances`, and TC-431 stays green. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:366; spec/test-cases/TC-431-backend-provider-wire-vectors.md:22-35; proposals/backend-provider-v1/vectors.json:1127-1135 | correct-requirement-no-evidence |
| FND-002 | low | The closed-cause table sits under "Wire spellings of probabilistic, timed and real-time results", and its refusal rule applies to "these items". Model-subject temporal causes, `undecided-successor` and `replay-parity` already among them, are not covered by that heading, so a reader could argue the `unknown-cause` refusal does not cover `no-instance`. The schema enum enforces it anyway. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:84-100 | wrong-requirement |
| FND-003 | low | TC-294 step 7 says "a subject whose universe is empty". A subject has one universe per population, and step 5 is the no-universe case, so step 7 should name `config_history` with an empty universe list to separate the two. | spec/test-cases/TC-294-verdict-strength-and-stopped-runs.md:32-33 | wrong-requirement |
| FND-004 | low | Id collision: draft PR #200 (certificate rejection locus) also adds FR-331-AC-24 and BP-11..BP-17. Whichever merges second must renumber. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:366 | wrong-requirement |

## Verdict

The cause spelling, the semantics and the agreement with QSL are correct. The
schema and vectors are consistent and pass TC-431. FND-001 should be fixed
before merge: add a TC-431 row and a trace tag for AC-24, and add a
`no-initial-state` result and a misspelled-cause adverse vector, or narrow
AC-24 to what the vector shows. The low findings are optional.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-005 | low | The new adverse vector `temporal-cause-no-initial-state-underscore` opens with `{` at column 0 instead of the file's 4-space array indent. It is the only such line besides the root brace. The JSON is valid, so this is layout only. | proposals/backend-provider-v1/vectors.json:3132 | wrong-requirement |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (2ab5425). The criterion is renumbered FR-331-AC-28. TC-431 row BP-18 cites it, and `#[trace("TC-431","FR-331-AC-28")]` is on the positive and adverse tests; quoin matrix reports it tagged. The `temporal-outcome` vector carries result 6 `no-initial-state` beside result 5 `no-instance`. Adverse vectors `temporal-cause-no-instances` and `temporal-cause-no-initial-state-underscore` are refused. cargo test backend_provider_v1: 9/9 pass. Dispositions, results and accounting all run 0..6. |
| FND-002 | fixed | Round 1: the heading is now "Wire spellings of result causes and proof bases". |
| FND-003 | fixed | Round 1: TC-294 step 7 names `config_history` with an empty universe list, and adds a second subject with no universe that expects `requires-bound`. |
| FND-004 | fixed | Round 1: renumbered FR-331-AC-28 and BP-18, past draft #200's FR-331-AC-24..27 and BP-11..17. Whichever of #200 and #280 lands second rebases vectors.json. |
