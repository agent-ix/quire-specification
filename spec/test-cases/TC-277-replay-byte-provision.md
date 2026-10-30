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
through the replay reader and executor. Within one request, test duplicate
source labels and duplicate domain-package identities. Across two separate
requests against a package previously replayed by the first request, test
changed source or definition bytes under the same four labels, including a
malformed changed document. Also test an unselected domain-package entry.
Observe document selection and the resulting ordinary typed refusal or
recompile outcome; do not compare digest values.

## Vectors

| Vector | Mutation | Expected result | Criteria |
| --- | --- | --- | --- |
| BP-01 | Complete ordered list of source and definition documents labeled by the identities in `package.sources`, plus each selected domain package by declared identity | Recompilation proceeds from the supplied documents | FR-323-AC-5, FR-323-AC-27 |
| BP-02 | Remove a source or definition identity named by `package.sources` | Refused `missing_import`/`missing-selection`, naming the missing labels and `package.sources`, before recompilation | FR-323-AC-5 |
| BP-05 | Supply two source or definition documents with the same four QSL FR-001 labels | Refused `invalid_package`/`conflicting-definition`, naming both entries before recompilation | FR-323-AC-25 |
| BP-06 | A proved package with two direct `dependency_selections`, one of whose dependencies imports a third library; the package reference includes the full derived dependency closure, and the provision carries every source including the transitive-only library | Each dependency recompiles from its selected documents and is supplied to its direct or transitive imports; replay uses the compiled result | FR-323-AC-7 |
| BP-07 | BP-06 with one dependency source edited to remove an export the proved package calls | Recompile refuses `missing_declaration`/`missing-name` at the call; no result | FR-323-AC-7 |
| BP-08 | BP-06 with dependencies swapped, repeated or unreferenced | Refused `invalid_package`/`invalid-value` at `/package/dependencies` for swapped, repeated and unreferenced entries, with order precedence as FR-323-AC-7 specifies | FR-323-AC-7 |
| BP-09 | BP-06 with a dependency entry removed | Refused `missing_import`/`missing-selection` at the import the entry supplied | FR-323-AC-7 |
| BP-12 | BP-06 with one entry whose source has the same four QSL FR-001 labels as another entry's source | Refused `invalid_package`/`conflicting-definition`, naming both entries before recompilation | FR-323-AC-7 |
| BP-15 | BP-06 with proved package and dependency `refinement_internals` values; then with a spelling naming no node | First replay recompiles and uses the result; unresolved spelling refuses `missing_declaration`/`missing-name` at the corresponding package or dependency pointer | FR-323-AC-17 |
| BP-16 | Run an earlier request, then a separate later request against the package using the same four source labels with different supplied bytes | The later request consumes its own supplied bytes; there is no comparison with the earlier request's bytes | FR-323-AC-29 |
| BP-17 | BP-16, with the later request's changed source document malformed or failing checking | The stage reading the later request's document emits its ordinary typed refusal | FR-323-AC-29 |
| BP-18 | Supply two domain-package documents declaring the same identity in one request, regardless of their other document members | Refused `invalid_package`/`conflicting-definition`, naming both entries before recompilation | FR-323-AC-25 |
| BP-19 | Supply a domain-package entry whose declared identity is not selected by any recompiled source | Recompilation proceeds; the unselected entry has no effect | FR-323-AC-26 |

## Expected Results

BP-01 and BP-06 are admitted and use the supplied documents. BP-02, BP-05,
BP-07 through BP-09, BP-12 and BP-18 produce their named typed refusals.
BP-16 consumes the bytes supplied by the later request without comparing them
to the earlier request; BP-17 returns the owning stage's typed refusal. BP-19
has no effect. BP-15's unmodified request is admitted and its unresolved-name
mutation refuses during recompilation.
