---
id: SR-881
title: "Spec review (integrity) — PR #174 Q9 ceremony deletion, STD-145 content-only ModelOwner, digest-chain deletion"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@1f0e91b; PR #174 diff against origin/main: spec/**, proposals/checked-package-v2/**, proposals/quire-v1/**, docs/**, plan/Plan-008 (deleted), README.md, AGENTS.md"
review_set: subset
---
# Spec review (integrity) — PR #174

## Summary

Ticket: STD-145 (pointer on QSL-395). This review covers the spec part of the
PR #174 diff at 1f0e91b, checked against the wave-b and wave-c owner rulings.
It looked at structure, consistency and references.

What was checked:

- **Rulings applied.** The diff deletes the following:
  - the stored rule-document digests in both locks;
  - `value-complete-rules.json`;
  - the digest/revision fields of `model-complete-rules.json`;
  - the successor-manifest rules;
  - tc_195, tc_233 and tc_281;
  - FR-201-AC-6;
  - the FR-090/FR-250 revision+digest keys (FR-090-AC-6, FR-250-AC-4);
  - the FR-322 catalog rev/SHA pin;
  - the capability ledger and routing ledger docs;
  - Plan-008;
  - FR-016, IT-001, NFR-001, FR-335 and interface_018;
  - the SHA256SUMS files.

  STD-145 removes `version` from `ModelOwner` in both schemas, the vectors,
  FR-322 owner recovery, AC-28, AC-29 and TC-279. A `model-owner-carries-version`
  schema refusal replaces the old version-only distinctness case.
- **Canonical content identity kept.** Node ids, `package_id`, the
  `quire.model.effective-declaration/v1` and `effective-view/v1` vectors
  (unchanged in this diff) and the model selection `digest` used by FR-322-AC-32
  intake all survive. MS-03's content check is left to #176, as the ruling says.
- **Validation.** `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md'`
  gives 140 failing documents on the head and 142 on main. The two fewer are
  the deleted FR-335 and interface_018. Comparing the normalized outputs shows
  no new error or warning.
- **References.** No AC id is cited without a definition. No relative
  Markdown link in a changed file points at a deleted file.

## Verdict

**Approve with changes.** There are two medium findings and four low findings.
FND-001 finishes the digest-chain deletion inside checked-package
`DefinitionRef`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The digest chain is only half deleted. Both V2 schemas still require every package `DefinitionRef` to carry `revision`, `digest_domain` and `digest`. Those are a version label and a raw-byte hash of a prose definition document, and they still enter `package_id`. The locks no longer hold them, AC-17 now matches by authority and identity only, and the FR-201 hashing sentence is gone. As a result nothing produces or checks these members, and a wrong value is admitted. A prose-document digest is version tracking, not content identity, so the fix is to delete the three members from `DefinitionRef` in both schemas, every fixture and vector, and the FR-322 text. That changes `package_id` and node-vector digests, which must be recomputed. Restoring a hash check is not the fix. | spec/objects/interfaces/FR-322-checked-package-artifact.md:134-141,673; proposals/checked-package-v2/schema.json:25; proposals/checked-package-v2/node-identity-preimage.schema.json:30 |
| FND-002 | medium | Pin and provenance records the Q9 sweep missed, two of them in files this PR edits: (a) `proposals/quire-v1/capabilities.md` keeps quire-research `blob/<sha>/<path>` links, which disclose the private tree's structure, plus SHA-pinned TL "owner revisions"; (b) `report.md` keeps a quire-research commit SHA and "pinned TL sources"; (c) `reviews/data/verification-reference-*.txt` and `-coverage.*` are committed gate logs. | proposals/quire-v1/capabilities.md:31,33,36-37; proposals/quire-v1/report.md:25-27; reviews/data/ |
| FND-003 | low | The `DefinitionRef` shape is inconsistent across the locks. In `complete-model-lock.json`, the `requires` entry is `{authority, identity}`, while its 18 catalog entries and all 20 `complete-value-lock.json` entries keep `revision`, which is a version label. Drop `revision` everywhere, together with FND-001. | proposals/quire-v1/definitions/complete-model-lock.json:5 |
| FND-004 | low | References to the deleted FR-016 and NFR-001 remain. Plan-005 has `relationships` targets `ix://agent-ix/quire-specification/FR-016` and prose citing FR-016 and NFR-001. The shared-reference fixture README cites FR-016/TC-029. `quire validate` does not flag them. | plan/Plan-005-fs05-interchange-acceptance/plan.md:9,28,35,63,96; plan/Plan-005-fs05-interchange-acceptance/tasks/Task-001-accept-fs05.md:11; proposals/shared-reference-2-draft/fixtures/README.md:27 |
| FND-005 | low | The spec index row for FR-032 still reads "Accepted by FS05 for explicit versioned evolution/migration". This PR retitled FR-032 to "Refuse unsupported required features" and removed its migration ACs. | spec/spec.md:899 |
| FND-006 | low | This PR rewrote FR-322-AC-28, but no test carries a `FR-322-AC-28` trace. The `model-owner-carries-version` refusal runs under the FR-322-AC-15 trace, so the matrix row "FR-322-AC-28 / TC-279" has no traced test. The trace gap predates this PR. | tests/checked_package_v2.rs:4251; spec/complete-v1/tests.md:969 |

## Dispositions

Round 1, reviewed at 87d32f40fd7ef0b86990e749067247cc821331d9. I recomputed with JCS SHA-256 all 64 preimage/sha256 vectors under `proposals/checked-package-v2/` (including the re-keyed `integer-div-floor`, `integer-div-euclidean`, `integer-div-truncating`, `text-eq-nfc` and `text-eq-binary-utf8` operation vectors) and all 5 fixture `package_id`s. Every one matches. `quire validate` reports 140 failing documents, the same as round 0, with no new errors.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 87d32f4 |
| FND-002 | fixed | 87d32f4 |
| FND-003 | fixed | 87d32f4 |
| FND-004 | fixed | 87d32f4 |
| FND-005 | fixed | 87d32f4 |
| FND-006 | fixed | 87d32f4 |
