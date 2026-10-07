---
id: SR-1309
title: "Spec review of PR #196 (QSL-654): ProofBound carries its ADR-014 domain kind"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@de376ea8da8678238b60b23decdf471ec61aca5c; PR #196 diff against origin/main a73731e (commits 3e350df and de376ea): FR-290 bounded-registration domains paragraph and registration-cause list, FR-272 and native-diagnostics invalid_capability rows, FR-462 hello paragraph, FR-331 manifest domains paragraph and FR-331-AC-22, TC-431 BP-09, FR-331 Items bullet on ProofBound kind, closed-extent sentence, FR-331-AC-21, TC-431 BP-08, schema DomainKind/ProofBound/Extent, five adverse vectors; cross-checked against FR-290, FR-368, FR-361, QSL ADR-014 §4 and qsl-semantics/src/family/requirements.rs DomainKind::finite_kind (quire-spec-language origin/main)"
review_set: subset
---
# Spec review of PR #196 (QSL-654)

## Summary

Ticket: QSL-654. The PR gives every `ProofBound` a required `kind` from a new
shared `DomainKind` vocabulary, which the unbounded `domains[]` entry also
uses. It adds a kind/bound pairing rule, FR-331-AC-21, TC-431 BP-08 and five
adverse vectors. The second commit, de376ea, follows the plan ruling (QSL #661
FND-003): a descriptor that advertises a `bounded` mode must carry a nonempty
`domains`. Its absence is a new per-registration refusal,
`invalid_capability`/`absent-domains`, covered by FR-331-AC-22, TC-431 BP-09
and one adverse vector. This review covers the whole PR at de376ea.

Examined:
- FR-331 Items bullet "Every `ProofBound` … is `{domain, kind, bound}`" (examined)
- FR-331 closed-extent sentence (examined)
- FR-331-AC-21 (examined)
- TC-431 BP-08 (examined)
- schema `DomainKind`, `ProofBound` pairing `allOf`, `Extent` unbounded `domains[].kind` (examined)
- FR-290 bounded-registration `domains` paragraph and registration-cause list (examined)
- FR-272 and native-diagnostics `invalid_capability` rows (examined)
- FR-462 hello registration-refusal paragraph, TC-428 PW-09 (examined)
- FR-331 manifest `domains` paragraph, FR-331-AC-22, TC-431 BP-09 (examined)
- FR-290 extent classification and advertised-mode table (context_only)
- FR-368 temporal `subject.proof_bounds`, FR-361 substituted proof bounds (context_only)
- QSL ADR-014 §4 domain table, qsl-semantics `DomainKind::finite_kind` (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No owner states what negotiation does with a proof bound's `kind`. FR-331 says "The kind lets negotiation compare each bounded domain with a candidate's manifest `domains`". FR-290's new paragraph says the same, "which negotiation compares with a bounded item's proof-bound kinds". FR-290 still states no rule for that comparison: no step in the candidate procedure, and no disposition or cause for a bounded item whose proof-bound kind the single candidate does not list in `domains`. Its advertised-mode table still settles every `bounded` item by "the negotiation arm's own disposition for the IR form". So two FRs promise a comparison that neither defines, and CG's PV-4 routing would have to invent the outcome. Add the rule to FR-290's negotiation section: when it runs, for example at the single candidate's arm; the disposition and cause for an unlisted kind, for example `unsupported` with `unsupported_projection`/`unsupported-requested-capability` naming the kind; an AC; and a TC-271 row. Also reword FR-331's "routes on its mode alone": under FR-290 a bounded extent settles by the arm's disposition, not by mode. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:167-171, spec/objects/protocol/FR-290-protocol-claim-kind.md:148-154 |
| FND-002 | low | FR-331-AC-21's last clause, "a bounded extent with empty `bounds` carries no kind", cannot fail. An empty list has no entries that could carry a kind. Drop the clause, or replace it with a check that can fail. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:345 |
| FND-003 | low | A bad `domains` is refused two different ways. An absent `domains` is a registration refusal, `absent-domains`. A `domains` that is present but empty, holds a non-boundable kind such as `quantity`, or repeats a kind is refused by the manifest-stage schema. FR-462's hello paragraph does not list those defects as FR-290 registration causes, so its `malformed-body` row turns them into a `tool-failure` that settles items `failed`. Either widen the FR-290 cause to cover an empty or invalid `domains` too, for example by having `absent-domains` cover "no boundable domain kind", or say explicitly that an invalid `domains` is a wire failure. Then FR-462 and FR-290 agree. | spec/objects/protocol/FR-290-protocol-claim-kind.md:148-154, spec/functional/tooling/FR-462-speak-the-plugin-wire.md:94-100 |
| FND-004 | low | FR-462-AC-9 now covers the `absent-domains` registration refusal, through FR-462's hello paragraph, but its test row TC-428 PW-09 still exercises only `unknown-kind`, `unknown-mode` and `unknown-evidence-kind`. Add a fourth plugin to PW-09 that advertises a `bounded` mode with no `domains`, and expect `absent-domains`. | spec/test-cases/TC-428-plugin-wire.md:30 |

## Verdict

The wire change itself is correct. The pairing rule matches
`DomainKind::finite_kind` exactly: collection or population gets
`Cardinality`, integer gets `IntegerRange`, recursive gets `Depth`, and loop,
infinite-trace and quantity get none. I checked all 21 kind/bound
combinations against `#/$defs/ProofBound` with Python jsonschema, and each is
admitted exactly when `finite_kind` pairs it.

The seven spellings name ADR-014 §4's seven unbounded domains one to one.
ADR-014 gives these domains no wire spellings of its own. FR-331 has owned
these spellings since #194, so this PR only moves them into one shared
definition. Carrying `kind` on a temporal `subject.proof_bounds` entry is
consistent with FR-368 and FR-361, which say only "any substituted proof
bounds" and leave the shape to FR-331's `ProofBound`.

Each of the five adverse vectors is refused at the patched `ProofBound`
member. `bounded-descriptor-without-domains` is refused at `/manifest/0`,
which is `required`.

A new cause was needed. FR-290's registration causes before this PR were
`absent-kind`, `unknown-kind`, `unknown-mode`, `unknown-evidence-kind` and
`duplicate-backend`, and none of them is about a missing `domains`. Reusing
`unknown-evidence-kind`, which already stands for an absent value, would
mislabel the defect. `absent-domains` is added consistently: FR-290 defines
it and lists it with the registration causes, FR-272 and native-diagnostics
both add it before `not-statistical` and retain the backend identity for a
registration cause, and the FR-462 hello paragraph includes it. The catalog's
stale "nine causes" count was dropped, which is correct: the count was ceremony
and was already wrong.

Not mergeable as is. Fix FND-001 in this PR by adding the comparison rule
to FR-290, which now promises it. FND-002 to FND-004 are small and belong in
the same round. Once those are fixed it is mergeable. The QSpec suite was not
rerun at de376ea: the coordinator asked me not to while the build lock is
busy, and the author reports it green.
