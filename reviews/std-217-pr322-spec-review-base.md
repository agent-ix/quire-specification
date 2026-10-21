---
id: SR-13691
title: "Spec review of QSpec PR #322 (STD-217) refinement step occurrences"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@dcc70c07d1a0905407a5242fbbbb5620727aae33 vs main 6ab82c5; spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
---

## Summary

Ticket: STD-217 (PR #322, head dcc70c07d1a0905407a5242fbbbb5620727aae33; context IR-942, QSL PR #1157). Base checklist over the new "Refinement step occurrences" paragraph, FR-322-AC-88/89 and the TC-233 step. The ordinal rule (initial obligation 0, step row `i` in `steps` order at `i + 1`) agrees with the FR-468 certificate order (base part, then step parts in `steps` order) and with QSV FR-123 (`Initial` orders before every `Step`, steps by zero-based row position). It is distinct from the FR-322-AC-87 ordinal of a derived node, and the text says so. The region rule is consistent with FR-322-AC-52, because both regions come from the owner's own declaration and rows. The text describes behaviour, not status.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 refinement step occurrences | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322 generated role | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-88 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-89 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| TC-233 refinement step step | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md | examined |
| FR-322-AC-87 | spec/objects/interfaces/FR-322-checked-package-artifact.md | context_only |
| FR-322-AC-52 | spec/objects/interfaces/FR-322-checked-package-artifact.md | context_only |
| FR-468 | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | context_only |
| FR-375-AC-19 | spec/functional/analysis/FR-375-declare-model-refinement.md | context_only |
| QSL FR-146 | agent-ix/quire-spec-language spec/functional/FR-146-write-per-step-simulation-records-for-a-refinement.md | context_only |
| QSV FR-123 | agent-ix/quire-semantic-value spec/functional/FR-123-step-simulation-record-and-obligation.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The occurrences are defined as "one for each obligation of its per-step simulation proof", but FR-468 states a per-step proof only for a refinement with no hidden field, no history field and no `any` row (FR-468-AC-1: such a declaration "has no per-step query"). The reader rule still requires `generated` ordinals 0 to `n` on every refinement owner. For a refinement with an `any` row, a producer that follows the definition emits no `generated` occurrence and the reader refuses it at `/source_map`. Either require the occurrences only when FR-468's per-step proof applies, or define them by step row independently of FR-468, and add a vector for the case chosen. QSL FR-146 confirms the producer side: FR-146-AC-1/AC-2 say a refinement with a hidden field, a history field or an `any` row (`RegisterHistory`, `Coin` with `side` hidden, `commitB -> any`) writes no `StepSimulationRecord`, so mandatory occurrences would claim obligations that do not exist. | spec/objects/interfaces/FR-322-checked-package-artifact.md:647-672; spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md:26,71 |
| FND-002 | medium | The unchanged definition still reads "The role `generated` is the role of every occurrence of a *compiler-derived node*, a node the compiler adds that no source text declares". The new paragraph gives `generated` occurrences to a refinement owner, which source text declares. The definition is now false as written, and a reader that enforces it (generated only on nodes with no `declaration` occurrence) refuses every refinement owner. Amend the definition so it names both uses of `generated`. | spec/objects/interfaces/FR-322-checked-package-artifact.md:636-641 |
| FND-003 | low | "a `generated` ordinal greater than `n`" is already covered by "a role and ordinal pair other than the two above", so the list has a redundant item. Drop it, or say that it is an example of the general item. | spec/objects/interfaces/FR-322-checked-package-artifact.md:667-669 |
| FND-004 | low | "the region of the node's `declaration` occurrence" and "the region of the source row" are singular, but FR-322 lets an occurrence name multiple ordered regions. Say "the regions", meaning an equal ordered list, so that a multi-region declaration has one required value. The test compares against a one-element list. | spec/objects/interfaces/FR-322-checked-package-artifact.md:658-660 |

## Verdict

Not mergeable: FND-001 (high) and FND-002 (medium) must be resolved first. QSL PR #1157 should not adopt the rule until FND-001 decides which refinements carry the occurrences. The ordinal convention itself is sound and consistent with FR-468, FR-375-AC-19, FR-322-AC-87 and QSV FR-123.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | fe8930c: occurrences only on a proof-eligible owner (no hidden field, no history field, no `any` target, the FR-468 / QSL FR-146 predicate); a non-eligible owner carries the declaration alone; FR-322-AC-91 with RS-12..RS-15. |
| FND-002 | fixed | fe8930c: the `generated` definition now names the per-obligation occurrences of a refinement owner. |
| FND-003 | fixed | fe8930c: the ordinal-above-n case is folded into the outside-admitted-pairs item. |
| FND-004 | fixed | fe8930c: "exactly the one region" of the row and of the declaration. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | Proof-eligibility hinges on "no hidden field (FR-378)", which FR-378 defines at source level ("An abstract field with no row is hidden"). FR-322 does not say which wire members the reader compares: the abstract model's fields from the supplied abstract package, reached through the body's `abstract` key, against the `objects` entries' field rows. Every vector takes eligibility from a precomputed `hidden` boolean, so no vector tests that derivation. `histories` and the `any` target are body members and are fine. State the wire-level predicate, for example a field of an abstract type `T_A` that no field row of the `objects` entry for `T_A` names, read from the supplied package selected by `abstract`, and add one vector that derives `hidden` from those members. The predicate itself matches FR-468 and QSL FR-146. | spec/objects/interfaces/FR-322-checked-package-artifact.md:656-658; proposals/checked-package-v2/refinement-step-occurrence-vectors.json |
| FND-006 | low | The second reader bullet ("SHALL require the `declaration` occurrence at ordinal 0 and one `generated` occurrence for each ordinal 0 to `n`") is not limited to proof-eligible owners. Read on its own, it requires `generated` occurrences on a non-eligible owner, which the third bullet forbids. Split it: require the `declaration` occurrence on every refinement owner, and require the `generated` ordinals on a proof-eligible owner only. | spec/objects/interfaces/FR-322-checked-package-artifact.md:673-674 |
