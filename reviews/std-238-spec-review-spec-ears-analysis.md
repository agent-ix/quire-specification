---
id: SR-14811
title: "EARS conformance review of the certificate-rejected catalog statements (QSpec PR 200, STD-238)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@9c60fe7e8347315e5726475ef78d63cc0faae4a5 against merge base 97bfd65e7193d71253f7b84d0ded6218ef8569a5 (PR 200, ticket STD-238); spec/objects/interfaces/FR-331-backend-provider-envelope.md; spec/objects/interfaces/FR-418-zone-certificate.md; spec/objects/interfaces/FR-412-probability-certificate.md; spec/objects/interfaces/FR-413-probabilistic-counterexample.md; spec/test-cases/TC-431-backend-provider-wire-vectors.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-431
    type: reviews
---
# EARS conformance review of the certificate-rejected catalog statements

## Summary

Ticket: STD-238. This review covers the requirement statements PR 200 adds
or changes: FR-331's catalog and reader paragraphs, FR-418's rejection
paragraph, FR-412-AC-3/AC-8 and FR-413-AC-6/AC-8. TC-431's new Expected
Results prose was read for behaviour-only wording.

Examined with no finding:

- The catalog paragraph. It names its trigger and response: "a reader SHALL
  refuse an unknown rule, an unknown locus form and a rule at a locus not
  listed with it before consumption".
- The reader-binding paragraph. It is an ubiquitous SHALL with an explicit
  refusal response for absent, conflicting or ambiguous context.
- FR-418's rejection text and FR-418-AC-2. Each failure is paired with its
  rule and locus.
- FR-412-AC-3. Each failure is paired with its rule and locus, and the
  statement is testable.
- The simulation-step and step-verdict tables. Their members are closed and
  can be checked.

## Verdict

**Minor changes.** Two low findings on wording. Neither changes the wire.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Requirement and test-case text carries review status and fixture commentary instead of behaviour. FR-331 says "Synthetic fixture bytes such as `AQ` and `Ag` are transport-shape samples only, with no canonical meaning or semantic admission claimed. All required rejection outcomes remain required." FR-331's Dependencies says "These shapes still require checker-context and round-trip qualification ... no source amendment closes it." TC-431's Expected Results say "Native reader and runtime qualification for BP-13/14/17 is UNRUN" and "These actual-reader controls are authored obligations and remain UNRUN." Run status and fixture caveats are a build log, not system behaviour. Fix: drop these sentences, or move the fixture caveat into the vectors README. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:278; spec/objects/interfaces/FR-331-backend-provider-envelope.md:624; spec/test-cases/TC-431-backend-provider-wire-vectors.md:136; spec/test-cases/TC-431-backend-provider-wire-vectors.md:150 |
| FND-002 | low | Several new SHALL statements have no bound and no observable response. Examples: "The reader SHALL check the complete closed context and translation-state algebra against the originating checked property, including canonical sets, occurrence paths, past memory, windows and acceptance obligations", "check the owner's complete algebra in the actual checked claim/model context", and "The reader SHALL rederive all projections, draws, clock/tail/control post-keys and elapsed/optional trace delay". They name no failure response beyond a closing "refuse before consumption" elsewhere, and they delegate the whole meaning to QSL FR-338/FR-196/FR-202. Two implementers could check different subsets. Fix: state each one as "When <component> differs from its re-derivation, the reader SHALL refuse <cause> before consumption", or cite the owning QSL criterion instead of restating it. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:203; spec/objects/interfaces/FR-331-backend-provider-envelope.md:224; spec/objects/interfaces/FR-331-backend-provider-envelope.md:267 |
