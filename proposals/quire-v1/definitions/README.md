# Native Quire v1 definition set

FS01 accepts and reserves the identity hierarchy and dependency direction in
this set. Each file contains its semantic text and links to its requirements and
dependencies. Identity-role acceptance is not semantic-definition adoption,
compiler/backend implementation or qualification. Those statuses remain explicit
in each artifact and owning FS02–FS06/family ticket. The complete-value
[package selection](value-complete.md#package-selection) separates the
selectable artifacts (always-selected, conditional, alternative and rule
artifacts) from per-package selection: the Unicode text profile is selected only
by `text_bearing` packages, the IEEE profile only by `ieee_operation` packages,
and exactly one integer-division profile only by `integer_div_rem` packages.
Listing a profile does not select it.

FS03 accepts the exact `state-core → state-queries → state-graph` semantic
closure below, and acceptance does not imply complete implementation or qualification.
Other definition families remain editable candidates under their owning tickets.

| Definition | Current file |
| --- | --- |
| `ix:native` / edition `1-draft` | [Edition and shared grammar](edition.md) |
| `quire.state.core/v1` | [State core](state-core.md) |
| `quire.state.queries/v1` | [Predicates and queries](state-queries.md) |
| `quire.state.graph/v1` | [Finite graphs](state-graph.md) |
| `quire.temporal.bounded-facet/v1` | [Common temporal meaning](temporal-common.md) |
| Event-position temporal root | [Event positions](temporal-event-position.md) |
| Fixed-sample temporal root | [Fixed samples](temporal-fixed-sample.md) |
| Timestamped-window temporal root | [Timestamped windows](temporal-timestamped-window.md) |
| `quire.protocol.finite-global/v1` | [Protocol and choreography](protocol-finite.md) |
| `quire.observation.binding/v1` | [Observation binding](observation-binding.md) |
| `quire.observation.progress/v1` | [Observation progress](observation-progress.md) |
| `quire.observation.range/v1` | [All-clock range correspondence](observation-range.md) |
| `quire.package.composed/v1` | [Linked-package interpretation](package-reference.md) |
| `quire.native.diagnostics/v1` | [Diagnostic interpretation](native-diagnostics.md) |
| `quire.value.complete/v1` | [Complete value root](value-complete.md) |
| `quire.value.text.unicode-17.0.0/v1` | [Unicode text profiles](value-text-unicode-17.md) |
| `quire.value.integer-division.truncating/v1` | [Truncating integer division](value-integer-division-truncating.md) |
| `quire.value.integer-division.floor/v1` | [Floor integer division](value-integer-division-floor.md) |
| `quire.value.integer-division.euclidean/v1` | [Euclidean integer division](value-integer-division-euclidean.md) |
| `quire.value.ieee754-2019-default/v1` | [IEEE binary profile](value-ieee754-2019-default.md) |
| `quire.value.accounting/v1` | [Scalar accounting schedule](value-accounting.md) |
| `quire.value.compound-unit/v1` | [Evaluator-owned compound units](value-compound-unit.md) |
| `quire.simulation.sampler/v1` | [Simulation sampler](simulation-sampler.md) |
| `quire.model.complete/v1` | [Complete model root](model-complete.md) |
| `quire.model.complete.rules/v1` | [Complete-model normalization rules](model-complete-rules.json) |
| `quire.temporal.complete/v1` | [Complete temporal root](temporal-complete.md) |

## Header-selectable layers

Each AD-003 state, value, model and timed-model layer is selected by a
`profile` declaration naming its identity
([FR-001](../../../spec/functional/foundation/FR-001-select-native-profile.md)).
A user selects a layer to keep a unit inside the subset the backends serving
its witness request accept.

| Layer | Requires | Admitted forms | Admits | Witness request (FR-290 kind, FR-331 extent) |
| --- | --- | --- | --- | --- |
| `quire.state.core/v1` | none | [State core](state-core.md) | bounded scalars, explicit rationals, presence and control, and finite ordered sequences with authored maxima in invariants, preconditions and postconditions | `operation-contract`, `bounded` |
| `quire.state.queries/v1` | `quire.state.core/v1` | [Predicates and queries](state-queries.md) | state core plus typed acyclic named predicates and the ordered queries filter, map, count and sum | `operation-contract`, `bounded` |
| `quire.state.graph/v1` | `quire.state.queries/v1` | [Finite graphs](state-graph.md) | state queries plus explicit finite typed universes, references, object identity and reachability | `operation-contract`, `bounded` |
| `quire.value.complete/v1` | `quire.state.core/v1` | [Complete value root](value-complete.md) | state core plus complete scalars, records, tuples, unions, recursive values, collections and pure functions | `value-validity`, `bounded` |
| `quire.model.complete/v1` | `quire.state.graph/v1`, `quire.value.complete/v1` | [Complete model root](model-complete.md) | state graph and complete values plus conformance, redefinition, dispatch and systems-model structure | `operation-contract`, `bounded` |
| `quire.temporal.complete/v1` | `quire.model.complete/v1` | [Complete temporal root](temporal-complete.md) | complete model plus timed models: the time source, clock fields and clock places, resets, atomic clock constraints in guards, time invariants and urgency | `operation-contract`, `bounded` |

The package selection of [`quire.value.complete/v1`](value-complete.md#package-selection)
applies when a unit's selected layers include that layer; a unit selecting only
state layers selects the `edition` role only. The `Requires` column holds the
profile-layering edges that
[FR-453](../../../spec/functional/tooling/FR-453-preserve-parent-profile-refusals-under-layering.md)
compares.

The [package contract](../package-contract.md), [state contract](../state-contract.md),
[protocol contract](../protocol-contract.md) and [observation contract](../observation-contract.md)
are current semantic inputs to these definitions.

FS03 accepts the ordered state closure, FS04 accepts the temporal/observation
closure, and FS05 accepts the linked-package
interpretation plus its selected strict reference/canonical/migration contracts.
Other listed family definitions retain their own acceptance tickets. Runtime identity, source provenance and refusal of
mismatched inputs remain required by FR-030/032/035. This authoring workflow does not weaken those product semantics
or rewrite previously accepted profiles.
