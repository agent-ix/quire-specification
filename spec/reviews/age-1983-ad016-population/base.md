---
id: SR-202611101500002
title: "base review of AD-016 kernel row PopulationId amendment (PR #647)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; spec/assurance/AD-016-semantic-family-extension-path.md:353"
review_set: subset
---

## Summary

Ticket: AGE-1983. PR agent-ix/quire-specification#647 (head 643c8b3d) amends the
AD-016 kernel shared-type row (line 353) to list `PopulationId`, describe it as a
`Value` component type, add it to the QSL-`model`-only constructor clause, keep
`PopulationBinding` in QSL `model`, and repoint the location at the
`quire-exact` repository.

Ground truth checked: quire-exact `origin/main` has `Value::Population(PopulationId)`
(`src/value.rs:558`) and `pub struct PopulationId(NodeKey)` with one public
constructor `PopulationId::new(NodeKey)` (`src/identity.rs:343-349`), exported from
`src/lib.rs:127`; the crate states it "never derive[s] identities from digests"
(`src/lib.rs:17-18`). QSL consumes it as a git dependency on
github.com/agent-ix/quire-exact (QSL `Cargo.toml:14`) and QSL has no
`crates/quire-exact`, so the new location cell is correct. "None depends on QSL
check or model" and "only QSL model calls the EffectiveId and PopulationId
constructors" are consistent (the first is the type's dependency, the second its
caller). `PopulationBinding` staying in QSL `model` matches ADR-013 O-13 and QC-21
and the kernel (`src/value.rs:22-31`).

One defect: the new parenthetical calls `PopulationId` "the opaque digest carried by
`Value::Population`", which contradicts FR-201 "Structural identities" ("never a
digest"), the same cell's "no hash and no preimage" clause, QSL ADR-013 QC-21
("opaque structural-key newtype") and the kernel. One pre-existing low drift in
AD-016's other rows is also recorded.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | high | Amended cell calls `PopulationId` "the opaque digest carried by `Value::Population`"; FR-201 says a structural identity is "never a digest", the same cell says PopulationId is compared componentwise "with no hash and no preimage", QC-21 says "structural-key newtype", and the kernel is `PopulationId(NodeKey)`. A consumer (RT/CG/footprint build) implementing from AD-016 hashes the binding and fails FR-201-AC-11 componentwise equality. Fix: replace "the opaque digest" with "the opaque structural identity". | spec/assurance/AD-016-semantic-family-extension-path.md:353; spec/objects/foundation/FR-201-canonical-identity-domain.md:47-48 |
| FND-002 | low | Pre-existing, outside the diff: AD-016 rows at lines 126 and 425 name `ValueType::Population(u64)`, but the kernel type this row now lists is `ValueType::Population(Option<u64>)` (quire-exact `src/value.rs:392`). | spec/assurance/AD-016-semantic-family-extension-path.md:126; spec/assurance/AD-016-semantic-family-extension-path.md:425 |

## Verdict

FAIL until FND-001 is fixed (one-word change). Location claim, constructor clause,
dependency clause and `PopulationBinding` exclusion all verified correct.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1, commit 170341ed: AD-016:353 now reads "`PopulationId` (the opaque structural identity carried by `Value::Population`, the `NodeId` of the population declaration node)"; no "digest" remains outside spec/reviews/. Consistent with FR-201:47-48, :64, AC-11 and quire-exact `PopulationId(NodeKey)`. |
| FND-002 | fixed | Round 1, commit 170341ed: AD-016:126 and :425 now name `ValueType::Population(Option<u64>)`, matching quire-exact src/value.rs:392 and QSL's use (`ValueType::Population(None)`, qsl-eval/src/simulation/model.rs:1456). |
