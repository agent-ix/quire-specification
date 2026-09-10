---
id: US-009
title: "Assess observed workflow outcomes without inventing evidence"
type: US
relationships:
  - target: ix://agent-ix/quire-specification/StR-002
    type: traces_to
---
## Story

**As a** workflow assessor
**I want** to assess related workflow instances from explicitly bound observations and progress
**So that** a result distinguishes success, violation, pending, incompleteness, and refusal without treating telemetry guesses as business facts.

## Context

The common O1/O2 example contains an external payment provider, split shipments,
payment attempts/effects, and refunds. A transport receipt, trace ID, timestamp,
or EOF is not by itself a business relationship, causal ordering, complete history,
or Boolean valuation. This story drives OB01–03 and consumes the other owners'
contracts rather than replacing them.

## Acceptance Examples (Illustrative)

### US-009-EX-1

Given a refund explicitly linked to O1's payment effect, the assessor can use it
for O1 and cannot silently use it for concurrent O2.

### US-009-EX-2

Given a quiet stream and an authorized progress watermark, the assessor can settle
an eligible deadline; without that authority it reports pending or incomplete.

### US-009-EX-3

Given the same admitted observations and progress, batch replay and incremental
assessment expose the same settled disposition.

