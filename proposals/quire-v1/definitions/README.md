# Composed definition candidates

These exact-byte documents make the edition and all three families' selections
reviewable. All listed revisions are **candidates**, not adoption, compiler
support or completed conformance. The selected rule snapshot is
`033fcda3cc9f964d018b466faa12cd14c8a9878a` for shared state rules, including
retained rational division; temporal/protocol rules select
`697ffd08b6937d4701beacbb0e0052f9e1ca4728`; protocol revisions `1-draft.2`/`1-draft.3` select
`15beef776f632310e77a18c838bd7f7a4a1088a1`. No family meaning is inferred from
the existence of a definition file.

| Selection identity | Definition | Revision | SHA-256 of exact definition bytes | Scope |
| --- | --- | --- | --- | --- |
| `ix:native / 1-draft` | [edition.md](edition.md) | `1-draft.1` | `86622803e6e8f010a75d150ee5c4f6de55f41e27829ff1acb48876d11df3fbe6` | Grammar, package binding and common admission rules |
| `quire.state.core/v1` | [state-core.md](state-core.md) | `1-draft.2` | `6480cc1459438b7fd28719acf99885bf854af7d201b57cd262ad7e0dc966242d` | State clauses with the retained core value operations |
| `quire.state.queries/v1` | [state-queries.md](state-queries.md) | `1-draft.2` | `3aa8efda949277a5df552c3dd4f659c44a91ebdfaf6dba68a383d793c8be9429` | Core plus acyclic Boolean predicates and ordered queries |
| `quire.state.graph/v1` | [state-graph.md](state-graph.md) | `1-draft.2` | `8eb33fa23455a8d6551d700c3bf23b2a690a9b7d2b30ea4a1b043af19ab5476b` | Queries plus finite reference identity, navigation and reachability |
| `quire.temporal.bounded-facet/v1` | [temporal-common.md](temporal-common.md) | `1-draft.2` | `0dc3b1bbbe7ab65392678d48ed5dc6a78babba98e20daeff2977148e10a1291b` | Required temporal facet; not a source root |
| `quire.temporal.event-position.false-extension/v1` | [temporal-event-position.md](temporal-event-position.md) | `1-draft.2` | `2bbbf401a452fd4defb7d8c22e2a50cf967350646b9ef97e941247c4c5363e88` | Bounded future/past over semantic-event positions |
| `quire.temporal.fixed-sample.false-extension/v1` | [temporal-fixed-sample.md](temporal-fixed-sample.md) | `1-draft.2` | `27733200394737d3550faa42a055e6724d648928b52ca9222e5ab42cf5a3bb74` | Bounded future/past over an exact sampled clock |
| `quire.temporal.timestamped-event.finite-window/v1` | [temporal-timestamped-window.md](temporal-timestamped-window.md) | `1-draft.2` | `d6331592934444aded53018910b7b6eea78026fc34435505e0086ee4dc271a0f` | Bounded future/past over complete timestamped windows |
| `quire.protocol.finite-global/v1` | [protocol-finite.md](protocol-finite.md) | `1-draft.1` | `12a52691ece065a429dc0a64c2d0bfc34cb2c16e3a2bdc65e7b432baf499ceb5` | Preserved earlier candidate; result-completeness disagreement recorded |
| `quire.protocol.finite-global/v1` | [protocol-finite-1-draft-2.md](protocol-finite-1-draft-2.md) | `1-draft.2` | `c1718f4e1f19f05ee297b074627127ef74088c0a32e48f3452f8258c4bab492f` | Preserved result correction with the earlier state dependency closure |
| `quire.protocol.finite-global/v1` | [protocol-finite-1-draft-3.md](protocol-finite-1-draft-3.md) | `1-draft.3` | `43dd2bc6810b867930d51b4aa2c64817b06207af221527433f91fa0c8b708ec7` | Decisive truth and independent closure, with corrected state form dependencies |
| `quire.package.composed/v1` | [package-reference.md](package-reference.md) | `1-draft.1` | `0df176dbb831117bd27443f7ef8075d9a130bcbc225022b440acbce111c62b9d` | Whole linked-package interpretation; not a source root |
| `quire.observation.binding/v1` | [observation-binding.md](observation-binding.md) | `1-draft.1` | `9fb77da233e792b7d96dda814fe71ec2e86707253ff85dce990f29b83c1bf3e1` | Candidate binding meaning and static/assessment-input separation; dependency closure pending |
| `quire.observation.progress/v1` | [observation-progress.md](observation-progress.md) | `1-draft.1` | `8c16aa039f6a537d7da8a8320b173241dc535bfb7f7dfa5cbf3b644559ebac4f` | Candidate scoped assertions and restoration basis; dependency closure pending |

The earlier [catalog and all definition bytes at `7832e9d`](https://github.com/agent-ix/quire-specification/tree/7832e9d4b4f56013d22d96303057bcc070eb480b/proposals/quire-v1/definitions)
remain available for exact historical selections. State/temporal revision
`1-draft.2` corrects the core catalog's unsupported indexing claim and refreshes
dependent digests; protocol `1-draft.3` consumes that closure. The grammar,
numeric rules, temporal truth tables and protocol settlement rules are unchanged.
Old selected dependencies must be supplied from their pinned snapshot, never
replaced with a current same-path file whose digest differs.

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
profile S = "quire.state.graph/v1" version "1-draft.2"
  digest "sha256:8eb33fa23455a8d6551d700c3bf23b2a690a9b7d2b30ea4a1b043af19ab5476b";
profile T = "quire.temporal.timestamped-event.finite-window/v1" version "1-draft.2"
  digest "sha256:d6331592934444aded53018910b7b6eea78026fc34435505e0086ee4dc271a0f";
profile P = "quire.protocol.finite-global/v1" version "1-draft.3"
  digest "sha256:43dd2bc6810b867930d51b4aa2c64817b06207af221527433f91fa0c8b708ec7";
```

This header fragment still requires a supplied matching edition definition,
the exact dependency files, explicit model imports and actual declarations.
It is not an admitted program or a claim that a compiler recognizes the new
selection. State-core and queries may be selected for their declared subsets;
neither alone satisfies a full-v1 capability claim.

A whole-package semantic reference selects `quire.package.composed/v1` and keeps
the exact inner clause selections. It cannot substitute S, T or P for the outer
container interpretation. Its explicit declaration/family feature set describes
package contents, not execution or full-v1 support. The package payload wire and
strict reader remain separately selected L2/FS05 contracts; no whole-package
canonicalizer is claimed by this candidate.

The file digests bind normative bytes; repository revisions and links locate
the corresponding snapshots. They are not typed-expression canonical hashes or
source-package fingerprints. [checksums.sha256](checksums.sha256) records an
authoring byte census from the repository root, not an executable verifier.
A semantic or byte change after this candidate is shared requires a new
definition revision and new dependency digests; it cannot replace the old
selection in place. Historical bytes remain recoverable at their pinned revision.
The census checks current working-tree bytes. The preserved protocol `1-draft.1`
definition continues to select its earlier rule snapshot; current FR-061 and
protocol-contract bytes belong to `1-draft.2` and cannot stand in for that snapshot.

Temporal identities retain FR-090's three meanings; the common temporal facet
alone cannot select one. Temporal/protocol roots consume the exact shared
value/predicate interface while retaining their own declaration permissions.
Source admission and backend/mapping support remain separate, especially for
past time and timestamped windows unsupported by current TL targets.

F's contribution through `32c95e9` is integrated. Its binding/progress candidate
bytes are indexed above, but their relative selected-contract links and external
dependencies still need exact immutable selections before they form a closed
definition set. Indexing their bytes alone does not supply those dependencies.
F also needs to preserve B's new settlement/closure/support dimensions in its
consumer contract. B's reviewed PR #19 resolves the earlier FR-061 versus E
decisive-prefix mismatch; protocol `1-draft.3` retains that correction. D's
standard-facing model/configuration contribution and composed review remain open.

TC-020/030/031/032/040/049 specify positive and mutated-selection controls. Structural
Quire validation and this byte census do not qualify a profile implementation.
The selected composed baseline review and explicit adoption still precede
dependent implementation.
