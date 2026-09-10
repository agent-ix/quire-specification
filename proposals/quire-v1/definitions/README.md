# Composed definition candidates

These exact-byte documents make the edition and all three families' selections
reviewable. Revision `1-draft.1` is a **candidate**, not adoption, compiler
support or completed conformance. The selected rule snapshot is
`033fcda3cc9f964d018b466faa12cd14c8a9878a` for shared state rules, including
retained rational division; temporal/protocol rules select
`697ffd08b6937d4701beacbb0e0052f9e1ca4728`. No family meaning is inferred from
the existence of a definition file.

| Selection identity | Definition | Revision | SHA-256 of exact definition bytes | Scope |
| --- | --- | --- | --- | --- |
| `ix:native / 1-draft` | [edition.md](edition.md) | `1-draft.1` | `86622803e6e8f010a75d150ee5c4f6de55f41e27829ff1acb48876d11df3fbe6` | Grammar, package binding and common admission rules |
| `quire.state.core/v1` | [state-core.md](state-core.md) | `1-draft.1` | `cd1d930761833a554369845d0bf6112ba20fb8280cf0fbf9e7d9954a337fbbc8` | State clauses with the retained core value operations |
| `quire.state.queries/v1` | [state-queries.md](state-queries.md) | `1-draft.1` | `2c740d005c1b9dbcb5b18f77f558eb9403ecf713e0a74b922e51d39f51f24b32` | Core plus acyclic Boolean predicates and ordered queries |
| `quire.state.graph/v1` | [state-graph.md](state-graph.md) | `1-draft.1` | `1d5ae5e65b60d51fd2b0de653fb232f2ef23bcf977d0fb5ec81da49b75331c65` | Queries plus finite reference identity, navigation and reachability |
| `quire.temporal.bounded-facet/v1` | [temporal-common.md](temporal-common.md) | `1-draft.1` | `0f03afe04bb60062d7ec5bb6051ce5e7baf3d507ad14441b2352867c5fd06d61` | Required temporal facet; not a source root |
| `quire.temporal.event-position.false-extension/v1` | [temporal-event-position.md](temporal-event-position.md) | `1-draft.1` | `2d8f3f54fab0d36f105c71e32c6505538ae38d5d2625fbe5a106e6fc07194a8c` | Bounded future/past over semantic-event positions |
| `quire.temporal.fixed-sample.false-extension/v1` | [temporal-fixed-sample.md](temporal-fixed-sample.md) | `1-draft.1` | `681c488548d4992656c0cab6f4a275c381327a586e246f250a335c9c23f64ad8` | Bounded future/past over an exact sampled clock |
| `quire.temporal.timestamped-event.finite-window/v1` | [temporal-timestamped-window.md](temporal-timestamped-window.md) | `1-draft.1` | `7d08c19cfc574c076d24461b1aa48f4d8d4448c51d75e667d3292da5ba6ad388` | Bounded future/past over complete timestamped windows |
| `quire.protocol.finite-global/v1` | [protocol-finite.md](protocol-finite.md) | `1-draft.1` | `12a52691ece065a429dc0a64c2d0bfc34cb2c16e3a2bdc65e7b432baf499ceb5` | Finite global choreography and explicit claim boundaries |

Core → queries → graph describes explicit semantic extension. Required-definition
references run in the opposite direction, ending at the edition; the graph is
acyclic. Every dependency carries its own exact digest. The edition selects
specific grammar/package rule files; each profile selects the relevant semantic
rules and an explicit admitted-form catalog. A native `using` alias selects one
root, never a union of whatever happens to be installed.

For a package with full state, timestamped temporal and protocol scope, the
source can select these exact roots:

```text
language "ix:native" edition "1-draft";
profile S = "quire.state.graph/v1" version "1-draft.1"
  digest "sha256:1d5ae5e65b60d51fd2b0de653fb232f2ef23bcf977d0fb5ec81da49b75331c65";
profile T = "quire.temporal.timestamped-event.finite-window/v1" version "1-draft.1"
  digest "sha256:7d08c19cfc574c076d24461b1aa48f4d8d4448c51d75e667d3292da5ba6ad388";
profile P = "quire.protocol.finite-global/v1" version "1-draft.1"
  digest "sha256:12a52691ece065a429dc0a64c2d0bfc34cb2c16e3a2bdc65e7b432baf499ceb5";
```

This header fragment still requires a supplied matching edition definition,
the exact dependency files, explicit model imports and actual declarations.
It is not an admitted program or a claim that a compiler recognizes the new
selection. State-core and queries may be selected for their declared subsets;
neither alone satisfies a full-v1 capability claim.

The file digests bind normative bytes; repository revisions and links locate
the corresponding snapshots. They are not typed-expression canonical hashes or
source-package fingerprints. [checksums.sha256](checksums.sha256) records an
authoring byte census from the repository root, not an executable verifier.
A semantic or byte change after this candidate is shared requires a new
definition revision and new dependency digests; it cannot replace the old
selection in place. Historical bytes remain recoverable at their pinned revision.

Temporal identities retain FR-090's three meanings; the common temporal facet
alone cannot select one. Temporal/protocol roots consume the exact shared
value/predicate interface while retaining their own declaration permissions.
Source admission and backend/mapping support remain separate, especially for
past time and timestamped windows unsupported by current TL targets.

F's observation binding contribution at `c5cec84` is integrated, but exact
producer interface selections and D's standard-facing model/configuration
contribution remain outstanding. The protocol candidate explicitly records the
decisive-prefix/open-execution result disagreement with FR-061. It does not
declare that disagreement resolved or the whole profile consistent.

TC-031/032/040/049 specify positive and mutated-selection controls. Structural
Quire validation and this byte census do not qualify a profile implementation.
The selected composed baseline review and explicit adoption still precede
dependent implementation.
