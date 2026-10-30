---
id: TC-277
title: "Replay provision selects current documents by QSL FR-001 labels"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: verifies
---

# TC-277: Replay provision selects current documents by QSL FR-001 labels

## Description

Show that an FR-323 replay request supplies an ordered, unkeyed list of source,
definition and domain-package documents. Source and definition documents are
selected by the four QSL FR-001 labels; domain packages are selected by their
declared package identity. The consumer does not classify supplied bytes as
stale by comparing their digest with an earlier value.

## Preconditions

- QSL FR-001 defines source identity labels `authority`, `identity`,
  `revision_namespace` and `revision`.
- FR-323 defines the package's source selections and ordered `byte_provision`.

## Test Procedure

Construct replay requests whose `byte_provision` contains the source,
definition and domain-package bytes needed by the package reference. Run them
through the replay reader and executor. Repeat with a required entry absent,
a duplicate source identity, changed bytes under an existing source identity,
a malformed changed document, duplicate domain-package identity and an
unselected domain-package entry. Observe document selection and the resulting
ordinary typed refusal or recompile outcome; do not compare digest values.

## Vectors

| Vector | Mutation | Expected result | Criteria |
| --- | --- | --- | --- |
| BP-01 | Complete ordered list of source and definition documents labeled by the identities in `package.sources`, plus each selected domain package by declared identity | Recompilation proceeds from the supplied documents | FR-323-AC-5, FR-323-AC-27 |
| BP-02 | Remove a source or definition identity named by `package.sources` | Refused `missing_import`/`missing-selection`, naming the missing labels and `package.sources`, before recompilation | FR-323-AC-5 |
| BP-03 | Change source bytes while keeping its four QSL FR-001 labels unchanged | The changed bytes are consumed as the current document; no digest or stale-dependency refusal | FR-323-AC-28 |
| BP-04 | Change bytes so a supplied current document is malformed or fails checking | The stage that encounters the document emits its ordinary typed refusal, not a digest-based stale-dependency cause | FR-323-AC-28 |
| BP-05 | Supply two source or definition documents with the same four QSL FR-001 labels | Refused `invalid_package`/`conflicting-definition`, naming both entries before recompilation | FR-323-AC-25 |
| BP-06 | A proved package with two direct `dependency_selections`, one of whose dependencies imports a third library; the package reference includes the full derived dependency closure, and the provision carries every source including the transitive-only library | Each dependency recompiles from its selected documents and is supplied to its direct or transitive imports; replay uses the compiled result | FR-323-AC-7 |
| BP-07 | BP-06 with one dependency source edited to remove an export the proved package calls | Recompile refuses `missing_declaration`/`missing-name` at the call; no result | FR-323-AC-7 |
| BP-08 | BP-06 with dependencies swapped, repeated or unreferenced | Refused `invalid_package`/`invalid-value` at `/package/dependencies` for swapped, repeated and unreferenced entries, with order precedence as FR-323-AC-7 specifies | FR-323-AC-7 |
| BP-09 | BP-06 with a dependency entry removed | Refused `missing_import`/`missing-selection` at the import the entry supplied | FR-323-AC-7 |
| BP-10 | Supply two domain-package documents declaring the same identity, regardless of their other document members | Refused `invalid_package`/`conflicting-definition`, naming both entries before recompilation | FR-323-AC-25 |
| BP-11 | Supply a domain-package entry whose declared identity is not selected by any recompiled source | Recompilation proceeds; the unselected entry has no effect | FR-323-AC-26 |
| BP-12 | BP-06 with one entry whose source has the authority and identity of another entry's source | Refused `invalid_package`/`conflicting-definition`, naming both entries before recompilation | FR-323-AC-7 |
| BP-15 | BP-06 with proved package and dependency `refinement_internals` values; then with a spelling naming no node | First replay recompiles and uses the result; unresolved spelling refuses `missing_declaration`/`missing-name` at the corresponding package or dependency pointer | FR-323-AC-17 |

## Expected Results

BP-01 and BP-06 are admitted and use the supplied documents. BP-02, BP-05,
BP-07 through BP-10 and BP-12 produce their named typed refusals. BP-03 uses
the supplied changed bytes without a stale-byte comparison. BP-04 returns the
owning stage's typed refusal. BP-11 has no effect. BP-15's unmodified request
is admitted and its unresolved-name mutation refuses during recompilation.
