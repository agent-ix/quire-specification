---
id: SR-872
title: "Integrity review — STD-116/117/119/121 abstraction relation and refinement gates"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@896aa54d5cc4fdc17325c918959fe130294adcd4; PR #171; FR-450, FR-451, FR-452, FR-453, TC-410..414, spec/complete-v1/tests.md"
review_set: subset
---
# Integrity review — STD-116/117/119/121 abstraction relation and refinement gates

## Summary

Ticket: STD-116. Completeness, consistency and AC-to-TC coverage over the
new FR-450..453 and TC-410..414. Every AC has a TC row in tests.md and a
vector in its TC. Pre-existing `quire validate` failures in tests.md
(column assert) and FR-290 (enumeration id, identifier cells) also fail on
main and are not findings.

## Verdict

**FAIL as it stands** on FND-001: an AC lists a valid Rust identifier as
malformed, so no correct implementation can pass it.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-450-AC-5 and TC-411 AR-05 list the Rust path `"ab"` as malformed, but `ab` is a valid Rust 2021 identifier under FR-450's own "Rust spellings" rule; the AC cannot be met. | spec/functional/ir-proof/FR-450-spell-abstraction-relation-declaration.md:134; spec/test-cases/TC-411-abstraction-declaration-refusals.md:35 |
| FND-002 | low | FR-452's tool-failure and not-applicable outcomes have no AC; TC-413 SV-07 tests not-applicable with no AC behind it, and no vector covers a prior-side tool failure. | spec/functional/tooling/FR-452-preserve-admission-across-specification-revisions.md:56-58,74-79 |
| FND-003 | low | FR-452 Inputs require "different revisions" while Behavior says the gate "compares nothing about" revision labels, and no outcome is stated for a pair whose document identities differ or revisions are equal. | spec/functional/tooling/FR-452-preserve-admission-across-specification-revisions.md:30-32,44-46 |
| FND-004 | low | TC-414 PL-06 expects "verdict pass", which is not an FR-301 verdict; FR-452 and TC-413 use "success". | spec/test-cases/TC-414-profile-layering-refinement-cases.md:38 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | bc860c9 |
| FND-002 | fixed | bc860c9 |
| FND-003 | fixed | bc860c9 |
| FND-004 | fixed | bc860c9 |
