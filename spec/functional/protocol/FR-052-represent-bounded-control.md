---
id: FR-052
title: "Represent bounded causal protocol control"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-006
    type: implements
  - target: ix://agent-ix/quire-specification/FR-050
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-220
    type: references
  - target: ix://agent-ix/quire-specification/FR-221
    type: references
  - target: ix://agent-ix/quire-specification/FR-227
    type: references
  - target: ix://agent-ix/quire-specification/FR-228
    type: references
---
## Description

When linking a protocol body, the protocol linker SHALL construct a finite causal graph for authored sequence, labeled choice, parallel branches, join, bounded iteration and termination without deriving causal edges from source order or timestamps.

## Inputs

Typed protocol nodes, authored control edges, finite iteration maxima, branch
join rules, termination declarations, and authored control-to-temporal activation
relations.

## Outputs

An acyclic finite control representation with explicit branch and progress edges
plus a versioned declaration-local control-to-temporal activation mapping, or a
located typed refusal.

## Behavior

Sequence SHALL add only its authored order. Parallel branches SHALL admit every
interleaving that preserves each branch's causal edges. A join SHALL name the
branches and completion rule it awaits. Iteration SHALL carry an authored finite
maximum and progress edge. The linker SHALL refuse missing joins, cyclic causal
dependencies, unbounded repetition and termination conditions that cannot be
reached within the declared finite graph.

When a control can establish an event-triggered protocol obligation, the
protocol linker SHALL retain its one authored temporal-activation relation in
the linked graph. The relation SHALL name the exact declaration-local control
and the exact temporal declaration whose checked result it consumes. The
protocol linker SHALL refuse an absent, duplicate, foreign, or non-event-
triggered relation; a source-table coordinate, matching profile, clock, display
name, or runtime observation SHALL NOT substitute for the authored relation.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-052-AC-1 | Split shipments S1 and S2 may interleave while each branch preserves its own order and both satisfy the authored join before completion. | Test (TC-052) |
| FR-052-AC-2 | Reordering equal-time events does not add a causal edge absent from the protocol. | Test (TC-052) |
| FR-052-AC-3 | A missing join input, causal cycle, zero-progress loop or absent finite iteration maximum refuses admission. | Test (TC-052) |
| FR-052-AC-4 | Changing any admitted semantic graph content—including a node/control kind, edge, guard or constraint value graph and dependency, event kind or binding, channel/FIFO rule, await rule, compensation reference, finish rule, branch, join or iteration maximum—requires a different qualified protocol-definition identity; source loci remain provenance rather than graph-key members. | Test (TC-052) |
| FR-052-AC-5 | Reusing one qualified protocol-definition identity with different complete typed graph content is an identity contradiction and refuses admission rather than selecting either graph. | Test (TC-052) |
| FR-052-AC-6 | Each event-triggered control has exactly one linked temporal-activation relation naming one in-graph temporal declaration; an absent, duplicate, foreign or inferred relation refuses admission. | Test (TC-052) |

## Dependencies

- [FR-050](FR-050-bind-protocol-instances.md).
- Agent A's shared grammar and predicate contracts.
