---
id: US-012
title: "Drive authorized workflows from formal decisions"
type: US
relationships:
  - target: ix://agent-ix/quire-specification/StR-002
    type: traces_to
---

# US-012: Drive authorized workflows from formal decisions

## Story

**As an** application owner
**I want** typed formal decisions to feed explicitly authorized idempotent adapters
**So that** specifications can guide workflows without evaluators silently causing effects.

## Context

The adapter boundary preserves authorization, decision identity, external
receipts and observation feedback while all semantic evaluators remain pure.

## Acceptance Examples (Illustrative)

### US-012-EX-1: Idempotent authorized effect

- **Given** one authorized decision is delivered twice
- **When** the adapter processes both deliveries
- **Then** it requests at most one external effect and relates both deliveries to one receipt
