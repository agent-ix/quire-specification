---
id: SR-592
title: "Evidence-method review of complete scalar contracts"
type: SpecReview
analysis: evidence
scope: "QSpec #68 scalar property, vector, boundary and consumer qualification evidence"
review_set: all
evaluated_revision: "cd4c0a0 plus QSpec #68 working-tree remediation"
review_date: "2026-09-15"
relationships:
  - target: ix://agent-ix/quire-specification/issues/68
    type: reviews
---

## Summary

PASS for preimplementation evidence design. TC-185–194 define generators,
oracles, shrinkers, canonical examples, adverse definition closures and named
resource seams. They remain planned until QSL implements them. I04 schema/vector
qualification is executable now and does not self-attest the future QSL evaluator.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-5921 | high | Resolved: implementation-relative exact/one-less language was replaced by named NFR-071 charge denial with atomic outcome assertions. | value-accounting.md; TC-185–194 |
| FND-5922 | high | Resolved: I04 V2 enum member, derived dimension and affine-unit vectors plus evaluator-owned compound-unit vectors have executable stale/noncanonical mutations in their separate identity domains. | node-identity-vectors.json; value-compound-unit-vectors.json; TC-217; checked_package_v2.rs |
| FND-5923 | medium | Resolved: decimal loss tuples, valid UTF-8 lexicographic bytes, full IEEE classes and same-type equality positives/negatives have concrete oracles. QSL #118 unit/text amendment (PR #72 review): text length after normalization, text bound positions, profile-specific text charges, unit charge-order boundaries, edge paths and quantity equality and ordering have concrete limit vectors including integer-target positions and composite quantity equality (TC-186 T13–T16; TC-187 U14–U29; TC-194 E24). | TC-185; TC-186; TC-193; TC-194 |
| FND-5924 | low | No TC-185–194 status was promoted; implementation tags and runtime evidence remain QSL #118/#119 work. | spec/complete-v1/tests.md |
| FND-5925 | high | Resolved: the qualification fixture verifies all 21 catalog artifacts byte-for-byte and proves retained stale definition and rule digests are rejected. F1 re-review adds exact catalog completeness, one-slot occupancy, closed trigger and selection-code assertions plus accepted/refused selection vectors; F5 re-review binds them to FR-201-AC-6, now owned by TC-233 rather than the FR-311 corpus case TC-231, through `#[trace]` attributes. | complete-value-lock.json; complete-value-selection-vectors.json; checked_package_v2.rs |
| FND-5926 | high | Resolved: IEEE comparison, classified-exceptional and finite-arithmetic paths have total conditional charge schedules; finite irrational square root has a fixed-width allowance and concrete denial vector rather than an impossible rational-size oracle. QSL #118 IEEE amendment: the classified path is limited to NaN, infinity, invalid and divide-by-zero outcomes so zero operands take the finite path; cross-width comparison is type-time `ill_typed`; strict `exact` refuses after `ieee.round` with `nearest-even` would-be flags only; width and exact-rational conversions have stated per-point widths; NaN width conversion keeps sign and payload or refuses an unrepresentable payload after `ieee.operands`; a reserved-intrinsic binding refuses with `conflicting-definition`. PR #73 review: IEEE §6.3 signed-zero results, mixed-width and IEEE-with-exact static refusals, IEEE-to-exact `maxparts` exact-intermediate sizing, exact-to-IEEE source sizing on the retained representation, the closed flagless `ieee_nan_payload_not_representable` language-policy refusal and every conversion path have hand-checked limit vectors. PR #73 re-review: §6.3 exact-zero sum/difference and FMA signs are stated with subtraction vectors, and IEEE-to-exact targets a grammar `Rational[..]` with charge-free membership refusal before retention while direct IEEE-to-`Decimal`/`Integer`/`Int[..]` is `ill_typed`. | value-accounting.md; FR-148; TC-193 F17–F31 |
| FND-5927 | medium | Resolved: definition-lock qualification checks path, role, identity, revision namespace/value, digest domain and raw bytes, with independent negative mutations for each field. F4 re-review: owner-to-lock joins are tested in isolation for source, definition, edition, model and model-export mismatches with lock and identity preimage kept consistent; every adverse, node-mutation, compound-unit, migration-refusal and selection vector list is asserted as an exact name/kind inventory. PR #70 re-review: each semantic mutation names its refusing check (`refused_by` schema or semantic) and is asserted refused by exactly that check; the owner-join mutation asserts the base joins and the mutant fails the nominal package lock join. | complete-value-lock.json; node-identity-vectors.json; value-compound-unit-vectors.json; checked_package_v2.rs |
| FND-5928 | high | Resolved: exact V1 and V2 positive fixtures prove both schemas remain valid while each rejects the other and V1 rejects a V2-only node member. | FR-322-AC-11; TC-217; checked_package_v1.rs; checked_package_v2.rs |
| FND-5929 | high | Resolved: independent SHA-256 locks cover all four frozen V1 publication files; a consistently V2-relabeled nominal V1 graph still refuses absent preimages; and the migration schema/vectors prove source/target package keys, total ordered node correspondence with basis identical exactly when digests match (flipped bases refuse) for identical and nominal rekeyed migrations, and exact typed refusal code and subject for missing, stale, ambiguous, incompatible and byte-only reconstruction. PR #70 re-review: missing and stale vectors cover edition and every selection list, two-source vectors prove missing precedes ambiguous and stale, V2 package ids are recomputed from lock and graph with excluded-member invariance, injected-excluded-member refusal and named-member sensitivity, V1 fixture ids (including the nominal migration source) bind their stored preimages, and equal digest bytes under the V1/V2 package domains compare unequal (FR-322-AC-14/16, FR-201-AC-7, TC-233). | FR-201-AC-7; FR-322-AC-13–16; TC-233; migration-correspondence-vectors.json; checked_package_v2.rs |
