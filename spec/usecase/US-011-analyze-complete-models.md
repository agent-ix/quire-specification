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
