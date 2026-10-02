---
id: FR-161
title: "Analyze native infinite-trace temporal properties"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-002
    type: references
  - target: ix://agent-ix/quire-specification/AD-009
    type: references
  - target: ix://agent-ix/quire-specification/AD-012
    type: references
  - target: ix://agent-ix/quire-specification/FR-090
    type: references
  - target: ix://agent-ix/quire-specification/FR-251
    type: references
  - target: ix://agent-ix/quire-specification/FR-360
    type: references
  - target: ix://agent-ix/quire-specification/FR-114
    type: references
  - target: ix://agent-ix/quire-specification/FR-361
    type: references
  - target: ix://agent-ix/quire-specification/FR-362
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
  - target: ix://agent-ix/quire-specification/FR-367
    type: references
  - target: ix://agent-ix/quire-specification/FR-368
    type: references
---

# FR-161: Analyze native infinite-trace temporal properties

## Description

When an infinite-trace temporal property is requested, the analyzer SHALL apply
the selected unbounded semantic profile and report only conclusions justified
by its declared soundness premises.

## Inputs

Checked infinite-trace formula, its model subject
([FR-361](FR-361-read-model-behaviours-as-temporal-traces.md)) or supplied
trace, its resolved fairness set
([FR-362](FR-362-apply-fairness-constraints.md)), provider capability/profile
and caller-configurable limits.

## Outputs

Proved, refuted, inconclusive, unsupported or failed result with its premises
and its witness or counterexample.

## Behavior

Unbounded always/eventually/until/release and past duals have infinite-trace
meaning independent of one solver encoding. A valid finite counterexample may
refute an applicable property. Finite prefixes, absence of a counterexample or
resource exhaustion cannot prove infinite satisfaction.

The unbounded operator forms this requirement analyzes are admitted only under
the `quire.temporal.infinite-trace/v1` profile ([FR-250](../../objects/temporal/FR-250-temporal-semantic-profile.md))
through the unbounded temporal operator grammar edition
([FR-048](../foundation/FR-048-bind-native-temporal-syntax.md),
[AD-012](../../assurance/AD-012-unbounded-temporal-operator-grammar.md)). The
same profile admits interval operators nested with unbounded ones
([FR-367](FR-367-evaluate-interval-operators-under-infinite-trace.md)). An
infinite-trace item classifies `unbounded` and is negotiated under FR-290's
candidate and advertised-mode tables
([FR-368](FR-368-negotiate-temporal-model-check-items.md)): with no candidate
for `temporal-satisfaction` it settles `unsupported` with a warning naming that
capability kind, and with a single candidate that advertises only `bounded`
for it, `unsupported` with `unbounded-extent`. Over a model subject, the
verdict is the every-behaviour verdict of
[FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md).

## Result disposition mapping

Each of the five Outputs labels is exactly one composition of the existing
assessment-execution, truth and settlement-basis axes; this requirement
introduces no new result axis. [FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md)
records the closed vocabulary and the exact FR-241/FR-242/FR-243 combination
each label requires. `inconclusive` is the infinite-trace projection of the
FR-243 `unsettled` basis restated by [FR-094-AC-7](FR-094-interpret-progress-history-and-closure.md).

## Infinite-trace semantics

For an infinite state trace `s` and position `i`, atom truth is read from
`s[i]`; `X p` means `p` at `i+1`; `G p` means `p` at every `j >= i`; `F p`
means `p` at some `j >= i`; and `p U q` means that some `j >= i` satisfies `q`
while every `k` from `i` through `j-1` satisfies `p`. Release is the dual
`p R q = not(not p U not q)`. Past operators use the finite prefix through `i`:
`H`, `O` and `S` are respectively historically, once and since, and triggered
is the dual `p T q = not(not p S not q)`. Previous, `Y p`, is the interval
operator `once[1,1] p`, false at position zero. Fairness restricts the
admitted infinite traces before formula evaluation: weak and strong fairness
of operations, each at `whole` or `each` granularity, as
[FR-362](FR-362-apply-fairness-constraints.md) defines them. A fairness
constraint over a trace that carries no enabledness is a missing fairness
premise and settles `unsupported`.

A proof establishes the formula for every admitted infinite trace. A
counterexample is a finite prefix plus a closed loop (lasso) or another
provider-declared complete witness whose replay denotes an admitted infinite
trace. A bare finite prefix can refute a safety obligation once its bad prefix
is complete, but it cannot establish liveness satisfaction.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-161-AC-1 | A provider proves or refutes only under the exact transition/fairness premises and returns replayable evidence. | Test (TC-200) |
| FR-161-AC-2 | A finite-prefix monitor returns pending/inconclusive rather than proved for an unresolved liveness property. | Test (TC-200) |
| FR-161-AC-3 | Unsupported operators, a missing fairness premise or exhausted analysis return distinct non-conclusive dispositions. | Test (TC-200, TC-292) |
| FR-161-AC-4 | Always, eventually, until, release and each past operator satisfy their inductive infinite-trace definitions on conformance lassos. | Test (TC-200) |
| FR-161-AC-5 | Fairness filters traces before evaluation and is included in every proof or counterexample replay identity. | Test (TC-200) |
| FR-161-AC-6 | Each returned result label matches exactly one FR-360 member and its exact FR-241/FR-242/FR-243 combination. | Test (TC-200) |
| FR-161-AC-7 | An infinite-trace item with no candidate for `temporal-satisfaction` settles `unsupported` with a warning naming that capability kind, and one whose single candidate advertises only `bounded` for it settles `unsupported` with `unbounded-extent`. | Test (TC-200, TC-301) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [FR-048](../foundation/FR-048-bind-native-temporal-syntax.md) and
  [AD-012](../../assurance/AD-012-unbounded-temporal-operator-grammar.md) admit
  the unbounded temporal operator grammar edition this requirement analyzes.
- [FR-090](FR-090-select-temporal-profile-and-clock.md) and
  [FR-250](../../objects/temporal/FR-250-temporal-semantic-profile.md) select
  the `quire.temporal.infinite-trace/v1` profile this requirement requires.
- [FR-251](../../objects/temporal/FR-251-tl-target-profile.md) records that an
  infinite-trace request resolves to an absent TL target rather than lowering.
- [FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md)
  records the closed result-disposition vocabulary this requirement's Outputs
  use.
- [FR-114](../observation/FR-114-assess-replay-and-incremental-consistently.md)
  states the QObs/temporal non-conclusive agreement this requirement's
  `inconclusive` result participates in.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
