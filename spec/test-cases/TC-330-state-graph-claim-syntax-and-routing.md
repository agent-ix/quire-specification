---
id: TC-330
title: "State-graph claim syntax, refusals and routing"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-390
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-290
    type: verifies
---

# TC-330: State-graph claim syntax, refusals and routing

## Description

Parse and admit the three state-graph clauses, refuse the constructs a state-graph claim excludes, and check the requested items, kinds, forms and the derived deadlock-freedom item.

## Test Procedure

Compile each source vector through the real local Rust boundary under the complete facets and record its parse tree or located refusal. For each admitted vector, build the request for a model subject and record its items, each item's FR-290 kind, extent, recorded obligation form and instance key, and the deadlock-freedom items. For SG-R07, register one candidate whose negotiation arm discharges `possible` only.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| SG-S01 | `possible Win using v over (g: G::Game) { g.phase = G::Phase::Won }` | form `possible`, one `over` parameter; one item per universe key |
| SG-S02 | `always possible Rec using v over (g: G::Game) { g.phase = G::Phase::Won }` | form `always possible`, `Q` read as `true` |
| SG-S03 | `always possible Rec using v over (g: G::Game) from (g.phase != G::Phase::Lost) { g.phase = G::Phase::Won }` | form `always possible` with its `from` predicate |
| SG-S04 | `unique path One using v over (j: M::Job) from (not j.a) to (j.done);` | form `unique path` with `X` and `Y` |
| SG-S05 | `possible P using v { true }` | one item, no `over` parameter, instance `null` |
| SG-R01 | `possible P using v over (g: G::Game) { pre(g.phase) = G::Phase::Mid }` | `unsupported_construct`/`expression-form` at the `pre` read |
| SG-R02 | a `possible` predicate that reads an operation anchor | `unsupported_construct`/`expression-form` at the anchor read |
| SG-R03 | a `possible` claim whose predicate holds `eventually` | `unsupported_construct`/`expression-form` at the operator |
| SG-R04 | an `always possible` claim with a weak fairness constraint | `unsupported_construct`/`expression-form` at the constraint |
| SG-R09 | `possible R using v over (c: C::Cfg) { c.n = 1 and eventually c.n = 2 }` | parses; `unsupported_construct`/`expression-form` at `eventually`, not a syntax error |
| SG-R10 | `possible N using v over (c: C::Cfg) { not eventually[0,5] c.n = 2 }`, `possible P using v over (c: C::Cfg) { (eventually c.n = 2) }` and `possible U using v over (c: C::Cfg) { c.a until c.b }` | each parses; `unsupported_construct`/`expression-form` at `eventually`, `eventually` and `until` |
| SG-R11 | `always possible Rec using v over (c: C::Cfg) from (always c.ok) { c.done }` and `unique path One using v over (c: C::Cfg) from (c.a) to (eventually c.done);` | each parses; `unsupported_construct`/`expression-form` at `always` and at `eventually` |
| SG-R12 | `always possible Rec using v over (c: C::Cfg) { fair weak C::Cfg::step; c.done }` | parses; `unsupported_construct`/`expression-form` at the constraint |
| SG-R13 | `possible P using v over (c: C::Cfg) { c.done; fair weak C::Cfg::step }` | syntax error at the `;` after `c.done`: a body is a predicate and ends there |
| SG-R14 | `possible W using v over (c: C::Cfg) { eventually[1,*] c.done }` | parses; `expression-form` at `eventually`, with no profile refusal |
| SG-R15 | `possible B using v over (c: C::Cfg) { if eventually c.a then c.b else c.c }`, `always possible F using v over (c: C::Cfg) from (c.a; fair weak C::Cfg::step) { c.done }` and `unique path One using v from (c.a) to (c.d); fair weak C::Cfg::step` | each a syntax error: a temporal operator in an `if` condition, a fairness constraint in a `from` operand, and a fairness constraint after a bodiless clause |
| SG-R16 | `possible N using v over (c: C::Cfg) { not c.n = 2 }` | parses as the `expr` `(not c.n) = 2`; no grammar refusal |
| SG-R05 | a request whose only items over subject `G` are SG-S01 and SG-S02 | one deadlock-freedom item for `G`, and no second one |
| SG-R06 | each admitted vector | kind `temporal-satisfaction`, extent unbounded, form recorded as an obligation form |
| SG-R07 | SG-S02 against a single candidate whose arm discharges `possible` only | `unsupported`, warned, `unsupported_projection`/`unsupported-requested-capability` |
| SG-R08 | `possible possible using v { true }`; then `predicate unique using v (): Boolean { true }` | syntax error at the second `possible` and at `unique`: both are reserved words in an `ident` position |

## Expected Results

Each admitted vector yields exactly its form, predicates and items, and each refused vector exactly its located refusal. No state-graph item is evaluated by a trace monitor.
