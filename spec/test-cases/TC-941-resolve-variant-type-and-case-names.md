---
id: TC-941
title: "Resolve a variant type name and a case literal"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-208
    type: verifies
---
# TC-941: Resolve a variant type name and a case literal

## Description

Verify that `Alias::Tier` names a variant type, that `Alias::Tier::Gold` is a
case literal of it, and the refusals of an undeclared case, an unknown type
name and a non-variant literal. Scope: FR-208-AC-17.

## Test Procedure

Use the M24 package, with a second variant type `Level` (cases `Gold`,
`Silver`), selected by a unit as `model Config`, and by a second unit as
`model Cfg`. In each unit, check an expression or clause for each case below.

1. `Config::Tier::Gold` at a `Config::Tier` slot, in a function
   `f(t: Config::Tier): Config::Tier { t }` call and in an
   `Option<Config::Tier>` literal.
2. The Text literal `"Gold"`, the Integer literal `0`, a bare `Gold` and
   `Config::Level::Gold` at a `Config::Tier` slot.
3. `Config::Tier::Silver` and `Config::Tier::gold`.
4. `Config::Nope::Gold`, and `Config::Nope` in a type position.
5. `Config::Tier::Gold < Config::Tier::Standard`, and
   `Config::Level::Gold == Config::Tier::Gold`.

## Expected Results

1. Each checks as type `Config::Tier`; the same expressions with `Cfg` resolve
   to the same type and case.
2. Each refuses `ill_typed`/`type-mismatch` at the expression, naming the
   expected and actual types.
3. Each refuses `ill_typed`/`type-mismatch` at the name, naming `Tier` and the
   case.
4. Each refuses `missing_declaration`/`missing-name` at the name.
5. The ordering refuses `ill_typed`/`operator-ineligible`; the cross-type
   equality refuses `ill_typed`/`type-mismatch` at check and never evaluates.
