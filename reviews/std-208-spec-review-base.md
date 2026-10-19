---
id: SR-13322
title: "Base checklist review of QSpec PR #288 (STD-208)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@07eb2ff; spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md, spec/functional/tooling/FR-306-compile-and-cache-exactly.md, spec/objects/interfaces/FR-331-backend-provider-envelope.md, spec/test-cases/TC-373-hybrid-verdict-map.md; ticket STD-208"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-193
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-306
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-373
    type: reviews
---

## Summary

Ticket: STD-208. PR: quire-specification#288 at 07eb2ff. Base checklist (id formats, duplicates, AC-to-TC links, validation) over `git diff origin/main...07eb2ff`. New ids FR-193-AC-8, FR-193-AC-9 and FR-331-AC-29 parse in `quire matrix --scope .` (untagged, like every other FR-193 criterion; QSpec has no engine to tag). No collision with open QSpec PRs: #287 (STD-211) adds FR-375/376/377 criteria and TC-310/311/315/316 rows; #286 is merged (FR-149); #200 reserves FR-331-AC-24..27, which this PR skips; #283 and #207 touch FR-331/FR-306 text but add no FR-193, FR-331-AC-29 or TC-373 ids. `quire validate --scope .` on the four changed files: FR-193, FR-306 and TC-373 pass; FR-331 fails with `[value_object] required 'id' is missing` and `semantic.record-invalid ... "fields" is a required property`, byte-identical on an unmodified `git archive origin/main` (2c2918b) copy, so pre-existing. Each new criterion names TC-373, and TC-373 now verifies FR-331.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-193-row-step | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-row-time | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-budgets-steps | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-budgets-time | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-budgets-identity | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-AC-8 | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-AC-9 | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-331-incomplete-row | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331-hybrid-solver-budgets | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331-request-identity | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331-AC-29 | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-306-outside-key | spec/functional/tooling/FR-306-compile-and-cache-exactly.md | examined |
| TC-373-HY-04 | spec/test-cases/TC-373-hybrid-verdict-map.md | examined |
| TC-373-HY-07 | spec/test-cases/TC-373-hybrid-verdict-map.md | examined |
| TC-373-HY-08 | spec/test-cases/TC-373-hybrid-verdict-map.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-331-AC-29's last clause (two requests differing only in `max_solver_time` have equal request identity) is exercised only by HY-08, but HY-08 cites FR-193-AC-9 alone; HY-04 and HY-07 cite FR-331-AC-29 and cover only its step and time-stop clauses. Add FR-331-AC-29 to HY-08's citation so every clause of AC-29 maps to a vector. | spec/test-cases/TC-373-hybrid-verdict-map.md:33; spec/objects/interfaces/FR-331-backend-provider-envelope.md:380 |

## Verdict

One low trace gap. ID allocation, AC-to-TC links and validation are otherwise clean; the FR-331 validation failures are pre-existing and identical on main, as the author reported.

## New findings (disposition pass 1)

Against head 5a9fc8d.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The new `Hybrid` schema definition is exercised by no vector: `proposals/backend-provider-v1/vectors.json` and `tests/backend_provider_v1.rs` contain no `hybrid` member, so the re-run `cargo test --test backend_provider_v1` (9 pass at 5a9fc8d) checks nothing this PR added. FR-331-AC-29's "a fractional `max_solver_time` on the wire is refused before consumption" is exactly a schema property. Add one positive TC-431 vector with `hybrid.limits` and adverse vectors with `max_solver_time: 1.5` and an unknown `hybrid.limits` member. | proposals/backend-provider-v1/schema.json:387-401 |

## New findings (disposition pass 2)

Against head 607417f.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The positive `hybrid-request` vector puts `hybrid.limits` on a Kani `value-validity` item (request_index 0, backend `kani`), but FR-290 (spec/objects/protocol/FR-290-protocol-claim-kind.md:130) maps a hybrid-model claim to `temporal-satisfaction`. The vector proves the `hybrid.limits` shape but models an item that cannot occur, and the schema does not tie `hybrid` to any kind (as with `temporal` and `probabilistic`). Move the member onto a `temporal-satisfaction` item, or add a note that the host item is a placeholder. | proposals/backend-provider-v1/vectors.json:1988-2035 |

## Dispositions

Round 1, against head 5a9fc8d.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 5a9fc8d: HY-08 now cites FR-331-AC-29 |
| FND-002 | fixed | 607417f: positive vector hybrid-request and five adverse vectors; cargo test --test backend_provider_v1 9/9 pass; each adverse vector is killed by the schema mutation that removes its rule (Count, number, no minProperties, no required, open limits) |
