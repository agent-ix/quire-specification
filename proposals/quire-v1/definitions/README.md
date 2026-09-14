# Native Quire v1 definition set

FS01 accepts and reserves the identity hierarchy and dependency direction in this
set. Each unsuffixed file contains the latest corrected semantic text and
links to its current requirements and dependencies. Identity-role acceptance is
not semantic-definition adoption, compiler/backend implementation or
qualification. Those statuses remain explicit in each artifact and owning
FS02–FS06/family ticket. Superseded draft copies live in Git history. No per-rule
digest table selects a different working-tree snapshot.

FS03 accepts the exact `state-core → state-queries → state-graph` semantic
closure at the revisions below. The `1-draft` labels remain unpublished selection
identities, and acceptance does not imply complete implementation or qualification.
Other definition families remain editable candidates under their owning tickets.
The PR revision identifies the coherent reviewed checkout; editing a draft does
not require a checksum refresh or another successor file.

| Definition | Draft revision | Current file |
| --- | --- | --- |
| `ix:native` / edition `1-draft` | `1-draft.2` | [Edition and shared grammar](edition.md) |
| `quire.state.core/v1` | `1-draft.3` | [State core](state-core.md) |
| `quire.state.queries/v1` | `1-draft.3` | [Predicates and queries](state-queries.md) |
| `quire.state.graph/v1` | `1-draft.3` | [Finite graphs](state-graph.md) |
| `quire.temporal.bounded-facet/v1` | `1-draft.3` | [Common temporal meaning](temporal-common.md) |
| Event-position temporal root | `1-draft.3` | [Event positions](temporal-event-position.md) |
| Fixed-sample temporal root | `1-draft.3` | [Fixed samples](temporal-fixed-sample.md) |
| Timestamped-window temporal root | `1-draft.3` | [Timestamped windows](temporal-timestamped-window.md) |
| `quire.protocol.finite-global/v1` | `1-draft.4` | [Protocol and choreography](protocol-finite.md) |
| `quire.observation.binding/v1` | `1-draft.3` | [Observation binding](observation-binding.md) |
| `quire.observation.progress/v1` | `1-draft.3` | [Observation progress](observation-progress.md) |
| `quire.observation.range/v1` | `1-draft.1` | [All-clock range correspondence](observation-range.md) |
| `quire.package.composed/v1` | `1-draft.2` | [Linked-package interpretation](package-reference.md) |
| `quire.native.diagnostics/v1` | `1-draft.1` | [Diagnostic interpretation](native-diagnostics.md) |

The [package contract](../package-contract.md), [state contract](../state-contract.md),
[protocol contract](../protocol-contract.md) and [observation contract](../observation-contract.md)
are current semantic inputs to these definitions. External producer and compiler
inventory selections remain explicitly identified at their consuming boundary;
they are not silently replaced with whatever another repository currently has.

At each owning semantic acceptance, freeze the relevant coherent definition
closure and its immutable revision/digest. Runtime identity, source provenance,
digest-domain separation and refusal of mismatched inputs remain required by
FR-030/032/035. This authoring workflow does not weaken those product semantics
or rewrite previously accepted profiles.
