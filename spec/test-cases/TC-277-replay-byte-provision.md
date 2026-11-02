---
id: TC-277
title: "Replay byte provision"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-323-AC-29
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-323-AC-30
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-323-AC-31
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-323-AC-32
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-323-AC-33
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-323-AC-34
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-323-AC-35
    type: verifies
---

# TC-277: Replay byte provision

## Description

Show that an FR-323 replay request's `byte_provision` supplies every source and
definition input the replay recompiles, located by
`{authority, identity}`, and every domain-package document, an entry of the ordered provision located by the identity it declares, with no version compared and no digest compared except the `source_integrity` of a `sources` entry, and that a second document under one key is refused as a duplicate definition, per
QSL ADR-013 O-26 and QC-1.


Scope: FR-323-AC-5, FR-323-AC-7, FR-323-AC-25..27 and FR-323-AC-29..35.

## Preconditions

- QSL ADR-011 §1: stages S1 to S4 recompile a checked package from
  supplied source.
- FR-322 owns the checked-package value.

## Test Procedure

Build FR-323 replay requests whose `byte_provision` supplies a complete
set of inputs, and the mutations that omit an input or supply a second
document under one key. Execute
each vector through the real local Rust boundary and read back the
recompilation outcome.

## Vectors

| Vector | Mutation | Expected result | Criteria |
| --- | --- | --- | --- |
| BP-01 | A `byte_provision` naming every source and definition input by `{authority, identity}` and every domain-package document as an entry located by its declared identity | Recompilation proceeds; no missing-input refusal | FR-323-AC-5 |
| BP-02 | A `byte_provision` missing one input the `package` reference's lock `sources` name | Refused `missing_import`/`missing-selection` naming the requested `{authority, identity}` and the `package` member that names it, before recompilation | FR-323-AC-5 |
| BP-03 | Withdrawn: the digest, staleness or package-id comparison this vector tested is removed. | Withdrawn | - |
| BP-04 | Withdrawn: the digest, staleness or package-id comparison this vector tested is removed. | Withdrawn | - |
| BP-05 | Withdrawn: the digest, staleness or package-id comparison this vector tested is removed. | Withdrawn | - |
| BP-06 | A proved package with two direct `dependency_selections`, one of whose dependencies imports a third library; the `package` reference names the proved package's `sources` and one `dependencies` entry per identity in the derived closure, in ascending UTF-8 identity order, each with its own lock `sources`; the provision carries every source, including the transitive-only library's | Each dependency recompiles from its entry's sources and is supplied to its direct or transitive imports; the proved package recompiles and the replay uses the result | FR-323-AC-7 |
| BP-07 | BP-06 with one dependency entry's source bytes replaced by an edit removing the export the proved package calls | Refused in the recompile, `missing_declaration`/`missing-name` at the call in the proved source; no result | FR-323-AC-7 |
| BP-08 | BP-06 with its first and second `dependencies` entries swapped (the third stays in place); then with one entry repeated; then with an extra entry no import reaches | Refused `invalid_package`/`invalid-value` at `/package/dependencies`, the first two before any recompile | FR-323-AC-7 |
| BP-09 | BP-06 with one `dependencies` entry removed | Refused `missing_import`/`missing-selection` at the import the entry supplied | FR-323-AC-7 |
| BP-11 | Withdrawn: the digest, staleness or package-id comparison this vector tested is removed. | Withdrawn | - |
| BP-12 | BP-06 with a second entry whose source has the authority and identity of the first entry's source | Refused `invalid_package`/`conflicting-definition` naming both entries, before any recompile | FR-323-AC-7 |
| BP-14 | Withdrawn: the digest, staleness or package-id comparison this vector tested is removed. | Withdrawn | - |
| BP-15 | BP-06 with the proved package a unit whose lock carries `refinement_internals` (`R`: `Spec::Twice::finish`) and one dependency whose own lock carries one; the `package` reference and that `dependencies` entry carry the lock values; then a spelling naming no node, and the same in the dependency's own member | The first replay recompiles each refinement with its list and uses the result. The unresolving spelling refuses `missing_declaration`/`missing-name` in the recompile at `/package/refinement_internals/<pointer>`, and in the dependency's member at `/package/dependencies/<n>/refinement_internals/<pointer>`; no result | FR-323-AC-17 |
| BP-16 | A `byte_provision` with no entry for a domain package the recompiled sources select | Refused `missing_import`/`missing-selection` naming the requested domain-package identity when the recompile reaches that selection, after the AC-5 input check and before any result | FR-323-AC-23 |
| BP-17 | A `byte_provision` with two domain-package entries declaring one identity and stating equal `package.version` members | Refused `invalid_package`/`conflicting-definition` naming both entries at decode, before recompilation; not `duplicate_selection` | FR-323-AC-25 |
| BP-19 | BP-17 with the two documents stating different `package.version` members | The same refusal: the two documents are not told apart by version, which is a member of the document and is never compared | FR-323-AC-25 |
| BP-18 | BP-01 plus one domain-package entry of an identity no recompiled source selects | Recompilation proceeds; the entry is not read, no refusal is reported | FR-323-AC-26 |
| BP-60 | A replay request whose `package.sources` entry claims a digest other than the SHA-256 of its `byte_provision` bytes (RP-R1); a dependency entry likewise (RP-R2) | Refused `invalid_package`/`source-integrity` at `/package/sources/0`, respectively `/package/dependencies/0/sources/0`, before any recompile | FR-323-AC-29, FR-323-AC-30 |
| BP-61 | A `sources` entry with no `source_integrity` (RP-R3); every claim holding (RP-P1) | Refused `invalid_package`/`source-integrity` at the entry; the second admitted | FR-323-AC-31, FR-323-AC-33 |
| BP-62 | A `sources` entry whose pair `byte_provision` does not carry (RP-R4) | Refused `missing_import`/`missing-selection` naming the pair, not `source-integrity` | FR-323-AC-32 |
| BP-63 | A malformed claim on a later dependency entry behind an entry without bytes (RP-R7); a digest mismatch at `/package/sources/0` ahead of absent dependency bytes (RP-R5); absent bytes at `/package/sources/0` ahead of a dependency mismatch (RP-R6) | `source-integrity` at the dependency entry; `source-integrity` at `/package/sources/0`; `missing_import`/`missing-selection` | FR-323-AC-34, FR-323-AC-35 |

## Expected Results

BP-01 and BP-06 find a complete provision admitted. BP-15's unmodified
request is admitted. BP-02, BP-07 through BP-09, BP-12, BP-16, BP-17
and BP-19 and BP-15's mutations find each incomplete or duplicate
provision refused with its named typed cause before any recompilation result
is produced.
