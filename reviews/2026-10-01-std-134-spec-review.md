---
id: SR-810
title: "Spec review of refinement and state-space reduction (STD-133, STD-134)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@6a46fe220e02d93679279e2fef7f390fd57d448a; FR-375 to FR-385, amended FR-013, FR-161, FR-177, FR-181, FR-271, FR-272, FR-290, FR-331, FR-340, native-diagnostics, TC-310 to TC-328, spec/spec.md, spec/complete-v1/tests.md"
review_set: all
evaluated_revision: "6a46fe220e02d93679279e2fef7f390fd57d448a"
review_date: "2026-10-01"
---

## Summary

Ticket: STD-134 (paired QSL-368); STD-133 (paired QSL-367). PR
quire-specification#169, diff `origin/main...HEAD` (3 commits, 41 files).
Design inputs: quire-spec-language ADR-020 (spec/367-refinement) and ADR-021
(spec/368-por 1e48e7a7).

Checked against the wave B/C owner rulings: no fixed caps (depth bounds here
are BFS exploration bounds, which name the limit, its value and how to raise
it); no pins or ledgers (FR-331's "exact tool pins" is removed); no compat
paths; EARS descriptions; every new AC has a behaviour TC; undefined settles
refuted with `undefined-evaluation` in FR-376 to FR-379 and FR-385; index
rows present for FR-375 to FR-385 and TC-310 to TC-328. `quire validate` on
the 41 changed files: 34 pass, and the 7 that fail fail identically on
`origin/main`.

The partial-order soundness check is SR-811.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-383 fixes the enabling footprint as "the precondition read footprint" and never defines "fairness class", but protocol steps' enabledness reads control, instance and queue locations (ADR-021 POR-7 uses ADR-027 FT-3), and TC-325's `Mark` relies on them | spec/functional/analysis/FR-383-explore-under-state-space-reduction.md:95-121 |
| FND-002 | medium | TC-311 step 4 expects an undefined `only` row to make `map` "undetermined"; FR-376-AC-3, FR-377-AC-1 and FR-379-AC-6 make it refute with `undefined-evaluation` | spec/test-cases/TC-311-population-valued-mapping-expressions.md:39,49-51 |
| FND-003 | low | FR-383-AC-5 says a fairness-visible transition "is expanded with its dependents", which is not what the rule says: a visible transition cannot be in a reduced ample set (C2) | spec/functional/analysis/FR-383-explore-under-state-space-reduction.md:174 |

FND-001 fix: say the enabling footprint is the set of locations the
transition's enabledness reads (for a model transition see SR-811 FND-001;
for a protocol step, its system's enabling footprint), and define a fairness
class as the transitions one authored or derived constraint covers (an
operation's or attempt node's steps for `whole`, one transition identity or
one thread's steps for `each`, and each default scheduler constraint's
thread target).

FND-002 fix: step 4 expects `refuted`, cause `undefined-evaluation`, for both
the no-match and the two-match `only`. Related, outside this PR: ADR-020 RM-8
and QS-9 (g), (m) still say `MappingUndetermined` for an undefined `only`,
against ADR-020 RE-5 and RU-6.

## Verdict

The refinement half (FR-375 to FR-380) follows ADR-020 and the undefined
ruling, apart from TC-311. The reduction half follows ADR-021 in structure,
but see SR-811 for two high soundness gaps. Not mergeable as it stands.

## Dispositions

Round 1, reviewed at ebfdad1c48b8bc3d30e84fd6597d55117bc37dee.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ebfdad1: FR-383 adds an "Enabling footprint" paragraph (protocol steps use the set their system supplies, covering control, instance, binder, queue and model locations) and a "Fairness class" definition, default scheduler classes included |
| FND-002 | fixed | ebfdad1: TC-311 step 4 now expects `refuted`, `undefined-evaluation`, for both `only` cases; FR-379-AC-6 and tests.md list TC-311 |
| FND-003 | fixed | ebfdad1: FR-383-AC-5 now says a visible transition is never in an ample set smaller than E(s), with the `Gate` vector in both guard placements |
