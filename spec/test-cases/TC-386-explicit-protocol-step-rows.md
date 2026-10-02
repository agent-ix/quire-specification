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

Abstract side: protocol `Req` with a single attempt `Ask` on operation
`ask()` and a `finish`. Concrete side: protocol `Impl` with `parallel P {
branch a send S via h …; branch b receive R via h of P::a::S …; } join all
[a, b];`, then attempt `Do` on operation `do()`, then `finish`. Write the
refinement declaration as each vector states and check it through the real
local Rust boundary.

## Vectors

| Vector | Rows | Expected result |
| --- | --- | --- |
| SR-01 | a row for `Do` only | refuses `missing_declaration`/`missing-name` once each for `fork Impl::P`, `join Impl::P`, `Impl::P::a::S`, `Impl::P::b::R` and the finish node |
| SR-02 | rows `step fork Impl::P -> stutter;`, `step join Impl::P -> stutter;`, `step Impl::P::a::S -> stutter;`, `step Impl::P::b::R -> any;`, `step Impl::Do -> node Req::Ask;`, finish `-> node Req::Done;` | admitted; the refinement holds |
| SR-03 | SR-02 plus a second row for `fork Impl::P` | refuses `invalid_model_binding`/`conflicting-binding` naming both rows |
| SR-04 | SR-02 with a second attempt node `Do2` on `do()`, an operation row `step Impl::do -> stutter;` and the node row for `Impl::Do` | admitted; the node row decides `Do`'s steps and the operation row decides `Do2`'s; adding a second operation row for `do()` refuses `invalid_model_binding`/`conflicting-binding` |
| SR-05 | SR-02 with `Do` mapped `-> stutter` while `do()` writes a mapped visible field | refuted at the `Do` step |
| SR-06 | SR-02 with `step Impl::P::a::S -> node Req::Ask;` | refuted: after `S`, `Ask` is enabled once, and the later `Do` step's label `Ask` has no enabled abstract step |
| SR-07 | the concrete parallel `memory tso`, SR-02's rows | refuses `missing_declaration`/`missing-name` naming the memory step kind `flush`; with `step memory flush -> stutter;` admitted |
| SR-08 | an abstract protocol whose profile's internal list is empty, and a concrete `stutter` row | the concrete internal step does not match any abstract step, since every abstract step is visible |

## Expected Results

Every admission, refusal and verdict matches the table; no concrete step class
is mapped by default.
