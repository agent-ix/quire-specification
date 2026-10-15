---
id: SR-501
title: "Integrity review of the domain object layer"
type: SpecReview
analysis: integrity
scope: "spec/objects/** at d6e19a5: FR-200..FR-206, FR-210..FR-217, FR-220..FR-228, FR-230..FR-239, FR-240..FR-249, FR-250..FR-259, FR-260..FR-269, FR-270..FR-274, FR-280..FR-283, and the behavioral requirements they extract from"
review_set: subset
review_date: "2026-09-11"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-200, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-201, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-205, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-206, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-216, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-222, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-223, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-224, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-225, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-235, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-239, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-240, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-248, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-249, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-258, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-270, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-283, type: reviews }
---
## Summary

The layer is well grounded. Every one of the 73 artifacts is registered in
`spec/spec.md`, every `specifies` edge from an object artifact has a matching
`references` back-edge on the behavioral requirement it names, and the sampled
quotations reproduce their source requirements accurately. The discipline of
recording an `## Open gap` instead of inventing a key is applied widely and,
in the causality and protocol families, rigorously.

The defects are concentrated in three places. First, the result family was
extended after it was written: four vocabularies that FR-240, FR-248 and
FR-249 describe as having no artifact yet now have one (FR-280, FR-281,
FR-282, FR-283), so four controlled vocabularies are defined twice, and two of
the duplicate pairs do not agree. Second, three artifacts record open gaps that
other artifacts in the same batch close by citation, or enumerate vocabularies
that a sibling artifact refuses to enumerate. Third, the `## Identity and
comparison` block is not uniformly complete, and the artifact set's own
acceptance criterion (FR-200-AC-1) does not check the two axes that are
missing.

I confirmed the three reported UNTYPEABLE reasons and they are real. FR-060
names global conformance, monitorability, local projection, refinement,
realizability and composition in prose only, across its Description and its
five acceptance criteria, and fixes no label spelling for any of them.
Participation is named as a retained axis by FR-061 and FR-115 with no member
list, and the only enumerated participation vocabulary in the repository is
FR-062's E02 *target* column. The four progress and closure axes are named by
FR-061 as facts "with exact scope, authority and boundary identities" with no
member states; FR-094-AC-3 states their mutual independence, not a closed
vocabulary. Two of those three reasons are correctly recorded. The third is
contradicted inside the layer itself (FND-004).

## Verdict

CHANGES REQUESTED. The structural extraction is sound and the citation
discipline is good, but the result family carries four duplicated vocabularies
with two live disagreements (FND-001, FND-002), and three artifacts state a gap
or a member list that a sibling artifact contradicts (FND-003, FND-004,
FND-008). These are consistency defects internal to the layer, not defects in
the behavioral requirements, and each is resolvable without an owner ruling.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Four vocabularies are defined twice. FR-240 says assumption state and claim strength each "still warrants its own `enumeration` artifact at a later identifier; this artifact does not claim to be that artifact"; FR-248 says the dependence relation "has no separate `enumeration` artifact yet"; FR-249 says the six refusal codes "have no separate `enumeration` artifact yet". FR-280, FR-281, FR-282 and FR-283 exist at this revision and are registered in `spec/spec.md`. No relationship edge connects either member of any duplicate pair. This is both a stale statement of fact and an atomicity defect: FR-240, FR-248 and FR-249 each carry a second object inline. | FR-240; FR-248; FR-249; FR-280; FR-281; FR-282; FR-283 |
| FND-002 | high | The two refusal-code tables disagree on which code a digest mismatch takes. FR-249 glosses `invalid-source-result` as "The source result fails its strict reader" and puts digest substitution under `identity-mismatch` ("substituting any identity or digest refuses the projection"); FR-283 glosses `invalid-source-result` as "did not validate, or its recomputed `sha256-jcs` digest did not match" and glosses `identity-mismatch` as a supplied identity disagreeing with the derived one. FR-062 assigns neither; FR-062-AC-4 lists "An invalid source" and "missing exact context" as separate refusal conditions. A consumer matches these codes by exact string, so the divergence is load-bearing. | FR-249; FR-283; FR-062-AC-4 |
| FND-003 | high | FR-239 records as unresolved a question the prose answers and two sibling artifacts cite. FR-239's open gap reads "no requirement states what the bound counts: attempts, retries after the first attempt, or attempts excluding repeated receipts". FR-049-AC-6 states "Retry bounds count the initial attempt", and FR-225 and FR-228 both quote that sentence. FR-239 neither cites FR-049 nor lists it under Dependencies. | FR-239; FR-049-AC-6; FR-225; FR-228 |
| FND-004 | high | FR-240 and FR-258 disagree about the four progress and closure axes. FR-240 open gap 3 states that FR-061 "never lists their member states" and that "No member list is invented here". FR-258 then gives Progress state as "Open or covered through a named point" and Closure state as "Open or closed", and places both in the FR-258 key. "covered through a named point" appears in no requirement; FR-094-AC-3 states only that the axes "remain independently represented". One artifact refuses to enumerate the axes and the other enumerates and keys on them. | FR-240; FR-258; FR-061; FR-094-AC-3 |
| FND-005 | medium | The `## Identity and comparison` block silently omits axes on three artifacts, and the batch's own acceptance criterion does not catch it. FR-200's identity discipline mandates six declarations. FR-201, FR-205 and FR-206 carry no `Causal order` bullet; FR-201 additionally carries no `Digest domain` bullet, although FR-201 is itself the digest-domain vocabulary. FR-200-AC-1 checks only "an identity kind, a key, an absence rule, and either a key order or an explicit statement" — four of the six. FR-203 and FR-204 show the intended form, writing "Causal order — none." explicitly. | FR-200-AC-1; FR-201; FR-205; FR-206; FR-203; FR-204 |
| FND-006 | medium | FR-200's bounded context contains exactly one artifact by declared relationship: a single `contains` edge to FR-201. FR-200-AC-1, FR-200-AC-2 and FR-200-AC-3 are each scoped to "every object artifact contained in this context", so all three are checkable over one artifact. The other 71 artifacts cite FR-200 only in Dependencies prose. | FR-200; FR-201 |
| FND-007 | medium | FR-222, FR-223 and FR-224 type their `channel` property as FR-216 (channel premise, a `value_object`) rather than FR-215 (protocol channel, the `entity`), in both the property bullet and the frontmatter `references` edge. FR-215 states that "The six premise components above are the value object FR-216" and `contains` it. Under FR-216's declared key, two distinct declared channels with identical premises would compare equal. | FR-222; FR-223; FR-224; FR-215; FR-216 |
| FND-008 | medium | The noun `attempt` is typed twice with incompatible identity and the two artifacts do not reference each other. FR-225 declares identity kind *derived* and key (`static declaration path`, `workflow instance identity`, `iteration identity`); FR-235 declares "Identity kind — not established" and "Key — not established". FR-210 records the unresolved question of whether a compensation attempt is an `attempt` or a member of `compensation`, so nothing tells a consumer which key applies to a compensation attempt. | FR-225; FR-235; FR-210 |
| FND-009 | medium | FR-205 declares two mutually inconsistent key orders under one heading: "the exact arithmetic order over the normalized value, and, for collection comparison, lexicographic over (`numerator`, `denominator`) in normalized form". They disagree — 1/3 and 1/2 normalize to (1,3) and (1,2), so arithmetic order puts 1/3 first and lexicographic order puts 1/2 first. FR-200 mandates "a total order over the key", singular. | FR-205; FR-200 |
| FND-010 | medium | FR-216 declares a componentwise key order over a component set that contains a collection: "sender, then receiver, then message types, then ordering policy, then delivery cardinality and duplication premise". FR-054 binds "message types" as a plural component, and neither FR-054 nor FR-216 nor FR-215 establishes an order within a message-type set, so the declared order is not total. FR-216-AC-4 asserts the order is used for collection comparison, which it cannot totally perform. | FR-216; FR-054; FR-216-AC-4 |
| FND-011 | medium | The `## Status` disclaimer is present on 40 of 73 artifacts and absent from 33. It is absent from every artifact in the result and protocol families and from all five diagnostics artifacts — the artifacts most likely to be read as adopted vocabularies by a consumer. FR-200 carries the disclaimer; FR-240, FR-249, FR-271 and FR-283 do not. | FR-240; FR-249; FR-271; FR-283; FR-200 |
| FND-012 | low | Five diagnostics artifacts derive substantially all of their content from a document that states of itself "Proposed, not adopted or implemented", including FR-271's 33-member code vocabulary and FR-272's closed cause-variant tables. FR-047 establishes the catalogued-classification obligation but names no catalog, no code and no byte digest. Each artifact flags the status, but FR-283 then holds its own codes "distinct from the snake_case native producer codes of FR-271", which makes a non-adopted vocabulary load-bearing for a requirement-grounded one. | FR-270; FR-271; FR-272; FR-273; FR-274; FR-047; FR-283 |
| FND-013 | low | Confirmed: the three reported UNTYPEABLE reasons are real as stated. FR-060 names its six claim kinds in prose across its Description and five acceptance criteria without a label spelling; FR-061 and FR-115 name participation with no member list and FR-062's only enumeration of it is the E02 target column; FR-061 names the four progress and closure axes without member states. Two of these three were later given their own artifacts under FND-001 and one was not, so FR-240's single "warrants its own artifact at a later identifier" sentence is now true of only the claim-kind vocabulary. | FR-060; FR-061; FR-115; FR-062; FR-240 |
