---
id: SR-401
title: "Failure-domain review of the composed native Quire v1 foundation"
type: SpecReview
analysis: failure-domain
scope: "Composed A/B/E/F native contracts, FR-020, FR-030–062, FR-090–095, FR-110–116, FR-130 and associated constraints/cases"
review_set: all
evaluated_revision: "788b161bd1b7fd5d632cc42c949f497914df3ce6"
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-061, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-114, type: reviews }
---
## Summary

The composed contracts explicitly handle identity substitution, cyclic references,
pure predicates, finite control, unavailable observations, resource stops and
historical supersession. One remaining specification defect prevents a common
failure interpretation: a blanket replay rule suppresses otherwise decisive results.
These are requirements findings; planned tests are not implemented evidence.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-002 | high | F's replay requirement still makes every open prefix non-settled/refused, and AC-2 keeps every open prefix distinct from violation. This contradicts both decisive-witness satisfaction and decisive-counterexample violation on an open subject in the newly integrated B/E/F result contracts. Owner: F. Qualify the rule by lack of decisive support and extend the replay discriminator to both settled-open outcomes. | FR-114:15,22; TC-144; FR-061-AC-8; FR-094-AC-7; FR-115-AC-5; TC-145 | wrong-requirement |

## Concrete discriminators

1. Replay and incrementally assess the same open history containing either an
   admitted `p` witness for the eventuality or an admitted `not p`
   counterexample for `always[0,30] holds(p)`. The respective satisfied and
   violated truths must agree in both modes while closure stays open. A third
   prefix with neither decisive basis stays pending. TC-145 covers the result
   adapter, but TC-144 currently supplies no corresponding settled-open replay
   case and FR-114 prohibits it.

## Failure-domain checks

| Domain | Reviewed rule and disposition |
| --- | --- |
| Extension points and purity | FR-033 and protocol-contract.md exclude I/O, callbacks, mutation, reflection and ambient monitor state from predicates. Evaluation failure cannot choose a branch. No executable plugin hook is selected. |
| Entity identity | Package roles use owning declaration, source region and kind; graph equality uses universe, type and object ID; protocol occurrence identity additionally includes workflow and enclosing iteration ordinals. O1/O2, delivery/attempt/effect, and registration/activation cannot substitute for each other. |
| Cycles and topology | Predicate and definition cycles refuse; finite reference cycles are admitted with at-most-once expansion and positive-length reachability; structural containment remains acyclic. Protocol unfolding is bounded and acyclic with explicit progress and all-branch join. |
| Work exhaustion | NFR-010/020/040/050 distinguish semantic maxima from runtime limits, including nested sequences, predicate fan-out, active instances, history and retained results. No fabricated false result or silent eviction is authorized. |
| Missing authority and restoration | FR-094 and observation-progress.md reject foreign/regressing assertions, missing restoration basis and false EOF closure; quiet time advances only the selected clock under scoped authority. |
| Historical identity | Later contradictions create linked superseding/invalidating results and retain earlier bytes. The native-source ruling and historical definition candidates remain separate selections. |
| Cross-owner unresolved seams | SR-403 records F's unclosed exact dependency selections. SR-406 records F/E window-to-clock correspondence; an incomplete mapping is not filled with a timestamp or hash default. |

## Review boundary

F's result-consumer correction through `921993bbbef92a8010aec46d46277498de5e0288`
is present. FND-002 concerns remaining clauses after that
correction, not an assertion that F's result-consumer contribution is absent.
No implementation, builds, hosted CI, tests or external repository edits were
performed for this review. Closure requires reviewed contract changes and
updated distinguishing cases; execution and qualification follow later.
