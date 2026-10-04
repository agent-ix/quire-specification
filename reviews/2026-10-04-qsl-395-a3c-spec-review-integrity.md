---
id: SR-1291
title: "QSL-395 A3c spec review of quire-specification PR #187, recomputed-identity mismatches refuse content-mismatch"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@97cc67d9af00e10daf928ad3501264b77c7801a5; PR #187 diff against origin/main: proposals/quire-v1/definitions/native-diagnostics.md, spec/objects/foundation/FR-272-typed-diagnostic-cause.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/objects/interfaces/FR-323-native-runtime-envelope.md, spec/test-cases/TC-277-replay-byte-provision.md; leftover sweep over spec/ and proposals/ (reviews/ excluded); cross-repo check against QSL task/395-a3c-content-mismatch at b3a2a6e32"
review_set: subset
---
# QSL-395 A3c spec review of quire-specification PR #187

## Summary

Ticket: QSL-395 (A3c). PR: quire-specification#187, branch
task/content-mismatch-spelling, head 97cc67d. Spec-only.

Ruling checked (agreed with the owner, matches FR-272): `byte-digest-mismatch`
is a raw-bytes digest compared to its selection or declared reference;
`content-mismatch` is a selected or pinned content identity (a `sha256-jcs`
digest or a `package_id`) compared to the one recomputed from the subject.

Review set: base checklist plus integrity/consistency, with a cross-repo
check against the QSL A3c branch.

- **Changed sites.** FR-322 `dependency_selections` binding, FR-323 replay
  rules 4 and 5, FR-323-AC-7, and TC-277 BP-05, BP-07 and BP-11 all compare a
  pinned or declared `package_id` with a recomputed one. All now say
  `stale_dependency`/`content-mismatch`. Each follows the ruling.
- **Leftover sweep.** `byte-digest-mismatch` remains in spec/ and proposals/
  only at: the FR-272 and native-diagnostics cause enumerations, FR-272:155
  (generic digest-domain prose), the checked-package-v2 schema enum, and
  TC-277 BP-02 and BP-03. No other recomputed-identity site uses it. FR-154,
  FR-150, FR-321, FR-322-AC-43, FR-364, TC-280, TC-295 and model-complete.md
  already say `content-mismatch` for a recomputed `sha256-jcs` digest. BP-02
  and BP-03 are wrong in other ways (FND-001, FND-002).
- **Catalog edits (FR-272 and native-diagnostics.md).** They go beyond the
  brief, but they are needed. Before this PR the catalog defined
  `content-mismatch` only for a retained identity or a selection's digest, so
  the new `package_id` spelling in FR-322 and FR-323 had no catalog meaning.
  The two edits agree with each other and with the ruling, and the closed
  cause sets are unchanged. One placement nit (FND-004).
- **Cross-repo (QSL A3c at b3a2a6e32).** QSL `DependencyIdentityMismatch`
  and `PackageIdMismatch` docs now say `content-mismatch`, which matches
  FR-323. QSL FR-071:109 and `ReplayRequest::decode` split a byte-provision
  digest mismatch by domain: raw-byte domain is `byte-digest-mismatch`,
  `sha256-jcs` is `content-mismatch`. QSpec BP-03 does not (FND-002). QSL
  ADR-013:937 is not edited on the A3c branch. It still spells both a missing
  input and a bytes/digest mismatch `content-mismatch`. QSL code refuses a
  missing input as `IncompleteByteProvision` with code `missing_declaration`
  (FND-001).
- **Validation.** `cargo test --test checked_package_v2` (which compiles
  native-diagnostics.md and FR-272) was run under locked-build.sh at 97cc67d: 71
  passed, 0 failed. `quire validate --scope . 'spec/**/*.md'` on head
  and on origin/main: both 145 documents failed structural validation, and
  the sorted outputs (paths normalised) are identical. No new failure.

## Verdict

Changes requested: two medium and two low findings. Every changed site
follows the ruling, and the catalog edits are consistent and needed. The two
mediums are both in TC-277 and both disagree with QSL. BP-02 spells a missing
input as a digest mismatch, which the catalog's own row rules out ("A missing
artifact cannot supply an invented actual digest"). BP-03 spells a
`sha256-jcs` entry's mismatch `byte-digest-mismatch`, against the ruling and
against QSL FR-071 and the A3c code.

Recommendation for BP-02, for both repos: `missing_import`/`missing-selection`,
retaining the requested `RawSourceRef` digest record (domain and digest) and
its locus in the `package` reference. FR-271 defines `missing_import` as "a
requested typed dependency that is not present", which fits an input
requested by digest. It also matches FR-323 rule 2 and BP-09, where a removed
dependency entry is `missing_import`/`missing-selection`. BP-01 already says
"no missing-input refusal". `missing_declaration` (QSL's current code) is "a
requested name", and no name is involved. QSL then needs: ADR-013:937 split
into a missing input (`missing_import`/`missing-selection`), a raw-byte
mismatch (`byte-digest-mismatch`) and a `sha256-jcs` mismatch
(`content-mismatch`), and `IncompleteByteProvision` moved to
`Code::MissingImport` with the `missing-selection` cause.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-277 BP-02 expects a `byte_provision` missing an input named by a `RawSourceRef` digest to refuse `stale_dependency`/`byte-digest-mismatch`. A missing input has no actual bytes, so there is no actual digest to compare. The catalog row says "A missing artifact cannot supply an invented actual digest", and BP-01 calls this case a "missing-input refusal". The two repos disagree three ways: QSpec says `byte-digest-mismatch`, QSL ADR-013:937 says `content-mismatch`, and QSL code (`IncompleteByteProvision`) says `missing_declaration`. **Fix:** BP-02 expects `missing_import`/`missing-selection` naming the requested digest record, before recompilation. QSL aligns ADR-013:937 and `IncompleteByteProvision` to the same spelling. | spec/test-cases/TC-277-replay-byte-provision.md:39; proposals/quire-v1/definitions/native-diagnostics.md:67; spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md:44 |
| FND-002 | medium | TC-277 BP-03 expects any `byte_provision` entry whose bytes do not match its declared digest to refuse `byte-digest-mismatch`. FR-323 says the provision also carries domain-package inputs, which are keyed by a `sha256-jcs` digest. Under the ruling, a `sha256-jcs` digest that differs from the one recomputed from the document is `content-mismatch`, as FR-322-AC-43 and FR-154 already say. QSL FR-071:109 and the A3c `ReplayRequest::decode` refuse exactly that case `content-mismatch`. The PR body says BP-03 covers source bytes only, but the vector does not say so. **Fix:** limit BP-03 to an entry under a raw-byte digest domain, and add a vector: a `sha256-jcs` domain-package entry whose document recomputes to another digest refuses `stale_dependency`/`content-mismatch` naming the selected and the recomputed digest. | spec/test-cases/TC-277-replay-byte-provision.md:40; spec/objects/interfaces/FR-323-native-runtime-envelope.md:68-72 |
| FND-003 | low | BP-05 now expects the refusal to name the declared and the recompiled `package_id`. It traces to FR-323-AC-5, but AC-5 covers only provision completeness and `RawSourceRef` digests. It does not state rule 5 (the proved `package_id` comparison) or its payload, so no AC states what BP-05 checks. This gap predates the PR, but the PR widened BP-05's expectation. **Fix:** add the rule-5 refusal and its two named ids to FR-323-AC-5, or to a new AC that BP-05 traces to. | spec/test-cases/TC-277-replay-byte-provision.md:42; spec/objects/interfaces/FR-323-native-runtime-envelope.md:63-67,124 |
| FND-004 | low | FR-272's third column is "Required payload". The PR adds a sentence there that defines `byte-digest-mismatch` ("is a raw-bytes digest that differs from…"). That is a meaning, not a payload. native-diagnostics.md's column is "cause distinction and required payload", where the sentence fits. **Fix:** state the payload in FR-272 ("for `byte-digest-mismatch`, the declared digest and the digest of the supplied raw bytes"), and leave the definition to the catalog row and FR-271. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:54,62 |

## Dispositions

Round 1, reviewed at 659cfce78bcb6c156f263633854cd8bd5a85ad1c (fix range
97cc67d..659cfce). Each fix was checked against the diff. `quire validate
--scope . 'spec/**/*.md'` at 659cfce gives output identical to 97cc67d's
(145 failures that were already there, none new). No new findings this round.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 659cfce |
| FND-002 | fixed | 659cfce |
| FND-003 | fixed | 659cfce |
| FND-004 | fixed | 659cfce |
