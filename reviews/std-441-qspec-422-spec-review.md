---
id: SR-16304
title: "spec-review of STD-441 function-value parity form of the obligation-identity domain (quire-specification PR 422)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@150bcdf1; spec/objects/foundation/FR-201-canonical-identity-domain.md; spec/test-cases/TC-952-obligation-identity-digest-domain.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
  - target: ix://agent-ix/quire-specification/TC-952
    type: references
---

# Spec review of STD-441 function-value parity form of the obligation-identity domain (quire-specification PR 422)

## Summary

Ticket: STD-441. One spec-review pass over the PR's diff against origin/main (main is an ancestor of head 150bcdf1).

Examined, clean:

- The FR-201 `quire.ir-proof.obligation-identity/v1` row adds the function-value parity form: exactly `subject_kind`, `node`, `occurrence_key` {ordinal: 0, role: declaration} and `arguments` {parameter, position, type} in declared order, with no obligation kind. It agrees with QSL FR-484 and ADR-013 O-09 at QSL #1383 head b88c1ddf. The minting split (CG for the clause, application, function, parity and per-step forms; QSL alone for function-value parity) agrees with FR-484.
- The domain label is outside the hashed bytes (FR-201 "Identity and comparison"). The FR-484 vectors recompute as bare SHA-256 over the canonical text (hand-derived; see SR-16303).
- FR-201-AC-10 adds the parameter node id, position and declared type node as identity members, and adds the function-value form to the outside-preimage equality. TC-952 fixture V1 and vector OI-13 cover each changed member, the source_span equality and the separation from the operator form.
- No drop, narrowing or deferral language. OI-13 and V1 are new on no other open QSpec PR. PR 207 (draft) also touches FR-201, but not the obligation-identity row or AC-10.
- No test-parsed fixture or vector is edited, so no QSL conformance run is needed.

Gates: `quire validate --scope . 'spec/**/*.md'` gives exactly 86 failure lines. `cargo test --test checked_package_v2 --test backend_provider_v1` passes 24 + 134.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - | missing-requirement |

## Verdict

Mergeable on its own. It lands together with QSL #1383, which carries the blocking finding SR-16303 FND-001. If that fix changes the form (it should only add packet members to FR-484's retained inputs), re-check this PR.
