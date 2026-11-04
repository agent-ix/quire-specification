---
id: FR-259
title: "Native-to-TL correspondence record"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-095
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-251
    type: references
  - target: ix://agent-ix/quire-specification/FR-258
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-026
    type: references
---
# FR-259: Native-to-TL correspondence record

## Description

When an admitted native temporal clause is lowered, the bridge SHALL emit a TL subject only for a correspondence whose admitted domain preserves the native operator, predicate, clock, capture, history and closure meaning.

FR-095 Outputs name the record and its derived status: "A generated TL
formula/valuation request plus a source/result-correspondence record, or a typed
unsupported/refused mapping naming every unmatched semantic dimension. The
generated TL artifact is internal derived output and never editable source."

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| native_source_identity_and_exact_clause_expression_span | SourceRegion | 1..1 |  |
| clause_requirement_identity | String | 1..1 |  |
| language_and_temporal_profile_definition | TemporalSemanticProfile | 1..1 |  |
| model_type_predicate_bindings | String | 1..1 |  |
| evaluation_anchor | TemporalEvaluationAnchor | 1..1 |  |
| immutable_capture_environment | ActivationCapture | 1..1 |  |
| clock_observation_binding | TemporalClockBinding | 1..1 |  |
| interval | BoundedTemporalInterval | 1..1 |  |
| closure_history_premises | String | 1..1 |  |
| tl_formula_profile_proposition_identities | TlTargetProfile | 1..1 |  |
| bridge_definition | String | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| native_source_identity_and_exact_clause_expression_span | multiplicity not stated in source; see Field sources |
| clause_requirement_identity | type not stated in source; multiplicity not stated in source; see Field sources |
| language_and_temporal_profile_definition | multiplicity not stated in source; see Field sources |
| model_type_predicate_bindings | type not stated in source; multiplicity not stated in source; see Field sources |
| evaluation_anchor | multiplicity not stated in source; see Field sources |
| immutable_capture_environment | multiplicity not stated in source; see Field sources |
| clock_observation_binding | multiplicity not stated in source; see Field sources |
| interval | multiplicity not stated in source; see Field sources |
| closure_history_premises | closure and history premises of FR-257 and FR-258; type not stated in source; see Field sources |
| tl_formula_profile_proposition_identities | multiplicity not stated in source; see Field sources |
| bridge_definition | type not stated in source; see Field sources |


FR-095 enumerates the preserved axes verbatim. For a supported mapping, the
bridge preserves:

| Property | Source | Note |
| --- | --- | --- |
| Native source identity and exact clause/expression span | FR-095 Behavior | Span identity per [FR-203](../foundation/FR-203-source-region.md). |
| Clause/requirement identity | FR-095 Behavior | |
| Language and temporal profile definition | FR-095 Behavior | [FR-250](FR-250-temporal-semantic-profile.md). |
| Model/type/predicate bindings | FR-095 Behavior | |
| Evaluation anchor | FR-095 Behavior | [FR-253](FR-253-temporal-evaluation-anchor.md). |
| Immutable capture environment | FR-095 Behavior | [FR-232](../protocol/FR-232-activation-capture.md). |
| Clock/observation binding | FR-095 Behavior | [FR-252](FR-252-temporal-clock-binding.md). |
| Interval | FR-095 Behavior | [FR-255](FR-255-bounded-temporal-interval.md). |
| Closure/history premises | FR-095 Behavior | [FR-257](FR-257-history-boundary.md), [FR-258](FR-258-temporal-decision-scope.md). |
| TL formula/profile/proposition identities | FR-095 Behavior | Target profile per [FR-251](FR-251-tl-target-profile.md). |
| Bridge definition | FR-095 Inputs, FR-095 Behavior | "an exact bridge-definition identity naming the TL syntax and evaluator profiles it targets". |

For every mapped evaluation result, FR-095 states that the record "shall also
preserve assessment-execution state; decision-scope progress/closure;
surrounding-execution progress/closure; truth; settlement basis; the exact
decision-support identities; completeness; and any superseded-result relation."
Those dimensions "remain native/shared-result metadata outside the TL formula
and valuation request when the selected TL profile does not encode them."

## Identity and comparison

- **Identity kind** — *derived*. The record has no authored identity; it is
  computed from the native subject, the bridge definition and the TL target it
  names, and it changes whenever any of those inputs changes.
- **Key** — every preserved axis in the tables above, compared componentwise and
  nothing else. Display name, source path beyond the retained span identity,
  message text, locale and rendering never participate
  ([FR-009](../../functional/foundation/FR-009-preserve-object-identity.md),
  [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)). FR-095-AC-4
  states the rule directly: "Mutating any native/TL profile, source/span,
  model/predicate, clock, capture, interval, closure/history or bridge identity
  breaks correspondence and names that axis." Equal formula bytes are not
  equality: FR-095 states that "Equal formula bytes or equal TL truth shall not
  manufacture, erase or merge them", and FR-095-AC-6 refuses "a changed period
  or event-count reinterpretation even where the TL formula bytes match".
- **Key order** — lexicographic over the key components, for collection
  comparison only. It carries no causal meaning, no lowering order and no
  precedence between correspondences.
- **Causal order** — none of its own. Any order the record carries is the
  admitted causal/sequence order of the preserved clock/observation binding
  (FR-252), never derived from key order, timestamp or ingestion order.
- **Absence** — an unmatched or unsupported axis is absent, and absence compares
  only to absence. FR-095 requires "a typed unsupported/refused mapping naming
  every unmatched semantic dimension", and FR-095-AC-7 holds that "mutating or
  omitting one axis cannot yield a healthy native result". An absent
  correspondence is never repaired by substitution: FR-095 states that "The
  temporal bridge shall not use a backend, syntax or historical result to repair
  an unknown mapping, substitute a profile or synthesize missing
  correspondence."

## Identifier namespaces stay separate

FR-095 states that the bridge "shall keep TL proposition identifiers distinct
from native predicate, signal and capture identities", that it "shall lower only
total native Boolean predicates to TL propositions", and that it "shall not
coerce numeric, nullable, unavailable or pending values". FR-095-AC-5 holds that
"A numeric, nullable, unavailable or pending native value cannot enter the TL
proposition population."

FR-095 also states that the bridge "shall not treat a formula match without its
valuation, capture and clock mapping as semantic correspondence."

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-259-AC-1 | Mutating any one preserved axis yields an unequal record and names that axis. | Test |
| FR-259-AC-2 | Two records with equal TL formula bytes and a different native profile, clock or interval compare unequal. | Test |
| FR-259-AC-3 | An unsupported request yields an absent correspondence rather than a substitute formula. | Test |
| FR-259-AC-4 | A TL proposition identifier is never compared equal to a native predicate, signal or capture identity. | Test |
| FR-259-AC-5 | Every result axis named by FR-095 is present in the record for a supported mapping. | Test |
| FR-259-AC-6 | Target identities validate through target owner readers, and the correspondence reference is distinct from every owner identity. | Test |

## Dependencies

- [FR-095](../../functional/temporal/FR-095-preserve-native-tl-correspondence.md)
  establishes the preserved axes, the support table and the substitution
  prohibitions.
- [FR-251](FR-251-tl-target-profile.md) supplies the TL target labels.
- [FR-258](FR-258-temporal-decision-scope.md) supplies the progress/closure axes
  the record preserves.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Artifact identity allocation

FR-095 now allocates generated identities to their selected TL owner readers
and the correspondence reference to the bridge. Two correspondence records may
name one equal generated formula only when every formula-owner input is equal;
they remain unequal if another preserved correspondence axis differs.

## Status

Proposed identity allocation for `quire-specification#40`.
