---
id: SR-13406
title: "Integrity review of QSpec PR #294 (QSL-567): visibility fairness against FR-365, FR-430/FR-368 and QSL ADR-025"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@cb8f1d8; spec/functional/memory/FR-437-explore-release-acquire-memory.md, spec/test-cases/TC-391-litmus-conformance-vectors.md; ticket QSL-567"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-437
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-365
    type: reviews
---

## Summary

Ticket: QSL-567. PR: quire-specification#294 at cb8f1d8 (cb8f1d8f612c067ee9c120211aed42dd46b3526b), scoped to `git diff refs/remotes/origin/main...cb8f1d8f6` (origin/main 0ad3768). The diff touches two files. FR-437's visibility fairness paragraph replaces the whole-SCC rule (for each thread and location the SCC loads, the SCC holds a load of the mo-last message) with an edge Streett condition: per pair (thread `b`, location `ℓ`), `R` is the edges on which `b` loads `ℓ` and `G ⊆ R` those that load the mo-last message; an SCC passes when every pair has no `R` edge or a `G` edge, and otherwise the failing pair's `R` edges are removed and the rest decomposed and tested again. FR-437-AC-7 and TC-391 row LT-12 add the three-state example. No schema, vector or code changed. This is the QSpec half of QSL-567; the QSL half is quire-spec-language#1084 at 0455abe (ADR-025 MF-2/MF-5, FR-228, TC-673), which another reviewer covers for the algorithm.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-437 | spec/functional/memory/FR-437-explore-release-acquire-memory.md | examined |
| FR-365-passing-component | spec/functional/temporal/FR-365-build-canonical-temporal-counterexample.md | examined |
| FR-430-fairness-set-members | spec/functional/protocol/FR-430-apply-protocol-fairness.md | examined |
| FR-368-fairness-kinds | spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md | context_only |
| QSL-ADR-025-MF-2 | quire-spec-language:spec/decisions/ADR-025-weak-memory-models-for-parallel.md | examined |
| QSL-ADR-025-MF-5 | quire-spec-language:spec/decisions/ADR-025-weak-memory-models-for-parallel.md | examined |
| QSL-FR-228 | quire-spec-language:spec/functional/FR-228-derive-memory-fairness-constraints.md | examined |
| FR-434-contributions | spec/functional/memory/FR-434-select-memory-models-for-parallel.md | context_only |
| FR-436-flush-fairness | spec/functional/memory/FR-436-explore-total-store-order.md | context_only |
| FR-439-replay | spec/functional/memory/FR-439-bound-replay-and-report-weak-memory-runs.md | context_only |
| schema-Fairness | proposals/backend-provider-v1/schema.json | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-365 owns the SCC search behind every liveness verdict and canonical counterexample. It defines a passing component by acceptance, weak and strong constraints only. Its only refinement removes the states that enable a failing strong constraint, and the first passing component is "first in discovery order, depth-first through refinement". FR-437 now adds a second refinement, which removes edges. But FR-365 does not include visibility fairness in the passing component test, does not say how edge removal combines with strong-constraint state removal, and does not list the visibility obligations for the greedy walk, that is, a `G` edge for each pair the passing component still loads. QSL ADR-025 MF-2 at 0455abe says the decomposition runs "together with every other constraint", and that CX-5's walk takes each pair the passed SCC still loads as an obligation. QSpec has neither. So two implementers can choose different first passing components, which breaks FR-365-AC-1's byte-identical counterexample. And a walk that follows FR-365 as written can close a loop with no `G` edge, which FR-439's replay then refuses as unfair. Fix: in FR-365, add the visibility pairs to the passing-component test, add edge removal to the refinement rule in a fixed order (for example, in one step remove the `R` edges of every failing pair and the states of every failing strong constraint, then decompose), and add one `G` edge per loaded pair to the walk's obligations. Add an FR-437 → FR-365 relationship. | spec/functional/memory/FR-437-explore-release-acquire-memory.md:121-135 |
| FND-002 | medium | FR-430 makes every member of a protocol fairness set `{kind, target, granularity, origin}` with `kind` weak or strong. FR-368 routes an item by those kinds: a backend whose `fairness_kinds` lacks a kind in the set gets `unsupported`. FR-437 gives the visibility member's origin and target, but not its kind or granularity. Before this change, QSL ADR-025 MF-5 said "the memory constraints are weak", which at least implied `weak`. PR #1084 replaces that with "visibility fairness is the Streett condition of MF-2", so neither repo now says what `kind` the member carries. A backend that advertises only `weak` and implements only weak SCC checks would still receive `ra` liveness items and could keep applying the whole-SCC check, the unsound rule this PR removes. Fix: state the member's kind and granularity in FR-437, and state what FR-368's kind check requires of a candidate for it. The condition has the shape of strong fairness (if `R` infinitely often, then `G` infinitely often), but over taken edges rather than enabled states, so it may need its own kind. Related and not new: QSL FR-228's Outputs and AC-1 say one visibility constraint per (thread, loaded location), while FR-437 has one member with the target `{"type":"visibility","model":"ra"}` and defines the pairs inside it. Settle the cardinality in the same change, so the fairness sets (and so the obligation identities) agree. | spec/functional/memory/FR-437-explore-release-acquire-memory.md:121-123 |

## Verdict

Not mergeable as is: two medium cross-spec gaps that the new rule opens. FR-365 has no place for an edge-removal refinement or visibility obligations, and the visibility member has no `kind` now that QSL no longer calls it weak. I searched all of QSpec (spec/, proposals/, tests/, docs/) for SCC, strongly connected, visibility, mo-last, stale and memory fairness. No other text states the whole-SCC rule: FR-437 was its only QSpec statement, and FR-439's "flush or visibility constraints" and FR-434's contribution row do not depend on its form. Nothing in the schemas or vectors depends on the old rule. The backend-provider `Fairness` def carries only `kind`, `operation` and `granularity`, with `kind` weak or strong, and the `weak-fairness`/`strong-fairness` reasons enum belongs to FR-418's timed zone certificate. No vector names visibility fairness. `cargo test --test backend_provider_v1 --test checked_package_v2` passes on the head (9 and 108). Flush fairness (FR-436, weak `each`) is unaffected, because a weak constraint is decided soundly on the whole SCC. QSpec has no protocol-liveness SCC certificate, so QSL FR-228-AC-5's certificate clause has no QSpec counterpart to align. Note for the #1084 reviewer: QSL FR-228-AC-5 claims the claim is `refuted` with `p q p q …`, and TC-673 step 5 checks "a protocol with that shape". The same objections as SR-13405 FND-001 apply there: `r → q` by `a` cannot undo `b`'s load, and the default weak scheduler constraint for `b` rejects `{p, q}` unless `scheduling adversarial;`.

## New findings (disposition pass 1)

Against head ad420fd (ad420fd80fc30c73df63e284475534604291b157). The fix commit touches FR-437, FR-365, FR-430 and TC-391, and commits this review's round-0 SR files unchanged under `reviews/`. `quire validate --scope .` exits 0 on every touched file, with output identical to main for the four spec files. `cargo test --test backend_provider_v1 --test checked_package_v2` passes 9 and 108. `quire matrix` lists FR-437-AC-7 and FR-365-AC-8 as untagged, as are every other FR-437 and FR-365 AC on main. Refinement oracle (`oracle.py`, `oracle_big.py` in the reviewer scratchpad): over 4,000 random graphs (2-5 states, up to 9 edges, 0-2 strong constraints, 1-2 visibility pairs) the one-step refinement finds a passing component exactly when a brute-force search over strongly connected edge sets finds a fair accepting closed walk (0 mismatches, 1,967 fair). Over those graphs and 20,000 larger ones (3-8 states, up to 16 edges, 1-3 of each), every permutation of the strong-constraint and pair order gives the same first passing component (0 differences), and every passing component's kept edges hold a `G` edge for each pair they load (0 failures). Sequential refinement (strong first, or visibility first) also never gave a different first component, so the one-step rule is a sufficient way to make the order fixed, not the only one.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | FR-365-AC-8 cites TC-300, but TC-300 does not cover it. TC-300 is the Ladder model subject with two strong constraints. It has no `ra` subject, no visibility pair and no step or expected result for AC-8. The AC's second clause, "an SCC that fails both a strong constraint and a visibility pair", names no concrete graph, so a test author has to invent one. Its first clause says "Over an `ra` subject whose SCC `{p, q, r}` ...", but TC-391 now says, correctly, that this graph has no protocol form unless a stale load leaves the state unchanged and the protocol uses `scheduling adversarial;`. So AC-8 should say "a synthesized SCC", as AC-7 does. Fix: add a TC-300 step (or a new TC) that passes synthesized SCCs to the search. One is AC-7's graph. Another fails one strong constraint and one pair at once, for example: states 0-3, `s` enabled only at 0 and never taken, edges 0→1, 1→0, 1→2 (a stale load), 2→3, 3→1 and 3→3, with 3 accepting. One refinement step removes state 0 and the edge 1→2, and the first passing component is `{3}` with the loop 3→3. Give that step its expected first component and loop. Also, FR-365 jumps from AC-6 to AC-8 with no AC-7, so renumber AC-8 as AC-7 unless the number is reserved. | spec/functional/temporal/FR-365-build-canonical-temporal-counterexample.md:138 |

## Dispositions

Round 1, against head ad420fd.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
