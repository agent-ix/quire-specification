# Native predicate and sequence-query definition

Profile identity: `quire.state.queries/v1`; revision: `1-draft.1`.
Language: `ix:native` / `1-draft`. **Proposed, not adopted or implemented.**

## Required definition

| Identity | Revision | Definition artifact | Exact byte digest |
| --- | --- | --- | --- |
| `quire.state.core/v1` | `1-draft.1` | [State core](state-core.md) | `sha256:cd1d930761833a554369845d0bf6112ba20fb8280cf0fbf9e7d9954a337fbbc8` |

## Explicit extension

Retain all state-core forms and constraints. Add typed acyclic named Boolean
predicate declarations/calls, `filter`, `map`, `count` and `sum`.
Queries preserve occurrence order and duplicates; sum performs the explicit
same-representation/same-unit transfer into its named result domain, with exact
zero and every-prefix bounds. No implicit flattening, general casts or unit
conversion is introduced.

Predicate parameters and captures are explicit immutable typed inputs.
A reusable predicate has no ambient self, operation result or historical state.
An imported callee keeps its checked profile, type and anchor interpretation.
General value-returning functions, recursive calls, dynamic dispatch, overloads
and hidden effects remain refused. Reference/object identity, navigation and
reachability remain refused until the graph extension is selected.

Other families may consume this profile's checked Boolean predicate and shared
value-expression interface with explicit argument/anchor bindings. That interface
does not itself grant temporal/protocol declaration admission or turn a monitor
status into a Boolean. A caller selecting this profile cannot upgrade a callee
bound to state core.

## Selected extension rules

State-core rule files remain exact inherited dependencies. Additional rules
are selected at repository revision `033fcda3cc9f964d018b466faa12cd14c8a9878a`:

| Rule | Exact byte digest |
| --- | --- |
| [FR-033](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-033-admit-reusable-predicates.md) | `sha256:4293764b1235dfe11f9face4f0170a7adfeaaac6a1908396ae1d83340073029e` |
| [FR-034](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-034-bind-cross-family-predicates.md) | `sha256:949a26295572db24b313aa562b86f4709eed9224c87a192c77f33ceac05ff5f8` |
| [FR-041](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-041-evaluate-ordered-queries.md) | `sha256:d87858b46cbf650ba399f585fda38b3923e1734d014f7232f6831b355dcab799` |
| [FR-042](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-042-select-pre-state-reads.md) | `sha256:e76f521af441fe881e90032adafcf9150c85ebe38fc18d6cd231b073031b1c17` |
