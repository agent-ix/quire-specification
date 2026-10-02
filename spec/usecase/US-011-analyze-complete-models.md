---
id: US-011
title: "Analyze complete models with interchangeable exact methods"
type: US
relationships:
  - target: ix://agent-ix/quire-specification/StR-002
    type: traces_to
---

# US-011: Analyze complete models with interchangeable exact methods

## Story

**As a** verification engineer
**I want** runtime, testing, simulation, proof and analysis methods to consume the same checked package
**So that** their conclusions and counterexamples are comparable and replayable.

## Context

Method limitations must remain distinct from language meaning, and every result
must carry the exact domain, assumptions, provider and replay lineage.

The same checked package also states probabilistic and real-time intent: how
likely a failure is under a declared workload, how often a latency stays under
a deadline, whether a timed protocol meets its deadlines, and whether a task
set is schedulable. The engineer checks these claims exactly where the model
admits it, or measures them by sampling where it does not, and a measurement is
reported as a measurement, never as an exact conclusion.

## Acceptance Examples (Illustrative)

### US-011-EX-1: Counterexample replay

- **Given** a proof backend reports a bounded counterexample
- **When** the same typed input is replayed through the native runtime
- **Then** both methods produce the same clause verdict or an explicit parity failure

### US-011-EX-2: Temporal property over every behaviour

- **Given** a finite model subject and a liveness claim with a weak fairness
  constraint
- **When** an explicit-state analysis checks the claim
- **Then** it returns `proved` over every fair behaviour, or `refuted` with a
  lasso counterexample that replays without the engine, or a non-conclusive
  verdict that names its depth, cause or limit

### US-011-EX-3: Probabilistic bound, exact and measured

- **Given** a model with random parameters, a workload and a claim that a fault
  occurs within 1,000 steps with probability at most 0.001
- **When** the claim is checked with exact evidence and, separately, with
  statistical evidence at stated confidence parameters
- **Then** the exact check settles `proved` with a certificate, and the
  statistical check reports `measured: accepted` with its error bounds, and
  neither is read as the other

### US-011-EX-4: Real-time deadline

- **Given** a timed model whose reply must follow a request within 3 ms, and a
  task set with worst-case execution times
- **When** the deadline claim and the schedulability claim are checked
- **Then** each settles with exact timed evidence that replays, or with the
  named reason it cannot be decided
