---
id: SR-14716
title: "spec-integrity analysis of STD-244 (QSpec PR #345)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@073ee8c1bd971d53c52138ad16f2b50e9c7e5e87 (diff against origin/main 97bfd65); spec/functional/analysis/FR-381-check-symmetric-populations.md, spec/functional/expressions/FR-145-convert-and-query-collections.md, spec/test-cases/TC-320-symmetric-population-authoring.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-381
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-145
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-320
    type: reviews
---
# SR-14716: spec-integrity analysis of STD-244 (QSpec PR #345)

## Summary

Ticket: STD-244. PR: quire-specification#345, head 073ee8c.

The FR-381/FR-145 contradiction the ticket names is resolved: over an unannotated population an out-of-catalog fold is now FR-145's `operator-ineligible`, catalog folds are not breaking forms, and the per-fold precedence matches QSL FR-150 (source type decides; the identity-observing cause wins because it names the population). FR-145's catalog paragraph points back at the precedence, so it no longer reads as `operator-ineligible` in every case. FR-145 checks typing before the catalog, so `type-mismatch` still comes first. `fold<Tally>(acc, p in self.peers: acc + 1, identity: 0)` is a catalog step (`acc OP t`, `t` = `1` of type `Integer`, `+`). Codes and causes are catalogued (FR-271, FR-272, native-diagnostics). FR-381-AC-5 is a new id that no other open PR uses. No stale "unannotated ... admitted" statement remains in spec/ or proposals/. The lead default (adopt QSL's precedence) leaves no unbuildable AC. Gates: `quire validate` 134 (baseline 134), touched files unchanged versus main; cargo pass (14 + 117).

Units examined:

- `FR-381-AC-2` (examined)
- `FR-381-AC-5` (examined)
- `FR-381` (examined)
- `FR-381-breaking-forms` (examined)
- `FR-381-precedence` (examined)
- `FR-145` (examined)
- `FR-145-convert` (context_only)
- `TC-320` (examined)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Major: the breaking-forms list narrows "an order-sensitive traversal of a `Set` or `Bag` ..., such as `fold` or `reduce`" to `fold`/`reduce` only. QSL FR-150's form 1, whose precedence the PR adopts, keeps "or another traversal that depends on element order". FR-145 orders a `convert` of a set or bag to a sequence or ordered set by ascending canonical key, which for references is identity order, and it admits any pure step for a sequence fold. So `fold` over `convert<Sequence<Reference<ConfigVersion>>[0,3]>(self.peers)` with an element-choosing step is now admitted over a symmetric population. The conversion bullet covers converting a reference, not a collection. This contradicts FR-381's own SHALL statement (refuse every form that observes identity other than by equality, dereference, membership or passing as a value). Fix: restore the other-order-dependent-traversal class, naming a set or bag `convert` to a sequence or ordered set, and give it an AC and a TC-320 row. | spec/functional/analysis/FR-381-check-symmetric-populations.md:65-75 |
| FND-002 | low | Minor: "Precedence over FR-145" picks the cause from the fold's source type alone, while "Scope of the check" limits `identity-observing-form` to the listed clause kinds (pre/postconditions, invariants, temporal atoms, `terminal when`, state constraints, refinement rows). The cause for an out-of-catalog fold over such a source in another position, such as an FR-146 function body, is not stated. Scope the precedence to the clauses the check covers, or extend the scope. | spec/functional/analysis/FR-381-check-symmetric-populations.md:93-101 |

## Verdict

Not mergeable as is: one major finding (FND-001, traversal class narrowed below QSL FR-150 and FR-381's own statement) and one minor (FND-002).
