---
id: TC-938
title: "Operation bodies are unique by resolved operation identity"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-151
    type: verifies
---
# TC-938: Operation bodies are unique by resolved operation identity

## Description

Verify that one body per operation identity succeeds, while a second body
resolving to the same identity is refused regardless of its local label.
Scope: FR-151-AC-16.

## Test Procedure

1. Use TC-196's valid model fixture G with operations `A/size` and `B/size`,
   where `B/size` redefines `A/size`, and its unit fixture S2 with one body
   targeting each operation identity.
2. Compile S2 as the single-body-per-identity control.
3. Add `body secondSizeA using V on M::A::size() { 3 }` to S2, retaining its
   existing `sizeA` body and all other inputs, then compile again.

## Expected Results

The control compiles and links the two distinct operation identities. The
second compile refuses with
`conflicting_declaration`/`duplicate-declaration` for the two bodies
resolving to `A/size`; changing their declaration order or local labels does
not change the refusal. No dispatch table is produced for the refused link.
