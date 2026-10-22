---
id: SR-13810
title: "Integrity review of PR #325: retire the domain-package digest recompute on model intake"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@3a52c6820e06ce83c3bf888f950292f2c7205332; PR #325 diff against origin/main; FR-150, FR-154, FR-272, FR-321, FR-322, IT-010, TC-195, TC-234, TC-280, proposals/quire-v1/definitions/model-complete.md, native-diagnostics.md, proposals/checked-package-v2/model-member-type-vectors.json, tests/model_member_types.rs; context FR-323, FR-330, FR-331, FR-364, FR-462, TC-277, TC-428, TC-931, TC-933, TC-937; QSL qsl-package/src/emit.rs at c0d35b696^ and origin/main"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-154
    type: reviews
---
# Integrity review of PR #325

## Summary

Ticket: STD-126. The PR removes the model-intake check that recomputes the
`sha256-jcs` digest of a supplied domain-package document and refuses
`stale_dependency`/`content-mismatch` when it differs from the selected
digest. It deletes FR-322-AC-43, TC-280 MS-03, vector MS-03 and its test, and
edits FR-150, FR-154, FR-321, FR-322, FR-272, IT-010, TC-195, TC-234,
model-complete.md and native-diagnostics.md to match. It keeps
`digest-domain-mismatch`, `missing-selection`, `wrong-model-selection` and
`package_id` as the canonical identity.

Which check the ticket describes. The ticket says FR-322 read "compare[s]
each locked artifact's digest against caller-supplied evidence" and that the
evidence is "the emitter's own lock copied back (QSL own_evidence,
qsl-package/src/emit.rs:954-983)". At that revision (QSL c0d35b696^,
emit.rs:955-983) `own_evidence` inserts one artifact digest for each of
`lock.sources`, `lock.edition.definition`, `lock.definition_selections` and
`diagnostics.catalog`. It inserts nothing for `model_selections`, and no
domain-package document. So the check that passes by construction is the
source, definition and catalog digest-vs-evidence check, not the
domain-package recompute.

That check is already gone on main. PR #176 (61d08b7, titled with STD-126)
replaced FR-322's "recomputes all required raw and semantic digests before
graph admission" with the identity-member wording and added FR-322-AC-42 ("a
package whose lock names source and definition digests is admitted with no
source or definition bytes and no digest evidence supplied"). IR-457 and
QSL-343 (QSL #548, c0d35b696) removed the reader and emitter halves. QSL
`own_evidence` at origin/main now carries required features only.

PR #176 kept the domain-package recompute on purpose. It is "the one content
check the reader makes on that document" (main FR-322 Identity and
validation), and SR-860 records "Model selection makes one content check
(MS-03, `content-mismatch`)". That check does not pass by construction: the
caller supplies real document bytes, and the emitter's evidence never
included them. It is the conditional half of the ticket's ask ("and FR-154 if
it carries the domain-package digest re-verification"), and FR-154 did carry
it.

Conclusion: the PR does not remove the wrong check. The check the ticket
describes was removed earlier, by #176. This PR does the remaining FR-154
clause the ticket names. It is a correct, narrow fulfilment of what remained
of the ticket. It is not the ticket's core: that is already done on main.

The stale_dependency family the PR leaves alone (the owner scope question is
open; this review does not decide it):

| Family | Where | What it compares | Is it the ticket's check? |
| --- | --- | --- | --- |
| Dependency package binding | FR-322:124-138, FR-322-AC-84, FR-330:355-362 and FR-330-AC-9, TC-937, TC-931, TC-933, dependency-selection-vectors.json | a supplied dependency package's recomputed `package_id` against the lock's `dependency_selections` entry | No. It compares `package_id`, which the ticket says to keep as the canonical identity, against a separately supplied package. |
| Replay recompile identity | FR-323:68-81 (rules 4, 5), FR-323-AC-7, AC-9, TC-277, TC-278 | `package_id` recompiled from `byte_provision` sources against the request's `package_id` and dependency entries | No. Same `package_id` reasoning. |
| Replay byte provision | FR-323:83-97, FR-323-AC-5, TC-277 BP-03, BP-13 | raw-byte digest of a supplied source/definition entry (`byte-digest-mismatch`), and the `sha256-jcs` digest of a supplied domain-package document against its key (`content-mismatch`) | BP-13 is the same domain-package recompute this PR removes, on the replay path. The raw-byte check compares supplied bytes to their declared digest, not to evidence copied from the lock. |
| Backend provider and plugin wire | FR-331:142-148, FR-462:78-85, TC-428 | a decoded package's recomputed `package_id` against `request.package_id` | No. `package_id` reasoning. |
| Temporal replay state | FR-364:144, 166, FR-364-AC-4, TC-295 | a step's recorded post-state digest against its successors | No. It is not a package or artifact digest. |

If the owner's ruling reaches the byte-provision path, FR-323:94-97 and
TC-277 BP-13 are the closest match to what this PR removed.

Gates at the head: `quire validate --scope . 'spec/**/*.md'` reports 134
failing documents, the same count as main. The failures in touched files
(FR-272, FR-321 and FR-322 value-object `id` and record errors) are main's
existing ones. `cargo test --test checked_package_v2 --test
backend_provider_v1 --test model_member_types` passes (117, 14 and 23 tests).
No live reference to FR-322-AC-43, TC-280 MS-03, vector MS-03 or the deleted
test remains in spec/, proposals/ or tests/. The remaining mentions are in
historical reviews/ files, and TC-388's MS-03 is an unrelated row. Ids are
unique. The new wording describes behaviour.

## Verdict

Not mergeable as written. FND-001 is high: FR-150-AC-3 still requires the
refusal this PR removes. FND-002 and FND-003 are low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-150-AC-3 still requires "a stale package digest" to refuse normalization, while the FR-150 statement now says the digest is not recomputed from a supplied document, and TC-195 N05, its only row, no longer tests a stale digest | spec/functional/type-model/FR-150-preserve-model-normalization-provenance.md:158,133-135; spec/test-cases/TC-195-model-normalization-provenance.md:58 |
| FND-002 | low | The `content-mismatch` definitions in native-diagnostics.md and FR-272 drop the form "a selection's digest that differs from the digest recomputed from the supplied document", but FR-323 byte_provision and TC-277 BP-13 still refuse in exactly that form ("naming the selected and the recomputed digest") | proposals/quire-v1/definitions/native-diagnostics.md:67; spec/objects/foundation/FR-272-typed-diagnostic-cause.md:72; spec/objects/interfaces/FR-323-native-runtime-envelope.md:94-97; spec/test-cases/TC-277-replay-byte-provision.md:49 |
| FND-003 | low | FR-150's new clause "the digest is not recomputed from a supplied document" has no scope, while FR-323's replay byte_provision still recomputes a `sha256-jcs` domain-package document's digest. It should say that intake does not recompute it | spec/functional/type-model/FR-150-preserve-model-normalization-provenance.md:134-135; spec/objects/interfaces/FR-323-native-runtime-envelope.md:94-97 |

## Dispositions

Disposition pass 1, against PR head f14d1d8b11d8b8f40694c248777fbde513773202
(fix commit f14d1d8). The head contains origin/main (014f0ce). `quire
validate --scope . 'spec/**/*.md'` still reports 134 failing documents, the
same as main, and the touched files add no new failure. `cargo test --test
checked_package_v2 --test backend_provider_v1 --test model_member_types`
passes (117, 14 and 23 tests).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-150-AC-3 now reads "A wrong digest domain or a conflicting phase 4 derivation refuses normalization with its named cause, and a supplied document whose own canonical digest differs from the selected digest is admitted." TC-195 N05 gained a third case: "then with `A/x`'s multiplicity changed and the selection unchanged" expecting "admitted with three declarations, the changed multiplicity on `A/x` (FR-150-AC-3)". |
| FND-002 | fixed | native-diagnostics.md `content-mismatch` now includes "or, in replay byte provision (FR-323), a `sha256-jcs` domain-package document whose digest differs from the digest it is keyed by"; FR-272 includes "the digest a replay byte-provision domain-package entry is keyed by and the digest recomputed from its document". Both match FR-323 byte_provision and TC-277 BP-13. |
| FND-003 | fixed | FR-150 now reads "A ModelSelection whose digest domain is not `sha256-jcs` is refused `stale_dependency`/`digest-domain-mismatch`; model intake does not recompute the digest of a supplied document." The clause is scoped to model intake and no longer conflicts with FR-323 replay byte provision. |
