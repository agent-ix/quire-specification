---
id: SR-202611141457001
title: "Base review of the STD-665 statistical.workers registration"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; PR #654 at a5e9aa28, diff origin/main...HEAD: spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md (Inputs, Workers section, FR-408-AC-15..17), spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md (Behavior 5), spec/objects/interfaces/FR-331-backend-provider-envelope.md (request member list), spec/test-cases/TC-356-regenerative-long-run.md (WK-01..04); compared with QSL FR-191 and FR-255 on agent-ix/quire-spec-language main"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-408, type: references }
  - { target: ix://agent-ix/quire-specification/FR-461, type: references }
  - { target: ix://agent-ix/quire-specification/FR-331, type: references }
  - { target: ix://agent-ix/quire-specification/TC-356, type: references }
---

## Summary

Ticket: STD-665. Base checklist over the FR-408 amendment that registers the statistical worker
count `statistical.workers` (QSL FR-191 "Workers", FR-255 "Worker count setting names"), the FR-331
request member, the FR-461 Behavior 5 note and TC-356 vectors WK-01 to WK-04. Checked against QSL
FR-191 and FR-255 on quire-spec-language main. No statement contradicts QSL: the merge order, the
charging of budgets and stopping rules in merge order, the discard of samples beyond the stop,
result equality for every count, cancellation as a one-worker prefix, refusal of 0 with
`invalid_runtime_input`/`invalid-value`, default 1, and "no limit row, not in identity or
provenance" all agree. Four gaps remain.

## Verdict

FAIL on four medium findings; no high finding.

Examined and clean:
- FR-408 Inputs now list `workers`; the FR-331 `request` member list adds `workers` (default 1)
  next to `max_witnesses`, matching FR-408.
- Result equality: FR-408 adds certificates, findings and verdict to QSL's list (decision,
  counts, estimates, witnesses, provenance, stop); these are QSpec-level outputs derived from the
  same run, so the addition is consistent with QSL, not a contradiction.
- Cancellation bullet agrees with QSL FR-191 (result at some trace index, every sample before it
  counted, none after; index may vary between runs).
- QSL's "every sampler and subject SHALL support workers above 1" is not copied; QSpec only
  permits concurrency (MAY) and requires equal results, which a serial provider satisfies. No
  contradiction.
- TC-356 WK-01 to WK-04 map to FR-408-AC-15 to AC-17 and use fixtures already defined in TC-355
  and TC-356.
- `cargo test --locked --test acceptance_criterion_ids` passes on a5e9aa28 (2 passed).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-331 now says a statistical request carries `workers`, but the wire schema FR-331's Field sources name for `request` (`proposals/backend-provider-v1/schema.json` `Probabilistic`, `additionalProperties: false`) has no `workers` member, so a request that carries it fails the schema; the schema is not amended and no follow-up is recorded | spec/objects/interfaces/FR-331-backend-provider-envelope.md:134; proposals/backend-provider-v1/schema.json:540-561 |
| FND-002 | medium | FR-408 "Workers" does not bound look-ahead: QSL FR-191 says at most `workers` samples are drawn and not merged, so a stop has drawn at most `workers - 1` beyond it; QSpec charges `max_draws` in merge order and discards samples beyond the stop with no bound, so for a non-QSL FR-331 provider no budget bounds physical draws (a provider may draw arbitrarily far ahead and still conform). Not a contradiction (QSL is stricter) but a gap in the cost guarantee budgets give | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:316-323 |
| FND-003 | medium | The form of the `workers` 0 refusal is unstated: FR-408's other request refusals settle `invalid-request` at negotiation; this one says "refuse ... no engine runs for that item" with no disposition, stage or outcome, so one provider can return a negotiation disposition and another a refused result, and FR-408-AC-16 accepts both | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:314-315 |
| FND-004 | medium | FR-461 Behavior 5's new sentence registers `explore.workers` and `model_check.workers`, which no QSpec requirement owns, cites FR-408 "Workers" (statistical only) for all three, relies on "the settings operation", which QSpec does not define, leaves Behavior 3/4 (every entry point; unknown setting names refuse) silent on whether worker names are accepted, e.g. by `--limit`, and no FR-461 criterion covers it | spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md:87 |
