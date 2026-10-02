# Native predicate and sequence-query definition

Profile identity: `quire.state.queries/v1`. Language: `ix:native` / `1-draft`.
**FS03 accepts these semantics as the predicate/query extension over state core;
implementation and backend qualification remain independently reported.**

Required definitions and rule links resolve within this checkout. Runtime
references select each definition by its immutable identity.

## Required definition

| Identity | Definition artifact |
| --- | --- |
| `quire.state.core/v1` | [State core](state-core.md) |

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

State-core rules remain inherited dependencies. The following current rules
supply the explicit query and predicate extension.

| Rule |
| --- |
| [FR-033](../../../spec/functional/foundation/FR-033-admit-reusable-predicates.md) |
| [FR-034](../../../spec/functional/foundation/FR-034-bind-cross-family-predicates.md) |
| [FR-041](../../../spec/functional/foundation/FR-041-evaluate-ordered-queries.md) |
| [FR-042](../../../spec/functional/foundation/FR-042-select-pre-state-reads.md) |
