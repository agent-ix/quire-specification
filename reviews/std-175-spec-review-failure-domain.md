---
id: SR-12895
title: "Failure-domain review of STD-175 zero-instance hyperproperty items (QSpec PR #281)"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@1d130b62a9b40320f97c76d09017b967e1002157; PR #281 diff against origin/main 4f65ed2: FR-395, FR-399, TC-340; context FR-361, TC-337"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-399
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-395
    type: reviews
---
# SR-12895: Failure-domain review of STD-175 (QSpec PR #281)

## Summary

Ticket: STD-175. This pass looks for edge cases in the new zero-instance
rule. The empty-universe case and the no-universe (`requires-bound`) case
are well formed, and both can be built on the TC-337 vault subject.

## Verdict

One medium finding, the same one SR-12892 records for QSL.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The "type that no object of the universes conforms to" case comes from FR-361's `over` parameter, which ranges over every conforming object in any population. FR-395 binds `v: V::T` to "an object of a population of alias V's model", and its instances are that population's universe keys. So when the population exists, "no conforming object" is just an empty universe, which is case 1. When it does not exist, the parameter names nothing, and FR-395 does not say whether that is a check refusal or `no-instance`. On TC-337's vault subject (one population, `vaults` of `Vault`), VS-05's second case either repeats the first or is a static error, so FR-399-AC-11's second clause cannot be tested as a distinct runtime case. Drop it for hyper parameters, or define a parameter that resolves to no population as a refusal at the parameter. Keep this aligned with QSL FR-172. | spec/functional/analysis/FR-395-analyze-hyperproperties-over-behaviours.md:62-75; spec/functional/analysis/FR-399-settle-hyperproperty-results.md:86-92; spec/functional/analysis/FR-399-settle-hyperproperty-results.md:178; spec/test-cases/TC-340-hyper-vacuity-and-stutter.md:30 |
