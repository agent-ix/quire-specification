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

The owner selected base plus all seven Quoin analyses. The eight formal
SpecReview artifacts are linked from [the base review](../spec/reviews/base.md).
Reviews were performed on the pinned revision recorded in each artifact and
remain conditional on their findings. Human acceptance, TC coverage and a test
matrix are not implied. The optional gap-analysis semantic comparison was
declined and was not run.

Further dependent implementation waits for specification review and the required
shared-contract acceptance. The initial profile bytes and existing fixture
identities remain unchanged. The FS05 wire schema is a draft structural contract;
its presence does not supply a producer/consumer implementation or qualify B/C's
existing interfaces.

## Authoring remediation from consumer feedback

Agent C's FS03 feedback on `e15b429` identified missing precondition/parameter and
activation expectations. FR-012 now includes pre versus post examples for an
immutable invocation parameter. FR-006 contains explicit false/true antecedent
and interrupted consequent-entry expectations, including serialized semantic
event tuples. These remain draft requirements and authored expectations. The
historical 51-case JSON packet and its rule/profile digests are unchanged;
accepted event/source/profile bindings and an independent reader are still needed.

FR-to-story relationships now use the catalog's `traces_to` verb. The installed
catalog describes `implements` as interface/contract fulfillment, explicitly
excluding requirement lineage. The requirements themselves retain their IDs.

The common SpecReview authoring pack has been fetched once for the selected review set. Its installed process manifest has SHA-256
`d08ce71c02ce871fed4c7fb5cea1af1feb5a0b1004f8458fd26deecc2aa7b720`.
The archetype duplication is within that file: ADR/Plan/Review/SpecReview/Standard
appear in both `archetypes` and `artifact_types`; no module-search environment
override was present. `quire schema --module <process-module> SpecReview` still
returns the required frontmatter schema and Summary/Findings assertions. This
inspection does not establish an error-free registry or complete a review.

## Review validation checkpoint

The eight analysis reports were validated with the explicit repository scope.
All 36 spec/review documents were grammar-clean. Quire still emitted the
six installed-registry errors described above despite exit 0. Report bodies
have the required Summary and nonempty Findings tables. No catalog files were
changed. Raw coverage/advisor output and tool/module provenance are retained in
spec/reviews/data; numeric-threshold advice is distinguished from reviewer
method recommendations.
