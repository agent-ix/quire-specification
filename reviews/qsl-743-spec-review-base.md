---
id: SR-8712
title: "base review of PR #244 phase-2 qualify list"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@2989920af9570f8858b8dee479aa8b35729005cc; proposals/quire-v1/definitions/model-complete.md (phase-2 qualify row, new record/event/variant paragraph); spec/test-cases/TC-195-model-normalization-provenance.md (N14, N15, N16)"
review_set: subset
---

## Summary

Ticket: QSL-743. Base checklist review of the PR #244 diff, checked against the
built QSL normalizer (qsl-semantics `model/normalize.rs` and `model/index.rs`
on QSL `impl/business-model-meanings` @ 998761f) and QSL FR-056 "Variant types"
/ "Effective identity of a variant type". The variant rules (owner-less
qualify fact `[V]`, a case owned by its type and keyed by its own case node
key, no inherit fact, no object universe, reorder invariance) and the record
and event type-level rules (one owner-less qualify fact, then one inherit fact
per supertype path, no object universe) all hold in the code. Four defects:
the list still omits three kinds the normalizer qualifies, N16's fixture is
refused as written, N14 leaves the case node keys unstated, and N15/N16 do not
pin the inherited member's derivation, where code and spec disagree.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Phase-2 qualify row still omits state machines, processes and persistence interfaces (and operations owned by a persistence interface), which the normalizer qualifies (`normalizable_types`); the ticket outcome "names every model type the normalizer qualifies" is unmet, and with model-complete.md:207-211 the spec now reads as denying them effective declarations | model-complete.md:287; model-complete.md:207-211 |
| FND-002 | high | N16 declares event types `E` and `E2` with no `occurrenceField`; model-complete.md:163 and the QSL normalizer refuse such an event type `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.occurrenceField`, so the vector's expected admission is unreachable | TC-195:66; model-complete.md:163 |
| FND-003 | medium | N14 names cases `Gold` and `Silver` but not their IR node identities; under TC-195's "a node written X has identity ix://test/orders/X" convention they read as `ix://test/orders/Gold`, which FR-154 refuses as not the minted `variant/V1-Gold` slot | TC-195:64 |
| FND-004 | medium | N15/N16 state "(R2, R/f) one inherit fact with inputs [R, R/f]" without saying it is the only fact; the built normalizer gives an inherited member a qualify fact then the inherit fact, while QSpec (N01, vector `n01-member-B-x`) has the inherit fact only, so the vector cannot detect the divergence | TC-195:65; TC-195:66; TC-195:59 |

## Verdict

Not mergeable as is: FND-001 (ticket scope) and FND-002 (unsatisfiable vector)
are high. Clean units: the phase-2 row's variant-case clause matches FR-056 and
`record_member_owners` (a `Variant` record is owned by `case.owner`, keyed by
its own record key); the new paragraph's variant claims (no inherit fact: an
`Enum` record carries no generals; no object universe: `object_types()`
excludes variant, record and event types; reorder invariance: members sorted
by key) are true of the code; N14-N16 ids are free on every remote branch and
open PR (N11-N13 are taken by draft #209); TC-195 passes `quire validate`.

The divergence behind FND-004 predates this PR. QSL's phase-3 member pass
(`normalize.rs`, comment "An inherited effective member starts with its own
qualify fact") contradicts QSpec's published N01 vector; no QSL test checks
the `(B, A/x)` digest. That belongs to QSL, not QSpec.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | The new rule "its `supertypes` list is not read" holds only when the reader admits the list. A variant `supertypes` entry that names no type, names a type of another kind, or closes a cycle raises a semantic-ir reader diagnostic, and QSL then refuses `unsupported_construct`/`declaration-form` at `supertypes` (intake `business_generalization_refusal`, where a variant type is not a same-meaning-supertype kind). N14 step 3 names no entries, so its "the run equals the first" is unverifiable. The rationale also cites a QSL function, where the spec should state the rule | model-complete.md:303-306; TC-195:64 |
| FND-006 | high | N17's fixture is refused under FR-208 and the built code, so the admission it expects cannot happen: `S` declares no operation (FR-208 state-machine: one or more operations, and a transition's trigger names one), `S2` declares no state, transition or operation of its own (QSL checks each machine's own), `P` declares no `identityFields`, and `PI` has no `persists` list (FR-208-AC-7) | TC-195:67; FR-208 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 17ffad9 |
| FND-002 | fixed | 17ffad9 |
| FND-003 | fixed | 17ffad9 |
| FND-004 | fixed | 17ffad9 |
