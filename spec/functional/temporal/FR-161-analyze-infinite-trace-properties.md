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
  - target: ix://agent-ix/quire-specification/FR-341
    type: references
  - target: ix://agent-ix/quire-specification/FR-114
    type: references
---

# FR-161: Analyze native infinite-trace temporal properties

## Description

When an infinite-trace temporal property is requested, the analyzer SHALL apply
the selected unbounded semantic profile and report only conclusions justified
by its declared soundness premises.

## Inputs

Checked unbounded formula, transition/model subject, fairness assumptions,
provider capability/profile, limits and exact tool pins.

## Outputs

Proved, refuted, inconclusive, unsupported or failed result with premises,
witness/counterexample and provenance.

## Behavior

Unbounded always/eventually/until/release and past duals have infinite-trace
meaning independent of one solver encoding. A valid finite counterexample may
refute an applicable property. Finite prefixes, absence of a counterexample or
resource exhaustion cannot prove infinite satisfaction.

The unbounded operator forms this requirement analyzes are admitted only under
the `quire.temporal.infinite-trace/v1` profile ([FR-250](../../objects/temporal/FR-250-temporal-semantic-profile.md))
through the unbounded temporal operator grammar edition
([FR-048](../foundation/FR-048-bind-native-temporal-syntax.md),
[AD-012](../../assurance/AD-012-unbounded-temporal-operator-grammar.md)). Until
a liveness backend registers the infinite-trace capability, an unbounded
formula settles `unsupported` with a warning naming the required capability.

## Result disposition mapping

Each of the five Outputs labels is exactly one composition of the existing
assessment-execution, truth and settlement-basis axes; this requirement
introduces no new result axis. [FR-341](../../objects/temporal/FR-341-infinite-trace-result-disposition.md)
records the closed vocabulary and the exact FR-241/FR-242/FR-243 combination
each label requires. `inconclusive` is the infinite-trace projection of the
FR-243 `unsettled` basis restated by [FR-094-AC-7](FR-094-interpret-progress-history-and-closure.md).

## Infinite-trace semantics

For an infinite state trace `s` and position `i`, atom truth is read from
`s[i]`; `X p` means `p` at `i+1`; `G p` means `p` at every `j >= i`; `F p`
means `p` at some `j >= i`; and `p U q` means that some `j >= i` satisfies `q`
while every `k` from `i` through `j-1` satisfies `p`. Release is the dual
`p R q = not(not p U not q)`. Past operators use the finite prefix through `i`:
`Y`, `H`, `O` and `S` are respectively previous, historically, once and since,
with `Y p` false at position zero. Fairness restricts the admitted infinite
traces before formula evaluation.

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
| FR-161-AC-3 | Unsupported operators, missing fairness/model closure or exhausted analysis return distinct non-conclusive dispositions. | Test (TC-200) |
| FR-161-AC-4 | Always, eventually, until, release and each past operator satisfy their inductive infinite-trace definitions on conformance lassos. | Test (TC-200) |
| FR-161-AC-5 | Fairness filters traces before evaluation and is included in every proof or counterexample replay identity. | Test (TC-200) |
| FR-161-AC-6 | Each returned result label matches exactly one FR-341 member and its exact FR-241/FR-242/FR-243 combination. | Test (TC-200) |
| FR-161-AC-7 | Until a liveness backend registers the infinite-trace capability, an unbounded formula settles `unsupported` with a warning naming the required capability. | Test (TC-200) |

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
- [FR-341](../../objects/temporal/FR-341-infinite-trace-result-disposition.md)
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
