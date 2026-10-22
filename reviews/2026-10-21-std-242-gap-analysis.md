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

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | high | The new vector HB-13 takes HB-02 (universes of 2 and 3 keys) with "one key of each universe admitted by no member record", which leaves 1 and 2 admitted keys, so the product has 1 × 2 = 2 instances. Its expected result says "2 × 2 = 4 instances", which no correct implementation produces. Either expect 2 instances (1 × 2), or make only one key of the 3-key universe unadmitted, which gives 2 × 2 = 4 with one excluded key. | spec/test-cases/TC-337-hyperproperty-syntax-and-binding.md:40 |
| FND-003 | low | HB-13 expects "the two unadmitted keys are excluded keys of the item", but FR-361 defines excluded keys for one parameter `over (x: T)` and FR-395 does not say how a two-parameter clause's excluded keys combine (one list per parameter, or one list in which order). State the expected excluded keys per parameter (`d`: one key, `e`: one key), or state the combination rule in FR-395. | spec/test-cases/TC-337-hyperproperty-syntax-and-binding.md:40 |

## Dispositions

Round 1, reviewed at c76569d355a84c98005f0a07f720651303570fea (contains
origin/main). Gates as recorded in SR-14689 round 1: 134 failing documents
(baseline 134), no diagnostics on TC-337; `checked_package_v2` and
`backend_provider_v1` pass, 117 + 14. The instance counts were recomputed by
an independent script: HB-02 gives 2 × 3 = 6; HB-13 as written gives
1 × 2 = 2 (FND-002).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | c76569d |

After excerpts:

- FND-001 (TC-337 HB-02): "a clause over aliases `S` and `N` with parameters
  `(d: S::Device, e: N::Device)` and universes of 2 and 3 keys, each key
  admitted by a member record of an initial state | admitted; 6 instances over
  the product of the admitted keys"

## Verdict

Every changed AC has a TC row. The HB-02 vector of FR-395-AC-2 is not
updated for the admitted-key rule (FND-001).

Disposition pass 1: FND-001 is fixed. The new HB-13 vector has a wrong
expected instance count (FND-002, high) and an undefined excluded-key oracle
for a two-parameter clause (FND-003, low).
