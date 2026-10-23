---
id: SR-16329
title: "spec-review of FR-466 predicate-join signature (quire-specification PR 401)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@a8460cfc; spec/objects/interfaces/FR-466-checked-package-protocol-control-bodies.md; spec/functional/protocol/FR-426-apply-protocol-steps.md; spec/test-cases/TC-948-checked-package-protocol-control-bodies.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-466
    type: references
  - target: ix://agent-ix/quire-specification/FR-426
    type: references
  - target: ix://agent-ix/quire-specification/TC-948
    type: references
---

# Spec review of FR-466 predicate-join signature (quire-specification PR 401)

## Summary

Ticket: STD-404. One spec-review pass over `git diff origin/main...a8460cfc`, checked for the no-drop/no-narrowing/no-deferral rule, testable vectors (key vectors derived by hand), id uniqueness across all open PR heads, cross-repo names, mergeability and gates.

Gate: `quire validate --scope . 'spec/**/*.md'`: 86 failure lines (origin/main also 86). `cargo test --test checked_package_v2 --test backend_provider_v1`: 23 + 129 passed. No test-parsed fixture changed, so no QSL conformance run.

Examined, clean:

- The signature follows STD-404 and the text already on main ("a total pure function over the explicitly captured branch dispositions"). FR-425 gives a thread the dispositions `running`, `completed` and `cancelled`; a join step reads only the first two because an `outstanding cancel` cancels the owner and every thread at or below it in the same step (FR-426-AC-13), so one Boolean per joined label loses nothing a join can observe.
- PA14 to PA17 by hand over `joined [east, west]`: two `boolean` parameters and a `boolean` result are admitted; one or three parameters, an integer second parameter and an integer result each refuse `ill_typed`/`type-mismatch` at `predicate`. TC-377 JP-06 (`f` true when `a` and `c` completed) stays consistent with one Boolean per label.
- Ids: FR-466-AC-15 and PA14 to PA17 appear on no other open PR head or on main (PA12 and PA13 exist on main). Test-parsed fixtures are untouched, so no QSL conformance run is needed.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | AC-15 and PA14 state "its first parameter bound to `east`" and "the first parameter receives `east`'s disposition", but the reader only checks parameter and result types, so no reader vector can observe which disposition a parameter receives. The one place that behaviour is stated, the new FR-426 `join` clause (true for `completed`, false for `running`, ascending label order), has no AC or TC vector: FR-426-AC-2 and TC-377 JP-06 name a predicate without fixing the parameter order or the true/false mapping. Fix: add an FR-426 evaluation vector (for example `f(a, b)` true only when `b` is completed, observed through a join enabled at exactly that state), and drop the binding claim from the reader AC. | spec/objects/interfaces/FR-466-checked-package-protocol-control-bodies.md:AC-15; TC-948 PA14 | correct-requirement-no-evidence |
| FND-002 | low | FR-171 still says a predicate join settles when "its pure predicate is true over the immutable branch-result map". The PR defines the predicate's input as one Boolean per joined label and does not touch FR-171, so the two descriptions of the same input differ ("branch-result map" against Booleans). Fix: point FR-171 at FR-466 step 4 or use the same wording. | spec/functional/protocol/FR-171-evaluate-dynamic-control.md:42-44 | wrong-requirement |

## Verdict

Mergeable after FND-001 (medium) are fixed or dispositioned by the lead.

Conflicts with main: none (clean merge-tree). No other open PR head touches FR-466, FR-426 or TC-948.
