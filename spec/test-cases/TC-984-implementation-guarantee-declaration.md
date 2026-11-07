---
id: TC-984
title: "Implementation guarantee declaration"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-490
    type: verifies
---

# TC-984: Implementation guarantee declaration

## Description

Check, emit and read back a `guarantee-decl` over TC-982's counter, and show
the slots it derives and each refusal.

Scope: FR-490-AC-1 to FR-490-AC-9.

## Test Procedure

The unit selects `quire.model.complete/v1` and
`quire.temporal.infinite-trace/v1`, declares TC-982's counter model, the
liveness clause `Eventually2` (`always eventually holds(c.value = 2)` under
`fair weak each Spec::Counter::inc`), the safety clause `NonNegative`
(`always holds(c.value >= 0)`) and the relation `A` binding the counter type,
its population and `inc` to `"counter::Counter::inc_cas"`. Run each case
through the real local Rust boundary:

1. `guarantee G of Eventually2 using A { initial to "counter::Counter::new";
   progress Spec::Counter::inc decreases "counter::retries_left";
   environment Sched by "scheduler" fair weak each Spec::Counter::inc; }`.
2. The same declaration `of NonNegative`.
3. Case 1 in a package without `quire.model.complete/v1`; with no `initial`
   member; with `initial to counter::Counter::new` unquoted.
4. `of Missing`; `using Missing`; `of` a finite-trace clause; `of` a clause
   over another model alias.
5. `progress Spec::Counter::dec ...` (no such operation); `bound 0`;
   `decreases "9lives"`; two `progress` members for `inc`.
6. Remove `inc`'s frame binding from `A`.
7. Emit case 1, read the bytes back and compare; drop `relation` from the
   node's `dependencies`; then swap two `progress` entries of a two-entry
   declaration.
8. Compile the unit with `A` and no `guarantee-decl`, and without `A`.
9. Case 1 without its `progress` member; case 1 without its `environment`
   member.

## Expected Results

1. The guarantee item and the slots model-claim (with `stutter_closure`),
   initialization, transition-simulation, frame, environment, no-divergence,
   fairness-correspondence, in that order, each with a distinct identity.
2. Model-claim (no `stutter_closure`), initialization, transition-simulation,
   frame and environment slots only.
3. `unsupported_construct`; `invalid_syntax`; `invalid_syntax` at the token.
4. `missing_declaration`/`missing-name` twice; `invalid_model_binding`/
   `malformed-declaration` naming the clause twice.
5. `missing_declaration`/`missing-name`; `invalid_model_binding`/
   `malformed-declaration` twice; `invalid_model_binding`/
   `conflicting-binding` naming both members.
6. The package checks; only the guarantee item refuses
   `missing_declaration`/`missing-name`.
7. The same declaration, slots and identities, each slot's source span the
   member that produced it; `invalid_semantic_graph` twice.
8. The same derived items in both.
9. An unfilled no-divergence slot; an unfilled environment slot naming
   `fair weak each Spec::Counter::inc`; no refusal.
