---
id: AD-012
title: "Unbounded temporal operator grammar edition"
type: ArchitectureDescription
status: accepted
owner: quire-agent-a
system: native Quire unbounded (infinite-trace) temporal operator grammar
relationships:
  - target: ix://agent-ix/quire-specification/AD-003
    type: references
  - target: ix://agent-ix/quire-specification/AD-002
    type: references
  - target: ix://agent-ix/quire-specification/AD-009
    type: references
  - target: ix://agent-ix/quire-specification/FR-048
    type: realizes
  - target: ix://agent-ix/quire-specification/FR-090
    type: realizes
  - target: ix://agent-ix/quire-specification/FR-161
    type: realizes
---

# Unbounded temporal operator grammar edition

## System Boundary

AD-003 requires that new grammar be added "only under a new exact edition
definition" and never by reinterpreting a historical identifier or bytes. This
architecture is that edition definition for the interval-less temporal operator
forms ([FR-161](../functional/temporal/FR-161-analyze-infinite-trace-properties.md))
already named by the `quire.temporal.infinite-trace/v1` facet in AD-003's
Accepted profile hierarchy and by [FR-250](../objects/temporal/FR-250-temporal-semantic-profile.md).
It owns the grammar edition, not the infinite-trace soundness contract, which
AD-002 and AD-009 continue to own.

Before this edition, `always p`, `eventually p` and every other bare temporal
operator spelling with no bracketed interval was a syntax error under every
selected profile: the shared grammar's `temporal-unary`/`temporal-relation`
productions require an `interval` token between the operator and its operand.
This edition adds a sibling pair of productions,
`temporal-unary-unbounded`/`temporal-relation-unbounded`, that never carry an
`interval` token, admitted only under the existing `ix:native` / `1-draft`
edition and only when the clause selects `quire.temporal.infinite-trace/v1`.
It reuses the eight existing reserved keywords (`eventually`, `always`, `once`,
`historically`, `until`, `release`, `since`, `triggered`); no keyword is added,
removed or reassigned, and no `0-draft` or bracketed-`1-draft` byte sequence
changes meaning.

## Views

The following views define the admission path for the new grammar edition and
the reasoning that keeps it a sibling production rather than a widened
historical one.

### Admission path

| Stage | Owner | Required selection | Failure behavior |
| --- | --- | --- | --- |
| Source recognition | A | `1-draft` edition; the exact interval-less production named in [the shared grammar](../../proposals/quire-v1/shared-grammar.md#unbounded-temporal-operator-grammar-edition-infinite-trace) | Ungrammatical outside the new production: located syntax refusal |
| Profile admission | A | `quire.temporal.infinite-trace/v1` selected ([FR-090](../functional/temporal/FR-090-select-temporal-profile-and-clock.md), [FR-250](../objects/temporal/FR-250-temporal-semantic-profile.md)) | Any other selected profile: `unsupported_construct`, per FR-048-AC-9 |
| Frontend binding | A | [FR-048](../functional/foundation/FR-048-bind-native-temporal-syntax.md) typed leaf/scope/capture rules, unchanged | Same typed refusals FR-048 already defines |
| TL lowering | Contract IR | none; there is no TL target | Absent target, per [FR-251](../objects/temporal/FR-251-tl-target-profile.md) and [FR-095](../functional/temporal/FR-095-preserve-native-tl-correspondence.md) |
| Analysis | E | selected sound infinite-trace provider | `proved`/`refuted`/`inconclusive`/`unsupported`/`failed` per [FR-161](../functional/temporal/FR-161-analyze-infinite-trace-properties.md) and [FR-341](../objects/temporal/FR-341-infinite-trace-result-disposition.md); `unsupported` with a warning naming the missing liveness capability until a liveness backend registers |

### Why a new production and not a widened `interval`

The Complete-V1 facet grammar already widens `interval` to `[a, (uint | '*')]`
under the infinite-trace profile. That widened production still requires
brackets and a lower bound; it is bounded-interval syntax with an open upper
end, not the interval-less spelling this ticket requires. Reusing `interval`
for the fully unbounded form would require accepting a bracket-free spelling
inside a production whose non-terminal name and every other caller assume
brackets are present, which is exactly the "reinterpret old bytes" AD-003
forbids. A disjoint sibling production keeps the old `interval` non-terminal,
its callers and its refusal behavior for every other profile completely
unchanged.

## Decisions

| Decision | Alternatives rejected | Consequence |
| --- | --- | --- |
| Add `temporal-unary-unbounded`/`temporal-relation-unbounded` as new productions | Widen `interval` to accept an absent bracket pair | The bounded grammar, its refusals and its test evidence (TC-049, TC-110) are untouched |
| Gate admission on the selected profile, not on edition alone | Make the bare spelling valid under every `1-draft` package | A source file that never selects the infinite-trace profile keeps today's syntax refusal for `always p` |
| Reuse existing reserved keywords | Mint new interval-less keyword spellings (e.g. `always-forever`) | No lexical surface grows; the same eight operators keep one spelling across both grammar forms |
| No TL lowering target for this form | Approximate with a bounded TL window | FR-095/FR-251 keep the absence explicit instead of emitting a semantically wrong finite formula |

## Risks

| Risk | Control | Owner |
| --- | --- | --- |
| A bare temporal spelling silently parses under a profile that did not intend infinite-trace meaning | Profile-gated admission (FR-048-AC-9) plus TC-238 mutation coverage across all four v1 profiles | A |
| The new production is read as widening the historical bounded grammar | Explicit non-reinterpretation clause above and in the shared grammar; disjoint non-terminal names | A |
| Absence of a registered liveness backend is read as a hold or blocking error | FR-161's explicit `unsupported`-with-warning settlement (FR-161-AC-7); no hold state exists in the result vocabulary | E |

## Acceptance and handoff

Accepted as the grammar-edition record AD-003's Extension points table
requires for this addition. TC-200 and TC-238 are the evidence inventory; no
implementation milestone may claim unbounded temporal operator support before
those test cases pass under the real local Rust boundary.
