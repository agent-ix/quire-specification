# Native bounded temporal common definition

Definition identity: `quire.temporal.bounded-facet/v1`; revision: `1-draft.3`.
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
all admitted continuations preserve it, with completed assessment execution,
a matching decisive settlement basis and complete exact decision support.
Otherwise its future remains pending with an unsettled basis. Missing support
makes that truth unavailable; a missing fact outside completed support remains
visible in completeness/adequacy without erasing the settled truth.
Decision-scope closure, surrounding-execution closure and input completeness
remain independent. Complete decision-scope closure permits the selected
boundary rule without closing the surrounding workflow.

Only a closed-complete trigger scope with no admitted trigger is inactive.
Open or missing/refused trigger evidence leaves activation unknown with its
distinct completeness/execution disposition. Neither becomes temporal truth.
Same-binding progress regression or a conflicting assertion revision produces a
typed contradiction/refusal without rolling back progress or earlier results.

Retain the clock, history, progress, source and population premises needed by
each result. A late contradiction identifies affected earlier results and their
premises without rewriting their bytes. Resource stops cannot produce Boolean
fallbacks. Replay and incremental consumers use the same selected semantics.

TL is an internal mapping target under FR-095, not the semantic authority for
these native profiles. Source admission of past or timestamped operators does
not claim that the current TL bridge supports them. Model/observation bindings
are exact linked/request inputs governed by the shared package contract.

## Selected temporal rules

The following rule table selects repository revision `19bbf7a2036168a5f14dddd946f696707dc45fde`.
Digests cover the entire exact file bytes; their locators do not authorize
substitution of later working-tree versions. External links select no implicit
runtime dependency or evidence store.

| Rule | Exact byte digest |
| --- | --- |
| [FR-048](https://github.com/agent-ix/quire-specification/blob/19bbf7a2036168a5f14dddd946f696707dc45fde/spec/functional/FR-048-bind-native-temporal-syntax.md) | `sha256:1cac8faaca860ac694102481f59b527ae2639f0ad94a1883f422ca0e75de7678` |
| [FR-090](https://github.com/agent-ix/quire-specification/blob/19bbf7a2036168a5f14dddd946f696707dc45fde/spec/functional/FR-090-select-temporal-profile-and-clock.md) | `sha256:0fb4f3f0a3f94eb52b636eb90886976e370dcbdcdbcfa1606eefbb326f21fa9c` |
| [FR-091](https://github.com/agent-ix/quire-specification/blob/19bbf7a2036168a5f14dddd946f696707dc45fde/spec/functional/FR-091-evaluate-bounded-future.md) | `sha256:e0909e4b5fa90747fc589a62265bdc2fcfd749e89481c5ec524e4905bf65bf57` |
| [FR-092](https://github.com/agent-ix/quire-specification/blob/19bbf7a2036168a5f14dddd946f696707dc45fde/spec/functional/FR-092-evaluate-bounded-past.md) | `sha256:f33190f80bc439bb883b8ae3b60994fda2e3684794751111e42693538ff3eb6c` |
| [FR-093](https://github.com/agent-ix/quire-specification/blob/19bbf7a2036168a5f14dddd946f696707dc45fde/spec/functional/FR-093-bind-temporal-activation-and-captures.md) | `sha256:62a44480ad357b0563e5a2ce251245eb924564bde59f00c989bf4822e0539ffe` |
| [FR-094](https://github.com/agent-ix/quire-specification/blob/19bbf7a2036168a5f14dddd946f696707dc45fde/spec/functional/FR-094-interpret-progress-history-and-closure.md) | `sha256:9edc2bfb74a8eadffb1c3713f035c4fcc474a79b1ed05da27295ee316f209297` |
| [FR-095](https://github.com/agent-ix/quire-specification/blob/19bbf7a2036168a5f14dddd946f696707dc45fde/spec/functional/FR-095-preserve-native-tl-correspondence.md) | `sha256:1fa1a8546f9e1419cee60adcd8cd4b1c1c444efe8bf225ce7befe7c06836d3fc` |
| [NFR-040](https://github.com/agent-ix/quire-specification/blob/19bbf7a2036168a5f14dddd946f696707dc45fde/spec/non-functional/NFR-040-bound-temporal-state.md) | `sha256:6044e889aceb244b81677bd003fb22267eeb16e41f996d931ff89016d90dd9c3` |

## Shared result interface and compatibility

The shared result interface selects
[FR-061 at B's reviewed revision](https://github.com/agent-ix/quire-specification/blob/2d8372c0ce22f43b313dba139880d00890f02984/spec/functional/FR-061-report-orthogonal-results.md),
exact file digest
`sha256:f56b11b7ee602a5d77ace8f3ac945661042d171f4935a82c1f7da184b6661c42`.
The selected interface is its assessment execution, independent scope/execution
progress and closure, truth, activation, settlement basis, exact decision
support and dependence lineage, completeness, and supersession rules, including
the corresponding result-combination rows. Protocol-specific claims, protocol
adequacy and canonical protocol-result serialization remain owned by the
protocol contract; this facet does not import protocol declaration permissions
or require an E02 projection to evaluate temporal source.

FR-095 requires the bridge correspondence record to retain the native/shared
result dimensions outside the TL formula when the selected TL wire does not
encode them. Equal TL truth or formula bytes do not supply missing metadata.

The earlier [1-draft.2 facet](temporal-common.md) and its exact rules remain
unchanged. This successor selects E's lifecycle corrections and B's shared
result interface; changing the selection changes the linked subject. The
interval algebra and three clock meanings are retained. Family reviews cover
B/E's contributions, not this composed definition or F's consumer mapping.
The baseline remains proposed pending integration and composed review.
