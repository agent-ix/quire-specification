---
id: SR-870
title: "Spec review — STD-116/117/119/121 abstraction relation, refinement gates, profile layering"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@896aa54d5cc4fdc17325c918959fe130294adcd4; PR #171; FR-353, FR-450, FR-451, FR-452, FR-453, FR-290, FR-301, TC-268, TC-410..414, proposals/quire-v1/shared-grammar.md, proposals/quire-v1/definitions/README.md, proposals/checked-package-v2 schemas"
review_set: subset
---
# Spec review — STD-116/117/119/121 abstraction relation, refinement gates, profile layering

## Summary

Ticket: STD-116 (with STD-117, STD-119, STD-121). Base checklist over
`git diff origin/main...HEAD` of PR #171, checked against the STD-119 owner
ruling (option A), QSL ADR-017 (origin/main) and QSL ADR-020 (draft PR #564,
§7). The five AD-003 `requires` edges in FR-453 match AD-003's hierarchy
table exactly. ADR-020's separation holds: FR-450 keeps the abstraction
relation a declaration distinct from `relation-clause`, FR-290 no longer
labels the operation-contract row a "refinement gate", and FR-453/FR-452
record no claim. The FR-301 pin wording and the tests.md row-count totals
were removed (correct under the no-ceremony rulings). `tc_233` fails on the
digest chain; #174 deletes it, so it is a merge item, not a finding.

## Verdict

**FAIL as it stands.** FND-001 makes every FR-453 child-side compile fail
profile selection, so the layering gate cannot run as written. Header
selection of each layer (the core of option A) has no owning FR/AC, and the
"real user subset" check is not tied to any target or tier.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-453 requires parent and child units byte-equal except the identity string of one `profile` declaration, but the grammar's `profile` declaration also carries `version` and `digest`, which differ per layer; a child unit keeping the parent's version/digest cannot select the child layer, so every case is a tool failure. | spec/functional/tooling/FR-453-preserve-parent-profile-refusals-under-layering.md:34; proposals/quire-v1/shared-grammar.md:100 |
| FND-002 | medium | Header selection of each AD-003 layer, with admission restricted to its admitted-form set and `unsupported_construct` naming the layer, is stated only in the definitions README and shared-grammar prose; FR-001 is not amended and no FR AC owns it outside the gate. | proposals/quire-v1/definitions/README.md:59-68; spec/functional/foundation/FR-001-select-native-profile.md:53 |
| FND-003 | medium | README says selecting a layer selects the layers it requires and lists value.complete as requiring state.core (edge E3), but value-complete.md's always-selected closure does not include state.core. | proposals/quire-v1/definitions/README.md:65,75; proposals/quire-v1/definitions/value-complete.md:10 |
| FND-004 | medium | The "real user subset" check (STD-119) is only a corpus-authored witness and a proper-subset test; the README's target/tier claims link to no lowering target or qualification tier that accepts the layer, so a layer no target accepts still passes. | spec/functional/tooling/FR-453-preserve-parent-profile-refusals-under-layering.md:104-116; proposals/quire-v1/definitions/README.md:70-76 |
| FND-005 | medium | FR-452 delegates the classification of compile and run results to quire-spec-language FR-340..344 and never states how a run result maps to admit/refuse, including that an undefined evaluation (refuted, UndefinedEvaluation) is a refusal. | spec/functional/tooling/FR-452-preserve-admission-across-specification-revisions.md:68-69 |
| FND-006 | medium | FR-452's prior-side rules overlap: a prior unsupported at compile is both "unresolved" and "fails to compile" (tool failure), and a prior compile refusal is both "not applicable" and tool failure; ADR-017 RF-2 resolves these by stage, FR-452 does not. | spec/functional/tooling/FR-452-preserve-admission-across-specification-revisions.md:53-58 |
| FND-007 | low | Normative layer text names what each layer lacks ("no named predicates, references or recursive values", "still with no references", "with no object identity or navigation", "never selects a layer that requires it") instead of stating what it admits. | proposals/quire-v1/definitions/README.md:65-76 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | medium | This PR changed FR-001 to select profiles by exact identity only, and the profile declaration no longer carries a digest, but FR-001-AC-3 and TC-005's procedure (select exact version/digests and mutate each) still test a definition digest; the AC tests nothing and is digest ceremony to delete. | spec/functional/foundation/FR-001-select-native-profile.md:73 |
| FND-009 | medium | The accepted_by names (oracle, kani, property-tests, cvc5-z3, verus) are not FR-331 backend identities and no FR defines them, and the witness check does not say which FR-290 kind each witness item requests, so the AD-010 negotiation the check relies on cannot be run as written. | spec/functional/tooling/FR-453-preserve-parent-profile-refusals-under-layering.md:111-117 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-010 | high | The witness item names no backend and takes the registry's whole candidate set for its kind, but FR-290's candidate table settles an unnamed item with more than one candidate as invalid-request/ambiguous-backend; once two registrants share operation-contract (Kani and the FR-349 SMT backend, as FR-290 itself notes), every witness regresses on a real registry, and TC-414 PL-06 hides this by using a one-backend test registry. | spec/functional/tooling/FR-453-preserve-parent-profile-refusals-under-layering.md:118-131,154 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 1f0e91b |
| FND-002 | fixed | bc860c9 |
| FND-003 | fixed | bc860c9 |
| FND-004 | fixed | bc860c9 |
| FND-005 | fixed | bc860c9 |
| FND-006 | fixed | bc860c9 |
| FND-007 | fixed | bc860c9 |
| FND-008 | fixed | 1f76aa1 |
| FND-009 | fixed | 1f76aa1 |
| FND-010 | fixed | 3a894ac |
