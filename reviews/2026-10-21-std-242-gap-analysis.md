---
id: SR-14691
title: "gap analysis of STD-242 changed ACs against TC rows (QSpec PR #346)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@56e00d4072ccec96deab7afff264450648bed9ab; spec/functional/analysis/FR-395-analyze-hyperproperties-over-behaviours.md, spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md, spec/test-cases/TC-290-model-behaviours-as-temporal-traces.md, spec/test-cases/TC-337-hyperproperty-syntax-and-binding.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-361
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-395
    type: reviews
---
# SR-14691: gap analysis of STD-242 (QSpec PR #346)

## Summary

Ticket: STD-242. Plan completion: not assessed. This is a manual mapping of
each changed AC to its TC rows. The PR is spec only. The TC-290 step 9
harness vector is listed as remaining work for the code lane.

- FR-361-AC-15 (changed) maps to TC-290 steps 7 and 9 and their expected
  results 7 and 9. Step 9 covers the creating operation, the excluded keys
  [`z`] and the `proved` verdict over `c` and `d`.
- FR-395-AC-2 (changed) keeps its TC-337 and TC-338 rows.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-395-AC-2 now counts instances per admitted key, but its vector TC-337 HB-02 still pairs the mutation "universes of 2 and 3 keys" with the expected result "admitted; 6 instances over the product of universes". It does not say that every key has an initial member record, so its expected count follows the old per-universe-key rule. State that each universe key is admitted by an initial member record, and say "6 instances over the product of the admitted keys". A vector with one unadmitted key (2 × 2 = 4 instances) would exercise the change. | spec/test-cases/TC-337-hyperproperty-syntax-and-binding.md:29 |

## Verdict

Every changed AC has a TC row. The HB-02 vector of FR-395-AC-2 is not
updated for the admitted-key rule (FND-001).

## Resolution

FND-001: TC-337 HB-02 states that each key is admitted by an initial member record and expects 6 instances over the product of the admitted keys; new vector HB-13 has one unadmitted key per universe and expects 4 instances.
