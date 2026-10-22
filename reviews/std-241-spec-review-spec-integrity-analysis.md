---
id: SR-14096
title: "integrity review of STD-241 typed ra key form (QSpec PR #336)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@7a168cec646cc13f12c56cb48854e3aaedee6351 (diff against std-ra-247-qspec e69cade8); spec/functional/memory/FR-437-explore-release-acquire-memory.md, spec/functional/memory/FR-439-bound-replay-and-report-weak-memory-runs.md, spec/test-cases/TC-394-memory-counterexample-wire-and-replay.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-437
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-439
    type: reviews
---
# SR-14096: integrity review of STD-241 (QSpec PR #336)

## Summary

Ticket: STD-241. The PR is stacked on std-ra-247-qspec (#333), so the
review covers only its own diff against that base. It has no QSL companion.

Code read: `qsl-eval/src/simulation/protocol/ra.rs` (`view_enc`,
`event_enc`, `graph_enc`, `enc`, `sc_event`) and
`qsl-replay/src/model_check/replay.rs`.

MK-09 and FR-437-AC-14 were checked by hand against the engine. After `Sx`
(`seq_cst`), there is one write event on `x` at depth 0, with no edge, no
`owed` entry and no summary. `sc_event` sets `left`'s `cur` frontier to its
own event (count 1), and `right`'s frontier stays empty. `x` and `y` are both
at depth 0. MK-10's claim `not (c1.r = 1 and c2.r = 1)` is refutable under
`seq_cst` (`Sx`, `Sy`, `Lx` and `Ly` each reading the new message), and its
graph carries edges, so the edge-removal variant can be run. The `ordinal`
tells apart the two live fences of one thread that FR-437-AC-13 keeps.

Gates: `quire validate` reports 134 failing documents, as on main, none of
them touched. `cargo test --test checked_package_v2 --test
backend_provider_v1`: 14 and 117 passed. The PR merges cleanly with #334 and
with #329.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-439-AC-10 and TC-394 MK-10 assert that an altered carried component is refused `stale_dependency`/`revision-mismatch`. Replay refuses a differing memory component with `content-mismatch` (`replay.rs` `MemoryComponent`), and QSL FR-227-AC-2, which the ticket's acceptance cites, says `content-mismatch`. The PR records the disagreement but opens no follow-up. | spec/functional/memory/FR-439-bound-replay-and-report-weak-memory-runs.md:157, qsl-replay/src/model_check/replay.rs:267 |
| FND-002 | medium | The typed view says "`at` holds every shared location; a pointer at a removed message encodes as the depth of its location's earliest live message", but `view_enc` omits an entry whose message was removed. Remaining work lists only `owed` and `ordinal`, and neither MK-09 nor MK-10 exercises the rule. Same root as SR-14090 FND-002. | spec/functional/memory/FR-437-explore-release-acquire-memory.md:70, qsl-eval/src/simulation/protocol/ra.rs:862 |
| FND-003 | low | FR-437's Key bullet still says events are "named by thread and message depth", while the typed event also carries `kind` and `ordinal`, which the fences of one thread need because they have no message. | spec/functional/memory/FR-437-explore-release-acquire-memory.md:176 |
| FND-004 | low | QSL FR-229's Outputs still describe views "as maps from location to message position", which contradicts the typed view (`{"type":"view","at":…depth…,"frontier","owed"}`). Nothing in the stack updates the QSL half. | spec/functional/FR-229-carry-the-memory-component-through-the-protocol-system.md:58 |

## Verdict

Findings. FND-001 needs a decision on which refusal cause is right: the QSL
and engine `content-mismatch`, or QSpec's `revision-mismatch`. FND-002
should go into the Remaining work or a test vector. The typed form is
otherwise complete and adopts the ticket's engine form, with two justified
additions. The PR body contains no review-finding paraphrase.
