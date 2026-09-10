# Native finite graph definition

Profile identity: `quire.state.graph/v1`; revision: `1-draft.1`.
Language: `ix:native` / `1-draft`. **Proposed, not adopted or implemented.**

## Required definition

| Identity | Revision | Definition artifact | Exact byte digest |
| --- | --- | --- | --- |
| `quire.state.queries/v1` | `1-draft.1` | [Predicates and queries](state-queries.md) | `sha256:2c740d005c1b9dbcb5b18f77f558eb9403ecf713e0a74b922e51d39f51f24b32` |

## Explicit extension

Retain the complete core and query contracts. Add identity-bearing object and
reference values, explicit dereference through their typed finite supplied
universe, object/reference identity equality and positive-length `reaches`.
Identity includes universe, declared object type and object identifier; source
sequences retain repeated reference occurrences. Reachability checks the target
before suppressing repeated expansion and expands each identity at most once.
A self-loop can establish positive-length self-reachability.

Snapshot-qualified references retain the observation they read. An identity
comparison across pre/post does not permit a field read to switch observations.
Missing targets, foreign universes, unknown closure and resource exhaustion
cannot become false reachability or empty populations. The exact model and
population bindings supply this authority; no scalar-to-object cast, implicit
foreign-key lookup, recursive record expansion or ambient store is introduced.

This is the state profile providing the complete required v1 state/query/graph
scope. It does not supply temporal/protocol semantics, realize their requested
backend claims, or qualify the original minimum-only ConfigVersion model.
That original model still needs an explicit authored finite-domain disposition
and exact relationship mapping; its historical example is preserved.

## Selected graph rules

Core/query definitions and their selected state-contract snapshot remain exact
inherited dependencies. Additional graph rules are selected at repository
revision `033fcda3cc9f964d018b466faa12cd14c8a9878a`:

| Rule | Exact byte digest |
| --- | --- |
| [FR-043](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-043-evaluate-finite-graph-relations.md) | `sha256:9787a857499b629d8d691173cd3a1fbe5971297add0b92d76c711c6c956ab072` |
