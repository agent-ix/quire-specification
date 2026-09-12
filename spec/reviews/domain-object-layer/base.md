---
id: SR-500
title: "Base review of the domain object layer"
type: SpecReview
analysis: base
scope: "Branch objects/domain-object-layer at d6e19a5: 73 object artifacts FR-200..FR-283, 38 behavioral FR frontmatter back-edges, and spec/spec.md registration"
review_set: subset
evaluated_revision: "d6e19a5"
review_date: "2026-09-11"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-200, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-231, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-244, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-267, type: reviews }
---
## Summary

The change adds a domain object layer: 73 artifacts under `spec/objects/` in
seven subsystems, each carrying `type: FR` plus an `object:` classification,
each extracting the identity, key, order and absence semantics of one noun from
the behavioral requirements that already state them. Thirty-eight behavioral FRs
gain `references` back-edges, and `spec/spec.md` registers every artifact with a
`contains` edge and a Requirements table row annotated with its object type.

The change claims to be structural: to add no semantics, to change no existing
requirement, and to report contradictions rather than resolve them. That claim
was tested rather than accepted. It holds.

All four hard constraints were verified against the diff. No existing FR
identifier changed: `git diff origin/main HEAD -- spec/functional` produces no
`id:` line in either direction. No consumer repository was touched, and no file
outside `spec/` was modified at all; the complete set of changed paths is
`spec/spec.md`, 38 files in `spec/functional` and the 73 new files under
`spec/objects`. The behavioral requirement diff is frontmatter-only and additive
in both directions: 141 added `type: references` lines with their matching
`target:` lines and not one deletion anywhere in `spec/functional`, so no
sentence of existing meaning was touched. `proposals/quire-v1/` is unmodified.

Existing meaning is preserved in substance and not only in the diff. 798 quoted
strings across the 73 artifacts were matched against the behavioral
requirements, the proposals and `docs/`, and all resolve to real source text.
Fifteen artifacts were read in full against their cited FRs. Contradictions are
reported rather than resolved throughout: 55 artifacts carry an `## Open gap`,
and the four cross-family divergences read here each list every conflicting
spelling verbatim and decide none of them. The layer is consistently willing to
answer a question with "the prose establishes none", which it does for the key
order of 35 objects and for the identity key of several more.

The defects found are confined to the object layer's own typing and are
detailed in [SR-504](../domain-object-layer-object.md). None of them alters the
meaning of a behavioral requirement.

## Verdict

PASS with changes required. The change does what it claims: it adds a structural
noun layer without editing a single line of existing requirement prose, and
without inventing a key, an order or a vocabulary member, with the one exception
recorded in FND-003. Three medium findings should be resolved before the layer
is cited as the single authority for its nouns: activation is typed twice in two
subsystems with no link between the two artifacts, FR-267 is typed as an
enumeration when its members are co-present facts, and FR-231 introduces an
undefended state name. None of the three requires a change to any behavioral
requirement, and none blocks the layer from landing ahead of them.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Activation is typed twice, in FR-231 (obligation, `state_machine`) and FR-244 (result, `enumeration`), over the same three FR-055 labels, with no reference between them in either direction. This is the one place the layer's own "say it once" goal fails. Detail and fix in SR-504 FND-001. | FR-231; FR-244; SR-504 |
| FND-002 | medium | FR-267 is typed `enumeration` but its four members are co-present facts of one observation, not alternatives of one slot; FR-113-AC-2 requires all four to "remain distinguishable" simultaneously. Detail and fix in SR-504 FND-002. | FR-267; FR-113; SR-504 |
| FND-003 | medium | FR-231's state diagram introduces `scope_linked`, a state name present in no requirement and undefined in FR-231's own state-meanings list. It is the only invented vocabulary member found in the change. The sibling state machine FR-234 discloses exactly this class of addition in its Open gap; FR-231 does not. Detail and fix in SR-504 FND-003. | FR-231; FR-234; SR-504 |
| FND-004 | low | The layer answers the key-order question two ways for no stated reason: 38 artifacts supply a lexicographic order that no requirement establishes, 35 record that none is established, and FR-200 states both positions in adjacent sections. The supplied orders are disclaimed and cause no semantic harm. Detail in SR-504 FND-004. | FR-200; SR-504 |
| FND-005 | low | Verified, no defect: all four hard constraints held. No FR identifier changed in either direction. No file outside `spec/` was modified and no consumer repository was touched. The `spec/functional` diff contains only added `target:` and `type: references` frontmatter lines, 141 of them, with zero deletions, so no existing requirement sentence was altered. `proposals/quire-v1/` is unmodified. Disposition: no action. | FR-051; FR-055; FR-058; FR-061; FR-112; FR-113 |
| FND-006 | low | Verified, no defect: the layer adds no semantics. Every one of 798 quoted strings resolves to real source text in the requirements, proposals or docs, and 15 artifacts were read in full against their cited FRs. Vocabularies checked verbatim against source include FR-236 against FR-058 Outputs, FR-243 and FR-281 against FR-061, FR-283 and FR-249 against FR-062, FR-251 against FR-095, and FR-266 against FR-112 and FR-113. Disposition: no action beyond FND-003. | FR-058; FR-061; FR-062; FR-095; FR-112 |
| FND-007 | low | Verified, no defect: contradictions are reported, not resolved. FR-242 records the `true`/`false` against `satisfied`/`violated` divergence across FR-061, FR-091 and FR-092 and asserts no identity between them. FR-244 records FR-093's "activated" against FR-055's `triggered`. FR-245 defers to the existing SR-208 FND-002 rather than re-deciding the adequacy word collision. FR-266 records the half-open observation window against the inclusive temporal deadline and notes that the only bridging sentence lives in a proposal, not a requirement. FR-224 names four rulings an owner would have to make and makes none. Disposition: no action; these are inputs to owner rulings, not review findings. | FR-224; FR-242; FR-244; FR-245; FR-266 |
