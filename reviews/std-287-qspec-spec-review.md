---
id: SR-16170
title: "Spec review — hybrid solver result wire form and hybrid trace (FR-193, FR-331)"
type: SpecReview
analysis: base
date: "2026-10-22"
scope: "agent-ix/quire-specification; spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md, spec/objects/interfaces/FR-331-backend-provider-envelope.md, proposals/backend-provider-v1/schema.json, proposals/backend-provider-v1/vectors.json, spec/test-cases/TC-373-hybrid-verdict-map.md, spec/test-cases/TC-431-backend-provider-wire-vectors.md, tests/backend_provider_v1.rs at f758128b7"
review_set: subset
evaluated_revision: "f758128b79d81177c7da3a4382214eda138e48b4"
relationships:
  - target: ix://agent-ix/quire-specification/FR-193
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-373
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-431
    type: reviews
---

## Summary

Ticket: STD-287 (unblocks QSL-900). PR quire-specification#384 adds FR-193
"Solver result on the wire" and "Hybrid trace" (`HybridTrace{start,
jumps[{transition, state}]}`, the reader's correspondence refusals, the trace
kept as unverified evidence), FR-193-AC-10, the FR-331 `solver-inconclusive{trace}`
cause and counterexamples note, schema defs `SolverInconclusive`, `HybridTrace`
and `HybridTraceState`, one positive and two adverse vectors, TC-373 HY-09,
TC-431 BP-30 with a `verifies` edge to FR-193, and two `#[trace]` tags in
`tests/backend_provider_v1.rs`. One SpecReview covers the PR. The code-review
lane (test tags, schema and vectors) is folded in. Reviewed
`git diff origin/main...HEAD` at f758128b against FR-331 (Certification,
Result entries, reading order, In-process providers), FR-468, FR-272, QSL
#1279 (FR-250, FR-447) and QSL FR-290 on main.

Consistent and clean: the schema moves `solver-inconclusive` out of the
payload-free enum into its own closed def, with no overlap in the `oneOf`. The
positive vector's trace is well formed. The two adverse vectors are refused at
their patched members. The trace is never a `counterexamples` entry, which
agrees with QSL FR-250 and the O-16 inconclusive category. The trace shape
(names, zero-based per-mode transition index, exact rational intervals) maps
one to one onto QSL's index-keyed `HybridTrace`. The three lead defaults are
flagged in the PR body. BP-30, HY-09 and FR-193-AC-10 are absent from
origin/main and from all 11 open QSpec PR heads (#382 holds BP-31). Gates:
`quire validate --scope . 'spec/**/*.md'` gives exactly 86 failure lines;
`cargo test --test checked_package_v2 --test backend_provider_v1` passes
23 + 119. QSL conformance (`cargo test -p qsl-package --lib checked_v2`,
QSPEC_DIR at this head) gives 43 passed and 3 failed. The 3 failures are the
known `extension_definition_*` and `published_extension_definitions_*` xd
tests on QSpec main, fixed by QSpec #365 and unrelated to this PR. The head
contains origin/main (4c687282).

## Verdict

NOT MERGEABLE as written. One high finding: the new wire rule forces
`uncertified` and reader-side method soundness on every hybrid provider, which
contradicts FR-331's `trusted` rule for plugin proofs and QSL FR-290. Three
medium findings: the reader's soundness decision has no defined input; a
`trace` is admitted on non-hybrid `solver-inconclusive` causes (FR-468); and
the correspondence refusals need a `hybrid-decl` that no v2 package carries.
Two low findings cover AC coverage and an over-broad "every other result"
bullet. All six can be fixed in this PR. This PR merges first, before QSL #1279.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new section requires every hybrid `proved` entry to carry `certification` `uncertified` and says the reader settles `proved` from the entry's `method` and "reads no soundness claim the provider makes about itself". FR-331's Certification rules fix `trusted` as the expected value for any proof from a backend loaded as a plugin (FR-305), so a plugin hybrid provider following FR-193 is refused `certification-mismatch`. QSL FR-290 (line 164) reads a plugin's `bounded-solver` proof as trusted, accepts all three methods including `flowpipe-unrounded`, and says FR-250's method-kind soundness rule applies only to solvers QSL runs. Scope the `uncertified` and method-soundness rules to first-party providers, or state the plugin case explicitly, consistent with FR-331 and QSL FR-290. | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:121-130 |
| FND-002 | medium | The reader is to settle `proved` "only where the entry's `method` is sound for its outcome". The schema types `method` as a free `Nonempty` string (schema.json line 688), QSpec names no closed method vocabulary (the spellings `delta-complete`, `flowpipe-outward-rounded`, `flowpipe-unrounded` exist only in QSL FR-290), and the entry does not carry the outcome (`unsat` vs a disjoint enclosure). A QSpec-conforming reader has no defined input to make the decision. Name the method spellings and which outcome each is sound for, or state that a `proved` entry is admitted only for a sound method and refuse others. | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:121-126 |
| FND-003 | medium | `solver-inconclusive{trace}` is one closed cause shared with non-hybrid engines: FR-468's per-step SMT queries settle a solver `unknown` as `inconclusive` `solver-inconclusive`. The schema now admits a `HybridTrace` on any item's `solver-inconclusive`, and FR-193's correspondence refusals are stated only against "the declaration". Nothing says what a reader does with a `trace` on a refinement or other non-hybrid item. Add a refusal (for example `invalid_result`/`unknown-member` at `/cause/trace` unless the item's subject is a `hybrid-decl`) and an adverse vector. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:264 |
| FND-004 | medium | The correspondence refusals need the source `hybrid-decl`'s modes, per-mode transition order and continuous variable names. A wire reader holds the request and the checked-package v2 bytes, and v2 has no hybrid node: QSL #1279 refuses to emit such a package and serves hybrid items in-process only (its lead default 7). FR-193 does not say where a reader gets the declaration, so for a process provider the refusals cannot be implemented. Say that these checks run against the producer's checked model, which today means in-process only, or tie them to the owner question on a v2 hybrid node. | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:167-173 |
| FND-005 | low | FR-193 lists six reader refusals. AC-10 and HY-09 test three of them (unknown transition, missing variable, lower > upper). A `mode` naming no mode, a `start` outside the first mode, a jump reaching a mode other than its transition's target, and an extra `enclosure` key have no AC case. Add them to AC-10 and HY-09. | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:191 |
| FND-006 | low | "Every other non-budget result SHALL be an `inconclusive` entry" also covers a missing solver or tool and a backend fault. FR-331 and FR-468 settle those `unsupported` or `failed`, and a frontend cancel settles `cancelled{source: deadline}`, which the next bullet's "deadline overrun" does not clearly name. Limit the bullet to the verdict map's inconclusive rows (`δ-sat`, flowpipe meeting the unsafe set, `unknown`, unsound method). | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:131-136 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Certification follows FR-331 (uncertified first-party, trusted plugin) and the reader-side soundness rule is dropped; the producer applies the verdict map. |
| FND-002 | fixed | The reader no longer decides soundness; FR-193 names no closed method vocabulary and a method outside the reader's types refuses as FR-331 does for any basis. |
| FND-003 | fixed | A reader rule now refuses a trace on a non-hybrid solver-inconclusive with invalid_result/unknown-member at /cause/trace, and AC-10 and HY-09 test it on an FR-468 refinement item. No schema vector is possible: an outcome-stage vector holds no request, so it cannot show whether the item is hybrid. The reader-level row follows the BP-25 precedent, so this is acceptable. |
| FND-004 | fixed | The correspondence checks are assigned to the reader holding the checked hybrid model, which is the in-process producer, with the reason (no v2 hybrid node). |
| FND-005 | fixed | AC-10 and HY-09 now cover all six refusals plus an extra enclosure key and the non-hybrid trace. |
| FND-006 | fixed | The inconclusive bullet is limited to the four inconclusive rows; the other rows keep their own value and cause, including tool absence, fault and frontend cancel. |
