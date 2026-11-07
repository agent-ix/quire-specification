---
id: SR-20261106202631
title: IR-1403 QSpec PR634 code-review review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-specification; IR-1403; proposals/quire-v1/shared-grammar.md;
  spec/functional/type-model/FR-142-evaluate-quantities-and-units.md; spec/test-cases/TC-187-quantity-and-unit-conversion.md
review_set: subset
---

## Summary

Spec-only review of PR634 at adbee96008e40f1a473fd2f9099daca8d383b8ac. The diff changes three Markdown sources only. No Rust embeds any changed file; Cargo gates and runtime gap analysis are inapplicable. No runtime implementation or U25 execution credit is claimed.

## Verdict

PASS — no defects found in the scoped source contract. This is no claim of runtime conformance.

## Assurance Context

No AssuranceProfile artifact exists in the repository specification tree; no enforced review policy applies. Unavailable context: runtime implementation and execution evidence, intentionally outside this source-only review. Active exceptions: none.

## Examined scope

```yaml
scope:
- id: FR-142-AC-39
  path: spec/functional/type-model/FR-142-evaluate-quantities-and-units.md
  role: examined
  excerpt: 'Over TC-187''s `kg`, `m` and `s` roots, `Quantity<kg*m^2*s^-2>` is accepted
    in parameter, result and conversion-target positions and names the checked compound
    with canonical root terms `kg:1`, `m:2`, `s:-2`. `Quantity<s^-2*kg*m*m>` has the
    same normalized compound identity. Bare `Quantity<kg>` remains declared, while
    `Quantity<kg^1>` and `Quantity<kg*m^0>` have the same one-term compound identity,
    distinct from declared `kg`; `Quantity<m*m^-1>` is the empty compound. The first
    U25 conversion can therefore be written `convert<Quantity<kg*m^2*s^-2>>(q)` for
    `q: Quantity<N_m>` and its result expl'
- id: FR-142-AC-40
  path: spec/functional/type-model/FR-142-evaluate-quantities-and-units.md
  role: examined
  excerpt: '`Quantity<>`, `Quantity<kg*>`, `Quantity<kg**m>`, `Quantity<m^>`, `Quantity<m^n>`
    and `Quantity<m^1.5>` refuse `invalid_syntax` at the malformed unit expression.
    `Quantity<kg*ghost^2>` and `Quantity<ghost^0>` refuse `missing_declaration`/`missing-name`
    at `ghost`; a dimension-only factor refuses that same unit lookup. Existing integer
    and frontend ceilings produce their existing typed refusal or limit result, never
    a truncated exponent or partial authoritative compound. Existing bare named-unit
    and named-unit carrier forms retain their meanings.'
- id: quantity-unit
  path: proposals/quire-v1/shared-grammar.md
  role: examined
  excerpt: quantity-unit = unit-term, { '*', unit-term } ;
- id: unit-term
  path: proposals/quire-v1/shared-grammar.md
  role: examined
  excerpt: unit-term     = qualified-name, [ '^', signed-int ] ;
- id: FR-142-quantity-source
  path: spec/functional/type-model/FR-142-evaluate-quantities-and-units.md
  role: examined
  excerpt: 'Complete-V1 source SHALL also accept a product of qualified unit names
    with

    optional signed integer powers in that first argument, for example

    `Quantity<kg*m^2*s^-2>`. Each factor SHALL resolve in the unit namespace,

    including a factor written with exponent zero. A product or explicit power

    SHALL form the checked compound unit of FR-322 using the existing compound

    normalization and admission rules: canonical root-unit identities, combined

    integer exponents, no zero terms and canonical term order. A bare qualified

    name SHALL keep its declared-unit meaning. In particular, a one-term or

    empty com'
- id: TC-187-U25-source
  path: spec/test-cases/TC-187-quantity-and-unit-conversion.md
  role: examined
  excerpt: 'For U25''s source target (FR-142-AC-39 and AC-40), parse and check a function

    from `Quantity<N_m>` whose first conversion is

    `convert<Quantity<kg*m^2*s^-2>>(q)` and whose second converts that result to

    `Quantity<J>`. Inspect the retained qualified factors, signed exponents and

    source locations, and the actual checked compound identity; compare the

    reordered spelling `s^-2*kg*m*m`. Also inspect bare `Quantity<kg>`, the

    one-term compound spellings `kg^1` and `kg*m^0`, and the cancelling compound

    `m*m^-1`. Test every malformed or unresolved factor of AC-40 in its own

    source unit and exercise the e'
- id: FR-142-compound-identity
  path: spec/functional/type-model/FR-142-evaluate-quantities-and-units.md
  role: context_only
  excerpt: 'A compound unit that

    a checked source type forms is a separate thing: the I04

    `scalar_type`/`compound_unit` node of FR-322 "Derived type forms and

    forwarding", identified by FR-322''s structural node id, whose terms follow

    this order.'
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
