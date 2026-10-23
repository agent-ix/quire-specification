---
id: SR-16281
title: "spec-review of Bytes, Duration and JsonObject native scalars (quire-specification PR 408)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@7b474f7f; spec/functional/type-model/FR-154-admit-domain-package-model.md; spec/objects/interfaces/FR-322-checked-package-artifact.md; spec/functional/type-model/FR-149-apply-complete-equality-matrix.md; spec/test-cases/TC-233-checked-package-and-scalar-contract-vectors.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-149
    type: references
---

# Spec review of Bytes, Duration and JsonObject native scalars (quire-specification PR 408)

## Summary

Ticket: STD-399. One spec-review pass over the PR's own delta (`git diff the head of #419...7b474f7f`), checked for the no-drop/no-narrowing/no-deferral rule, testable vectors (key vectors derived by hand), id uniqueness across open PR heads, and schema/vector consistency.

Examined, clean:

- Intake (FR-154-AC-21/22, TC-234 I15/I16), the Bytes and JsonObject literal spellings, and the KS-01/KS-02 equality results are consistent. JCS canonical text as the JsonObject payload makes equality an equivalence.
- TC-280 rows are now MT-20 to MT-23, with no collision with #409's MT-19. Gates at 7b474f7f: quire validate 86; cargo 134 + 24 passed.
- QSL conformance at QSL ef43bab4 with QSPEC_DIR at 7b474f7f: checked_v2 47 passed, 0 failed.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | A Set<Bytes> (MT-22) or a Set/Bag of JsonObject is admitted, but FR-144's canonical-key table has no Bytes or JsonObject row, and FR-181's typed canonical form has no `bytes` or `json_object` form. So the visiting order, the state-key encoding and the supplied-value form of such a collection, or of any state holding a Bytes or JsonObject field, are undefined. FR-149's key sentence is not FR-144's table. Fix: add FR-144 key rows and FR-181 forms (for example `{"type":"bytes","value":"<hex>"}` and `{"type":"json_object","value":"<JCS text>"}`) with vectors. | spec/objects/interfaces/FR-322-checked-package-artifact.md | wrong-requirement |
| FND-002 | high | FR-142 and FR-322-AC-99 make a source unit and a package unit of one name two distinct units that are never ambiguous. Here, a Duration member of a package refuses whenever the unit of compilation also declares its own `s`, which is the common case. That makes Duration unusable there for good. With #407, the rule "a base dimension whose declared name is Time" also matches every owner's Time. Fix: resolve a package member's Duration to its own selection's `s` (`ix://quire/unit/<p>/s`) when that exists, and use the source `s` only for a package that declares none. | spec/objects/interfaces/FR-322-checked-package-artifact.md | wrong-requirement |
| FND-003 | high | The new identities are prose only. The schemas' OperationIdentity enums do not list them (the body says "until STD-417"), so a package that uses them fails the V2 schema this PR ships. That is the same pending enum edit as #419 FND-001. It must be in the stack before merge. | spec/objects/interfaces/FR-322-checked-package-artifact.md | missing-requirement |
| FND-004 | medium | `2` is not a Bytes payload: a payload has two hex digits per octet, so the literal spelling rule refuses it. Use valid payloads, for example `10` before `2000` and `00` before `0000`. | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md | missing-requirement |
| FND-005 | medium | Duration means seconds only because of the unit's name. A package `s` of Time that is not the 1-second unit (for example a non-root `s` with scale 60 relative to its root) makes the "exact rational seconds" claim and FCD's ISO 8601 seconds wire form false. Require `s` to be the root of Time or to have scale 1 to the root, or refuse otherwise. | spec/objects/interfaces/FR-322-checked-package-artifact.md | wrong-requirement |

## Verdict

Not mergeable: FND-001, FND-002, FND-003 (high) block the PR; FND-004, FND-005 (medium) should be fixed in the same round.
