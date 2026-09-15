---
id: FR-033
title: "Admit total typed reusable Boolean predicates"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-005
    type: implements
---
## Description

When linking a named predicate declaration or invocation, the checker SHALL admit it only when its body is a total Boolean expression over the declared parameter types and selected model semantics, with an acyclic predicate-call dependency graph.

## Inputs

Native predicate declarations with explicit parameter types, bodies parsed by the shared expression grammar, invocation arguments, exact selected profiles and model declarations.

## Outputs

Typed predicate identities and call references with source locations, or a located type, definedness, duplicate-declaration or cycle refusal.

## Behavior

Calls resolve by exact declaration identity and parameter order/type, including
declared units and optional-value obligations. There is no implicit numeric,
collection or truthiness coercion. Pure local bindings and calls do not read
ambient objects, clocks, I/O or mutable monitor status. The predicate-only facet
rejects recursion. The complete function facet admits recursion only when its
well-founded termination obligation is discharged; unresolved/ambiguous calls
and unguarded partial operations refuse. Static termination is not a runtime
resource bound, so selected execution budgets still apply. The historical
no-user-call profile is unchanged.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-033-AC-1 | An acyclic predicate called from two families resolves to the same checked declaration and preserves each call site. | Test (TC-033) |
| FR-033-AC-2 | Wrong arity, wrong type/unit, unresolved or ambiguous names and a non-Boolean root refuse before evaluation. | Test (TC-033) |
| FR-033-AC-3 | Predicate-only and historical profiles refuse direct or mutual recursion; the complete function facet admits a recursive call graph only with a checked well-founded decrease and reports every involved declaration/call on failure. | Test (TC-181) |
| FR-033-AC-4 | An unguarded optional or partial operation refuses definedness; a supported guarded total body is admitted without adding ambient access. | Test (TC-033) |
| FR-033-AC-5 | The historical first-profile no-call rule continues to refuse calls; reuse requires an explicitly selected extension. | Test (TC-033) |

## Dependencies

- [Shared drafting foundation](../../../proposals/quire-v1/shared-foundation.md).
- [Composed architecture](../../assurance/AD-001-composed-native-language.md).
- [Planned acceptance matrix](../../composed-foundation/tests.md).

Accepted by FS03 for the composed state query/graph closure. It does not amend
historical accepted definitions or establish implementation coverage.
