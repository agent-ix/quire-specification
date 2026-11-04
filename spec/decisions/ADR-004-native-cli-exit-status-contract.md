---
id: ADR-004
title: "Native CLI exit status contract"
type: ADR
status: accepted
relationships:
  - target: ix://agent-ix/quire-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/quire-spec-language/FR-010
    type: depends_on
---
# ADR-004: Native CLI exit status contract

## Status

**Accepted.**

## Decision

The native CLI carries the exit status contract that
[FR-301](../functional/tooling/FR-301-expose-complete-cli.md) states: `0`
completed without violation or still pending on declared progress, `10` logical violation, `11` indeterminate temporal
clause, `20` invalid or refused
input, `21` unsupported, `22` incomplete and `30` tool failure.

`ix://agent-ix/quire-spec-language/FR-010` states the same contract, and its
acceptance criteria assert these codes.

## Context

[FR-300](../functional/tooling/FR-300-expose-rust-lifecycle-api.md) requires an
equivalent API request and CLI request to produce the same semantic
disposition. A status byte therefore identifies one disposition across both
surfaces.

Six codes separate the dispositions a caller acts on differently: a logical
violation is a result about the model, invalid or refused input is a result
about the request, unsupported names a capability the build does not carry,
incomplete names a run that reached no verdict, and tool failure names a fault
in the tool itself. A script branching on the status byte reaches the right
branch without reading diagnostic text.

## Consequences

- `FR-301` is the requirement that states the contract; `FR-010` states it for
  the language repository and asserts it in its acceptance criteria.
- The exit-code corpus named in `V1-A06` covers all six codes on both the API
  and the CLI surface.
