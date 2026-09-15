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
