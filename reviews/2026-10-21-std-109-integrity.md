---
id: SR-14683
title: "integrity review of STD-109 FR-181 successor tie-break, initial-state order and sampled start (QSpec PR #340)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@102b3433fba0642b48b42b05524f60bb31a0512a; spec/functional/runtime/FR-181-simulate-finite-executions.md, spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md, spec/test-cases/TC-210-finite-simulation-reproducibility.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-181
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-361
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-210
    type: reviews
---
# SR-14683: integrity review of STD-109 (QSpec PR #340)

## Summary

Ticket: STD-109. Diff reviewed: `git diff origin/main...102b343` (3 files).
The PR adds two FR-181 paragraphs (the successor tie-break, and the canonical
initial-state order and its indices), a sampled-start rule, FR-181-AC-16 to
AC-18, a TC-210 section, and an FR-361 "Model subject" sentence.

Checked against the ticket's three choices, which follow QSL FR-101 (QSL
PR #457): ascending post-state state-key bytes, ascending state-key byte
order with equal keys coalesced, and `t mod m` with no draw. All three match.

Checked against the consumers of an initial-state index. FR-392 already
reads "the subject's initial states in their canonical order", and its walk
`r` from initial state `i` uses trace index `i × witness_samples + r`.
FR-408 sample `i` of test `j` starts from test `j`'s initial state, with "no
draw selects the start". FR-364 `initial_state`, FR-393 `initial` and
FR-400's "in canonical order" index the same list. The new rule "a sampled
trace whose requirement names its initial state ... starts there" leaves
FR-392 and FR-408 unchanged. The `t mod m` default applies only where no
start is named, so it contradicts neither. FR-383's "first initial state in
subject order" reads the same canonical list.

The AC-16 case (equal transition identities, different post-states) is
reachable: FR-013-AC-1's successor relation ranges over frame post-states,
so one operation, receiver and argument vector can have more than one
post-state.

No other open QSpec PR edits FR-181 or TC-210, and AC-16 to AC-18 are unused
elsewhere. PR #346 also edits FR-361, in a different section, and
`git merge-tree` of the two heads is clean. Gates: `quire validate` reports
134 failing documents (main 134), and the three touched files have no
diagnostics. `cargo test --test checked_package_v2 --test backend_provider_v1`
passes, 117 + 14.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | One paragraph uses `m` for two quantities in adjacent sentences: "trace index `i · m + j` to sample `i` of test `j` among `m` tests", then "with `m` initial states in canonical order, trace `t` starts at ... `t mod m`". Under FR-408, `m` counts (initial state, binding) pairs, not initial states. Use a different symbol for the initial-state count. | spec/functional/runtime/FR-181-simulate-finite-executions.md:203-207 |
| FND-002 | low | FR-181-AC-18 ends "so the TC-210 sampler index-sequence vector is unchanged". That describes this edit's effect on a test document, not behaviour. The behaviour is that with one initial state every trace starts there, and the trace's draws are exactly those of its trace index. | spec/functional/runtime/FR-181-simulate-finite-executions.md:288 |
| FND-003 | low | The TC-210 AC-16 vector says the parent "visits `s1`'s successor first". `s1` is itself the successor, so the phrase can be read as `s1`'s own child at the next level. Say "visits `s1` before `s2`". | spec/test-cases/TC-210-finite-simulation-reproducibility.md:52-56 |

## Verdict

Faithful to the ticket and to QSL FR-101. It is consistent with FR-392,
FR-408, FR-364, FR-393 and FR-400, and it leaves no unbuildable criterion.
Three wording fixes remain (FND-001 to FND-003).

## Resolution

FND-001: the initial-state count is `k` in the FR-181 sampled-start paragraph and in FR-181-AC-18; `m` keeps the FR-408 meaning.
FND-002: FR-181-AC-18 states that with one initial state every trace starts there and a trace's draws are exactly those of its trace index.
FND-003: the TC-210 vector reads "visits `s1` before `s2`".
