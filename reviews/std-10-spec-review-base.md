---
id: SR-13627
title: "Base review of STD-10 AD-015 cardinality-element note (PR #305)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; spec/assurance/AD-015-sum-types-and-case-exhaustiveness.md (PR #305 head 5c18355)"
review_set: subset
date: 2026-10-21
relationships:
  - target: ix://agent-ix/quire-specification/AD-015
    type: references
  - target: ix://agent-ix/quire-specification/FR-146
    type: references
---

# Base review of STD-10 AD-015 cardinality-element note (PR #305)

## Summary

Ticket: STD-10 (side ticket STD-235). PR #305 rewrites the AD-015 "Tracked separately" paragraph. The new claim is true against FR-146: the strictly-smaller list gives a cardinality element "never, because Complete V1 has no source form that proves a strictly smaller collection", so a cardinality position can only be equal on a call edge and can never be the position that discharges `decrease`. The paragraph itself carries no status or tracking wording. The author's claim that the rest of STD-10 is already on main holds on a spot-check: FR-143 `union-decl` and FR-143-AC-12, FR-146 "Case exhaustiveness" with `unproved-exhaustiveness` (also in FR-271, FR-272 and native-diagnostics.md), TC-262..TC-265, inventory rows V1-TYPE-032..034, and AD-003's recursion wording scoped "under AD-005" (d70cd64, #121). STD-235 exists in Backlog for the dead element. code-review not applicable: the diff has no code. quire validate: 134 failures on main and on the PR head, none in AD-015.

Units examined:

- `AD-015` (examined): spec/assurance/AD-015-sum-types-and-case-exhaustiveness.md
- `FR-146` (context_only): spec/functional/expressions/FR-146-check-total-pure-functions.md
- `FR-143-AC-12` (examined): spec/functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md
- `V1-TYPE-034` (examined): docs/v1-capability-inventory.md

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The heading `## Tracked separately` now titles a paragraph that tracks nothing; it still reads as a tracking note. Rename it for what the paragraph states (for example `## Cardinality measure element`) or fold the sentence into Risks. | spec/assurance/AD-015-sum-types-and-case-exhaustiveness.md:89 |
| FND-002 | low | Outside the diff, but within the brief's status-wording check: AD-015's `## Status` section keeps status wording ("Implementation and qualification remain tracked separately."). It is the only AD in spec/assurance with a Status section. Delete it, or defer it to STD-235's pass. | spec/assurance/AD-015-sum-types-and-case-exhaustiveness.md:95-98 |

## Verdict

Mergeable. Two low findings: the section heading and the untouched Status section still read as tracking/status notes. Neither is a wrong statement.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
