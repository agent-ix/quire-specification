# Native Quire v1 definition set

FS01 accepts and reserves the identity hierarchy and dependency direction in this
set. Each unsuffixed file contains the latest corrected semantic text and
links to its current requirements and dependencies. Identity-role acceptance is
not semantic-definition adoption, compiler/backend implementation or
qualification. Those statuses remain explicit in each artifact and owning
FS02–FS06/family ticket. Superseded draft copies live in Git history.
The complete-value [definition lock](complete-value-lock.json) separates the
qualification catalog (always-selected, conditional, alternative and rule
artifacts) from per-package selection: the Unicode text profile is selected only
by `text_bearing` packages, the IEEE profile only by `ieee_operation` packages,
and exactly one integer-division profile only by `integer_div_rem` packages.
Listing a profile in the catalog does not select it.

FS03 accepts the exact `state-core → state-queries → state-graph` semantic
closure at the revisions below. The `1-draft` labels remain unpublished selection
identities, and acceptance does not imply complete implementation or qualification.
Other definition families remain editable candidates under their owning tickets.

| Definition | Draft revision | Current file |
| --- | --- | --- |
| `ix:native` / edition `1-draft` | `1-draft.2` | [Edition and shared grammar](edition.md) |
| `quire.state.core/v1` | `1-draft.3` | [State core](state-core.md) |
| `quire.state.queries/v1` | `1-draft.3` | [Predicates and queries](state-queries.md) |
| `quire.state.graph/v1` | `1-draft.3` | [Finite graphs](state-graph.md) |
| `quire.temporal.bounded-facet/v1` | `1-draft.3` (accepted by FS04) | [Common temporal meaning](temporal-common.md) |
| Event-position temporal root | `1-draft.3` (accepted by FS04) | [Event positions](temporal-event-position.md) |
| Fixed-sample temporal root | `1-draft.3` (accepted by FS04) | [Fixed samples](temporal-fixed-sample.md) |
| Timestamped-window temporal root | `1-draft.3` (accepted by FS04) | [Timestamped windows](temporal-timestamped-window.md) |
| `quire.protocol.finite-global/v1` | `1-draft.4` | [Protocol and choreography](protocol-finite.md) |
| `quire.observation.binding/v1` | `1-draft.3` (accepted by FS04) | [Observation binding](observation-binding.md) |
| `quire.observation.progress/v1` | `1-draft.3` (accepted by FS04) | [Observation progress](observation-progress.md) |
| `quire.observation.range/v1` | `1-draft.1` (accepted by FS04) | [All-clock range correspondence](observation-range.md) |
| `quire.package.composed/v1` | `1-draft.2` (accepted by FS05) | [Linked-package interpretation](package-reference.md) |
| `quire.native.diagnostics/v1` | `1-draft.8` | [Diagnostic interpretation](native-diagnostics.md) |
| `quire.value.complete/v1` | `1-draft.2` | [Complete value root](value-complete.md) |
| `quire.value.text.unicode-17.0.0/v1` | `1-draft.1` | [Unicode text profiles](value-text-unicode-17.md) |
| `quire.value.integer-division.truncating/v1` | `1-draft.1` | [Truncating integer division](value-integer-division-truncating.md) |
| `quire.value.integer-division.floor/v1` | `1-draft.1` | [Floor integer division](value-integer-division-floor.md) |
| `quire.value.integer-division.euclidean/v1` | `1-draft.1` | [Euclidean integer division](value-integer-division-euclidean.md) |
| `quire.value.ieee754-2019-default/v1` | `1-draft.1` | [IEEE binary profile](value-ieee754-2019-default.md) |
| `quire.value.accounting/v1` | `1-draft.1` | [Scalar accounting schedule](value-accounting.md) |
| `quire.value.compound-unit/v1` | `1-draft.1` | [Evaluator-owned compound units](value-compound-unit.md) |
| `quire.simulation.sampler/v1` | `1-draft.1` | [Simulation sampler](simulation-sampler.md) |
| `quire.model.complete/v1` | `1-draft.2` | [Complete model root](model-complete.md) |
| `quire.model.complete.rules/v1` | `1-draft.2` | [Complete-model normalization rules](model-complete-rules.json) |
| `quire.model.effective-declaration.schema/v1` | `1-draft.2` | [Effective declaration schema](../../checked-package-v2/model-effective-declaration.schema.json) |
| `quire.model.effective-declaration.vectors/v1` | `1-draft.2` | [Effective declaration vectors](../../checked-package-v2/model-effective-declaration-vectors.json) |
| `quire.model.definition-lock/v1` | `1-draft.2` | [Complete model lock](complete-model-lock.json) |

## Header-selectable layers

Each layer of the AD-003 state and value/model hierarchy is a profile a
`profile` declaration selects by naming its identity. A declaration whose
`using` names that alias is admitted under exactly the layer's admitted-form
set, which is the set its definition file lists together with the sets of the
layers it requires. A form outside that set receives `unsupported_construct`
naming the selected layer. Selecting a layer selects the definitions it
requires; it never selects a layer that requires it. A user selects a parent
layer to restrict a unit to the subset that a lowering target or a
qualification tier accepts.

| Layer | Requires | Admitted-form set | User subset it serves |
| --- | --- | --- | --- |
| `quire.state.core/v1` | edition | [State core](state-core.md) | Embedded and runtime-monitor targets: bounded scalars and finite sequences with authored maxima, no named predicates, references or recursive values; the smallest tier to qualify. |
| `quire.state.queries/v1` | `quire.state.core/v1` | [Predicates and queries](state-queries.md) | Runtime monitors over bounded collections and logs: adds acyclic named predicates and ordered queries, still with no references. |
| `quire.state.graph/v1` | `quire.state.queries/v1` | [Finite graphs](state-graph.md) | Finite-model verification targets: adds explicit finite typed universes, references and reachability, with no conformance or dispatch. |
| `quire.value.complete/v1` | `quire.state.core/v1` | [Complete value root](value-complete.md) | Pure value contracts such as data validation and serialization rules: complete scalars, records, recursive values, collections and functions, with no object identity or navigation. |
| `quire.model.complete/v1` | `quire.state.graph/v1`, `quire.value.complete/v1` | [Complete model root](model-complete.md) | Full systems models: conformance, redefinition, dispatch and systems structure over the union of both parents. |

The five `requires` edges of this table are the profile-layering edges that
[FR-453](../../../spec/functional/tooling/FR-453-preserve-parent-profile-refusals-under-layering.md)
compares.

The [package contract](../package-contract.md), [state contract](../state-contract.md),
[protocol contract](../protocol-contract.md) and [observation contract](../observation-contract.md)
are current semantic inputs to these definitions.

FS03 accepts the ordered state closure, FS04 accepts the temporal/observation
closure, and FS05 accepts the linked-package
interpretation plus its selected strict reference/canonical/migration contracts.
Other listed family definitions retain their own acceptance tickets. Runtime identity, source provenance,
digest-domain separation and refusal of mismatched inputs remain required by
FR-030/032/035. This authoring workflow does not weaken those product semantics
or rewrite previously accepted profiles.
