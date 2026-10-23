---
id: SR-16306
title: "code-review of STD-442 backend-provider schema, vectors and tests (quire-specification PR 423)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@002fbf96; proposals/backend-provider-v1/schema.json; proposals/backend-provider-v1/vectors.json; tests/backend_provider_v1.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/TC-431
    type: references
---

# Code review of STD-442 backend-provider schema, vectors and tests (quire-specification PR 423)

## Summary

Ticket: STD-442. This reviews the schema, vector and test changes of quire-specification#423 at 002fbf96 against FR-193, FR-331-AC-41 and TC-431.

Examined, clean:

- Schema: `dynamics` on `BackendDescriptor` (nonempty, unique, from {discrete, hybrid}); forbidden off `temporal-satisfaction`; with `hybrid`, it requires `bounds.max_solver_steps` and `bounds.max_solver_time`. `requirements` on `RequestItem` (nonempty, unique, from {hybrid-dynamics}): it is allowed on `temporal-satisfaction` only, and an item carrying `hybrid` must have it. The `bounded-solver.method` enum and the `inductive-solver` basis (method const, certification `uncertified` or `trusted`, no certificate or evidence) are added, and `insufficient-horizon{horizon, jumps}` joins the inconclusive causes.
- I replayed every positive and all 16 new adverse vectors through a Python jsonschema 4.26 copy of the test (base-document errors excluded, located at the patch as `located_at_patch` locates them). Both positives are admitted, and each adverse vector is refused at its patched member.
- Mutation: removing the requirements-kind rule, the hybrid-implies-requirements rule, the hybrid-implies-bounds rule, the dynamics-only-on-temporal rule, `dynamics` `uniqueItems`, `minItems`, enum or `type`, or `requirements` `uniqueItems`, each makes its matching vector pass. So those rules are pinned.
- `certification-outcome`'s method is now `delta-complete`, and the vector still passes. The tests gain the `FR-331-AC-41`, `FR-193-AC-19` and `FR-193-AC-20` trace tags on the positive and adverse loops.
- `cargo test --test checked_package_v2 --test backend_provider_v1`: 24 + 139 pass.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | `hybrid-requirements-empty` and `hybrid-requirements-unknown-label` do not pin `requirements`' `minItems` or its closed enum. Their base item carries `hybrid`, so the "hybrid implies requirements holds hybrid-dynamics" rule refuses `[]` and `["stochastic-dynamics"]` anyway. Mutation confirms it: with `minItems` removed, or the enum widened to any string, both vectors are still refused at the patch. Fix: patch to `["hybrid-dynamics", "stochastic-dynamics"]` for the unknown label, and test `[]` on an item without `hybrid`, or add such vectors. | proposals/backend-provider-v1/vectors.json:7279-7316 | correct-requirement-no-evidence |

## Verdict

Mergeable on code-review grounds, with one low finding to fix in the PR's fix round. The PR as a whole is blocked by the spec-review (SR-16305).

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
