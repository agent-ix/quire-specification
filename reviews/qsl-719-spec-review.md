---
id: SR-7835
title: "Base specification review of QSpec PR #233"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@103dcc0707c26521a7628fb8611e30b15b2079e6; proposals/quire-v1/definitions/value-accounting.md; FR-151; FR-153"
review_set: subset
---

## Summary

Reviewed the changed accounting paragraph against FR-153's subtype-coverage rule and FR-151's conformance definition, including the claimed whole-tree stale-mechanism sweep. The predicate is equivalent and the sweep's remaining matches are either unrelated terminology or an explicit rejection of the old map; no defect found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS, clean on exact PR head `103dcc0707c26521a7628fb8611e30b15b2079e6`. FR-153 defines closure for queried `T` by coverage of every effective type conforming to `T`; a declared member covers itself and every type conforming to it. FR-151 defines conformance as equality of effective type or declared-supertype reachability, so it is reflexive and transitive. Therefore every type conforming to `T` is covered exactly when `T` conforms to at least one declared member type. The paragraph keeps this decision before the first query charge, adds no charge point/row, and preserves per-member `population.visit` charging.

The full tracked-tree search found the changed paragraph's “no separate retained descendants map” as a deliberate negative statement, not an instruction to retain or consult that mechanism. Other descendant hits are normalization/redefinition/dispatch or verification-plan semantics; retained-relation hits concern protocol/recovery state. No old allInstances lookup instruction remains.

## Checks

- Verified GitHub PR #233 remains open at exact head `103dcc0707c26521a7628fb8611e30b15b2079e6`; local worktree HEAD matches, and the PR diff contains only `proposals/quire-v1/definitions/value-accounting.md`.
- Read FR-151 and FR-153 at the reviewed checkout and checked their formal coverage/conformance statements against the edited paragraph.
- Ran a whole-repository case-insensitive search for the stale mechanism terms. Remaining matches are classified in the verdict and PR-body inventory.
- `git diff --check origin/main...HEAD` passed. No build or test applies to this prose-only change.
- No applicable AssuranceProfile with `review_selection` was found.
