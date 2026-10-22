---
id: SR-16183
title: "Spec review of quire-specification PR #382: STD-324, FR-435 ordering mismatch at the pre-check"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@2b77a1e38; PR #382 (git diff origin/main...2b77a1e38): spec/functional/memory/FR-435-classify-memory-accesses-and-orderings.md, spec/functional/memory/FR-439-bound-replay-and-report-weak-memory-runs.md, spec/test-cases/TC-389-memory-access-classification.md, spec/test-cases/TC-431-backend-provider-wire-vectors.md, proposals/backend-provider-v1/schema.json, proposals/backend-provider-v1/vectors.json, tests/backend_provider_v1.rs"
review_set: subset
evaluated_revision: "2b77a1e38"
relationships:
  - target: ix://agent-ix/quire-specification/FR-435
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-439
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-389
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-431
    type: reviews
---
# SR-16183: Spec review of quire-specification PR #382: STD-324, FR-435 ordering mismatch at the pre-check

## Summary

Ticket: STD-324. Reviewed `git diff origin/main...HEAD` at 2b77a1e38 against QSL FR-220 at main 2c5d82d37 (Admitted orderings, EN-1 ordering rule, FR-220-AC-2, AC-14, References). The check-time rules, the four pre-check mismatch cases, the `{"type":"ordering-mismatch","access","ordering"}` spelling with `null` for none, multi-location first and `sc` admission are QSL's, stated as behaviour. FR-435-AC-9 carries all six FR-220-AC-14 vectors and its multi-location precedence case. The schema branch is closed (`additionalProperties: false`, `access` enum, `ordering` null or the five spellings). Mutation check: with main's schema.json and the PR's vectors, `tc_431_positive_vectors_are_admitted_with_owned_members` fails on `weak-access-shape-outcome`, so the new positive vector can fail. The four adverse vectors each patch one member. The `tests/backend_provider_v1.rs` change is two `#[trace("TC-431", "FR-439-AC-11")]` attributes on existing tests that iterate the vectors file, which is correct. Ids FR-435-AC-9, FR-439-AC-11, TC-389 AC-14 and BP-31 are unique across main and every open PR (BP-30 is #384, BP-32 is #387). Gates at the head: validate 86 failure lines (main 86); cargo 23 + 119 passed. QSL conformance `cargo test -p qsl-package --lib checked_v2` (QSL main 2c5d82d37, QSPEC_DIR at this head): 43 passed, 3 failed, the same three `extension_definition_*`/`published_extension_definitions_*` tests that fail identically with QSPEC_DIR at QSpec main (control run), caused by QSpec #365's absent file; QSL reads no backend-provider-v1 file. Merge: conflicts textually with #384 and #387 in vectors.json (positive and adverse tails), TC-431 (frontmatter `verifies` list with #384, table tail with both) and, with #387, the trace attributes in tests/backend_provider_v1.rs. Every order of #382/#384/#387 conflicts after the first merge; each conflict is append-both. Cleanest order: #384, then #382, then #387, so TC-431 rows land append-only as BP-30, BP-31, BP-32 and #387 (blocked on its QSL half) goes last; #382 then rebases once onto main plus #384.

Method: quoin spec-review (base), with integrity, EARS, criterion-strength (by hand: no Jev client) and failure-domain checks folded in; code-review of the `tests/` change where present is folded in here. Gap-analysis does not apply (spec, vectors and trace tags only).

Units examined:

- `FR-435 Admitted orderings` (examined): spec/functional/memory/FR-435-classify-memory-accesses-and-orderings.md
- `FR-435-AC-2` (examined): spec/functional/memory/FR-435-classify-memory-accesses-and-orderings.md
- `FR-435-AC-9` (examined): spec/functional/memory/FR-435-classify-memory-accesses-and-orderings.md
- `FR-439 weak-access-shape shapes` (examined): spec/functional/memory/FR-439-bound-replay-and-report-weak-memory-runs.md
- `FR-439-AC-11` (examined): spec/functional/memory/FR-439-bound-replay-and-report-weak-memory-runs.md
- `TC-389 AC-02` (examined): spec/test-cases/TC-389-memory-access-classification.md
- `TC-389 AC-14` (examined): spec/test-cases/TC-389-memory-access-classification.md
- `TC-431 BP-31` (examined): spec/test-cases/TC-431-backend-provider-wire-vectors.md
- `FR-435 Locations and classification` (context_only): spec/functional/memory/FR-435-classify-memory-accesses-and-orderings.md
- `QSL FR-220 References` (context_only): agent-ix/quire-spec-language:spec/functional/FR-220-check-access-orderings-fences-and-access-classification.md

## Verdict

**MERGEABLE after a fix**: one medium finding, one low. Needs a rebase after whichever of #384/#387 lands first.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | 'Data place' is not a QSpec term: FR-435 defines locations, shared and branch-local (a register of its thread), and no QSpec file uses 'data place' on main. The check-time rule 'any ordering on an attempt that touches no data place' is load-bearing: FR-435-AC-9's `local`/`release` vector (an attempt that writes only a branch-local register) must pass the check-time rule to reach the pre-check, which holds only if a register counts as a data place. A reader who takes 'data place' to mean a shared location refuses that attempt at check time and the vector cannot reach `ordering-mismatch`. Say 'touches no location, shared or branch-local' (or define the term). | spec/functional/memory/FR-435-classify-memory-accesses-and-orderings.md:95-108 |
| FND-002 | low | STD-324 asks for FR-220's References to stop claiming the QSpec state; the PR body says QSL needs no further edit. QSL FR-220 References still reads 'QSpec owes the matching change (Linear STD-305): FR-439 gains the ordering-mismatch shape, and FR-435's check-time ... refusal ... becomes this pre-check outcome'. Once this PR merges that sentence is stale and names the wrong ticket (the QSpec half is STD-324). It needs a one-line QSL follow-up (behaviour-only: 'QSpec FR-435 and FR-439 state the same rule and shape'), which this QSpec PR cannot carry. | agent-ix/quire-spec-language:spec/functional/FR-220-check-access-orderings-fences-and-access-classification.md:217-222 |
