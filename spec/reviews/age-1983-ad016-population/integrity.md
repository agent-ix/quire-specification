---
id: SR-202611101500003
title: "integrity review of AD-016 kernel row PopulationId amendment (PR #647)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification; spec/assurance/AD-016-semantic-family-extension-path.md:353"
review_set: subset
---

## Summary

Ticket: AGE-1983. Cross-document consistency of the amended AD-016 kernel row
against FR-201 (lines 43, 64, AC-11), quire-exact `origin/main`, and QSL ADR-013
O-13 Population row (line 413) and QC-21 (line 1295).

The amended cell asserts `PopulationId` "ha[s] the exact structural members of
FR-201" and also cites QC-21 and labels it "population binding admission". These
are two different identities. FR-201:64 defines `PopulationId` as the `NodeId` of
the `relation`/`population` node and says "the binding, anchor and declared maximum
are not components"; the kernel matches (`PopulationId(NodeKey)`, doc "The admitted
`relation`/`population` declaration node identity"). QSL ADR-013 O-13/QC-21 instead
mint a `PopulationId` per binding admission from a seven-member preimage (domain
package, `population_key`, `Direct`/`Pre`/`Post` role, declared maximum, member set,
pre-binding id). The root conflict sits in QSL ADR-013 (outside this repo); this PR
imports it into AD-016 by the "population binding admission" label and the QC-21
citation.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | medium | Amended cell labels `PopulationId` "population binding admission" and cites QC-21 (per-admission seven-member preimage), while the same cell and FR-201:64 make it the population declaration `NodeId` with binding and maximum excluded, as the kernel implements. Scenario: QSL `model` follows the cited QC-21 and expects a `Direct` and a `Post` admission of one population to get distinct `PopulationId`s; `PopulationId::new(NodeKey)` can only carry the node, so they collide. Fix: describe it as "(population reference, the `NodeId` of the population declaration, FR-201)" and cite QC-21 only for the constructor-caller rule; raise the ADR-013 QC-21/O-13 vs FR-201 preimage conflict with the QSL lane. | spec/assurance/AD-016-semantic-family-extension-path.md:353; spec/objects/foundation/FR-201-canonical-identity-domain.md:64 |

## Verdict

FAIL until FND-001 is fixed in the AD-016 cell. Internal consistency of the
constructor/dependency clauses and the location cell verified clean.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1, commit 170341ed: "population binding admission" removed; the cell describes `PopulationId` as the `NodeId` of the population declaration node, matching its own "exact structural members of FR-201" clause, FR-201:64 and the kernel. QC-21 is now cited only for "only QSL `model` calls the `PopulationId` constructor". No new contradiction: "opaque newtype" over a NodeId matches `PopulationId(NodeKey)`, and FR-201-AC-11's exact-member equality holds. |
