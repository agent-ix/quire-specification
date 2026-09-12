---
id: SR-504
title: "Object review of the domain object layer"
type: SpecReview
analysis: base
scope: "FR-200..FR-283, the 73 object artifacts under spec/objects/ and the 38 behavioral FRs they cite"
review_set: subset
evaluated_revision: "d6e19a5"
review_date: "2026-09-11"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-200, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-231, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-244, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-267, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-224, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-234, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-236, type: reviews }
---
## Summary

Seventy-three object artifacts type the nouns of the Quire assessment domain in
seven subsystems. FR-200 sets the discipline the other seventy-two answer to:
every object declares an identity kind, a key, a key order, a causal order, an
absence rule and a digest domain, and where the prose establishes none of these
the artifact records that fact instead of supplying one.

The central risk in a change of this shape is invented semantics. That risk was
tested directly. Seven hundred and ninety-eight quoted strings across the 73
artifacts were extracted and matched against the behavioral requirements, the
`proposals/quire-v1` definitions and `docs/`, normalizing whitespace, blockquote
markers, link syntax and elisions. Every quotation resolves to real source text
in context. Fifteen artifacts were then opened alongside their cited FRs and
read line by line: FR-200, FR-205, FR-224, FR-226, FR-231, FR-234, FR-236,
FR-242, FR-243, FR-244, FR-249, FR-251, FR-266, FR-267 and FR-281, against
FR-039, FR-051, FR-054, FR-055, FR-058, FR-061, FR-062, FR-093, FR-094, FR-112
and FR-113. No vocabulary member, no key component and no interval convention
was found that the prose does not state.

The restraint is real and unusually consistent. Thirty-five artifacts decline to
supply a key order at all, in the exact words "none is established". Fifty-five
carry an `## Open gap` section, and each one read here records a question and
refuses to answer it rather than closing it quietly. FR-224 is the clearest
case: delivery is required by three requirements to be countable and is given no
distinguishing component by any of them, and the artifact says so in four
numbered terms an owner would have to rule on.

The defects found are typing and duplication defects, not invention defects.
One noun is typed twice in two subsystems. One artifact is typed as an
enumeration when its members are co-present facts. One state machine introduces
a state name no requirement supplies, where its sibling state machine discloses
exactly that class of addition.

## Verdict

PASS with changes required. The layer adds no semantics the behavioral
requirements do not already carry, and its handling of unresolved questions is
correct and disciplined. FND-001, FND-002 and FND-003 need resolution before the
layer is treated as the single authority for its nouns, because each one breaks
the rebuild test for a specific noun. FND-004 through FND-007 are recorded for
the owner and do not block.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Activation is typed twice. FR-231 (obligation, `state_machine`) and FR-244 (result, `enumeration`) both extract the same three labels `triggered` / `inactive` / `activation-unknown` from FR-055 and FR-061, both declare identity kind *value*, both declare a lexicographic key order over the label, and neither artifact references the other in its frontmatter, body or dependencies. A consumer asking "what is an activation state" gets two answers in two subsystems and no statement of which is authoritative. The split is defensible on its merits, lifecycle against reported vocabulary, but it is nowhere stated. Fix: make each artifact cite the other with a `references` edge and one sentence saying which question it answers. | FR-231; FR-244; FR-055; FR-061 |
| FND-002 | medium | FR-267 is typed `enumeration` but is not one. Its four members, Event time, Ingestion time, Causal relation and Clock uncertainty, are co-present facts of a single observation, not alternative values of one slot: FR-113-AC-2 requires that all four "remain distinguishable", and FR-113 Description retains all four at once. They are also spelled as capitalized phrases rather than exact-match labels, unlike every other enumeration in the layer, which is itself evidence they are not a vocabulary. The declared key order compounds it: FR-267 gives "the order of the rows above", a field order, where every sibling enumeration gives an order over a label. Fix: retype as `value_object` with four components, or as a taxonomy, and restate the key accordingly. | FR-267; FR-113; FR-113-AC-2 |
| FND-003 | medium | FR-231 introduces the state `scope_linked` as the initial state of its `stateDiagram-v2`. The string appears in no requirement, no proposal and no other artifact in the repository, and it is absent from FR-231's own "State meanings, each taken from the prose" list, so it is undefined even locally. The four transitions leaving it are individually cited, so no transition semantics are invented; the pre-activation state itself is. The correct handling already exists in this change: FR-234 introduces four comparable scaffolding states and discloses them in its Open gap, "No requirement names the lifecycle states as a closed labelled vocabulary; the state names above are the prose's own conditions, not authored labels." FR-231 carries no equivalent sentence. Fix: add the FR-234 disclosure to FR-231, and either define `scope_linked` in the state-meanings list as a diagram-only origin or replace it with an unlabeled entry point. | FR-231; FR-234; FR-055; FR-093 |
| FND-004 | medium | The layer is internally inconsistent about supplying key orders, and FR-200 states both sides. FR-200 Bounded Context says an object artifact "records that the prose does not establish it, rather than supplying one", while FR-200-AC-1 permits "either a key order or an explicit statement that the prose establishes none". Thirty-eight artifacts take the second reading and supply a lexicographic order over the label or over the key tuple; thirty-five take the first and supply none. No requirement establishes lexicographic collation for any of the 38. The supplied orders are carefully disclaimed in every case ("for collection comparison only", "carries no causal meaning", and in FR-242 the explicit note that `pending` is not between `satisfied` and `violated`), so no semantic harm follows, but the two populations answer the same question differently for no stated reason. Fix: FR-200 should license a structural collation default explicitly and name it, or the 38 should record that the prose establishes none. | FR-200; FR-242; FR-244; FR-266; FR-236 |
| FND-005 | low | Four artifacts carry `object: entity` in frontmatter while their own Identity sections decline to establish identity. FR-224 says "Identity kind — **not established**" and "Key — **none is established**"; FR-235 and FR-237 say "not established by the prose"; FR-260 and FR-263 say "the prose does not place [it] in FR-200's authored / derived / value trichotomy". The bodies are correct and the frontmatter is a reasonable structural claim, but the two disagree for a machine reader: a tool consuming `object:` alone concludes these nouns have identity, which is the very thing the body refuses to assert. Fix: either add a catalog value for a noun whose identity kind is unresolved, or state in FR-200 that `object: entity` asserts only that the prose requires the noun to be told apart, not that a key exists. | FR-224; FR-235; FR-260; FR-263; FR-200 |
| FND-006 | low | FR-236 asserts a wire convention its source does not state. FR-058 Outputs names "failed, timed-out, duplicate, pending and successful outcomes" in an English sentence, and FR-058-AC-2 distinguishes three of them; the five members are therefore correctly extracted. FR-236 then adds that matching is exact and that "a hyphenated and an underscored spelling are different labels", and asserts it in FR-236-AC-2. No requirement establishes a spelling convention for these five. Contrast FR-242 and FR-244, whose labels FR-061 and FR-062 do state verbatim as table values, and FR-283, which explicitly records the kebab-case against snake_case question as an Open gap. FR-236 has no Open gap. Fix: add one recording that FR-058 fixes the members but not their spellings. | FR-236; FR-058; FR-283 |
| FND-007 | low | The identifier blocks are contiguous per subsystem except for `result`, which occupies both FR-240..FR-249 and FR-280..FR-283, with FR-275..FR-279 unallocated between them. The overflow is harmless today but no rule is recorded anywhere for where the next result object or the next diagnostics object is allocated, and a concurrent author reading the seven blocks would reasonably take FR-275 for diagnostics. Fix: record the allocation rule in FR-200 or in `spec/spec.md`. | FR-280; FR-281; FR-282; FR-283 |
| FND-008 | low | Verified, no defect: the invented-semantics audit is clean. 798 quoted strings across all 73 artifacts were machine-matched against `spec/functional`, `spec/integration`, `spec/test-cases`, `proposals/quire-v1` and `docs/`, and every one resolves to real source text; the residual non-matches were all regex artifacts of blockquote markers, table-cell boundaries and elisions, and each was resolved by hand. Fifteen artifacts were read against their cited FRs in full. No invented vocabulary member, key component, interval convention or ordering claim was found beyond FND-003. Disposition: no action. | FR-039; FR-051; FR-058; FR-061; FR-062; FR-112; FR-113 |
| FND-009 | low | Verified, no defect: no Open gap is a resolved question in disguise, and no artifact quietly resolves a contradiction. Fifty-five artifacts carry an Open gap; those inspected each terminate in a refusal to decide, and several name the decision an owner would have to make. The four recorded divergences read in full each list every spelling verbatim and reconcile none: FR-242 on `true`/`false` against `satisfied`/`violated` across FR-061, FR-091 and FR-092; FR-244 on FR-093's "activated" against FR-055's `triggered`; FR-245 on the adequacy word collision, which correctly cites the prior SR-208 FND-002 rather than re-deciding it; and FR-266 on the half-open observation window against the inclusive temporal deadline, which records the only bridging sentence as living in a proposal rather than a requirement. Disposition: no action. | FR-242; FR-244; FR-245; FR-266 |
| FND-010 | low | Verified, no defect: the rebuild test passes for the nouns tested, including the hard case. FR-224 answers "what is a delivery" from one artifact, answers "how are two compared" with the grounded negative that no requirement supplies a distinguishing component, and answers "what order are they in" with the grounded negative that the only ordering sentence in the prose, FR-054-AC-2's per-channel per-key FIFO premise, neither orders deliveries nor totalizes. That is the correct answer rather than a missing one, and the artifact names the four rulings needed to change it. FR-205, FR-242, FR-243, FR-266 and FR-281 each likewise answer all three questions from one file. Disposition: no action, and the FR-224 Open gap is the strongest single artifact in the change. | FR-224; FR-051; FR-054; FR-205; FR-266 |
