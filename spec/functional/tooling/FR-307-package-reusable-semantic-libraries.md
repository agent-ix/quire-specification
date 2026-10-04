---
id: FR-307
title: "Package reusable semantic and domain libraries"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-002
    type: references
  - target: ix://agent-ix/quire-specification/FR-132
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---

# FR-307: Package reusable semantic and domain libraries

## Description

When importing a reusable Quire library, the linker SHALL resolve its exported
declarations, semantic definitions and model dependencies, and record the
resolved closure in the package lock.

## Inputs

Library package, export table, dependencies and supplied libraries.

## Outputs

Resolved qualified exports and dependency closure, or a conflict, cycle or
identity refusal.

## Behavior

A library exports its named declarations without acquiring a second source
authority. An importing package uses an imported name as a call `a::f(…)` of
an exported function whose parameter and result types are package-independent,
the use FR-322's `dependency_reference` admits. Imports are qualified;
diamond dependencies must select the same `package_id`.

## Library resolution contract

A library is identified by its library identity and its `package_id`, the
canonical content digest of its CheckedPackage V2. Resolution selects one
library per identity and produces a closed acyclic lock graph. Diamond paths
unify only when they reach the same `package_id`. Imports never inject
unqualified names implicitly.

## Import binding and export identity

The complete-unit import is `import "L" [as a];`. It names library identity
`L` and nothing else: it carries no version and no digest, and any other
token after the identity string, a `version` or `digest` token included, is
`refused { code: invalid_syntax, cause: unexpected-token }` at that token.
`L` is the FR-322 library identity: a non-empty string compared byte for
byte, with no segment structure. A supplied library is one library's
source, offered under its identity `L` alone, with no version string. The
import selects the library supplied under `L`. The linker compiles it
through the same stages as the importing unit, resolving its own imports
against the same supplied libraries, and its `package_id` is the one that
compile yields; a supplier never states a library's `package_id`, and an
import never states one either. That recomputed `package_id`, the
`quire.package.semantic/v2` `package_id` of the library's CheckedPackage
V2, is the library's selection in the importing package's lock, and it is
what binds the library's content. At most one library is supplied per
identity; a second, different package of the same identity is
`refused { code: invalid_package, cause: conflicting-definition }`,
retaining the identity and both packages' `package_id`s, and an import
whose identity no supplied library has is
`refused { code: missing_import, cause: missing-selection }` at the import's
identity string. Because an import carries no digest, no import-level
comparison detects an edited dependency; a stale dependency is caught at
replay by the `package_id` the lock records for it (FR-323).
An export identity is (library package key, export node key in that library's
graph). Imported declarations are named only as `a::Name`. The grammar makes
`as` optional: an import without `as` still selects and verifies the library,
but binds no qualifier and brings no name into scope. No import brings
unqualified names into scope, so a use of an imported name without its
qualifier, including every use of a name from an import without `as`, is
`refused { code: missing_declaration, cause: missing-name }`. When two imports
bind the same qualifier, each use is
`refused { code: ambiguous_declaration, cause: ambiguous-name }` and lists both
import paths.

`a::Name` resolves in two steps. First `a` resolves to exactly one package:
the library package whose import binds qualifier `a`, selected by its locked
`package_id`. Then `Name` resolves only through that package's
FR-322 `declaration` members: `qualified_name` is the package-local declaration
path, and the export node is the single node of that package whose
`declaration.qualified_name` equals the path `Name` spells, compared segment by
segment and byte for byte, with no case folding, Unicode normalization, suffix
match or search in another package. This covers scalar, composite and bounded
types, functions, declared value constants, models, protocols, claims and every
nominal kind; builtin and anonymous nodes carry no declaration and are never
exports. No
node of the library carrying that name is
`refused { code: missing_declaration, cause: missing-name }`. A resolved
export is admitted in the importing package as FR-322's `dependency_reference`
rule states: as the callee of a call to a function no type node reachable from
whose signature carries a `declaration`. Any other use of a resolved export,
such as `a::R` for an exported record `R` or a call of a function returning
`R`, is `refused { code: ill_typed, cause: operator-ineligible }` at the use.
The linker never recovers an export from source occurrences, source-map regions, display text,
node order or a node's body; a library whose own graph carries a duplicated
name or a declaration that disagrees with its nominal qualified declaration is
already refused by the FR-322 reader and exports nothing.

Resolution takes the transitive import closure in declaration order. Every
import of identity `L` in one compile selects the one library supplied under
`L`, so its paths reach one `package_id`. When checked packages are linked,
two dependency paths reach library identity `L` through the selections their
packages' locks record, and they unify only when those selections name the
same `package_id`. Any other combination is
`refused { code: invalid_package, cause: conflicting-definition }`, and the
refusal lists both dependency paths. Distinct `package_id`s never unify. An import
cycle is `refused { code: invalid_package, cause: definition-cycle }`, whose
dependency-edge payload lists the cycle path. A resolved closure is recorded in the importing package lock as one
selection per library identity, in ascending identity order.

Because `package_id` is the digest of the package's identity preimage, a reader
recomputes it before resolution. A package whose retained `package_id`
differs from the digest of its identity preimage is
`refused { code: invalid_package, cause: invalid-value }` at member path
`/package_id`, with no import or lock selection binding it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-307-AC-1 | A dependency diamond whose paths reach one `package_id` resolves one exact export identity and reproducible lock. | Test (TC-227) |
| FR-307-AC-2 | Conflicting definitions, import cycles or ambiguous unqualified names refuse with dependency paths. | Test (TC-227) |
| FR-307-AC-4 | An import binds the `package_id` its supplied library compiles to, recorded as the lock selection of its identity; a conflicting diamond or import cycle is `invalid_package` with its paths, and a duplicate qualifier is `ambiguous_declaration`; an import cycle uses cause `definition-cycle`, and a name from an import without `as` is `missing_declaration`. | Test (TC-227) |
| FR-307-AC-5 | A package whose retained `package_id` differs from the digest of its identity preimage is `invalid_package` with cause `invalid-value` at `/package_id`. | Test (TC-227) |
| FR-307-AC-6 | `import "test/geometry" as g;` binds to the library supplied under the identity `test/geometry` and no other, and the lock selection of `test/geometry` is the `package_id` that library's source compiles to; an import carrying a `version` or `digest` token after its identity string is `invalid_syntax`/`unexpected-token` at that token; two different packages supplied under one identity are `invalid_package`/`conflicting-definition` retaining the identity and both `package_id`s; an import of an identity no library is supplied under is `missing_import`/`missing-selection` at its identity string; and with `test/geometry` exporting a record `R` and a function `f` over builtin types, `g::f(x)` is admitted and a use `g::R` is `ill_typed`/`operator-ineligible` at the use. | Test (TC-227) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
