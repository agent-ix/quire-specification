---
id: TC-180
title: "Complete V1 profile bundle accounting"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-131
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-339
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-370
    type: verifies
---

# TC-180: Complete V1 profile bundle accounting

## Description

Link the full capability bundle, remove one required facet, and repeat with changed backend support.

## Test Procedure

Use the subject and inputs named by FR-131. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

Compile one complete-V1 source with an empty provider registry and again with
a registry holding a compile-time provider and a process plugin, and compare
the two checked packages and check outcomes (FR-339-AC-3).

Compile `temporal T using inf clock "c" on origin { always holds(true) }`, a temporal clause with no `over` parameter (FR-370-AC-13).

Compile each of FR-339-AC-5's `1-draft` units for each of `helper`, `rec`, `cast` and `Tuple`, and evaluate `w(1)` for each function so named; compile `record tuple { a: Integer; }`; and parse a `0-draft` clause using each of the four spellings (FR-339-AC-5).

## Expected Results

Every positive and boundary result matches FR-131; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
The two registry runs give byte-equal checked packages and the same check
outcome.
The clause with no `over` parameter refuses `invalid_syntax`/`unexpected-token` at the `clock` token and produces no node.
Each `1-draft` unit checks with no `invalid_syntax` and no `unsupported_construct`, and each `w(1)` is `1`; `record tuple` refuses `invalid_syntax`/`unexpected-token` at `tuple`; each `0-draft` use keeps its `unsupported_construct` refusal at its introducing token.
