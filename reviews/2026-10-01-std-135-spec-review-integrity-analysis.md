---
id: SR-821
title: "Integrity analysis of STD-135/STD-136: grammar, wire and cross-requirement consistency"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@c54d91e1a7e9c69eba77c5f4b1505d192a9cc0e9; git diff origin/main...HEAD (PR #168): proposals/quire-v1/shared-grammar.md, spec/objects/interfaces/FR-331, FR-393, FR-400, spec/objects/temporal/FR-341 (infinite-trace), spec/functional/analysis/FR-390, FR-391, FR-399, FR-401; read against open sibling PRs #169 and #170 at their origin heads"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-391
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-399
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-400
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-401
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
---
# SR-821: Integrity analysis of STD-135/STD-136: grammar, wire and cross-requirement consistency

## Summary

Ticket: STD-135 (with STD-136). PR agent-ix/quire-specification#168, draft.

Checked structural consistency inside the PR and against the sibling QSpec
lanes it builds on. STD-131 (#170) supplies the model subject, the fairness
grammar, the terminal stutter step and the infinite-trace vocabulary. Every
new FR id, AC id and TC id is unique on main. The index rows in spec/spec.md
and spec/complete-v1/tests.md match the files. The defects below are places
where this PR names or reuses a STD-131 artifact in a form that STD-131 does
not define.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `trace-fairness` uses `fairness-constraint`, which no production in the grammar defines. STD-131 (#170) names its production `fairness = 'fair', ['weak' \| 'strong'], ['whole' \| 'each'], qualified-name, ';'`. That production starts with `fair` and ends with `;`, so `fair { weak V::Vault::step }` (TC-337 HB-10, TC-341 PF-01, ADR-023 HS-6) does not parse with it. Define the constraint body this production admits, or reuse STD-131's production as written. | shared-grammar.md:485-486, shared-grammar.md:552-555 |
| FND-002 | medium | FR-400 spells the terminal stutter step as a whole `HyperStep` `{"type":"stutter"}` and says it is STD-131's stutter marker. STD-131's FR-361/FR-364 write it as an ordinary step whose `transition` is `{"type":"terminal-stutter"}`, with a `post_state`. That gives two wire forms for one step. | FR-400:34-35, FR-400:112 |
| FND-003 | medium | The PR edits `spec/objects/temporal/FR-341-infinite-trace-result-disposition.md` and cites `ix://agent-ix/quire-specification/FR-341` for the infinite-trace vocabulary. On main that id names two artifacts (it is also the checked-package state-clause body). The wave-C table says the infinite-trace FR-341 is now FR-360, which #170 renames it to. After #170 merges, these links and the FR-341 scope edit point at the wrong artifact or a removed file. | FR-391:10, FR-391:42, FR-391:116, FR-391:190, FR-399:12, FR-399:139, FR-341 (infinite-trace) Scope section |
| FND-004 | medium | FR-331 adds a per-item `model_check` member for model-check limits and the seed. #170 adds a `temporal` member whose `limits` covers every `temporal-satisfaction` item over a model subject, and state-graph and hyper items are such items. So the same limits sit in two members. The seed and `witness_samples` also change the result's settlement method and evidence, but neither is named in request identity. | FR-331:35 |
| FND-005 | medium | FR-400-AC-3 says a non-terminal stutter marker refuses `invalid_runtime_input`/`invalid-value` before replay. FR-400 Validation lists only the structural rule (a stutter marker followed by a non-stutter step). FR-401 makes a stutter marker at a non-terminal state a refused step that settles `ReplayParity`. One input gets two outcomes. A reader cannot know that a state is terminal before replay. | FR-400:86-87, FR-400:96, FR-401:43-47 |
| FND-006 | low | The body prose refers to STD-131 and STD-134 behaviour without linking the QSpec FRs that define it: "the model-subject verdict map", "a temporal atom over a model subject reads an absent `over` object", and the reduction preservation table. The wave-C table maps these to FR-361..FR-368 and FR-381..FR-385. FR-391 and FR-399 already relate to FR-363 but cite none of the others. | FR-390:54-55, FR-391:80, FR-391:99, FR-399:149 |

## Verdict

**Not mergeable as it stands.** FND-001 to FND-005 are real inconsistencies
in the grammar and the wire. Each one shows up once #170 merges, and #170
must merge first because FR-391 and FR-399 relate to FR-363, which only #170
adds. Within the PR itself, ids, matrices and index rows are consistent, and
every requirement traces to US-011.

## Dispositions

Round 1, reviewed at 2b3fae1 (diff origin/spec/wave-b-q1-temporal...HEAD).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 2b3fae1 |
| FND-002 | fixed | 2b3fae1 |
| FND-003 | fixed | 2b3fae1 |
| FND-004 | fixed | 2b3fae1 |
| FND-005 | fixed | 2b3fae1 |
| FND-006 | fixed | 2b3fae1 |
