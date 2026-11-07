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
  - target: ix://agent-ix/quire-spec-language/FR-515
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-527
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-052
    type: references
---
## Description

When lowering an admitted native temporal clause, the bridge shall emit a TL
subject only for a correspondence whose admitted domain preserves the
native operator, predicate, clock, capture, history and closure meaning.

## Inputs

An admitted `quire.checked-package/v2` package, its QSL FR-515 checked temporal
subject view and the QSL FR-527 classification of that subject against FR-045's
support table; the native owner's admitted evaluation evidence and actual
FR-052 report; total-Boolean predicate projections that Contract IR FR-350
builds from the native `holds` leaves and explicit valuation cells; selected
observation and clock bindings; and an exact bridge-definition identity naming
the TL syntax and evaluator profiles it targets. Occurrence and instance
selections are independently bound inputs, not consequences of a shared clause
node, display name or equal formula.

## Outputs

A generated TL formula/valuation request plus a source/result-correspondence
record, or a typed unsupported/refused mapping naming every unmatched semantic
dimension. The generated TL artifact is internal derived output and never
editable source.

## Behavior

### Checked owner inputs and boundary

The bridge shall consume the actual checked package, subject view and
classification through the public owner APIs, preserving the opaque
admitted-package binding that QSL retains from the checked-package owner. It
shall first verify the subject belongs to the supplied borrowed package and
the classification retains that package's admission, then reclassify the
subject with the same surrounding-execution closure and compare the complete
result. Equal bytes, graph contents, package names, profile definitions or
clause nodes do not authorize a separately admitted package. An unchanged clone
may retain an admission; a subject view's original borrow and the
classification's retained admission are distinct checks.

The bridge shall select a declared clause occurrence by its original
node/role/ordinal key and authenticate its corresponding profile/clock,
valuation instance, observation and anchor under Contract IR FR-350. The
classification's sealed definition selection does not select an occurrence or
supply an evaluation result. A public disposition or retained field changed
without the owner selection is not an admitted correspondence.

QSL owns classification and native temporal evaluation. The bridge in
`quire-integration` owns projection, correspondence, strict projection/join
reading and cross-owner result comparison under Contract IR FR-350. Projection,
join and strict reading shall invoke neither evaluator and shall accept no
caller-authored mirror of an owner admission or result. TL owners retain
formula, trace/history, request and mapped-result admission and evaluation.

### Mapping domain

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
identity and exact clause/expression span; clause/requirement identity;
language and temporal profile definition; model/type/predicate bindings;
evaluation anchor; immutable capture environment; clock/observation binding;
interval; closure/history premises; TL formula/profile/proposition identities;
and bridge definition. Selected model package identities shall come from the
checked package owner's `selected_model_packages` in its admitted identity
order. The checked package carries no source model alias map; the bridge shall
not infer aliases from package names or node order, accept a caller-authored
alias table as authority, or substitute labels for structural identities. A
required fact absent from the owner contract remains unestablished.

For every mapped evaluation result, the correspondence record shall also
preserve assessment-execution state; decision-scope progress/closure;
surrounding-execution progress/closure; truth; settlement basis; the exact
decision-support identities; completeness; and any superseded-result relation.
Those dimensions remain native/shared-result metadata outside the TL formula
and valuation request when the selected TL profile does not encode them. Equal
formula bytes or equal TL truth shall not manufacture, erase or merge them.
A correction shall retain the complete immutable direct predecessor owner
values. Owner-defined non-evaluating validation shall establish that relation;
a matching result identifier, truth or label alone is insufficient. A missing,
foreign, skipped or different full predecessor value shall not establish
supersession even when its result identifier is equal.

The bridge shall lower only total native Boolean predicates to TL propositions.

The bridge shall not coerce numeric, nullable, unavailable or pending values.

The bridge shall keep TL proposition identifiers distinct from native
predicate, signal and capture identities.

The bridge shall not treat a formula match without its valuation, capture and
clock mapping as semantic correspondence.

Every generated formula, history/trace, request and mapped result shall pass
its selected owner's public reader. The bridge shall retain the exact admitted
owner values, their public names and versions where present, and selected
definition references. The bridge-owned validated correspondence shall expose
the complete preserved typed members; changing a member shall change that
member's exposed value. It shall compute no content hash, digest, synthetic
owner identity or derived record reference. A correspondence value is not a
native or TL owner identity, and label equality is not owner-value equality.

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
| FR-095-AC-8 | Every generated formula, trace/history, request and mapped result passes its target owner's public reader and is retained as the exact admitted typed value with its public name/version and selected definition references. Mutating a preserved typed key changes the corresponding exposed member and cannot pass strict rereading; no content hash or synthetic record reference substitutes for an owner value. | Test (TC-117) |
| FR-095-AC-9 | An infinite-trace unbounded-property request returns a typed unsupported mapping naming the absent TL target without emitting a substitute formula. | Test (TC-200) |
| FR-095-AC-10 | Independently admit identical producer bytes twice and obtain a subject/classification from the first admission: projection with the second admission refuses before TL construction. Original admission membership survives an unchanged package clone, but a subject borrowed from one package cannot authorize another borrow. Changing a public classification disposition, premise or retained fact while keeping its sealed selection refuses on complete reclassification. | Test (TC-117) |
| FR-095-AC-11 | Two occurrences sharing a temporal clause node retain distinct node/role/ordinal selections and their original name/clock rows; selected model package identities retain their checked-owner order and no source alias is inferred. Missing or substituted occurrence/instance/observation selections refuse before TL construction. For a correction, substituting a different complete prior owner result with the same result identifier fails non-evaluating predecessor validation and yields no final agreement Boolean. | Test (TC-117) |

## Dependencies

- Contract IR FR-350 implements this correspondence: it constructs and
  validates the formula and valuation projection, correspondence record and
  result join. This requirement supplies the temporal meaning.
- QSL FR-045 owns the support table; FR-515 supplies the checked temporal
  subject of the admitted checked-package/v2 and FR-527 supplies its
  authenticated classification and owner-issued package binding. FR-052/FR-043
  supply the actual native evaluation report and complete predecessor
  validation. QSL does not construct the TL correspondence. FR-050's legacy
  compiled-protocol `/2` classifier is not this checked-view bridge input.
- tl-syntax FR-011 (past-operator semantics; FR-011-AC-4 admits no future or
  mixed node under `tl-syntax.past-operators/v1`), FR-013 and FR-014 (the
  formula editions), tl-mltl FR-018 (trace and history wire) and quire-mltl
  FR-001 and FR-003 (request and result admission, result mapping).
- TL syntax/evaluation retains its own semantic-profile and release authority.

## Status

Accepted by FS04 as the exact native-to-TL correspondence contract. Contract IR
FR-350 is its implementation; QSL FR-045 owns classification and supplies the
bridge inputs.
