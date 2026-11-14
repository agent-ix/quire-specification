---
id: SR-202611141355001
title: "Base review of the STD-664 environment-refused cause"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; PR #653 at e410e74e, diff origin/main...HEAD: spec/objects/foundation/FR-272-typed-diagnostic-cause.md (environment_refused row, FR-272-AC-28, FR-272-AC-29), spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md (environment_refused row), proposals/quire-v1/definitions/native-diagnostics.md (environment_refused row), spec/test-cases/TC-047-emit-typed-located-causes.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-272, type: references }
  - { target: ix://agent-ix/quire-specification/FR-271, type: references }
  - { target: ix://agent-ix/quire-specification/TC-047, type: references }
  - { target: ix://agent-ix/quire-specification/TC-233, type: references }
---

## Summary

Ticket: STD-664. Base checklist over the FR-272 amendment that adds code `environment_refused`
with cause `environment-refused` and payload `{backend, resource}` for a host that denies a
resource a built backend needs (QSL FR-294, FR-295 rule 6, FR-560; decision accepted on QSL-602
2026-11-13). The three catalog rows agree on code, cause, category unsupported, `backend` in
`{aot, jit}` and the single-member `resource` set. One defect: the FR-271 row does not carry
what FR-272-AC-28 requires of it.

## Verdict

FAIL on FND-001 (medium). FND-002 and FND-003 are low.

Examined and clean:
- Author decisions: code `environment_refused`, cause tag `environment-refused`, `backend`
  `aot`/`jit`, `resource` closed set with only `executable-memory`, and the stated
  exclusions from `unimplemented_capability`, `resource_exhausted` and internal fault all match
  the QSL-602 accepted scope.
- FR-272-AC-25 is untouched; the `unimplemented_capability` rows still list exactly its three
  causes, and FR-272-AC-28's "neither row lists `environment-refused`" agrees with it.
- Schema `code`/`cause_tag` enum deferral: no AC in this repo ties the FR-271 values table to
  the V2 schema code enum (FR-322-AC-47 constrains pairings only), so deferring the
  `proposals/checked-package-v2/schema.json` change to the coding ticket breaks no criterion.
- "Category unsupported (exit 21)" in the FR-272 row is consistent with FR-306 ("unsupported
  (exit 21)"); the exit code is already stated in this repo, so the parenthetical is not new
  vocabulary.
- TC-233 step text and relationships cover FR-272-AC-28; TC-047 relationships and paired-failure
  row cover FR-272-AC-29.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-272-AC-28 requires the FR-271 values row to name the `backend` and `resource` payload keys and to state that `executable-memory` is the only member of the `resource` set; the FR-271 row says only "resource `executable-memory`" in prose, names no payload keys and states no closed set, so TC-233's AC-28 check fails against the PR's own text. Either extend the FR-271 row or drop FR-271 from AC-28's payload and closed-set clauses | spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md:78; spec/objects/foundation/FR-272-typed-diagnostic-cause.md:207 |
| FND-002 | low | The new payload key `resource` is not added to the "Canonical cause payload contract" ordered name list, so its position in canonical field order is undefined (FR-272-AC-27 added `request_index` and `family` there; `backend` is a pre-existing omission) | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:212-216 |
| FND-003 | low | The TC-047 paired-failure row asserts "removing the host denial removes it" (beyond FR-272-AC-29) and does not say how the test induces a host denial of executable memory | spec/test-cases/TC-047-emit-typed-located-causes.md:42 |
