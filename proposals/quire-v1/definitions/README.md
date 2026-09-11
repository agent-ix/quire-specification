# Current native Quire v1 draft

This prototype is one working specification. Each unsuffixed file below contains
the latest corrected definition and links to its current requirements and
dependencies in this checkout. Superseded draft copies live in Git history.
No per-rule digest table selects a different working-tree snapshot.

The revision labels carry forward the latest candidates; these remain editable
drafts, not published immutable definitions. The PR revision identifies the whole
reviewed draft. Editing it does not require a checksum refresh or another
successor file.

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

At acceptance, freeze the coherent baseline and assign the immutable definition
identities needed by implementations. Runtime identity, source provenance,
digest-domain separation and refusal of mismatched inputs remain required by
FR-030/032/035. This simpler authoring workflow does not weaken those product
semantics or rewrite previously accepted profiles.
