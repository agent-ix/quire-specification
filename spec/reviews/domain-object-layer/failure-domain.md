---
id: SR-502
title: "Failure-domain review of the domain object layer"
type: SpecReview
analysis: failure-domain
scope: "Identity confusion, absence handling, digest-domain crossing, order soundness and topological edge cases across spec/objects/** at d6e19a5, against FR-047, FR-049 to FR-062, FR-090 to FR-095 and FR-110 to FR-116"
review_set: subset
evaluated_revision: "d6e19a5"
review_date: "2026-09-11"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-220, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-224, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-225, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-226, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-228, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-235, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-240, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-248, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-250, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-255, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-256, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-258, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-259, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-266, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-267, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-268, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-270, type: reviews }
---
## Summary

Order soundness is the strongest part of this layer. Every artifact I read
separates a key order from the authored causal order, and most carry an
explicit denial that the key order is readable as causality, grounded in
FR-052-AC-2 ("Reordering equal-time events does not add a causal edge absent
from the protocol"), FR-113 ("Arbitrary timestamp order ... NOT establish
causality") and FR-090 ("not use timestamp order alone to establish causality
or break a tie for an order-sensitive operator"). I found no artifact whose key
order could be mistaken for a precedence or causal order without the artifact
itself saying it is not one. FR-252-AC-3, FR-253-AC-2, FR-255-AC-5, FR-256-AC-3
and FR-266-AC-3 each make that denial checkable.

Absence handling is likewise near-uniform: every key I examined states that an
absent component compares only to absence, and several go further by naming the
specific default that is forbidden (FR-201 "never defaulted to
`raw-artifact-digest`", FR-244 "never defaulted to `inactive`", FR-282 "never
defaulted to `independent`"). I found no key component that silently defaults.

The failures are in identity, and they cluster on exactly the four
communication nouns. I verified the three reported cases and each is real. I
extend all three: in two of them the artifact records that no tuple exists
while the tuple it does declare positively merges the things the prose requires
to be distinct, which is a worse condition than an acknowledged gap. Separately,
one artifact puts mutable state into a key, and the layer's own digest-domain
audit is incomplete by two artifacts.

## Verdict

CHANGES REQUESTED, with the identity findings escalated. FND-001 through
FND-004 each describe a requirement that cannot be satisfied by any
implementation reading only this layer plus the behavioral requirements.
FND-002 and FND-003 additionally record that the declared keys actively merge
subjects the prose separates, which the artifacts do not say. The order and
absence discipline needs no change.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Confirmed and extended: `delivery` has no identity kind, no key and no key order, while three requirements depend on one. FR-224 declares all three "not established". FR-051-AC-1 requires that one message "sent once, delivered twice" retains "cardinalities 1, 2 and 1 for the respective subjects", which requires two deliveries to be told apart; FR-059 requires reporting "duplicated" events; FR-054-AC-1 admits the duplicate. Every candidate component is excluded by name — trace identifier (FR-051-AC-4), payload (FR-051), timestamp (FR-113) — and the choreography surface removes the node-occurrence key by stating that duplicate transport receipts create no further node occurrence. Extension not recorded by FR-224: because no key exists, its `Digest domain` bullet is vacuous, so the FR-201 domain-before-bytes rule has nothing to attach to for the one subject that arrives entirely from transport. | FR-224; FR-051-AC-1; FR-051-AC-4; FR-054-AC-1; FR-059; FR-201 |
| FND-002 | high | Confirmed and extended: the `attempt` key cannot discriminate a retry from its original. FR-225 declares key (`static declaration path`, `workflow instance identity`, `iteration identity`); FR-058-AC-1 requires that "A failed refund attempt followed by one successful retry retains both attempts". All three components are constant across a retry unless the retry is authored inside a bounded iteration, which no requirement requires. Extension: FR-058-AC-1's example is a *refund* attempt, which is compensation, so it falls to FR-235, not FR-225 — and FR-235 declares its key "not established" and does not record the retry-discrimination problem at all. The artifact that owns the worked example is the one that does not analyse it, and the artifact that analyses it does not own the example. | FR-225; FR-235; FR-058-AC-1; FR-049-AC-6 |
| FND-003 | high | Confirmed and extended: `effect` has a key but no sameness rule. FR-226 records that FR-051-AC-2 ("Relabeling a duplicate delivery as a second business effect refuses the binding or yields the authored duplicate-effect violation") and FR-058-AC-2 ("A timed-out attempt, failed attempt and duplicate effect remain three distinct outcomes") give different dispositions. Extension not recorded: FR-226's declared key is the same occurrence triple as FR-225's, and that triple is constant across two effect claims at one static effect node in one workflow instance. Two duplicate effects therefore already compare *equal* under the key the artifact declares, which makes "duplicate business effect" undetectable rather than merely undefined. FR-226-AC-3 records that no tuple exists; nothing records that the declared tuple merges them. | FR-226; FR-051-AC-2; FR-058-AC-2; FR-225 |
| FND-004 | high | Mutable state is inside a key. FR-258's key is "(scope role, scope identity, progress state, closure state, input completeness)". Progress, closure and completeness are exactly the facts that change over the life of one scope, so the same decision scope at two progress points yields two unequal FR-258 values. FR-094 requires progress to be monotonic within its identified binding and FR-094-AC-8 detects a same-binding progress regression; both require recognising two assertions as concerning one scope, which this key prevents. FR-261 and FR-264 show the correct treatment, each excluding the varying part ("The admitted member set produced by applying the rule is not part of the rule's key"). | FR-258; FR-094; FR-094-AC-8; FR-261; FR-264 |
| FND-005 | high | The layer's digest-domain audit is incomplete by two artifacts. FR-201 requires the selected identity domain to be compared before the bytes. Six artifacts record the missing domain explicitly (FR-261, FR-262 gap 3, FR-263 gap 2, FR-264, FR-268 gap 1, FR-270). Two place a digest in the key and do neither: FR-250 keys on "the definition revision and digest that FR-090 Inputs require" and states only that the digest "retains the domain it was derived under", naming no member, and FR-250 carries no `## Open gap` section at all; FR-259 keys on the native source digest and the bridge-definition digest with the same formula, and its open gap covers a different question. | FR-250; FR-259; FR-201; FR-090; FR-095; FR-270 |
| FND-006 | medium | Several artifacts declare an exact-string key over a label spelling that the artifact itself records as non-existent. FR-217 states "No requirement fixes a spelling for either kind; the labels above ... are not established wire strings", then keys on the label and orders it lexicographically. FR-234's open gap states that "the state names above are the prose's own conditions, not authored labels". FR-267's open gap states that FR-113 "nowhere fixes a label spelling". FR-274's open gap states that the catalog gives the three location kinds "no discriminator tags or identifier spellings". The failure mode is silent: an unmatched label is indistinguishable from an absent one, and each artifact's Absence rule treats absence as a distinct state that receives no default. | FR-217; FR-234; FR-267; FR-274; FR-113 |
| FND-007 | medium | FR-267 supplies an order from its own layout. Its key order is "the order of the rows above, for collection comparison only", over a label whose spelling the same artifact says no requirement establishes. Every comparable artifact with no established order writes "none is established". FR-200 states the rule being broken: where the prose does not establish a key or an order, the object artifact records that it does not, "rather than supplying one". | FR-267; FR-200; FR-113-AC-2 |
| FND-008 | medium | The half-open versus inclusive boundary conflict is recorded, but not where it is acted on. FR-255 and FR-266 both record it correctly, quoting FR-112 (`[start, end)`), FR-113 (`start <= instant < end`), FR-091-AC-5 ("an eligible event exactly at the deadline participates before settlement") and FR-094-AC-1. Neither FR-256 nor FR-268 records it, yet those two are the artifacts that describe the settlement act itself: FR-256's key names an "interval" whose boundary is inclusive per FR-255, while FR-268 restricts progress and closure to "start <= instant < end". A witness exactly at the deadline settles under FR-256 and does not exist under FR-268, and neither artifact cross-references the other or the recorded conflict. | FR-256; FR-268; FR-255; FR-266; FR-091-AC-5; FR-094-AC-1; FR-113 |
| FND-009 | medium | A join completion rule is a key component drawn from a vocabulary that does not exist. FR-220's key is the linked protocol identity together with "its branches, its join rules and its iteration maxima", taken from FR-052-AC-4. FR-228's open gap 1 records that FR-052 admits more than one completion rule while FR-049-AC-3 names only the all-branch join, so FR-228 enumerates no completion-rule vocabulary. FR-220's own open gap records that its key list may be incomplete; it does not record that a component of the key it does declare is untyped, so two graphs cannot be compared for equality on that component. | FR-220; FR-228; FR-052-AC-4; FR-049-AC-3 |
| FND-010 | medium | Acyclicity of `derived` decision-support lineage is stated; acyclicity of the second lineage relation is not. FR-248 and FR-282 both carry FR-061-AC-7's refusal of "missing, self-referential or cyclic derived lineage". Neither states the scope: FR-248's key contains no enclosing result identity, so nothing requires a `derived` predecessor to lie inside one result. Separately FR-240 declares a second predecessor relation, "Causal order — supersession lineage only", grounded in FR-061-AC-9, with no acyclicity rule anywhere: no requirement and no artifact forbids two results from superseding each other. | FR-248; FR-282; FR-240; FR-061-AC-7; FR-061-AC-9 |
| FND-011 | low | Order soundness verified clean. I found no artifact whose declared key order could be read as, or silently used as, a causal or precedence order. Where a key order exists over something an operator could misuse — anchors (FR-253), history boundaries (FR-257), intervals (FR-255), window bounds (FR-266), clock bindings (FR-252) — the artifact states the denial and makes it checkable in an acceptance criterion. FR-252-AC-3 and FR-253-AC-2 specifically forbid citing the key order as an admitted order or as a tie-break for an order-sensitive operator, which is the exact prohibition of FR-090 and FR-090-AC-4. | FR-252-AC-3; FR-253-AC-2; FR-255-AC-5; FR-256-AC-3; FR-266-AC-3; FR-090-AC-4 |
| FND-012 | low | Absence handling verified clean. Every key component I examined states that absence compares only to absence, and no key component defaults. The strongest cases name the forbidden default outright: FR-201 ("never defaulted to `raw-artifact-digest`"), FR-244 ("never defaulted to `inactive`, which is a positive assertion"), FR-282 ("never defaulted to `independent`"), FR-239 ("never compared to zero, to an unbounded policy or to an implicit default"), FR-263 (a population with unknown membership is "neither an empty population nor a complete one"). | FR-201; FR-239; FR-244; FR-263; FR-282 |
