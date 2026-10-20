---
id: SR-13373
title: "Integrity review of QSpec PR #292 (STD-211): failure precedence across a refinement step"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@e78332c; spec/functional/analysis/FR-377-decide-model-refinement.md, spec/test-cases/TC-315-refinement-undetermined-and-replay.md; ticket STD-211"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-377
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-315
    type: reviews
---

## Summary

Ticket: STD-211. PR: quire-specification#292 at e78332c (e78332ce731d91e077cc66b834bbee948eda0926), scoped to `git diff refs/remotes/origin/main...e78332ce7` (origin/main d053fb2). The diff is spec-only, over two files: FR-377 gains "Failure precedence across a step" and the budget paragraph, FR-377-AC-12/13 are added, and TC-315 gains step 9 with cases (a) to (h) and verifies edges to AC-12/13. Integrity review: I checked cross-spec consistency between the new FR-377 rules and FR-377-AC-9 (one-state precedence), the FR-379 settlement table (including the incomplete/limit-reached row), FR-378 (history failures), FR-383 (fault = failed) and FR-375 (step rows), and against QSL main 9567ed6 (FR-141 check_step read order and AC-9/10/11, FR-142, FR-144 and FR-145). The ticket brief's "FR-143" has no refinement content in QSpec (QSpec FR-143 is record values), so I checked the out-of-type history and unmapped-type undetermined reads against QSpec FR-378 and FR-377, and against QSL FR-140 (`OutOfType`, `UnmappedType`) and FR-141. Both agree that the two reads are undetermined. Gates: `cargo test --test checked_package_v2 --test backend_provider_v1` passes on the head (9 and 108). `quire validate --scope .` (quire 0.36.2, engine 0.50.2; the rollout note names 0.36.1/0.50.1, so the binary is newer, not older) exits 0 on FR-377 and on TC-315, with only the duplicate-archetype, duplicate-inverse-edge and inline-schema warnings, the same as on main. A whole-tree `quire validate --scope . 'spec/**/*.md'`, with paths normalised, is byte-identical between the head and main (134 failing documents, all already on main). `quire matrix` lists FR-377-AC-12/13 `untagged`, as AC-1..11 are on main too, because QSpec tags no Rust test for FR-377 and its implementation tests live in QSL. Method `code-review` was not run, because no code or test file changed; the Cargo gates stand in for it.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-377-failure-precedence-across-a-step | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| FR-377-step-budget-paragraph | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| FR-377-AC-12 | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| FR-377-AC-13 | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| FR-377-AC-9 | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| FR-379-mapping-undetermined | spec/functional/analysis/FR-379-report-and-replay-refinement-counterexamples.md | examined |
| FR-379-verdict-table | spec/functional/analysis/FR-379-report-and-replay-refinement-counterexamples.md | examined |
| FR-378-history-failures | spec/functional/analysis/FR-378-check-refinement-auxiliary-state.md | examined |
| FR-383-read-footprint-fault | spec/functional/analysis/FR-383-explore-under-state-space-reduction.md | examined |
| QSL-FR-141-check_step-read-order | quire-spec-language:spec/functional/FR-141-decide-one-concrete-step-against-the-abstract-model.md | examined |
| QSL-FR-141-AC-11 | quire-spec-language:spec/functional/FR-141-decide-one-concrete-step-against-the-abstract-model.md | context_only |
| QSL-FR-142/FR-144/FR-145 | quire-spec-language:spec/functional/FR-142..FR-145 | examined |
| FR-375-step-rows | spec/functional/analysis/FR-375-declare-model-refinement.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-377 now treats the receiver and an unmapped-type read as their own reads, but FR-379 was not updated to match, although STD-211 asked for the order to be stated in "FR-377/FR-379". FR-379's verdict rows, its `undefined-evaluation` `where`, its `mapping-undetermined` definition and its `Undefined` counterexample `row` each list only "a field row, a step-row argument or a history update". FR-375 keeps the receiver apart from the arguments ("one receiver expression and one argument per abstract parameter"). As a result, the settlement of an undefined or incomplete receiver, and what `row` and `where` name for it, are left to inference. FR-379's `mapping-undetermined` definition also leaves out the unmapped-type cause, which case (e) expects. Add the receiver and unmapped-type reads to FR-379's rows and cause definition, or say in FR-377 that the receiver counts as a step-row argument for FR-379. | spec/functional/analysis/FR-377-decide-model-refinement.md:103-118 |
| FND-002 | low | AC-12 is narrower than the body. The body says that an undefined read refutes "even when an earlier read is refused, incomplete, out of its declared type or of an unmapped type". AC-12 drops "refused" from that list, while AC-9 keeps it for one-state rows, and step 9 has no refused read. Add "refused" to AC-12, or say why a step read cannot be refused. | spec/functional/analysis/FR-377-decide-model-refinement.md:218 |
| FND-003 | low | Once this lands, QSL FR-141's closing sentence is stale. It calls the cross-step order "QSL's extension" of a precedence "which QSpec FR-377 states for the rows of one map(s, h) only", but FR-377 now states the cross-step order itself. STD-211 says "then QSL cites it", so QSL FR-141 needs a follow-up edit that cites FR-377-AC-12/13. The order, fault, undefined-over-undetermined, budget-stop and incomplete-row rules otherwise agree with QSL FR-141, FR-142, FR-144 and FR-145. | quire-spec-language:spec/functional/FR-141-decide-one-concrete-step-against-the-abstract-model.md:167-169 |
| FND-004 | low | "A failed read outranks it" uses "failed" for any read that did not return a value. The same paragraph, FR-383 and AC-12/13 use `failed` for the settlement of an internal fault. Write "a read that returned no value (fault, undefined or undetermined) outranks it", so that "failed" keeps one meaning. | spec/functional/analysis/FR-377-decide-model-refinement.md:120-125 |

## Verdict

Not mergeable on this method alone: one medium finding (FR-379 was not updated for the receiver read and the unmapped-type read), plus three low. Consistent: the post-state rows use AC-9's one-state order; a fault settles `failed` and takes no part in the order, as FR-383 and QSL FR-141/142/144 have it; undefined outranks undetermined, as in AC-9 and QSL FR-141; out-of-type history is undetermined, as in FR-378, FR-379 and QSL FR-140; an unmapped type is undetermined, as in FR-377-AC-11 and QSL FR-140; the budget stop settles the FR-379 incomplete/limit-reached row, as QSL FR-141-AC-9/FR-142/FR-144 do, and "never `failed`" holds at the FR-331 result level; and replay parity of the reported read matches QSL FR-145.
