---
id: ADR-0104
title: "Native CLI exit status contract"
type: ADR
status: proposed
relationships:
  - target: ix://agent-ix/quire-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/quire-spec-language/FR-010
    type: depends_on
---
# ADR-0104: Native CLI exit status contract

## Status

**Open question for the architect.** One command surface has two exit-status
contracts.

## Context

[FR-301](../functional/tooling/FR-301-expose-complete-cli.md) fixes the exit
codes at `0` completed without violation, `10` logical violation, `20` invalid
or refused input, `21` unsupported, `22` incomplete and `30` tool failure.

`ix://agent-ix/quire-spec-language/FR-010` fixes them at `0` successful syntax,
`1` refusal, `2` usage or I/O failure and `3` incomplete resource exhaustion,
and asserts that mapping in five of its acceptance criteria.

[FR-300](../functional/tooling/FR-300-expose-rust-lifecycle-api.md) requires an
equivalent API request and CLI request to produce the same semantic
disposition, so a script reading a status byte reads a different disposition
depending on which requirement the build satisfied.

## Question

Which exit status contract does the native CLI carry, and which requirement
states it for both repositories?

## Consequences

Until the question is answered, the refusal and incomplete dispositions have
two status bytes each, and no build can satisfy both requirements.
