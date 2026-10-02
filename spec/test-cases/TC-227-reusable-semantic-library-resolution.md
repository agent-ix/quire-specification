---
id: TC-227
title: "Reusable semantic library resolution"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-307
    type: verifies
---

# TC-227: Reusable semantic library resolution

## Description

Resolve a diamond and reject conflict, cycle, ambiguity and a package whose `package_id` does not match its preimage.

## Test Procedure

Use the subject and inputs named by FR-307. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

Select the `quire.value.complete/v1` definitions through
[`complete-value-lock.json`](../../proposals/quire-v1/definitions/complete-value-lock.json).
Unless a row states limits, run it under limits large enough that no charge is
denied. Integer literals, parameters, `let` names and field projections make no
charge. Every `ill-typed` expectation is a type-checking
`refused { code: ill_typed }` made before any charge.

Build libraries and importing packages as CheckedPackage V2 artifacts, and
compare each import digest with the library `package_id`.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| L01 | L@1 exports `function f using v(x: Int[0, 9]): Boolean pure { x < 5 }`. `import "L" digest "<package_id of L@1>" as l;` using `l::f(x)`; then the same import with the bare 64-hex SHA-256 of L's raw source bytes | admitted, the call's callee the export identity (L package key, `f` node key); `refused { code: stale_dependency, cause: byte-digest-mismatch }` |
| L02 | `import "L" digest "<package_id of L@1>";` without `as`, then a use `f(x)` of L's export; then a use `L::f(x)` | the import is admitted and verified but binds no qualifier; `refused { code: missing_declaration, cause: missing-name }` at `f`; the same refusal at `L::f`, because the library name is not a qualifier |
| L03 | two imports of different libraries both `as a`, then `a::f(x)` | each use is `refused { code: ambiguous_declaration, cause: ambiguous-name }` listing both import paths |
| L04 | P imports A and B; A and B each import L with the same `package_id` and call `l::f`; then B imports L with a different `package_id` | one export identity for `f` and one lock selection for L; `refused { code: invalid_package, cause: conflicting-definition }` listing `P -> A -> L` and `P -> B -> L` |
| L05 | A imports B and B imports A | `refused { code: invalid_package, cause: definition-cycle }` whose dependency-edge payload lists `A -> B -> A` |
| L06 | P imports libraries `Z` and `A` in that source order | the lock lists `A` then `Z` |
| L08 | L' is L with a changed export, presented with L's `package_id`; then P imports L' by that digest | `refused { code: invalid_package, cause: invalid-value }` at member path `/package_id`, because the recomputed digest of L''s identity preimage differs; P's import resolves no L' and binds no lock selection |
| L09 | `import "test/geometry" digest "sha256:<package_id of test/geometry@1>" as g;`, then the digest in uppercase hex, with 63 characters and with 65 characters; then the bare-hex digest with `test/geometry@1` supplied as `test/geometry`, together with a library supplied as `test/geometry/extra`; then two libraries supplied as `test/geometry`; then no library supplied as `test/geometry`; then `test/geometry` supplied from source that compiles to a `package_id` other than the digest; then, with `test/geometry` also exporting a record `R`, a use `g::R` | `refused { code: invalid-digest }` at the digest string, four times; admitted, bound to the library supplied as `test/geometry` and to no other; `refused { code: invalid_package, cause: conflicting-definition }` retaining both libraries' identity and version; `refused { code: missing_import, cause: missing-selection }` at the import's identity string; `refused { code: stale_dependency, cause: byte-digest-mismatch }` naming the digest and the compiled `package_id`; `refused { code: ill_typed, cause: operator-ineligible }` at `g::R` |

## Expected Results

Every positive and boundary result matches FR-307; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
