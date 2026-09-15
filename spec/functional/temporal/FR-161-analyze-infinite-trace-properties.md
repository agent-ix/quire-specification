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

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
