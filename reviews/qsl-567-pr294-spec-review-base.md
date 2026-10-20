---
id: SR-13405
title: "Base review of QSpec PR #294 (QSL-567): FR-437 visibility fairness as an edge Streett condition"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@cb8f1d8; spec/functional/memory/FR-437-explore-release-acquire-memory.md, spec/test-cases/TC-391-litmus-conformance-vectors.md; ticket QSL-567"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-437
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-391
    type: reviews
---

## Summary

Ticket: QSL-567. PR: quire-specification#294 at cb8f1d8 (cb8f1d8f612c067ee9c120211aed42dd46b3526b), scoped to `git diff refs/remotes/origin/main...cb8f1d8f6` (origin/main 0ad3768). The diff touches two files. FR-437's visibility fairness paragraph replaces the whole-SCC rule (for each thread and location the SCC loads, the SCC holds a load of the mo-last message) with an edge Streett condition: per pair (thread `b`, location `ℓ`), `R` is the edges on which `b` loads `ℓ` and `G ⊆ R` those that load the mo-last message; an SCC passes when every pair has no `R` edge or a `G` edge, and otherwise the failing pair's `R` edges are removed and the rest decomposed and tested again. FR-437-AC-7 and TC-391 row LT-12 add the three-state example. No schema, vector or code changed. This is the QSpec half of QSL-567; the QSL half is quire-spec-language#1084 at 0455abe (ADR-025 MF-2/MF-5, FR-228, TC-673), which another reviewer covers for the algorithm.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-437 | spec/functional/memory/FR-437-explore-release-acquire-memory.md | examined |
| FR-437-AC-7 | spec/functional/memory/FR-437-explore-release-acquire-memory.md | examined |
| FR-437-AC-5 | spec/functional/memory/FR-437-explore-release-acquire-memory.md | examined |
| TC-391-LT-12 | spec/test-cases/TC-391-litmus-conformance-vectors.md | examined |
| TC-391-LT-11 | spec/test-cases/TC-391-litmus-conformance-vectors.md | examined |
| TC-391-procedure | spec/test-cases/TC-391-litmus-conformance-vectors.md | context_only |
| QSL-ADR-025-MF-2 | quire-spec-language:spec/decisions/ADR-025-weak-memory-models-for-parallel.md | examined |
| QSL-FR-228 | quire-spec-language:spec/functional/FR-228-derive-memory-fairness-constraints.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | LT-12 does not fit TC-391's own procedure. TC-391 writes every vector as a litmus protocol, checks it under `sc`, `tso` and `ra` at the real Rust boundary with default budgets, and compares the `ra` column with an independent RC11 checker. LT-12 is a three-state graph with `—` under `sc` and `tso`, and an RC11 checker has nothing to compare for it, so no step of the procedure says how to run it and two Expected Results lines (no memory budget reached, agreement with RC11) do not apply. The graph also cannot come from a protocol as labelled: `q → r` is a load by `b`, which changes only `b`'s part of the state (its position, register and views), and `r → q` is a step by `a`, which cannot undo that, so `r` would have to equal `q`. And in any protocol version, `b` is enabled at `p` and at `q` and the loop `p q` never steps `b`, so the default weak scheduler constraint for `b` (FR-430) rejects `{p, q}`; the lasso is fair only under `scheduling adversarial;`. Fix: add a TC-391 procedure step that passes the synthesized SCC straight to the visibility filter, with its own expected result, as QSL TC-673 step 3 does. Or move the row to a fairness-filter TC. If a protocol run is also wanted, use a shape a protocol can produce, such as a stale `b` load that leaves the state unchanged (a self-loop at `q`) under `scheduling adversarial;`, or a `p q` cycle in which `b` takes steps that do not load `ℓ`. | spec/test-cases/TC-391-litmus-conformance-vectors.md:49 |
| FND-002 | low | `R ∩ C` and `G ∩ C` intersect sets of edges with `C`, an SCC, which the text and FR-365 treat as a set of states. After a removal this matters: the edges of a sub-SCC are the edges that remain between its states, not every edge between them, and a removed `R` edge must not count again when the sub-SCC is tested. Say "the edges of `C`" once (the edges of the current subgraph with both ends in `C`), and write "the edges of `R` in `C` are removed". QSL ADR-025 MF-2 uses the same notation, so change both together. | spec/functional/memory/FR-437-explore-release-acquire-memory.md:121-135 |
| FND-003 | low | AC-7's last clause, "a rule that rejected the SCC for the stale load would miss it", describes the old rule, not this system's behaviour. No test can fail it, and it records the history of the fix. The clauses before it already pin the behaviour, so drop it. The AC also says the filter "accepts `{p, q}`", where FR-365 calls a component that meets every condition a **passing component** and QSL FR-228-AC-5 says "passes `{p, q}`". Use "passes" so the two repos use one term. (The body sentence "An SCC is therefore never rejected for a stale load alone" is a real constraint and matches QSL FR-228's SHALL NOT, so keep it.) | spec/functional/memory/FR-437-explore-release-acquire-memory.md:154 |

## Verdict

Not mergeable as is: one medium finding (LT-12 cannot be run by TC-391's procedure and its graph is not one a protocol can produce) and two low wording findings. The rule itself is sound, and I checked it by hand. SCC `{p, q, r}`: for the pair (`b`, `ℓ`), `R = {q → r}` and `G = ∅`, so `q → r` is removed. The remaining edges `p → q`, `q → p` and `r → q` split into `{p, q}` and the trivial `{r}`, which has no cycle. `{p, q}` has no `R` edge, so it passes, and `p` is accepting, so `p q p q …` is a fair accepting lasso. The whole-SCC rule would reject `{p, q, r}` and miss it, which is the false `proved`. The pass condition is right: when every pair with an `R` edge also has a `G` edge, a cycle covering every edge of a strongly connected `C` is fair and visits the accepting state. The lasso rule and AC-5 are unchanged and agree with the cycle definition. LT-11 still settles `proved`: the stale-read loop's SCC holds only `R` edges for (reader, flag), and removing them leaves no cycle. Machine closure still holds, because the mo-last message is always an allowed load choice. The paragraph follows QSL ADR-025 MF-2 at 0455abe almost word for word: the same `R`/`G`, the same fair-cycle and pass conditions, and the same "never rejected for a stale load alone". QSpec leaves out MF-2's termination sentence and its "together with every other constraint" (see SR-13406). EARS: the diff adds no SHALL statement, and the new prose and AC are concrete and testable apart from FND-003, so EARS conformance is folded into this review, not a separate artifact. Traceability: `quire matrix` lists FR-437-AC-7 as untagged, as AC-1 to AC-6 are on main. The tests live outside this repo. `quire validate` passes both touched files on the head and on main, with the same output.
