# Quoin specification workflow status

Recorded 2026-09-07. The [requirements index](../spec/spec.md) contains 28 draft
artifacts: one master, one StR, four US, sixteen FR, four NFR and two IT.
These specify FS01/FS02/FS03/FS05. The detailed state-core proposals supply design
inputs and authored examples; they do not establish accepted normative meaning
or independent consumer conformance.

## Authoring contract

Authoring follows the installed Quoin plugin 0.20.0 skills:

- `/home/peter/.claude/plugins/cache/quoin/quoin/0.20.0/skills/specify/SKILL.md`
  (SHA-256 `6f9075e93e35bb46dc23140bcfe648d9f56d621472f7f567cd0404c1c225cdbd`).
- `/home/peter/.claude/plugins/cache/quoin/quoin/0.20.0/skills/spec-review/SKILL.md`
  (SHA-256 `8c23b0f67e77d16a21a034885e7f78c835dff3c819ad9d7b23f121fe4489741d`).

The `quoin write` authoring pack was requested once for
`master-requirements,StR,US,FR,NFR,IT` using Quoin 0.23.1. It resolved the
organization to `agent-ix` from Git and selected the installed
`spec-artifacts-iso` module. Its manifest SHA-256 was
`f88971dfd4fa86e873c1216a8f607beee79258d2e88e78631a591050580249f0`.
The returned skeletons/schemas supply artifact structure; requirement prose and
examples are newly authored. No sample scenarios or implementation code were
copied. This does not choose standard-wide prose or reusable-artifact terms.

## Observed validation

Quire 0.31.0 was run with this repository as its explicit scope:

```sh
quire validate --scope "$PWD" 'spec/**/*.md' --strict --summary --diagnostics-format json
```

All 28 documents are grammar-clean after wording corrections. A separate local
check found unique artifact identities and resolved every local Markdown file
link and frontmatter relationship target in the two Agent A spec trees.

The installed catalog still emits error-severity diagnostics despite exit 0:
duplicate ADR/Plan/Review/SpecReview/Standard registrations and a duplicate
`part_of` inverse for `contains` and `aggregates`. Selecting the exact ISO module
removes the duplicate archetype diagnostics but still emits `DuplicateInverseEdge`;
both verbs declare that inverse in its manifest. No catalog definitions were
changed, and these runs are not recorded as error-free validation.

## Review and contract status

`/spec-review` is requested. Its required user selection of base, all, or a subset
of analyses remains pending. No formal `SpecReview` document, completed review,
human acceptance, formal TC coverage or test matrix is claimed by this record.
The older files under `reviews/` are freeform historical records, not completed
Quoin specification reviews.

Further dependent implementation waits for specification review and the required
shared-contract acceptance. The initial profile bytes and existing fixture
identities remain unchanged. The FS05 wire schema is a draft structural contract;
its presence does not supply a producer/consumer implementation or qualify B/C's
existing interfaces.
