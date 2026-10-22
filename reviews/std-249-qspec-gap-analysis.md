---
id: SR-14032
title: 'Gap analysis of PR #330: binder record of an abstract node (STD-249), changed ACs against TC rows'
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-specification@2f0a09d; git diff origin/main...2f0a09d (PR #330, branch std-protocol-249-qspec, based on main, not stacked): spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md, spec/test-cases/TC-386-explicit-protocol-step-rows.md; ticket STD-249'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-432
  type: reviews
- target: ix://agent-ix/quire-specification/TC-386
  type: reviews
---

## Summary

Ticket: STD-249. PR: quire-specification#330, head 2f0a09d. Method: gap analysis of the new ACs against their TC rows, done by hand from the spec. Spec-only PR, so no matrix run. Plan completion: not assessed. Tooling: quoin 0.28.3, quire 0.36.2 (engine 0.50.2), spec-artifacts-process@b82e42e.

FR-432-AC-14 (Test, TC-386) is covered by BR-01: the same five rows and the four binder records. FR-432-AC-15 (Test, TC-386) is covered by BR-02: the same six rows with the same split. Both share the spelling defect recorded as SR-14030 FND-001/002. No test is tagged with either AC yet; the code work is QSL-887, as the PR body says, so that is not a finding.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-432-AC-14 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:235 | examined |
| FR-432-AC-15 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:236 | examined |
| TC-386 BR-01 | spec/test-cases/TC-386-explicit-protocol-step-rows.md:48 | examined |
| TC-386 BR-02 | spec/test-cases/TC-386-explicit-protocol-step-rows.md:49 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-432-AC-15 and TC-386 BR-02 require the count refusals to retain `expected` and `actual` but give no values. FR-432 defines them as the two counts, so each row has a definite pair (2 and 3, 2 and 1, 2 and 1, 1 and 2), and FR-147-AC-14 pins its pair. Without them a test passes with the counts swapped or with type values. Fix: give the pairs in AC-15 and BR-02. | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:236 |

## Verdict

Both new ACs have a TC row. One low finding: the retained count values are not pinned.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | FR-432-AC-15 and TC-386 BR-02, as fixed, give `actual` for `node Spec::A::eb(1)` as "the type of `1`" and nothing more. QSpec types an integer literal by its context (FR-149: an integer literal is typed by the other operand), so with the expected field type Boolean the value is not determined by the spec, and a test written from BR-02 cannot assert it. Every other retained value in AC-15 is now pinned. Fix: name the type (for example an integer type, or the literal's own `Int[1, 1]` if that is the rule), or say `actual` is any integer type. | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:239 |

## Dispositions

Re-checked at head 3d88428. Gates at head 3d88428: `quire validate --scope . spec/**/*.md` reports 134 failed documents, none of them FR-432 or TC-386 (touched files add none); `cargo test --test checked_package_v2 --test backend_provider_v1` passes (117 and 14 tests). The head contains origin/main. FR-432-AC-14/15 and TC-386 BR-01/02 are used by no other open QSpec PR. The PR body states behaviour, defaults, gates and remaining work and paraphrases no review finding. The pairs given are the field count and the written count of each row, recomputed by hand.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fixed text at the new head: \| FR-432-AC-15 \| In the protocol of FR-432-AC-14, each refusal is `ill_typed`/`operator-ineligible` at the node retaining `expected` (the field count) and `actual` (the argument count): `node Spec::A::a1(self, 1, 2)` with 2 and 3, `node Spec::A::a1(self)` with 2 and 1, `node Spec::A::ev(true)` with 2 and 1, and `node Spec::A::eb(true, false)` with 1 and 2; `node Spec::A::eb(1)` refuses `ill_typed`/`type-mismatch` at the argument with `expected` `Boolean` and `actual` the type of `1`, and `node Spec::A::ev(true, true)` with `expected` `Int[0, 9]` and `actual` `Boolean`. \| Test (TC-386) \| |

## Fix pass

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | fixed | `node Spec::A::eb(1)` is replaced by `node Spec::A::a1(self, true)`, a type-mismatch at the argument `true` with `expected` `Int[0, 9]` and `actual` `Boolean`, in FR-432-AC-15 and TC-386 BR-02; the QSL half uses `a1(true)` with `actual` `Boolean`. |
