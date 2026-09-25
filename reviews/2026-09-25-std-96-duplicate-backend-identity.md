---
id: SR-628
title: "Spec review — STD-96 duplicate backend identity registration"
type: SpecReview
analysis: failure-domain
scope: "STD-96; branch std-96-duplicate-backend-id against origin/main acb8cf2: FR-290 Candidate set and negotiation (duplicate-identity rules, registration-cause scope), FR-290-AC-7 and new FR-290-AC-9 and AC-10, new TC-282, TC-271 CK-04, FR-272 duplicate-backend retention and FR-272-AC-9 verification, proposals/quire-v1/definitions/native-diagnostics.md, spec/protocol/tests.md and spec/complete-v1/tests.md matrix rows"
review_set: subset
---
# Spec review — STD-96 duplicate backend identity registration

## Summary

Base checklist plus failure-domain review of the STD-96 diff against
`origin/main` acb8cf2. The review covers only the diff.

The defect: FR-290 stated that registration order never changes a candidate
set, and also refused a repeated backend identity while the registration
already held stood. With two manifests under one identity, the held manifest,
and so the candidate's digest, was whichever registered first.

The rule adopted: the registry snapshot is a function of the set of admitted
registrations. Repeats of one identity with one manifest digest are one
registration and are not refused. Repeats with distinct digests, or with one
digest over differing manifests, refuse every registration of the identity
with the existing `invalid_capability`/`duplicate-backend`, once per distinct
digest, and the identity is unregistered. No new code or cause is added. The
rule keeps the existing locality: registrations of other identities stand, as
for every other registration cause.

Rejected in review: refusing the whole registry build. It would make one bad
backend drop every other backend, contradicting CK-04's "other registrations
stand". A smallest-digest-wins rule is order-independent but silently picks a
manifest the producer may not intend; a refusal names the conflict.

Checklist results: FR-290-AC-9 and AC-10 are testable and each backed by
TC-282; TC-282 verifies FR-290 and FR-272; both matrices carry the new rows;
IDs TC-282 and SR-628 were checked free on every local and remote branch.
`quire validate` on TC-282 reports no finding. FR-290, FR-272 and
`spec/protocol/tests.md` fail structural validation on `main` for existing
reasons (missing `id` for their object archetype, model-cell identifiers,
matrix column heading); the diff adds none.

## Verdict

**PASS after fixes**. All findings are fixed on this branch.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-290-AC-9 and AC-10 were inserted above AC-8, so the criteria table was out of order. Fix: move AC-8 above AC-9. | FR-290 Acceptance Criteria |
| FND-002 | medium | The same-digest, differing-manifest rule said "any other manifest member", but the QSL registry holds a descriptor (identity, digest, tool, advertised pairs), not the full manifest, so an implementer could not tell what to compare. Fix: "differ in any other member the registration carries, such as its pinned tool identity or its advertised pairs". | FR-290 Candidate set and negotiation |
| FND-003 | medium | A registration admitted before a conflicting one is withdrawn, but the text did not say where its refusal is reported; an incremental registry could return the second refusal only from the call that caused it and lose the first. Fix: the snapshot reports its registration refusals, so a withdrawn registration's refusal appears in every snapshot after the conflict. DB-08 checks it. | FR-290 Candidate set and negotiation; TC-282 DB-08 |
| FND-004 | low | DB-07's malformed registration `M` had no digest, so its refusal key and report order were undefined, and it did not show that a refused registration with a different digest creates no conflict. Fix: give `M` digest `d2` and key its refusal (`a`, `d2`). | TC-282 Test Procedure, DB-07 |
| FND-005 | low | DB-08 is a sequence, not a set, so the procedure's "every ordering" did not state what order it runs in. Fix: run DB-08 with `A1` first and with `A2` first. | TC-282 DB-08 |

## Dispositions

| ID | Disposition |
| --- | --- |
| FND-001 | Fixed in this PR. |
| FND-002 | Fixed in this PR. |
| FND-003 | Fixed in this PR. |
| FND-004 | Fixed in this PR. |
| FND-005 | Fixed in this PR. |
