---
id: SR-2026103110
title: "Base spec review of STD-611 (QSpec PR #551)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; FR-322 model-owned field projections, TC-281 projection vectors, checked-package V2 schema, tests/model_member_types.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-281
    type: reviews
---

## Summary

The base review examined the new field-read discriminator and its closed wire values, requirement, vectors, and tagged qualification test. It found one unbounded applicability edge for read_form on non-model record projections.

## Coverage

- Reviewed FR-322's model-object field projection rule and FR-322-AC-34.
- Checked TC-281 RP-09 through RP-13, the OperationMember schema, and tests/model_member_types.rs.
- quire matrix --scope . --format tsv reports FR-322-AC-34 as tagged by tests/model_member_types.rs:985:1.
- git diff --check origin/main...HEAD passed.
- FR-322 validation at PR head 188f4b238a5870569da582e0aacf9e9c40d93510 and base c5deb1106af66bc6b414f1ce85ec83c2775022ba both exited 3. After normalizing the repository path, diagnostics were byte-identical: the same two structural errors (missing id; record requires fields) and the same three warnings (legacy Properties form; EARS unclassifiable; EARS missing subject). No new FR-322 diagnostics were introduced.
- No applicable AssuranceProfile was found under spec/; direct review applies.

## Verdict

Finding to fix. FR-322 uses “inapplicable” for rejected read_form values but does not say whether a field projection over a non-model record may carry this model-source discriminator.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | State that a non-model record projection carrying read_form is refused as malformed_wire, and add a vector. The current schema permits the optional member on every field OperationMember, while FR-322-AC-34 only says a model-object projection requires it and tests inapplicability on an undereferenced reference. | FR-322-AC-34; OperationMember |


## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The fix states that read_form is reserved for model-object projections over model.deref and ordinary record projections must omit it; carrying it refuses malformed_wire. RP-14 covers this malformed case. |
