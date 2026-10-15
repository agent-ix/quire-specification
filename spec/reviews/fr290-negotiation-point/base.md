---
id: SR-604
title: "Base review of the FR-290/AD-010 negotiation-point amendment"
type: SpecReview
analysis: base
scope: "quire-specification #116 residuals; FR-290; AD-010; AD-016"
review_set: subset
review_date: "2026-09-19"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-290, type: references }
  - { target: ix://agent-ix/quire-specification/AD-010, type: references }
---

## Summary

PR #118 delivered #116's solver-absence policy, FR-290-AC-4 and the
refinement-use rows V1-TOOL-011/012, V1-BACK-017 and V1-BACK-018. This
amendment closes two residuals. FR-290 and AD-010 now name the single
capability negotiation point that accepted AD-016 fixes, and the terminal
dispositions it settles. FR-290's QSL alignment pointer moves from QSL #190 to
QSL #229, the ticket that #190 was split into.

## Verdict

**PASS** — the diff adds no id and no acceptance criterion. It states the
negotiation point and dispositions exactly as AD-016 does, and it leaves the
reading of older four-kind QSL artifacts to QSL #229.

## Review set

Agent A selected base and dependency. The diff adds no FR, AC, TC or inventory id and no new obligation; it aligns FR-290 and AD-010 wording with accepted AD-016. Coverage, testability, formal, consistency and architecture analyses have no new subject in this diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Resolved: AD-016 spells the disposition `invalid-request` in its terminal-disposition rule and `invalid_request` in its CG capability-point row. FR-290 and AD-010 use the terminal-rule spelling `invalid-request`. AD-016 is accepted and not reopened here. | AD-016; FR-290; AD-010 |

## Checklist results

| Check | Result |
| --- | --- |
| ID formats | No FR, AC, TC or inventory id is allocated or changed. |
| Links | `../../assurance/AD-016-semantic-family-extension-path.md` from FR-290 and `AD-016-semantic-family-extension-path.md` from AD-010 resolve. AD-010 frontmatter adds an AD-016 `references` edge. |
| Consistency | Negotiation happens at CG `negotiate_*` and settles `supported`/`requires-bound`/`unsupported` (warned)/`invalid-request`, which matches AD-016's arrow-4 and terminal-disposition rule. QSL `Capability` stays language admission. |
| Wording | Current-state wording only. The text states what is admitted and names no compatibility path. |
| Coverage | FR-290-AC-4 is still covered by TC-060. The added text is descriptive and adds no new obligation. |

## Evidence

- Reviewed artifacts: `spec/objects/protocol/FR-290-protocol-claim-kind.md`,
  `spec/assurance/AD-010-target-neutral-ir-backends.md` and
  `spec/assurance/AD-016-semantic-family-extension-path.md`.
- `quire validate --strict --summary` on all four changed files reported zero
  findings.
