---
id: SR-680
title: "QSL-680 Union text-leaf settlement review"
type: SpecReview
analysis: base
scope: "FR-322 Structural leaf walk and AC-59–61; FR-440 Structural leaves and AC-9"
review_set: base
---

## Summary

The amendment reconciles QSpec's existing Union leaf grammar with QSL
FR-093's U1–U7 vectors. Every declared member participates; member names
and zero-based payload positions remain separate segments; wrappers,
recursion reentry and collection leaf sources preserve their existing
meaning. The base review checked the new criteria, full vector paths and
profiles against the QSL amendment, both V2 `LeafSegment` schemas and the
existing TC-395 qualification cases. No new producer or reader
implementation coverage is claimed for these criteria.

The schemas already admit member and recursion segments. FR-322 and
FR-440 already require `union` in the shared catalog's `structural_kind`;
the catalog remains owned by `quire-verification-contracts` and must be
checked there before the cross-repository settlement is complete.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The installed validator rejects FR-322's existing legacy object Properties shape, identically on untouched main and on the amended file. This is a baseline validation blocker, not a passing structural gate. | FR-322 |
| FND-002 | low | No defect found in the changed leaf rules or U1–U7 correspondence. The new ACs state obligations; producer implementation, new criterion test bindings and independent review remain separate gates. | FR-322-AC-59, FR-322-AC-60, FR-322-AC-61, FR-440-AC-9 |

## Validation boundary

On machine `terra`, `/home/peter/.local/bin/quire` reports quire 0.36.2
(engine 0.50.2), and `/home/peter/.local/bin/quoin` reports 0.28.3.
`quoin catalog validate` succeeds for all ten installed modules.
`quire validate --scope .
spec/objects/interfaces/FR-322-checked-package-artifact.md --summary`
exits 3 with `[value_object] required 'id' (frontmatter_field(id)) is
missing` and `semantic.record-invalid: ... "fields" is a required
property`, despite FR-322's authored frontmatter `id`. Copying the
untouched main version to a temporary file and validating it with the
same scope produces the same two errors. The accompanying
`semantic.legacy-properties-form` warning identifies the existing
bullet-list Properties representation. The amended FR-440 passes.

The repository has no Makefile or index-completeness command. Both
requirements already occur in the master index's structured
relationships; this amendment adds no artifact requiring a new index
entry. Quire acceptance-section readback confirms AC-59–61 and AC-9.

The existing six TC-395 tests pass, including member-path grammar in both
schemas and recursive Union equality. The complete locked Cargo suite
passes 124 tests; `cargo fmt --all -- --check` and offline locked Clippy
with all targets, all features and warnings denied also pass. These runs
qualify the existing assertions, not the new criteria's full coverage.
The README's full Quire scope exits 3 with 143 failing documents,
including the FR-322 error above and unknown `log` archetypes in plan
artifacts. The changed FR-440 and this review validate successfully;
the full scope is not reported as passing.
