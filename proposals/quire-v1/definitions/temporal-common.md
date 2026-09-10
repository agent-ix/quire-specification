# Native bounded temporal common definition

Definition identity: `quire.temporal.bounded-facet/v1`; revision: `1-draft.2`.
Language: `ix:native` / `1-draft`. **Proposed, not adopted or implemented.**
This is a required semantic facet, not a selectable source profile by itself:
a concrete temporal root must select exactly one FR-090 clock/boundary meaning.

## Required shared interface

| Identity | Revision | Definition artifact | Exact byte digest | Consumed interface |
| --- | --- | --- | --- | --- |
| `quire.state.graph/v1` | `1-draft.2` | [State/query/graph](state-graph.md) | `sha256:8eb33fa23455a8d6551d700c3bf23b2a690a9b7d2b30ea4a1b043af19ab5476b` | Checked shared values, finite queries/graphs and explicit Boolean-predicate references |

The concrete temporal roots admit temporal declarations. They reuse the shared
expression checker for `holds(expr)`, activation guards and capture initializers;
they do not import state declaration permissions. Authored reusable predicates
select their own state profile. Arguments retain that callee's checked meaning.
A temporal expression environment supplies its declared current input and
immutable captures, with the trigger available only in its activation scope.
There is no ambient self, operation result, `pre(expr)` or monitor-to-Boolean
conversion. Explicitly supplied pre/post values remain ordinary anchored data.

## Temporal forms and obligations

Admit temporal constants, grouping, temporal Boolean connectives and the eight
bounded operators: eventually, always, until, release, once, historically, since
and triggered. Intervals are inclusive with integer 0 ≤ a ≤ b; unbounded forms
are refused. Strong previous is expressed as `once[1,1]`; no separate weak
previous spelling or boundary is added.

`holds(expr)` is an atomic temporal valuation. A source constant `true` remains
distinct from `holds(true)`; false-extension behavior cannot be optimized away.
Until/since require the left operand only between the interval's lower bound
and the witness; release/triggered use the selected Boolean duals.

Each activation has its own exact semantic trigger/origin identity and captures.
A duplicated receipt cannot create another trigger. Missing required current
valuations, captures or history remain incomplete/refused; they are not false
atoms or empty populations. A valid open prefix settles a Boolean only when
all admitted continuations preserve it. Otherwise its future remains pending.
Complete closure, open execution and a closed incomplete file remain distinct.

Retain the clock, history, progress, source and population premises needed by
each result. A late contradiction identifies affected earlier results and their
premises without rewriting their bytes. Resource stops cannot produce Boolean
fallbacks. Replay and incremental consumers use the same selected semantics.

TL is an internal mapping target under FR-095, not the semantic authority for
these native profiles. Source admission of past or timestamped operators does
not claim that the current TL bridge supports them. Model/observation bindings
are exact linked/request inputs governed by the shared package contract.

## Selected temporal rules

All rule files below are selected at repository revision `697ffd08b6937d4701beacbb0e0052f9e1ca4728`.
Digests cover the entire exact file bytes; their locators do not authorize
substitution of later working-tree versions. External links select no implicit
runtime dependency or evidence store.

| Rule | Exact byte digest |
| --- | --- |
| [FR-048](https://github.com/agent-ix/quire-specification/blob/697ffd08b6937d4701beacbb0e0052f9e1ca4728/spec/functional/FR-048-bind-native-temporal-syntax.md) | `sha256:1cac8faaca860ac694102481f59b527ae2639f0ad94a1883f422ca0e75de7678` |
| [FR-090](https://github.com/agent-ix/quire-specification/blob/697ffd08b6937d4701beacbb0e0052f9e1ca4728/spec/functional/FR-090-select-temporal-profile-and-clock.md) | `sha256:dc74377395a7d6bfc6189ab3c403b566c1279764112cd841f1dd1b71e0121331` |
| [FR-091](https://github.com/agent-ix/quire-specification/blob/697ffd08b6937d4701beacbb0e0052f9e1ca4728/spec/functional/FR-091-evaluate-bounded-future.md) | `sha256:92ed046dd295249f983cb7a6bdc807c53763e224a9e72c601ba1c937fcdbf791` |
| [FR-092](https://github.com/agent-ix/quire-specification/blob/697ffd08b6937d4701beacbb0e0052f9e1ca4728/spec/functional/FR-092-evaluate-bounded-past.md) | `sha256:f33190f80bc439bb883b8ae3b60994fda2e3684794751111e42693538ff3eb6c` |
| [FR-093](https://github.com/agent-ix/quire-specification/blob/697ffd08b6937d4701beacbb0e0052f9e1ca4728/spec/functional/FR-093-bind-temporal-activation-and-captures.md) | `sha256:71300d32a9ec636cd729e967cb505ead2d38ae46ca8a0d5e0eef7547a5a90dc2` |
| [FR-094](https://github.com/agent-ix/quire-specification/blob/697ffd08b6937d4701beacbb0e0052f9e1ca4728/spec/functional/FR-094-interpret-progress-history-and-closure.md) | `sha256:b9469550f77332d26df235b2f988285ab01cadb3ff6947a2adf8931bd81c92b9` |
| [FR-095](https://github.com/agent-ix/quire-specification/blob/697ffd08b6937d4701beacbb0e0052f9e1ca4728/spec/functional/FR-095-preserve-native-tl-correspondence.md) | `sha256:da30fd6f70810b41b671a2eb00658c5c5c15f99939151d21c995234bd2a2b73b` |
| [NFR-040](https://github.com/agent-ix/quire-specification/blob/697ffd08b6937d4701beacbb0e0052f9e1ca4728/spec/non-functional/NFR-040-bound-temporal-state.md) | `sha256:6044e889aceb244b81677bd003fb22267eeb16e41f996d931ff89016d90dd9c3` |

The portable mapping must retain decisive temporal truth together with an open
execution. B's current FR-061 completeness terminology still needs that composed
agreement; this temporal facet does not amend B's result schema or claim the
complete baseline is ready for adoption.
