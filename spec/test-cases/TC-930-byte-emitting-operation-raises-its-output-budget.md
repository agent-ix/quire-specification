---
id: TC-930
title: "A byte-emitting operation raises its output-byte budget"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-461
    type: verifies
---

# TC-930: A byte-emitting operation raises its output-byte budget

## Description

Exercise the caller-selected output-byte budget at the formatter boundary,
including the setting name, exact boundary and absence of partial output.

## Test Procedure

1. Give the byte-emitting operation an admitted input whose canonical output
   contains formatting whitespace, a comment and a terminating newline.
2. Run it with bounds from zero through one byte below, exactly at and above
   the emitted byte length.
3. Repeat the below-bound and exact-bound cases through the typed limits field
   and through the CLI settings operation using `format.output_bytes`.

## Expected Results

Every bound below the emitted length returns the operation's typed byte-limit
outcome naming `format.output_bytes` and returns no partial output. The exact
bound succeeds and includes the terminating newline. The typed limits field and
the CLI operand select the same bound and produce the same bytes.

## Status

Planned. This case is the QSpec contract for QSL-673's concrete formatter row.
