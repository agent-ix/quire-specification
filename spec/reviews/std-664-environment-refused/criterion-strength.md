---
id: SR-202611141355003
title: "Criterion strength review of the STD-664 environment-refused cause"
type: SpecReview
analysis: criterion-strength
scope: "agent-ix/quire-specification; PR #653 at e410e74e, diff origin/main...HEAD: spec/objects/foundation/FR-272-typed-diagnostic-cause.md (environment_refused row, FR-272-AC-28, FR-272-AC-29), spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md (environment_refused row), proposals/quire-v1/definitions/native-diagnostics.md (environment_refused row), spec/test-cases/TC-047-emit-typed-located-causes.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-272, type: references }
  - { target: ix://agent-ix/quire-specification/FR-271, type: references }
  - { target: ix://agent-ix/quire-specification/TC-047, type: references }
  - { target: ix://agent-ix/quire-specification/TC-233, type: references }
---

## Summary

Ticket: STD-664. Jev is not installed on this host (`jev not found`), so the Jev-calibrated
judgement was skipped and the criteria were judged manually against spec text only.

FR-272-AC-28 can fail: a missing cause, an extra cause, a missing `backend` or `resource` key,
a second `resource` member, a missing category or `environment-refused` appearing in the
`unimplemented_capability` or `resource_exhausted` row each fail it. FR-272-AC-29 can fail: a
`Fault`, `unimplemented_capability` or `resource_exhausted` diagnostic, a wrong `backend` or
`resource`, or a diagnostic that changes under a larger work budget each fail it. Adverse cases are
covered by TC-047's paired row (unbuilt backend, untranslatable entry, small budget).

## Verdict

PASS with one low ambiguity.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-272-AC-28's last clause "neither the `unimplemented_capability` row nor the `resource_exhausted` row lists `environment-refused`" does not say which documents' rows; TC-233 reads it as both, but a checker that inspects only FR-272 satisfies the text | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:207 |
