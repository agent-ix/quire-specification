---
id: FR-095
title: "Preserve exact native-to-TL temporal correspondence"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-008
    type: implements
  - target: ix://agent-ix/quire-specification/FR-090
    type: references
  - target: ix://agent-ix/quire-specification/FR-091
    type: references
  - target: ix://agent-ix/quire-specification/FR-092
    type: references
  - target: ix://agent-ix/quire-specification/FR-251
    type: references
  - target: ix://agent-ix/quire-specification/FR-259
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-350
    type: references
---
## Description

When lowering an admitted native temporal clause, the bridge shall emit a TL
subject only for a correspondence whose admitted domain preserves the
native operator, predicate, clock, capture, history and closure meaning.

## Inputs

An exact native temporal subject from QSL FR-045; total-Boolean predicate
projections that Contract IR FR-350 builds from the native `holds` leaves and
the trace's explicit valuation cells; selected observation and clock bindings;
and an exact bridge-definition identity naming the TL syntax and evaluator
profiles it targets.

## Outputs

A generated TL formula/valuation request plus a source/result-correspondence
record, or a typed unsupported/refused mapping naming every unmatched semantic
dimension. The generated TL artifact is internal derived output and never
editable source.

## Behavior

The correspondence shall use the support table that QSL FR-045 classifies:

| Native request | Existing TL target | Disposition |
| --- | --- | --- |
| `quire.temporal.infinite-trace/v1` | No target | Unsupported on the infinite-trace dimension; there is no bounded TL formula to generate, and QSpec FR-161 analyzes unbounded requests natively |
| Timestamped-event finite-window | Current TL profiles | Unsupported; no index conversion is inferred |
| Both a bounded past and bounded future operator reachable from the root | No target | Unsupported on the mixed-direction dimension; the past-operator profile admits no future or mixed graph |
| Either false-extension profile, at least one bounded past operator defined by FR-092 and no future operator, with origin-complete history | `tl-syntax.past-operators/v1`, `mltl.origin-complete-history/v1`, and the selected position-history/past-evaluator contracts | Supported when every bridge premise below holds; a history cutoff is `incomplete` with cause `missing-history`, not unsupported |
| Event-position false-extension, a bounded future operator and no past operator, complete surrounding execution | `mltl.closed-trace/v1` | Supported when every bridge premise below holds |
| Event-position false-extension, a bounded future operator and no past operator, open surrounding execution | `mltl.online-prefix/v1` | Supported when every bridge premise below holds |
| Fixed-sample false-extension, a bounded future operator and no past operator | The same TL profiles, selected by surrounding-execution closure | Supported only with one total valuation per required sample and retained epoch/period/unit correspondence |
| Either false-extension profile, no bounded temporal operator reachable | The same TL profiles, selected by surrounding-execution closure | Supported; a proposition-only formula imposes no additional TL temporal obligation |

The bounded-past operator set is exactly the set defined by QSpec FR-092; this
row covers every member without a future operator. The table's infinite-trace
row applies to the whole profile. The timestamped finite-window row also
precludes a finite-window-to-closed-trace equivalence claim, because TL extends
atoms false beyond closure. The no-operator row covers each false-extension
profile and is separate from the bounded-future and bounded-past rows.

For a supported mapping, the bridge shall preserve the native source
digest and exact clause/expression span; clause/requirement identity;
language and temporal profile definition; model/type/predicate bindings;
evaluation anchor; immutable capture environment; clock/observation binding;
interval; closure/history premises; TL formula/profile/proposition identities;
and bridge definition.

For every mapped evaluation result, the correspondence record shall also
preserve assessment-execution state; decision-scope progress/closure;
surrounding-execution progress/closure; truth; settlement basis; the exact
decision-support identities; completeness; and any superseded-result relation.
Those dimensions remain native/shared-result metadata outside the TL formula
and valuation request when the selected TL profile does not encode them. Equal
formula bytes or equal TL truth shall not manufacture, erase or merge them.

The bridge shall lower only total native Boolean predicates to TL propositions.

The bridge shall not coerce numeric, nullable, unavailable or pending values.

The bridge shall keep TL proposition identifiers distinct from native
predicate, signal and capture identities.

The bridge shall not treat a formula match without its valuation, capture and
clock mapping as semantic correspondence.

The native source and bridge-definition byte digests shall use the FR-201
`raw-artifact-digest` domain. Generated formula, history/trace, request and
report identities shall be recomputed by their selected TL public readers.
The correspondence shall be independently addressable by the bridge-owned
derived artifact reference over the complete preserved key; it shall not reuse
a native or TL artifact identity.

For an event-position or fixed-sample future mapping, the temporal bridge shall
preserve the exact TL lower-bound convention and false-extension rule in
[FR-091](FR-091-evaluate-bounded-future.md).

The temporal bridge shall not use a backend, syntax or historical result to
repair an unknown mapping, substitute a profile or synthesize missing
correspondence.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-095-AC-1 | Independent native and TL evaluators emit equal truth/pending outcomes for every member of the declared finite exhaustive bounded-future formula/trace population in the event-position mapping domain; a proposition-only formula under either false-extension profile selects the matching target from surrounding-execution closure. | Test (TC-117) |
| FR-095-AC-2 | The one-position `always[0,1] p` vector rejects a finite-window-to-`mltl.closed-trace/v1` equivalence claim. | Test (TC-117) |
| FR-095-AC-3 | Timestamped-event and mixed past-and-future requests return typed unsupported mappings without emitting a substitute formula; a past-only request with origin-complete history maps to `mltl.origin-complete-history/v1`, and with a history-cutoff boundary is `incomplete` with cause `missing-history`. | Test (TC-117) |
| FR-095-AC-4 | Mutating any native/TL profile, source/span, model/predicate, clock, capture, interval, closure/history or bridge identity breaks correspondence and names that axis. | Test (TC-117) |
| FR-095-AC-5 | A numeric, nullable, unavailable or pending native value cannot enter the TL proposition population. | Test (TC-117) |
| FR-095-AC-6 | Fixed-sample lowering retains epoch/period/unit, reports an absent runtime sample as incomplete, and refuses a changed period or event-count reinterpretation even where the TL formula bytes match. | Test (TC-113) |
| FR-095-AC-7 | A supported TL result is accepted only when the correspondence record preserves assessment execution, both progress/closure axes, settlement basis, exact decision support, completeness and supersession; mutating or omitting one axis cannot yield a healthy native result. | Test (TC-117) |
| FR-095-AC-8 | Source/bridge definition digests use `raw-artifact-digest`; every generated artifact identity is accepted by its target owner reader; the bridge-owned correspondence reference changes with every preserved-key mutation and never substitutes for an owner identity. | Test (TC-117) |
| FR-095-AC-9 | An infinite-trace unbounded-property request returns a typed unsupported mapping naming the absent TL target without emitting a substitute formula. | Test (TC-200) |

## Dependencies

- Contract IR FR-350 implements this correspondence: it constructs and
  validates the formula and valuation projection, correspondence record and
  result join. This requirement supplies the temporal meaning.
- QSL FR-045 supplies the `mapping_support_v2` classification and retained
  subject; QSL FR-050 supplies the strict `/2` package and QSL FR-043 supplies
  the native `evaluate_v2` report. QSL does not construct the TL correspondence.
- tl-syntax FR-011 (past-operator semantics; FR-011-AC-4 admits no future or
  mixed node under `tl-syntax.past-operators/v1`), FR-013 and FR-014 (the
  formula editions), tl-mltl FR-018 (trace and history wire) and quire-mltl
  FR-001 and FR-003 (request and result admission, result mapping).
- TL syntax/evaluation retains its own semantic-profile and release authority.

## Status

Accepted by FS04 as the exact native-to-TL correspondence contract. Contract IR
FR-350 is its implementation; QSL FR-045 owns classification and supplies the
bridge inputs.
