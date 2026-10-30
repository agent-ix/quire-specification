---
id: TC-227
title: "Reusable semantic library resolution"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-307
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-307-AC-7
    type: verifies
---

# TC-227: Reusable semantic library resolution

## Description

Resolve a diamond and reject conflict, cycle and ambiguity.

## Test Procedure

Use the subject and inputs named by FR-307. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

Select the `quire.value.complete/v1` definitions through
[`value-complete.md`](../../proposals/quire-v1/definitions/value-complete.md).
Unless a row states limits, run it under limits large enough that no charge is
denied. Integer literals, parameters, `let` names and field projections make no
charge. Every `ill-typed` expectation is a type-checking
`refused { code: ill_typed }` made before any charge.

Build libraries and importing packages as CheckedPackage V2 artifacts, and
compare each lock selection with the identity its supplied library is
supplied under.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| L01 | L, supplied under identity `L`, exports `function f using v(x: Int[0, 9]): Boolean pure { x < 5 }`. `import "L" as l;` using `l::f(x)` | admitted, the call's callee the export identity (library identity `L`, `f` node_id), and the lock's selection of `L` the entry `{identity: L}` |
| L02 | `import "L";` without `as`, then a use `f(x)` of L's export; then a use `L::f(x)` | the import is admitted and verified but binds no qualifier; `refused { code: missing_declaration, cause: missing-name }` at `f`; the same refusal at `L::f`, because the library name is not a qualifier |
| L03 | two imports of different libraries both `as a`, then `a::f(x)` | each use is `refused { code: ambiguous_declaration, cause: ambiguous-name }` listing both import paths |
| L04 | P imports A and B; A and B each import L, supplied once, and call `l::f`; then P links A and B as checked packages whose locks select L with structurally unequal packages | one export identity for `f` and one lock selection for L; `refused { code: invalid_package, cause: conflicting-definition }` listing `P -> A -> L` and `P -> B -> L` |
| L05 | A imports B and B imports A | `refused { code: invalid_package, cause: definition-cycle }` whose dependency-edge payload lists `A -> B -> A` |
| L06 | P imports libraries `Z` and `A` in that source order | the lock lists `A` then `Z` |
| L09 | `import "test/geometry" as g;` with a library supplied as `test/geometry`, together with a library supplied as `test/geometry/extra`; then `import "test/geometry" digest "<64 lowercase hex>" as g;` and `import "test/geometry" version "1.0.0" as g;`; then two libraries supplied as `test/geometry`, first differing and then identical; then no library supplied as `test/geometry`; then, with `test/geometry` also exporting a record `R`, a use `g::R` | admitted, bound to the library supplied as `test/geometry` and to no other, the lock's selection of `test/geometry` the entry `{identity: test/geometry}`; `refused { code: invalid_syntax, cause: unexpected-token }` at the `digest` token and at the `version` token; `refused { code: invalid_package, cause: conflicting-definition }` before any compile, naming both supplied libraries by their identity, in both cases; `refused { code: missing_import, cause: missing-selection }` at the import's identity string; `refused { code: ill_typed, cause: operator-ineligible }` at `g::R` |
| L10 | A imports B, B imports C, and A has no direct import of C; compare the produced locks and recompile A against an altered A lock that also lists C | A emits only B, B emits only C, and A derives B and C in its closure. The altered A lock differs from recompilation and is not certified as canonical producer output |

## Expected Results

Every positive and boundary result matches FR-307; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
