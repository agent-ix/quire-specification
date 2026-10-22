---
id: US-006
title: "Assess distributed protocols without losing uncertainty"
type: US
relationships:
  - target: ix://agent-ix/quire-specification/StR-002
    type: traces_to
---
# US-006: Assess distributed protocols without losing uncertainty

## Story

**As an** assurance engineer assessing a distributed workflow
**I want** protocol, compensation and conformance outcomes to preserve what ran, what was observed and what remains unknown
**So that** I can distinguish demonstrated recovery from a green result caused by inactivity, missing data or an unsupported analysis.

## Context

An order can cross internal services and an external payment provider, split into
multiple shipments, retry a payment or refund, and recover only part of an
earlier effect. Transport delivery, business effect, workflow correlation and
complete observation are separate facts. The engineer needs one result contract
that keeps those distinctions visible across replay and later monitoring.

## Acceptance Examples (Illustrative)

### US-006-EX-1: Triggered compensation is demonstrated

- **Given** a payment effect, a failed shipment, a registered refund obligation and complete observations through closure
- **When** the engineer assesses the workflow
- **Then** the result identifies the trigger, participating roles, refund effect, recovery relation and completed conformance outcome

### US-006-EX-2: Missing provider evidence stays incomplete

- **Given** a triggered refund obligation whose external-provider response is not available and whose completeness authority reports an open history
- **When** the engineer assesses the workflow
- **Then** the result remains pending or incomplete and does not report the refund or recovery as demonstrated

### US-006-EX-3: Unsupported projection does not erase replay

- **Given** a finite protocol eligible for global replay but outside the selected local-projection fragment
- **When** the engineer requests both capabilities
- **Then** the global result remains available and the projection result states its unsupported or refused disposition

## Options (Exploratory)

The conformance engine may use a qualified protocol library or an owned bounded
algorithm. A later projection engine may consume the same representation. The
choice remains open.

## Constraints (Contextual)

Native Quire remains the single editable source language. The evaluator observes
recorded behavior and does not orchestrate messages or compensation. Existing
Quoin and Engineering Assurance interfaces retain evidence and execution
ownership instead of being recreated in the protocol component.

## Dependencies (Contextual)

This story consumes the shared composed-language identities and predicates,
model/configuration relationships, temporal clock/closure rules and observation
bindings. It informs PT01/PT02 and the result contract consumed by OB03.

## Priority and Risk (Informative)

Priority is high for v1. The central risk is a confident success result produced
from an inactive branch, incomplete external observations or identity collapse.

## Notes (Informative)

General realizability, projection, refinement and assume-guarantee composition
remain separately requested claims. Their absence does not remove the value of
bounded global conformance.

## Traceability (Informative)

The normative behaviors are refined by FR-050 through FR-061 and constrained by
NFR-020 and NFR-021 in this protocol family.
