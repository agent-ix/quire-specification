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

Verify that body uniqueness is enforced across source units after local model
aliases resolve, while distinct operation identities succeed together.
Scope: FR-151-AC-16.

## Test Procedure

1. Use TC-196's valid model fixture G with operations `A/size` and `B/size`,
   where `B/size` redefines `A/size`. Form one linked package from two
   complete-V1 source units. The first uses TC-195's profile selection under
   local alias `V`, model declaration `M` for G, and
   `body sizeA using V on M::A::size() { 1 }`, and retains S2's
   `invariant I using V on M::A at current { self.size() >= 1 }`. The second selects
   the same profile and exact same model identity/digest under local aliases
   `W` and `N`, and has `body sizeB using W on N::B::size() { 2 }`.
2. Compile those two units together as the single-body-per-operation-identity
   control.
3. Add `body secondSizeA using W on N::A::size() { 3 }` to the second unit,
   retaining the first unit's `sizeA` body and all other inputs, then compile
   the linked package again.

## Expected Results

The control compiles and links the bodies for the two distinct operation
identities, `A/size` and `B/size`; the invariant's dispatched call links as
in TC-196 D01. In the second compile, `M` and `N` resolve to the same model
identity, so the bodies in the distinct source units resolve to the same
`A/size` identity. Compilation refuses with
`conflicting_declaration`/`duplicate-declaration`, identifying both body
declarations; changing source-unit order or local alias spellings does not
change the refusal. No partial linked package or dispatch table is produced.
