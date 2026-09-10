# Composed definition candidates

These exact-byte documents make A's edition and state-profile selections
reviewable. Revision `1-draft.1` is a **candidate**, not adoption, compiler
support or completed conformance. The selected rule snapshot is
`033fcda3cc9f964d018b466faa12cd14c8a9878a`, including the retained rational-division correction.

| Selection identity | Definition | Revision | SHA-256 of exact definition bytes | Scope |
| --- | --- | --- | --- | --- |
| `ix:native / 1-draft` | [edition.md](edition.md) | `1-draft.1` | `86622803e6e8f010a75d150ee5c4f6de55f41e27829ff1acb48876d11df3fbe6` | Grammar, package binding and common admission rules |
| `quire.state.core/v1` | [state-core.md](state-core.md) | `1-draft.1` | `cd1d930761833a554369845d0bf6112ba20fb8280cf0fbf9e7d9954a337fbbc8` | State clauses with the retained core value operations |
| `quire.state.queries/v1` | [state-queries.md](state-queries.md) | `1-draft.1` | `2c740d005c1b9dbcb5b18f77f558eb9403ecf713e0a74b922e51d39f51f24b32` | Core plus acyclic Boolean predicates and ordered queries |
| `quire.state.graph/v1` | [state-graph.md](state-graph.md) | `1-draft.1` | `1d5ae5e65b60d51fd2b0de653fb232f2ef23bcf977d0fb5ec81da49b75331c65` | Queries plus finite reference identity, navigation and reachability |

Core → queries → graph describes explicit semantic extension. Required-definition
references run in the opposite direction, ending at the edition; the graph is
acyclic. Every dependency carries its own exact digest. The edition selects
specific grammar/package rule files; each profile selects the relevant semantic
rules and an explicit admitted-form catalog. A native `using` alias selects one
root, never a union of whatever happens to be installed.

For the complete v1 state scope, a source unit can select:

```text
language "ix:native" edition "1-draft";
profile S = "quire.state.graph/v1" version "1-draft.1"
  digest "sha256:1d5ae5e65b60d51fd2b0de653fb232f2ef23bcf977d0fb5ec81da49b75331c65";
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

Temporal profile identities remain those in FR-090; their exact definition files,
clock/binding correspondence and B's protocol/result definitions are not supplied
by these state candidates. Those families must explicitly select the exported
shared value/predicate interface and preserve its scopes, rather than importing
state declaration permissions wholesale. D/F's exact producer interfaces and
the decisive-prefix/open-execution result agreement remain integration work.

TC-032 and TC-040 specify positive and mutated-selection controls. Structural
Quire validation and this byte census do not qualify a profile implementation.
The selected composed baseline review and explicit adoption still precede
dependent implementation.
