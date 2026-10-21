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

## Expected Results

Every positive and boundary result matches FR-131; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
The two registry runs give byte-equal checked packages and the same check
outcome.
The clause with no `over` parameter refuses `invalid_syntax`/`unexpected-token` at the `clock` token and produces no node.
