# Current native Quire v1 draft

This prototype is one working specification. The current requirements, grammar
and contracts below are reviewed together at the PR revision. Git retains
working history; draft edits do not require per-file checksum refreshes or new
definition revisions.

| Part | Current source |
| --- | --- |
| Language grammar and parsing | [Shared grammar](../shared-grammar.md), FR-036–038 |
| Package, identity and binding | [Package contract](../package-contract.md), FR-030–035 and FR-130 |
| State, predicates, queries and graphs | [State contract](../state-contract.md), FR-039–046 |
| Temporal profiles and clocks | [FR-090–095](../../../spec/functional/FR-090-select-temporal-profile-and-clock.md), [temporal cases](../../../spec/temporal/tests.md) |
| Choreography and protocol results | [Native surface](../choreography-surface.md), [protocol contract](../protocol-contract.md), FR-049–062 |
| Observation binding, progress and ranges | [Observation contract](../observation-contract.md), [range correspondence](observation-range-1-draft-1.md), FR-110–116 |
| Producer model and ecosystem | [D's baseline at 6259d3a](https://github.com/agent-ix/filament-core-data/blob/6259d3a5b99088740df9bcc8e8d60f3720aaa603/docs/semantic-data-system/baseline-1-2.md), external FR-100–105 |
| Native diagnostic vocabulary | [Diagnostic definitions](native-diagnostics.md), FR-047 |

The earlier candidate files in this directory are retained for historical review
references. Their copied revision/digest tables are not the current draft's
dependency selectors or integration gates. Current corrections are in the
requirements and contracts above; no consumer should combine historical
candidate tables to infer the current draft.

At acceptance, freeze the coherent baseline and assign the immutable definition
identities needed by implementations. Runtime identity, source provenance,
digest-domain separation and refusal of mismatched inputs remain required by
FR-030/032/035. This simpler authoring workflow does not weaken those product
semantics or rewrite previously accepted profiles.
