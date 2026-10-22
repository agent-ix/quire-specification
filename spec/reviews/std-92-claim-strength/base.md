---
id: SR-622
title: "Base review of the claim-strength ownership move"
type: SpecReview
analysis: base
scope: "PR #154 (STD-92) at 41a2e0e: FR-281 recast from a local enumeration to a value object carrying the Engineering Assurance FR-022 vocabulary, with the FR-061, FR-240 and spec/spec.md amendments"
review_set: base
review_date: "2026-09-22"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-281, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-061, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-240, type: reviews }
---
## Summary

The change moves ownership of the four claim strengths to Engineering Assurance
FR-022 and leaves FR-281 stating only how a Quire result carries one: source
binding, exact-string identity, exactly one strength per advanced result, and
no ordering, dominance or promotion. FR-022 exists on Engineering Assurance
`main` and declares the same four wire spellings and the same unordered,
single-valued discipline, so behaviour is unchanged. FR-281-AC-3 is preserved
verbatim and no local member table remains.

Three coverage and wording gaps were found and fixed on the branch.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-281-AC-1..AC-3 named no test case and were absent from the protocol matrix and TC-061's criteria list, although `complete-v1/tests.md` already bound them to TC-061. Fixed: each AC cites TC-061 and both protocol matrix rows list AC-1..AC-4. | FR-281; TC-061 |
| FND-002 | low | TC-061 still said "enumerate every FR-281 strength" after FR-281 stopped enumerating, and exercised neither the reworded AC-1 nor AC-2/AC-3. Fixed: TC-061 enumerates the FR-022 strengths carried under FR-281, refuses variant labels, and checks non-promotion and no key-order dominance. | TC-061; FR-281 |
| FND-003 | low | FR-240's reading-aid note kept "where the two differ" after its claim-strength table was removed, leaving "the two" ambiguous. Fixed: the note names FR-280 as governing. | FR-240 |

## Checklist

| Gate | Result |
| --- | --- |
| ID formats | FR-281-AC-1..AC-4 unchanged; no id minted besides this review. |
| Link integrity | `ix://agent-ix/engineering-assurance/FR-022` resolves to an artifact on that repository's `main`. |
| Duplicate definition | None: FR-281, FR-240 and FR-061 name FR-022 and carry no member table. |
| Coverage | Every FR-281 AC maps to TC-061 in `spec/protocol/tests.md` and `spec/complete-v1/tests.md`. |
| Validation | `quire validate` reports no new diagnostic against `main`; the value_object record-schema error on FR-281 is the repository-wide condition every value_object already carries. |
