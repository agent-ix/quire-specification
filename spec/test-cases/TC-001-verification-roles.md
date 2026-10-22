---
id: TC-001
title: "Represent all six verification artifact roles"
type: TC
relationships:
  - target: "ix://agent-ix/quire-specification/FR-017"
    type: verifies
---
# TC-001: Represent all six verification artifact roles

## Description

Accepted portable contract control for FR-017-AC-1 using six separately
identified artifacts with actual bytes and exact artifact references. This artifact does not claim executed portfolio coverage.

## Test Procedure

For each of trace, fault-model, review-procedure, oracle, observation and
review-disposition, validate the 2-draft envelope/3-draft reference against the
new closed schema and recompute its byte digest. Submit each to the consumer's
same-role selection. Use all six inputs; an omitted role fails setup.

## Expected Results

All six shapes validate and each exact role matches. Schema validity and byte
verification are recorded separately from the planned B consumer observation.
No role proves reviewer authentication, oracle independence or logical truth.
