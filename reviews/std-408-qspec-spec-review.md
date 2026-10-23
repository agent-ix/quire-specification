---
id: SR-16276
title: "spec-review of temporal SMT solver budgets max_solver_steps and max_solver_time (quire-specification PR 411)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@799cc6d4; spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md; spec/functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md; spec/functional/analysis/FR-399-settle-hyperproperty-results.md; spec/objects/interfaces/FR-331-backend-provider-envelope.md; proposals/backend-provider-v1/schema.json; proposals/backend-provider-v1/vectors.json; spec/test-cases/TC-294-verdict-strength-and-stopped-runs.md; spec/test-cases/TC-302-temporal-request-subject-identity.md; spec/test-cases/TC-340-hyper-vacuity-and-stutter.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-368
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
  - target: ix://agent-ix/quire-specification/FR-399
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
---

# Spec review of temporal SMT solver budgets (quire-specification PR 411)

## Summary

Ticket: STD-408 (the QSpec half of QSL #1356). This is one spec-review pass
over `git diff origin/main...799cc6d4`. It covers:

- FR-368 "SMT solver budgets" and AC-11, and the request-identity
  exclusion of `max_solver_time`;
- FR-363 rule 3 and AC-11, the FR-399 table and AC-13, and the FR-331
  identity line;
- the `TemporalLimits` schema, the positive vector and the two adverse
  vectors;
- TC-294 step 10, TC-302 and TC-340 VS-08.

Checked and sound:

- The time budget sits outside the identity, follows the hybrid precedent
  in FR-331, and is never stored when it stops a run. It does not affect
  certificate rebuilds, because QSL FR-314 rebuilds only from the encoding
  limits.
- The stop order matches FR-363's existing rules 1 to 4.
- `max_solver_steps` is in the identity.

Ids FR-368-AC-11, FR-363-AC-11, FR-399-AC-13, VS-08 and both vector names
appear on no other open QSpec PR head.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Scope narrowing: the SMT encoding limits stay off the wire and out of the request identity. Lead default 4 keeps QSL's `smt_encoding.*` out of `TemporalLimits`. FR-363 rule 3 and FR-363-AC-11 still settle an encoder stop at `smt_encoding.max_constants` 7 as `limit-reached` naming that setting, but no request can carry 7, so the vector has no wire form. Raising such a limit is the remedy that V-7 promises, and a provider request cannot do it. That removes the capability STD-408 restores at the provider boundary (standing rule: narrowing is HIGH). The values also change the result (`limit-reached` against `proved`) yet are outside the request identity and cache key, against FR-368's own identity rule. Fix: add the encoding limits as `TemporalLimits` members (with `PositiveCount`, identity inclusion, defaults from the provider descriptor and vectors), matching QSL #1356. | spec/functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md:206-215, :244; proposals/backend-provider-v1/schema.json:315-332; spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md:166-171 | missing-requirement |
| FND-002 | low | FR-368-AC-11 and TC-302 say a `max_solver_time` of 0 is refused, but only `max_solver_steps` 0 has an adverse vector (`temporal-limits-solver-steps-zero`). Fix: add `temporal-limits-solver-time-zero`. | spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md:218; proposals/backend-provider-v1/vectors.json:4266-4289 | missing-requirement |
| FND-003 | low | A reused record can state a time budget that the reusing request never set. A completed result is reused across requests that differ only in `max_solver_time`, while QSL ADR-018 V-7 says every terminal record states the limit values used. Fix: state that a reused record's stated `max_solver_time` is the producing run's value, or that the stated values exclude wall-clock budgets. | spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md:167-171, :187-194 | missing-requirement |

## Verdict

**Not mergeable** until FND-001 is resolved together with QSL #1356 FND-003.
The solver-budget half is sound, and its identity split is correct.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | FR-368-AC-12 and TC-302 say a `max_constants` of 1.5 is refused before consumption, but no adverse vector carries it. The vectors are `temporal-smt-encoding-empty`, `-unknown-member` and `-zero`. Fix: add `temporal-smt-encoding-fractional`. | spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md (FR-368-AC-12); proposals/backend-provider-v1/vectors.json |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Fix commits 3b93dd53 and db088311 (head f3960d5d). `temporal.smt_encoding` is a closed `SmtEncodingLimits` object of `PositiveCount`s with `minProperties` 1. It is part of the request identity (FR-368, FR-331) and the cache key. FR-363 rule 3 and AC-11 now settle a stop at the carried member. New FR-368-AC-12 and adverse vectors. |
| FND-002 | fixed | New vector `temporal-limits-solver-time-zero`. |
| FND-003 | fixed | FR-368 states that a reused record gives the producing run's limit values, and AC-12 covers 5 reused for 10. |
| FND-004 | fixed | Round 2, fix commit c29f2da3. Vector `temporal-smt-encoding-fractional` (`max_constants` 1.5) is added and TC-302 cites it. `cargo test --test backend_provider_v1` at c29f2da3 gives 24 passed and 0 failed. |
