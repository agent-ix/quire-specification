---
id: SR-9041
title: "Dependency review of QSpec PR #250 (QSL-747 owner-nested member identities)"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-specification@06d9dbd4bd1256090a4fe7e80b9cbfce24d66179; spec/functional/type-model/FR-154-admit-domain-package-model.md, proposals/quire-v1/definitions/model-complete.md; compared with agent-ix/filament-core-data@72527ab5a629b50e1c15f62e8a83826accbcb5f7 (main) spec/functional/FR-095-mint-package-identity-and-provenance.md and crates/extraction-frontend/src/identity.rs"
review_set: subset
---

## Summary

Ticket: QSL-747. FR-154 is the consumer of the identities that FCD FR-095 mints.
The PR changes the consumer side only. On FCD main, FR-095 and its minter mint
the four kinds as global slots: `clause/<Name>-<clauseId>`,
`state/<Name>-<state>`, `transition/<Name>-<from>-<to>-<trigger>` and
`step/<Name>-<step>`. They are produced by `clause_identity`, `state_identity`,
`transition_identity` and `step_identity`, and FR-095-AC-6 and AC-7 assert
them. A Linear search found no FCD ticket that moves these four kinds to the
nested form.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | FR-154 still opens its identity rules with "A declaration's IR node identity is the identity filament-core-data FR-095 mints". The edited model-complete Identity row still says "as FR-154 states from FCD FR-095". The new nested rule then refuses, as `malformed-declaration`, the exact identities FR-095 mints for clauses, states, transitions and steps. Every FCD-lifted package with a clause, state machine or process therefore fails intake under the amended FR-154, and QSL already behaves this way. The spec contradicts its own cited source, and the producer has no tracked change to match it. FR-154 should say it departs from FR-095 for these kinds, and an FCD ticket for FR-095 should be filed. A transition has no name of its own, so that ticket must also say how FCD chooses a transition's member name. | spec/functional/type-model/FR-154-admit-domain-package-model.md:168-191; proposals/quire-v1/definitions/model-complete.md:162; FCD spec/functional/FR-095-mint-package-identity-and-provenance.md:114-118, FR-095-AC-6 | wrong-requirement |

## Verdict

One high finding: an upstream contract contradiction across repos. The
QSpec-internal direction (nested wins) is the planner's ruling and is not
disputed here.

## Dispositions

| FND | Outcome | Sha/Reason |
| --- | --- | --- |
| FND-001 | fixed | e8163084787a91ef8e80c45ac07d5584ba4f26f5 |
