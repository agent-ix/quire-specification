---
id: TC-386
title: "Explicit refinement rows for every protocol step class"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-432
    type: verifies
---

# TC-386: Explicit refinement rows for every protocol step class

## Description

Check that a refinement over a protocol subject requires one explicit row per
concrete step class, refuses missing and doubled rows, reads visibility only
from the rows, and decides visible and internal steps as FR-432 states.

## Test Procedure

Every vector uses the header `abstract Spec::Req`, with `Spec` the abstract
model alias, except where SR-09 says otherwise. Abstract side: protocol `Req` with a single attempt `Ask` on operation
`ask()` and a `finish`. Concrete side: protocol `Impl` with `parallel P {
branch a send S via h …; branch b receive R via h of P::a::S …; } join all
[a, b];`, then attempt `Do` on operation `do()`, then `finish`. Write the
refinement declaration as each vector states and check it through the real
local Rust boundary.

## Vectors

| Vector | Rows | Expected result |
| --- | --- | --- |
| SR-01 | a row for `Do` only | refuses `missing_declaration`/`missing-name` once each for `fork Impl::P`, `join Impl::P`, `Impl::P::a::S`, `Impl::P::b::R` and the finish node |
| SR-02 | rows `step fork Impl::P -> stutter;`, `step join Impl::P -> stutter;`, `step Impl::P::a::S -> stutter;`, `step Impl::P::b::R -> any;`, `step Impl::Do -> node Spec::Req::Ask;`, finish `-> node Spec::Req::Done;` | admitted; the refinement holds |
| SR-03 | SR-02 plus a second row for `fork Impl::P` | refuses `invalid_model_binding`/`conflicting-binding` naming both rows |
| SR-04 | SR-02 with a second attempt node `Do2` on `do()`, an operation row `step Impl::do -> stutter;` and the node row for `Impl::Do` | admitted; the node row decides `Do`'s steps and the operation row decides `Do2`'s; adding a second operation row for `do()` refuses `invalid_model_binding`/`conflicting-binding` |
| SR-05 | SR-02 with `Do` mapped `-> stutter` while `do()` writes a mapped visible field | refuted at the `Do` step |
| SR-06 | SR-02 with `step Impl::P::a::S -> node Spec::Req::Ask;` | refuted: after `S`, `Ask` is enabled once, and the later `Do` step's label `Ask` has no enabled abstract step |
| SR-07 | the concrete parallel `memory tso`, SR-02's rows | refuses `missing_declaration`/`missing-name` naming the memory step kind `flush`; with `step memory flush -> stutter;` admitted |
| SR-08 | an abstract protocol with no internal list supplied (empty), and a concrete `stutter` row | the concrete internal step does not match any abstract step, since every abstract step is visible |
| SR-09 | SR-02 headed `abstract Spec::Req`; then the same rows headed `abstract Spec` | the first selects the protocol and its `node Spec::Req::…` rows are admitted; the second is a model subject and the `node Spec::Req::Ask` and `node Spec::Req::Done` rows each refuse `invalid_model_binding`/`malformed-declaration` at the node |
| IL-01 | abstract protocol `Full` (`a1`, `a2` attempts of `ask()`, send `s` and receive `r` via channel `ch`, event `ev`, timeout `to`, fence `fe`, `parallel` `par`, template `T`, role `w`, trigger, `tso` memory with kind `flush`, `finish` `fin`), header `abstract Spec::Full`, rows mapping every concrete class, and the value `[{"refinement": "R", "internal": ["Spec::Full::fin", "lose Spec::Full::ch"]}]`; then its permutations, the entries in two arrays swapped, the value absent, `[]` and an object for another refinement | the first resolves to `Spec::Full::fin` then `lose Spec::Full::ch`, the permutations to the same list; the other three to the empty list with no refusal (FR-432-AC-7) |
| IL-02 | IL-01 with the value naming, one at a time, `Spec::Full::a1`, `Spec::Full::s`, `Spec::Full::r`, `Spec::Full::ev`, `Spec::Full::to`, `Spec::Full::fe`, `Spec::Full::fin`, template `Spec::Full::T`, `fork Spec::Full::par`, `join Spec::Full::par`, `cend Spec::Full::T`, `duplicate Spec::Full::ch`, `lose Spec::Full::ch`, `spawn Spec::Full::w`, `retire Spec::Full::w`, `activate Spec::Full`, `memory flush`; then `Spec::Full::par` | each of the seventeen makes exactly its class internal; the bare `par` path refuses `missing_declaration`/`missing-name` at its pointer (FR-432-AC-8) |
| IL-03 | IL-01 with, one at a time: value `{}`; `[{"refinement": "R"}]` at `/0`; `[{"refinement": 7, "internal": []}]` at `/0/refinement`; `"internal": "x"` at `/0/internal`; a key `visible` at `/0`; entry `7` at `/0/internal/0`; entries `""`, `frob x`, `Spec::Full`, ` Spec::Full::fin`; refinement `Nope`; entries `Other::Full::fin`, `Spec::Other::fin`, `Spec::Full::zz`, `lose Spec::Full::nochan`, `spawn Spec::Full::nobody`, `memory nokind`; two objects for `R`; `["Spec::Full::fin", "Spec::Full::fin"]`; then the value `[{"refinement": "R", "internal": ["Spec::Full::fin"]}]` under the header `abstract Spec` | the value `{}` and the next five defects `invalid_model_binding`/`malformed-declaration`, as do the four malformed entries; `Nope` and the six unresolved entries `missing_declaration`/`missing-name`; the two repeats `invalid_model_binding`/`conflicting-binding` carrying both pointers (`/1` and `/0`; `/0/internal/1` and `/0/internal/0`); the model-subject list `malformed-declaration` at `/0`; each at its RFC 6901 pointer with no admitted node (FR-432-AC-9) |
| IL-04 | IL-01 with the value naming `Spec::Full::a1` and the row `step Impl::Do -> node Spec::Full::a1(self);`; then the row `-> node Spec::Full::a2(self)`; then the value naming `lose Spec::Full::ch` | the first refuses `invalid_model_binding`/`conflicting-binding` naming the row and the entry; the other two are admitted (FR-432-AC-10) |

## Expected Results

Every admission, refusal and verdict matches the table; no concrete step class
is mapped by default.
