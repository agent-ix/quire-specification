---
id: TC-277
title: "Replay byte provision and source digests"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: verifies
---

# TC-277: Replay byte provision and source digests

## Description

Show that an FR-323 replay request's `byte_provision` supplies every source and
definition input the replay recompiles, keyed by declared
digest domain and digest, and every domain-package document, an entry of the ordered provision located by the identity it declares, with no version or digest compared, and that the `package` reference's `RawSourceRef`
digests are exactly the digests the executor recomputes and compares, per
QSL ADR-013 O-26 and QC-1.

## Preconditions

- QSL ADR-011 §1: stages S1 to S4 recompile a checked package from
  digest-addressed source.
- FR-322 owns the checked-package reference and its digest domains.

## Test Procedure

Build FR-323 replay requests whose `byte_provision` supplies a complete,
digest-matching set of inputs, and the mutations that omit an input, supply
mismatched bytes, or name a digest domain the input does not carry. Execute
each vector through the real local Rust boundary and read back the
recompilation outcome.

## Vectors

| Vector | Mutation | Expected result | Criteria |
| --- | --- | --- | --- |
| BP-01 | A `byte_provision` naming every source and definition input by matching digest domain and digest and every selected domain-package document as an entry located by its declared identity | Recompilation proceeds; no missing-input refusal | FR-323-AC-27 |
| BP-02 | A `byte_provision` missing one input the `package` reference's `RawSourceRef` digests name | Refused `missing_import`/`missing-selection` naming the requested digest record (domain and digest) and the `package` member that names it, before recompilation | FR-323-AC-5 |
| BP-03 | (BLOCKED-BY STD-583, owner: planner titan:QSL:plan; `byte-digest-mismatch`, `RawSourceRef`) A `byte_provision` entry under the raw-byte domain `quire.source.bytes/v1` whose bytes do not match its declared digest | Refused `stale_dependency`/`byte-digest-mismatch` naming the declared digest and the digest of the supplied bytes, before recompilation | FR-323-AC-28 |
| BP-04 | (BLOCKED-BY STD-583, owner: planner titan:QSL:plan; `digest-domain-mismatch`, `quire.source.bytes/v1`) A `byte_provision` entry under the wrong declared digest domain | Refused `stale_dependency`/`digest-domain-mismatch` | FR-323-AC-28 |
| BP-05 | A `byte_provision` whose bytes recompile to a `package_id` that differs from the `package` reference's declared `package_id` | Refused `stale_dependency`/`content-mismatch` (QSL's `PackageIdMismatch`) naming the declared `package_id` and the recompiled one, before any result is produced | FR-323-AC-9 |
| BP-06 | A proved package with two direct `dependency_selections`, one of whose dependencies imports a third library; the `package` reference names the proved package's `sources` and one `dependencies` entry per identity in the derived closure, in ascending UTF-8 identity order, each with its own lock `sources`; the provision carries every source, including the transitive-only library's | Each dependency recompiles from its entry's sources and is supplied to its direct or transitive imports; the proved package recompiles to the declared `package_id` | FR-323-AC-7 |
| BP-07 | BP-06 with one dependency entry's source bytes replaced by an edit to the body of a declaration the proved package does not call, which changes that dependency's `package_id` and leaves the recompile admitted (digests updated to match the new bytes); then an edit removing the export the proved package calls | Refused `stale_dependency`/`content-mismatch` naming that entry's `identity`, its declared `package_id` and the recompiled one, before the proved package's `package_id` is compared; no result. The removal instead refuses in the recompile, `missing_declaration`/`missing-name` at the call in the proved source, before that rule; no result | FR-323-AC-7 |
| BP-08 | BP-06 with its first and second `dependencies` entries swapped (the third stays in place); then with one entry repeated; then with an extra entry no import reaches | Refused `invalid_package`/`invalid-value` at `/package/dependencies`, the first two before any recompile | FR-323-AC-7 |
| BP-09 | BP-06 with one `dependencies` entry removed | Refused `missing_import`/`missing-selection` at the import the entry supplied | FR-323-AC-7 |
| BP-11 | BP-06 with one entry's `package_id` changed and its sources intact | Refused `stale_dependency`/`content-mismatch` naming that entry's `identity`, its `package_id` and the recompiled one; no result | FR-323-AC-7 |
| BP-12 | BP-06 with a second entry whose source has the authority and identity of the first entry's source | Refused `invalid_package`/`conflicting-definition` naming both entries, before any recompile | FR-323-AC-7 |
| BP-14 | BP-06 with the transitive-only entry C's sources edited by an edit B's recompile admits so they recompile to a new `package_id`, and C's declared `package_id` updated to match, while the selecting dependency B's sources and declared `package_id` stay unchanged | B recompiles against the edited C to a `package_id` different from B's declared `package_id`; refused `stale_dependency`/`content-mismatch` naming B's `identity`, B's declared `package_id` and its recompiled one; no result | FR-323-AC-7 |
| BP-15 | BP-06 with the proved package a unit whose lock carries `refinement_internals` (`R`: `Spec::Twice::finish`) and one dependency whose own lock carries one; the `package` reference and that `dependencies` entry carry the lock values; then each of: `package.refinement_internals` removed; the dependency entry's member removed; a spelling changed to another resolving class; a spelling naming no node; then the same in the dependency's own member | The first replay reaches the proved `package_id`. The removed unit member and the changed spelling each refuse `stale_dependency`/`content-mismatch` naming the request's `package_id` and the recompiled one; the removed dependency member names that entry's `identity`, `package_id` and the recompiled one; the unresolving spelling refuses `missing_declaration`/`missing-name` in the recompile at `/package/refinement_internals/<pointer>`, and in the dependency's member at `/package/dependencies/<n>/refinement_internals/<pointer>`; no result | FR-323-AC-17 |
| BP-16 | A `byte_provision` with no entry for a domain package the recompiled sources select | Refused `missing_import`/`missing-selection` naming the requested domain-package identity when the recompile reaches that selection, after the AC-5 input check and before any result | FR-323-AC-23 |
| BP-17 | A `byte_provision` with two domain-package entries declaring one identity and stating equal `package.version` members | Refused `invalid_package`/`conflicting-definition` naming both entries at decode, before recompilation; not `duplicate_selection` | FR-323-AC-25 |
| BP-19 | BP-17 with the two documents stating different `package.version` members | The same refusal: the two documents are not told apart by version, which is a member of the document and is never compared | FR-323-AC-25 |
| BP-18 | BP-01 plus one domain-package entry of an identity no recompiled source selects | Recompilation proceeds; the entry is not read, no refusal is reported | FR-323-AC-26 |

## Expected Results

BP-01 and BP-06 find a complete provision admitted. BP-15's unmodified
request is admitted. BP-02 through BP-05, BP-07 through BP-09, BP-11, BP-12
and BP-14, and BP-15's mutations find each incomplete, mismatched or stale
provision refused with its named typed cause before any recompilation result
is produced.
