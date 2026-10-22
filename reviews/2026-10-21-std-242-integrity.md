---
id: SR-14689
title: "integrity review of STD-242 admitted-key wording, excluded keys and AC-15 coverage (QSpec PR #346)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@56e00d4072ccec96deab7afff264450648bed9ab; spec/functional/analysis/FR-395-analyze-hyperproperties-over-behaviours.md, spec/functional/analysis/FR-399-settle-hyperproperty-results.md, spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md, spec/functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md, spec/test-cases/TC-290-model-behaviours-as-temporal-traces.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-395
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-399
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-361
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-363
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-290
    type: reviews
---
# SR-14689: integrity review of STD-242 (QSpec PR #346)

## Summary

Ticket: STD-242 (SR-13653 FND-021 to FND-023). Diff reviewed:
`git diff origin/main...56e00d4` (5 files).

- FND-021 (admitted-key wording). FR-395 "Object parameters", FR-395-AC-2
  and FR-399 now say "admitted key (FR-361)".
- FND-022 (creation limitation). FR-361 defines the item's excluded keys, and
  FR-363 "Scope" restates the rule. No wire member is added. The verdict
  carries its subject (FR-363-AC-7), so the keys can be derived from it, as
  the PR states. This is the lead default; the alternative is a wire warning
  or member.
- FND-023 (AC-15 coverage). TC-290 step 9 adds `make` (`creates Counter`)
  creating `z` at value 1. Step 9 starts from step 5's subject, which has
  no `drop`, with `inc` disabled. `c` and `d` stay at 0, and `z` is the only
  free key of `counters`, so the creation is determined and
  `always holds(x.value = 0)` is `proved` over `c` and `d`. The vector is
  buildable and its expected result follows.

Gates: `quire validate` reports 134 failing documents (main 134), and the
five touched files have no diagnostics. `cargo test --test checked_package_v2
--test backend_provider_v1` passes, 117 + 14. No other open QSpec PR touches
FR-395, FR-399, FR-363 or TC-290. PR #340 edits FR-361 "Model subject", and
`git merge-tree` of the two heads is clean.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The edited FR-395 "Object parameters" paragraph still says that a two-alias clause's "instances range over the product of their universes". That contradicts the new first sentence, and FR-361, which give one instance per admitted key: a universe key with no initial member record would enter the product. This leftover is the kind of wording FND-021 asked to fix. Say "the product of their admitted keys". | spec/functional/analysis/FR-395-analyze-hyperproperties-over-behaviours.md:70-71 |
| FND-002 | low | FR-363 "Scope" says "a universe key that no member record of an initial state admits ... is an excluded key of the item". It omits FR-361's restriction to the populations that overlap the parameter's type, so a key of an unrelated population reads as excluded. Add "of a population overlapping the parameter's type". | spec/functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md:184-189 |
| FND-003 | low | FR-361 defines the excluded keys as "the universe keys of the populations that overlap `T` that no member record of any initial state admits, in instance order". It does not say whether a key in two overlapping universes is listed once, as an instance is ("a key listed by several of them being one instance"), or once per population. Say that it is listed once. | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:150-152 |

## Verdict

Faithful to the ticket's three items, and step 9 is buildable. One
contradicting leftover remains in the FR-395 paragraph this PR edits
(FND-001). FND-002 and FND-003 are wording fixes.

## Dispositions

Round 1, reviewed at c76569d355a84c98005f0a07f720651303570fea (contains
origin/main). Gates: `quire validate --scope . spec/**/*.md` reports 134
failing documents (baseline 134), with no diagnostics on FR-395, FR-399,
FR-361, FR-363, TC-290 or TC-337. `cargo test --test checked_package_v2
--test backend_provider_v1` passes, 117 + 14. No other occurrence of
"product of their universes" remains in spec/ or proposals/.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | c76569d |
| FND-002 | fixed | c76569d |
| FND-003 | fixed | c76569d |

After excerpts:

- FND-001 (FR-395 "Object parameters"): "A clause over two aliases names one
  parameter per alias it reads, and its instances range over the product of
  their admitted keys."
- FND-002 (FR-363 "Scope"): "Over an `over` parameter it holds for the item's
  instances only: a universe key of a population overlapping the parameter's
  type that no member record of an initial state admits, including one only an
  operation could create, is an excluded key of the item"
- FND-003 (FR-361): "The item's **excluded keys** are the universe keys of the
  populations that overlap `T` that no member record of any initial state
  admits, in instance order, a key in several overlapping universes being
  listed once."
