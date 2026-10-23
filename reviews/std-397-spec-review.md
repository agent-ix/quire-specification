---
id: SR-16280
title: "spec-review of IEEE negate and float equality and ordering (quire-specification PR 419)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@eef398e9; spec/functional/type-model/FR-148-evaluate-ieee-floating-profiles.md; spec/functional/type-model/FR-149-apply-complete-equality-matrix.md; spec/objects/interfaces/FR-322-checked-package-artifact.md; spec/test-cases/TC-194-complete-equality-matrix.md; spec/test-cases/TC-233-checked-package-and-scalar-contract-vectors.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-148
    type: references
  - target: ix://agent-ix/quire-specification/FR-149
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---

# Spec review of IEEE negate and float equality and ordering (quire-specification PR 419)

## Summary

Ticket: STD-397. One spec-review pass over the PR's own delta (`git diff the head of #409...eef398e9`), checked for the no-drop/no-narrowing/no-deferral rule, testable vectors (key vectors derived by hand), id uniqueness across open PR heads, and schema/vector consistency.

Examined, clean:

- Hand-derived and correct: the negation patterns (sign flip, NaN payload and quiet bit kept); F34's 17 results; E38's 9; E39's 11; E40's 9. In particular `set[+0] = set[-0]` is false and `!=` true, `set[qnan] = set[qnan]` is true, `contains(set[+0], -0)` is false while `+0 = -0` is true, and `sequence[qnan] = sequence[qnan]` is false. The `<` and `<=` compositions give `-0 < +0` false and `-0 <= +0` true.
- The two relations never mix: numericEqual applies through records, options, unions and sequences, and membership applies inside a collection. The rule is consistent with FR-144 keys and with #403's removal-filter exclusion.
- Gates at eef398e9: quire validate 86; cargo 134 + 24 passed. QSL conformance: see the report.
- QSL conformance at QSL origin/main ef43bab4 with QSPEC_DIR at eef398e9: cargo test -p qsl-package --lib checked_v2 47 passed, 0 failed.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | The OperationIdentity enums of both checked-package-v2 schemas at this head do not list `quire.op.ieee.float32/64.negate`, and they do not list #408's `quire.op.bytes.*` and `quire.op.json_object.*`. So NF-01 and FR-322-AC-111's admitted application fail against this branch's own schema. The PR body leaves the enum edit to a later step (after QVC#21 and the owner's lock move), and no open branch carries it. That is a pending-dependency deferral. Not done until a commit with exactly the listed enum additions is in the stack, after QVC#21 merges and before this PR merges. The listed additions (negate after each fma; bytes.eq/ne/length/concat and json_object.eq/ne after timestamp.ne) match the prose of #419 and #408. | spec/objects/interfaces/FR-322-checked-package-artifact.md | missing-requirement |
| FND-002 | low | E38 lists nine comparisons but says "the same eight" for Float32. Say nine. | spec/test-cases/TC-194-complete-equality-matrix.md | wrong-requirement |
| FND-003 | low | The operators are called "the IEEE quiet comparisons", but IEEE 754 quiet predicates signal invalid on a signaling NaN, and these set no flag. State that this departs from IEEE on purpose (comparisons consume no NaN and set no flag), so an implementer does not "fix" it. | spec/functional/type-model/FR-148-evaluate-ieee-floating-profiles.md | wrong-requirement |

## Verdict

Not mergeable: FND-001 (high) block the PR.
