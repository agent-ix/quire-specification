---
id: SR-13326
title: "Scope-boundary review of QSpec PR #287 (STD-211)"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-specification@1e274e7; spec/functional/analysis/FR-375-declare-model-refinement.md, proposals/quire-v1/shared-grammar.md; QSL FR-135 in quire-spec-language PR #1038 (context)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-376
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-377
    type: reviews
---

## Summary

Ticket: STD-211. Checks where the new rules are allocated: language gap vs producer support set, grammar keywords vs identifiers, and which of the PR's eight design choices the spec itself records.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-375-unbuilt | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-gap | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-relationships | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-AC-16 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| GRAM-forms | proposals/quire-v1/shared-grammar.md | examined |
| GRAM-aggregate | proposals/quire-v1/shared-grammar.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `abstract-relationships` is a language gap placed in the producer support set. FR-375 says the refusal "is an implementation gap, not a source defect (FR-452 classes it `unsupported`)", and FR-272 defines `not-yet-implemented` as "a valid complete-V1 source form ... that the producer's support set does not build". For `abstract-relationships` (and `nested-reference-row`) the PR also says "this requirement states no meaning for either, so a producer's support set holds neither": no producer can build them, so users are told the producer is incomplete when the language is. QSL FR-135 (#1038) states it plainly: "`abstract-relationships` is a gap in QSpec ... not a feature this build has left unbuilt". Either classify these two as a language-level refusal (for example `declaration-form` naming the selected profile until a requirement defines them) or qualify the implementation-gap paragraph and FR-452 for them. | spec/functional/analysis/FR-375-declare-model-refinement.md:180-188,217-225; spec/objects/foundation/FR-272-typed-diagnostic-cause.md:67; QSL FR-135 (#1038) unbuilt-features bullet |
| FND-002 | low | Two scope effects of the contextual keywords are not recorded (in the spec or in the PR body's eight choices). Inside a refinement row `only(`, `seq(` and `Some(` always read as the new forms, so a concrete-model pure function named `only`, `seq` or `Some` cannot be called from a row; and `where` is already a keyword spelling in the temporal `aggregate-term` (`[ 'where', 'holds', ... ]`), so "remain identifiers elsewhere" is not true of `where`. State the shadowing and scope the `where` sentence. | proposals/quire-v1/shared-grammar.md:687-689,868 |

## Verdict

Mergeable on this lens after FND-001 is ruled on. On the eight design choices: all eight are listed in the PR body, but only some reach the spec. Recorded in the spec: 2 (abstract relationships), 3 (`Some` contextual), 4 (`null` refused), 5 (node order), 6 (value profile), 7 (no match count), 8 (unmapped types); choice 1's "step arguments and history updates are not addressed" is in the PR body only (SR-13325 FND-004). Not flagged anywhere: function-name shadowing by the contextual keywords (FND-002 here), the `?`-reading's `null`-to-absent collapse (SR-13323 FND-003), the optional-reference interaction with `nested-reference-row` (SR-13325 FND-003), the evaluator-stop settlement (SR-13325 FND-001) and the missing cause on the value-profile refusal (SR-13323 FND-002). Allocation is otherwise sound: the six population productions and `..` move to the shared grammar with FR-376 keeping meaning, and the ticket's item 8 (division law in source) is correctly left out after QSL restated FR-141-AC-7 with Euclidean `mod`.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The `declaration-form` ruling stretches FR-272's payload. FR-272 defines `declaration-form`'s payload as "the recognized form, owning declaration and exact selected profile that prohibits it", and FR-001 names the selected layer whose admitted-form set excludes the form. For abstract relationships no profile or layer prohibits the form: no profile admits it. FR-375 still names "the exact selected profile identity" (the infinite-trace alias), which reads as that profile prohibiting it, and FR-452 classes the refusal `prohibited`, implying another selection could admit it. Accepting the ruling, say in FR-375 (and in FR-272's row, or a note) that for this form the named profile is the refinement's selection and that no profile admits the form yet. | spec/functional/analysis/FR-375-declare-model-refinement.md:281-290,411; spec/objects/foundation/FR-272-typed-diagnostic-cause.md:67 |

## Dispositions

Round 1, against head 5a66bf9.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 5a66bf9: planner ruling applied: `abstract-relationships` leaves the support set; a refinement of a model with a relationship refuses `unsupported_construct`/`declaration-form` at the header alias, independent of any support set (FR-452 `prohibited`, not `unsupported`). The payload wording is a stretch of FR-272: see new FND-003 |
| FND-002 | fixed | 5a66bf9: the grammar now states that `only(`, `seq(` and `Some(` hide a same-named pure function inside a row, and that `where` is a keyword in `binder` and in the temporal `aggregate-term` |
