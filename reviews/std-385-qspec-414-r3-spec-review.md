---
id: SR-16265
title: "Round-3 spec-review of the hybrid model node fix-forward (quire-specification PR 414)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@e55b847bc1e69d5b54c7119053d463788938127a (fix commit ce3fec34 against SR-16257; origin/main...HEAD; cross-checked against every open PR head, including #415 at 7bbc566d and #408 at its head); spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md; spec/objects/interfaces/FR-322-checked-package-artifact.md; spec/test-cases/TC-966-checked-package-hybrid-model-nodes.md; tests/checked_package_v2.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-481
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/TC-966
    type: references
---

# Round-3 spec-review of the hybrid model node fix-forward (quire-specification PR 414)

## Summary

Ticket: STD-385. This round re-reviews #414 at head e55b847b: fix commit
ce3fec34 against SR-16257, plus a merge of main. Main is an ancestor of the
head. The companion QSL round is SR-16266.

Gates run by the reviewer:

- `quire validate`: 86 failure lines, the baseline.
- `cargo test`: `checked_package_v2` 136 passed and `backend_provider_v1`
  24 passed.
- QSL `checked_v2` on QSL #1364 head 41697826 with `QSPEC_DIR` at this
  head: 41 passed, 6 failed. Every failure is `UnsupportedNodeTag` at
  `/semantic_graph/nodes/39/node_tag`, the missing QSL reader code and
  nothing else.

Hand derivations:

- HY-21: under `timed/v1`, `[0, 10]` in the timed form (`0/1` and `10/1`,
  both ends `closed`) meets FR-370's fit, passes check 6 (clock
  `model-time`, activation `origin`) and check 7, and is admitted with
  horizon 10.
- HY-13 and HY-14 are main's rows unchanged.
- HY-22's fairness case: the temporal step runs before the hybrid step,
  and resolves each fairness member against an `object_type` declaring
  node (FR-370 "Fairness resolution" step 1). HY-01's package has none, so
  the expected `invalid_semantic_graph` at the fairness argument is not
  reached. A reader that follows the order refuses
  `invalid_model_binding`/`malformed-declaration` at the member's
  `declaration` target first.

Ids, checked against main and every open PR head in the repo:

- FR-481-AC-12 and AC-13, HN-15, HY-21 and HY-22 are clear of #415, which
  holds AC-8 to AC-11, HN-11 to HN-14 and HY-17 to HY-20, and of every
  other PR.
- The TC-233 rows RL-01 and RL-02 are clear.
- FR-322-AC-102 collides (FND-002).

Dispositions of SR-16257: FND-001 (restore the claim-form refusal
vectors) is withdrawn. That request was mine, and it was wrong under the
standing rule. The `model-steps` clock, the `each` activation, `once`,
captures and fairness are restored as admitted features by open #415
(STD-409..415). Pinning their refusal is a narrowing (FND-001 below).

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | [scope-deferral] FR-481-AC-12 and HY-22 pin permanent refusals of hybrid-claim features: an `each` activation (`invalid-value`), non-empty captures and fairness (`invalid_semantic_graph`), a `once` formula (`operator-ineligible`) and the clock `model-steps` (`invalid-value`). Open #415 (STD-409..415) restores every one of these as an admitted feature: HY-13 there admits `each` with a trigger, HY-14 admits `once`, and HN-12 carries a capture and a fairness member. The refusal clauses narrow the feature set and contradict #415, so whichever PR merges second contradicts the first. This round's change came from my own SR-16257 FND-001, which I withdraw. Fix: keep AC-12's `timed/v1` admission and its `holds`-reads-`h` refusal; drop its clock, activation, captures, fairness and `once` refusals and HY-22; leave checks 6 and 7 and their vectors to #415. HY-22's fairness case is also unreachable as written (see Summary). | spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:303; spec/test-cases/TC-966-checked-package-hybrid-model-nodes.md:58 | wrong-requirement |
| FND-002 | high | [id-collision] FR-322-AC-102 (the canonical `rational` literal spelling) collides with open #408 (STD-399, opened 2026-10-23T03:43Z), whose FR-322-AC-102 is the Bytes/JsonObject/Duration native-scalar criterion. The stacked #407, #413 and #418 carry the same AC-102. Across open PRs, FR-322-AC-101 to AC-109 are taken (#398 AC-101; #407/#408 AC-102 to AC-105 and AC-108; #410 AC-106, AC-107 and AC-109). Fix: renumber to FR-322-AC-110 in FR-322, the TC-233 `relationships` and rows RL-01/RL-02, and `tc_233_rational_literals_spell_one_canonical_magnitude`'s trace. | spec/objects/interfaces/FR-322-checked-package-artifact.md:2089 | wrong-requirement |

## Merge order

#414 still cannot merge before QSL's reader code admits the `hybrid` tag.
Merging it earlier turns six QSL `checked_v2` tests red, because QSL reads
the sibling QSpec checkout. #414 and #415 both rewrite FR-481 and TC-966.
Merge #414 first, with FND-001 fixed so that it no longer pins refusals, in
the same window as the QSL reader-code PR. Then rebase #415 onto it.

## Verdict

Not mergeable. There are two high findings: the refusal pins that #415
removes (FND-001), and the FR-322-AC-102 id collision with #408 (FND-002).
The QSpec gates are green, the timed admission is sound, and the other
ids are clear.
