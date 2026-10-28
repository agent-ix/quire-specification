---
id: FR-368
title: "Request and negotiate temporal items over a model subject"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-290
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-331
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-161
    type: references
  - target: ix://agent-ix/quire-specification/FR-361
    type: references
  - target: ix://agent-ix/quire-specification/FR-362
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
  - target: ix://agent-ix/quire-specification/FR-366
    type: references
---

# FR-368: Request and negotiate temporal items over a model subject

## Description

When a request carries a `temporal-satisfaction` item, the producer SHALL
carry the item's subject, fairness set and item kind in the
[FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) request
and its identity, and `negotiate_*` SHALL route the item only to a candidate
that decides the item's mode and every fairness kind in its fairness set.

## Inputs

A request item of kind `temporal-satisfaction`; the registry snapshot's
candidates and their manifests ([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)).

## Outputs

The item's FR-331 request members and identity, and one FR-331 disposition
and terminal record per item.

## Behavior

### Request members

A `temporal-satisfaction` item over a model subject carries, in the FR-331
request:

- `subject` — `initial_states`, the ordered list of the subject's initial-state
  snapshots as byte-provision references; `universes`, the universe of each
  population the subject reads; and `proof_bounds`, any substituted proof
  bounds ([FR-361](FR-361-read-model-behaviours-as-temporal-traces.md));
- `item_kind` — one of `formula`, `deadlock-freedom`, `time-lock-freedom`,
  `race-freedom` and `refinement` ("Item kinds" below); no other spelling is a
  kind, and one refuses before consumption;
- `abstract_subject` — present exactly on a `refinement` item: `package_id`, the
  id of the abstract subject's package (a package of the request's
  `dependencies`), and `initial_states`, the ordered list of the abstract
  subject's initial-state snapshots as byte-provision references, spelled as
  `subject.initial_states` is ([FR-377](../analysis/FR-377-decide-model-refinement.md)
  "Inputs"). Its universes are fixed by FR-377 and it has no proof bounds;
- `fairness` — the resolved fairness set
  ([FR-362](FR-362-apply-fairness-constraints.md)), empty for a
  `deadlock-freedom`, `time-lock-freedom` or `race-freedom` item;
- `max_depth` — the bounded-search horizon `k`, a method parameter of a
  bounded search that the result states
  ([FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md)); absent for
  an exhaustive search;
- `limits` — the caller-configurable resource limits only: `max_states`,
  `max_transitions`, `max_automaton_states` and `max_clocks` (the most live
  clocks a state may hold, [FR-415](FR-415-declare-clocks-and-timed-models.md);
  setting `model_check.max_clocks`, default 1024), the SMT solver budgets
  `max_solver_steps` and `max_solver_time` ("SMT solver budgets" below),
  plus any resource budget another temporal requirement adds, each
  defaulting to the routed provider's published default when the request
  omits it. `max_depth` is never a member of `limits`;
- `smt_encoding` — the SMT proof-query encoder's size limits ("SMT encoding
  limits" below), present only when the caller sets one.

The request identity and the item's obligation identity include `subject`,
`abstract_subject`, `item_kind` and `fairness`. Two items that differ in any of them are different
obligations, and a verdict on one never settles the other.

The request identity also includes `max_depth`, `limits`, except the
wall-clock member `max_solver_time`, and `smt_encoding`. A different
horizon or budget can produce a different result, so a result is reused only
for a request with the same request identity, and a result produced under one
`max_depth` or `limits` value is never reused for a request with another.

### Item kinds

The `item_kind` of a `temporal-satisfaction` item over a model subject is
exactly one of these five spellings, each owned by the requirement that
defines the item:

| `item_kind` | Item | Owner | Subject | Anchor (`claim.node`) | `fairness` |
| --- | --- | --- | --- | --- | --- |
| `formula` | an authored temporal clause | [FR-370](../../objects/interfaces/FR-370-checked-package-temporal-clause-body.md) | a model or protocol subject | the clause's own claim node | the clause's resolved set |
| `deadlock-freedom` | derived | [FR-366](FR-366-check-deadlock-freedom.md) for a model subject, [FR-429](../protocol/FR-429-report-protocol-deadlocks.md) for a protocol subject | that subject | the subject's anchor | empty |
| `time-lock-freedom` | derived | [FR-417](FR-417-settle-time-lock-freedom.md) | a timed model subject | the subject's anchor | empty |
| `race-freedom` | derived | [FR-438](../memory/FR-438-report-data-races.md) | a protocol subject with a weak `parallel` | the subject's anchor | empty |
| `refinement` | requested, one per refinement declaration checked | [FR-377](../analysis/FR-377-decide-model-refinement.md) | the concrete subject | the refinement node | the concrete fairness set `F_C` (the `assume` rows, [FR-375](../analysis/FR-375-declare-model-refinement.md) "Fairness rows"); `F_A` is read from the refinement node |

The three derived kinds have no authored clause, so no `fair` constraint
resolves for them ([FR-362](FR-362-apply-fairness-constraints.md)
"Syntax and resolution"): their fairness set is the empty set, which admits
every behaviour. A `deadlock-freedom` item over a protocol subject and one
over a model subject are the same kind; the subject tells them apart. Each
kind is part of the obligation identity (above), so the five never share a
verdict.

A transfer item ([FR-380](../analysis/FR-380-transfer-claims-through-refinement.md))
has no `item_kind`: it is settled by the verification planner from two
premises, "no backend is negotiated for it, and no exploration of the concrete
subject runs", so it is not a request item that reaches a provider and no
sixth spelling exists. Its two premises are request items of kind `formula`
and `refinement`.

### Claim and anchor of a derived or refinement item

A derived item has no authored claim and no source construct that is its own,
but the request still names a claim for every item
([FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
"Items"), and so does the `refinement` item. The claim of an item whose kind is
not `formula` is `{node, occurrence}` as follows, with nothing left to the
reader's choice:

- **Anchor node.** For a `refinement` item, the refinement node (FR-375). For a
  derived item over a protocol subject, the subject's checked protocol clause
  node ([FR-425](../protocol/FR-425-define-protocol-subjects-and-state-keys.md)
  "Protocol subject"). For a derived item over any other model subject, the
  `model` node of the model alias the subject's items name, the node
  [FR-476](../../objects/interfaces/FR-476-checked-package-model-terminal-declaration-body.md)
  `model` member and FR-402's `subjects` binding reference. The
  deadlock-freedom and time-lock-freedom items of one model subject therefore
  share a claim node, and the `timed_model` root ([FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)
  "Timed-model nodes"), owned by that model, is not a second anchor.
- **Occurrence.** The anchor node's occurrence of role `declaration` and
  ordinal 0 when it has one, otherwise its occurrence of role `expression` and
  ordinal 0 (a model declaration node carries no `declaration` member and its
  one occurrence has role `expression`, FR-322 "Occurrence roles"). The
  anchor node of every kind above has one of the two, so the pair always
  names a `source_map` entry. The choice depends on the node alone, not on the
  order of the node's `occurrences` array.

The pair is the `locus` of every `where` and payload that names the item
(FR-331), and the package source map gives its regions, so a derived item's
result is located at the model, protocol or refinement declaration that it
reports on. The claim enters the request identity with the item's other
members (FR-331 "Request identity").

### Extent and domains of a derived or refinement item

A derived item is the reachable-state invariant `always holds(I)` of
[FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md) "Property forms",
which that table puts under `quire.temporal.infinite-trace/v1`; a
`refinement` item reads its behaviours as FR-377 "Model subjects and
behaviours" does, maximal paths with a terminal stutter step, which is the
infinite-trace reading. So each classifies `unbounded` by "Extent" above,
with no opt-out. Its `extent` is `{classification: "unbounded", domains,
finite_bound_available: false}` and nothing else:

- `domains` holds the `infinite-trace` domain `{type: "node", node: <the
  claim's node>, path: []}` with kind `infinite-trace`, the same form an
  authored item carries for its own claim node, followed by the population
  domains that the item's subject reads without a maximum, as the boundedness
  architecture lists them for an authored item over the same subject
  (quire-spec-language#222, QSL ADR-014 §4). The infinite-trace domain is
  never boundable, so `finite_bound_available` is false.
- The item has no `extent.bounds` and no result bound of its own: the
  substitutions for the subject's populations are the subject's
  `proof_bounds`, which belong to the subject and are not moved into the
  extent (FR-331 "Items"). A derived item carries the same `subject` as the
  items it is derived from, `proof_bounds` included.

### Position of derived items in the request

`request_index` is the item's zero-based position (FR-331 "Items"), and
FR-360 joins a result to its item by it, so the position is part of the wire.
The request writer SHALL order the items by structure alone, never by a
digest, a hash or a map's iteration order:

1. The items the caller requested, authored and `refinement`, first, in the
   caller's order. Adding or removing a derived item, as a `terminal any`
   declaration does, never moves one of them.
2. Then the derived items, grouped by subject in the order in which each
   subject first occurs among the subjects of the items in step 1, and,
   within one subject, in the order `deadlock-freedom`, `time-lock-freedom`,
   `race-freedom`. A model subject and a protocol subject over the same model
   are two subjects. The time-lock-freedom item thus stands beside the
   deadlock-freedom item of its subject ([FR-417](FR-417-settle-time-lock-freedom.md)
   "The derived item").

Each subject has at most one item of each derived kind, so the order is total.
A subject with no item in step 1 has no derived item.

### Negotiation of a derived or refinement item

Each item of the five kinds is one item of kind `temporal-satisfaction`
([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)). It carries
neither `hybrid` nor `requirements`, since its dynamics are `discrete`
(FR-331 `manifest`: a descriptor with no `dynamics` decides `discrete` only),
and it routes through FR-290's candidate, domain-kind and advertised-mode
tables exactly as an authored `unbounded` item does. A `time-lock-freedom`
item over a dense subject is still a `discrete` item: clocks are the model's
(FR-415), not hybrid dynamics. Its fairness check ("Fairness kinds") never
trips for an empty set. The item reads its `limits` (`max_clocks` included
for a timed subject), `max_depth` and `smt_encoding` as any temporal item
does; an absent member takes the routed provider's published default, and
a derived or `refinement` item has no value of its own for them. The caller's
limit settings ([FR-461](../foundation/FR-461-name-the-limit-value-and-setting.md),
for example `model_check.max_states`) and `max_depth` are written once for the
request, so every temporal item carries the same `limits`, `max_depth` and
`smt_encoding`, the derived and `refinement` items included, and a member the
caller did not set takes the provider's published default. FR-331 has no
request-level `limits` member, so none is inherited from an authored item.

### SMT solver budgets

A temporal item settled by an SMT backend (k-induction or SMT unrolling)
carries the budgets of its solver run in `limits`, so
[FR-461](../foundation/FR-461-name-the-limit-value-and-setting.md)'s rules
for naming, raising and defaulting a limit apply to both. Each bounds every
query of the item: a base, step, unrolling or search query.

- `max_solver_steps` SHALL be a whole, positive count of solver steps as
  the solver counts its own deterministic resource steps. A report of it
  SHALL state the count as the solver counts it and SHALL NOT convert it to
  time. It is deterministic, so it is part of the request identity and the
  cache key ([FR-306](../tooling/FR-306-compile-and-cache-exactly.md)). Its
  setting is `smt.max_solver_steps`.
- `max_solver_time` SHALL be a wall-clock budget for each query, in whole
  seconds, independent of the backend's own wall-clock limit and of the
  provider deadline `deadline_ms`. The request writer rounds a fractional
  duration up to the next whole second, and a fractional or otherwise
  non-canonical value on the wire is refused before consumption. It is a
  wall-clock setting: the request identity and the cache key exclude it, no
  member of a stored record records wall time, and a result it stops is not
  cached or stored (FR-306). Its setting is `smt.max_solver_time`.
- A solver stopped by either settles FR-331 result `incomplete`,
  `limit-reached{limit, value, setting}` naming that member, its value and
  its setting, in the stop order of
  [FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md) "Undecided
  k-induction runs": a tripped cancellation handle and the backend's own
  wall clock or a `deadline_ms` overrun come first, and only the solver's
  own undecided answer comes after. Unlike a hybrid item's
  `hybrid.limits.max_solver_time`, whose exhaustion is `timed-out`
  ([FR-193](../analysis/FR-193-analyze-continuous-hybrid-models.md)), a
  temporal item's `max_solver_time` is a limit the request names on each
  SMT query, so its exhaustion is `limit-reached`.
- When the request omits a member, the provider SHALL apply its descriptor
  `bounds` (FR-331, FR-461). QSL's 100000000 steps and 60 seconds are the
  defaults of its own SMT settings (QSL FR-255); this requirement states no
  number. A provider that runs no solver reads neither member.
- A record that a request reuses states the limit values of the run that
  produced it. Two requests that differ only in `max_solver_time` share a
  request identity, so a completed result reused for the second states the
  first run's `max_solver_time`, never a value the second request set; a
  reader compares the stated value with its own setting only to know which
  budget bounded the producing run. A result that `max_solver_time` stopped
  is never stored, so no reused record carries that stop.

### SMT encoding limits

An SMT backend that encodes its proof queries with QSL's encoder (QSL
FR-315) bounds each query's size by the item's `smt_encoding`, a closed
object whose members are `PositiveCount`s, each a limit under
[FR-461](../foundation/FR-461-name-the-limit-value-and-setting.md):

| Member | Bounds | Setting |
| --- | --- | --- |
| `max_constants` | the `declare-const` commands of one query | `smt_encoding.max_constants` |
| `max_applications` | the applications of one step | `smt_encoding.max_applications` |
| `max_monitor_atoms` | the members of a monitor letter | `smt_encoding.max_monitor_atoms` |
| `max_monitor_states` | the monitor states a tabulation reaches | `smt_encoding.max_monitor_states` |
| `max_candidates` | the candidates of one expanded application | `smt_encoding.max_candidates` |
| `max_script_bytes` | the bytes of one printed query | `smt_encoding.max_script_bytes` |

- An absent `smt_encoding`, and an absent member, take the routed
  provider's published default (its descriptor `bounds`); QSL FR-255
  publishes QSL's. A present `smt_encoding` is never empty, and an unknown
  member, a zero and a non-integer value refuse before consumption.
- The values decide whether a query can be encoded at all, so they change
  the result (`limit-reached` against a proof) and are part of the request
  identity and the cache key ([FR-306](../tooling/FR-306-compile-and-cache-exactly.md)).
- An encoder stop at a member settles FR-331 result `incomplete`,
  `limit-reached{limit, value, setting}` naming the member, its value and
  its setting (FR-363 "Undecided k-induction runs", rule 3). A provider
  that does not use QSL's encoder reads no member.

### Extent

An item under the event-position false-extension profile classifies `bounded`.
An item under `quire.temporal.infinite-trace/v1` classifies `unbounded`, for
every subject, since its traces are infinite. FR-290's advertised-mode table
then settles it: an unbounded item on a candidate that advertises only
`bounded` for `temporal-satisfaction` settles `unsupported`,
`unsupported_projection`/`unbounded-extent`, with a warning.

### What an advertisement commits a candidate to

A candidate that advertises (`temporal-satisfaction`, `unbounded`) commits to
returning, for each item routed to it, exactly one
[FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md) verdict:

- `proved` only when the formula holds on every admitted infinite behaviour of
  the subject;
- `refuted` only with an [FR-364](FR-364-replay-temporal-counterexamples.md)
  counterexample that reproduces;
- `inconclusive` with cause `bound-reached` when its analysis completed a depth
  with no counterexample, which is a valid result of such a candidate;
- deciding exactly each fairness kind it advertises.

### Fairness kinds

A manifest backend descriptor that advertises `temporal-satisfaction` carries
`fairness_kinds`, the set of fairness kinds its candidate decides, a subset of
{`weak`, `strong`}. For an item with exactly one candidate, `negotiate_*`
SHALL settle an item whose fairness set holds a kind outside the candidate's
`fairness_kinds` as `unsupported`,
`unsupported_projection`/`unsupported-requested-capability`, with a warning
naming the capability kind `temporal-satisfaction`, the mode, the missing
fairness kind and the first constraint of that kind. The check runs after
FR-290's candidate and mode tables, at the single candidate's arm.

A registration that advertises (`temporal-satisfaction`, any mode) SHALL
be refused `invalid_capability`/`invalid-fairness-kinds`, keyed by backend
identity, when its `fairness_kinds` is absent, is not a list, holds a value
that is not exactly `weak` or `strong` (a non-string included) or repeats a
label. One cause covers the four defects, as `invalid-domains` covers its own;
the refusal retains the defect (`absent`, `not-a-list`, `unknown-label` or
`duplicate-label`) and, for `not-a-list`, the member as supplied (such as
`null`), and for `unknown-label` and `duplicate-label` the offending element. When several
defects hold the registry SHALL report the first in that order, and within
`unknown-label` or `duplicate-label` the first offending element in member
order. The registry SHALL NOT read or validate the `fairness_kinds` of a
registration that advertises no `temporal-satisfaction` pair; the FR-331 wire
schema separately refuses such a member on a manifest descriptor, and its
`enum`, `uniqueItems` and `array` types refuse a defective or non-list
`fairness_kinds` of a temporal descriptor on the wire before consumption, so
the registry check serves registrations that do not arrive on the wire. The
registry judges `fairness_kinds` after every advertised pair and the
`domains` member, so a registration that also has a pair or `domains` defect
is refused with that earlier cause. The refusal removes only that
registration.

Negotiation never drops or weakens a fairness constraint to fit a candidate: it
never reads a `strong` constraint as `weak`, an `each` constraint as `whole`,
or omits a constraint.

### No candidate

An item with an empty candidate set settles `unsupported`,
`unsupported_projection`/`unsupported-requested-capability`, with a warning
naming `temporal-satisfaction` (FR-290). This is FR-161's settlement for an
infinite-trace formula that no registered candidate decides.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-368-AC-1 | A temporal item over a model subject carries `subject` (initial-state references, universes, proof bounds), `item_kind`, `fairness`, `max_depth` when it is a bounded search, and `limits`; a request whose items differ only in one universe, one initial state, the item kind or one fairness constraint has a different request identity and a different obligation identity; a request whose items differ only in `max_depth` or one `limits` member has a different request identity, and a result recorded for one is not reused for the other. | Test (TC-302) |
| FR-368-AC-2 | An infinite-trace item classifies `unbounded` and an event-position false-extension item `bounded`; an infinite-trace item whose one candidate advertises only `bounded` settles `unsupported` with `unbounded-extent`. | Test (TC-301) |
| FR-368-AC-3 | An item with a `strong` constraint whose one candidate's `fairness_kinds` is {`weak`} settles `unsupported`, `unsupported-requested-capability`, with a warning naming `temporal-satisfaction`, the mode, `strong` and the constraint; it is never settled as if its constraint were weak. | Test (TC-301) |
| FR-368-AC-4 | An item with only `weak` constraints routed to a candidate advertising {`weak`, `strong`} settles through that candidate's arm with every constraint unchanged in kind and granularity. | Test (TC-301) |
| FR-368-AC-5 | An infinite-trace item with an empty candidate set settles `unsupported` with a warning naming `temporal-satisfaction`, and never holds or blocks. | Test (TC-200, TC-301) |
| FR-368-AC-6 | A candidate advertising (`temporal-satisfaction`, `unbounded`) returns one FR-363 verdict per routed item, and a bound-reached `inconclusive` from it is an accepted result, not a provider failure. | Test (TC-301) |
| FR-368-AC-7 | A request omitting `max_automaton_states` runs under the routed provider's published default, and the terminal record of a stopped run names the limit and its effective value. | Test (TC-298, TC-302) |
| FR-368-AC-8 | A registration advertising `temporal-satisfaction` whose `fairness_kinds` is absent, is `"weak"` or `null`, holds `"fair"`, `1` or `null` as an element, or is [`weak`, `weak`] is refused `invalid_capability`/`invalid-fairness-kinds` retaining `absent`, `not-a-list` (for the non-list forms), `unknown-label` (naming the element) or `duplicate-label` (naming `weak`), keyed by backend identity; [`weak`, `strong`] and [] (a backend that decides no kind) are admitted. | Test (TC-301) |
| FR-368-AC-9 | A registration holding both an unknown label and a repeated label reports `unknown-label`, naming the first unknown element in member order; a registration advertising only `value-validity` is admitted whatever its `fairness_kinds` holds, since the registry does not read it. | Test (TC-301) |
| FR-368-AC-10 | A manifest descriptor on the wire whose temporal `fairness_kinds` is absent, not a list, holds an unknown label or repeats a label fails the FR-331 schema, and so does a `fairness_kinds` member on a descriptor advertising no `temporal-satisfaction`. | Test (TC-431) |
| FR-368-AC-11 | A temporal item whose `limits` carry `max_solver_steps` 1000 and `max_solver_time` 5 is admitted, and its solver stops settle as FR-363-AC-11 states; two requests that differ only in `max_solver_time` have equal request identity and cache key bytes, two that differ only in `max_solver_steps` differ, and a result stopped by `max_solver_time` is stored by neither request; a `max_solver_time` of 1.5 on the wire, and either member at 0, is refused before consumption. | Test (TC-302) |
| FR-368-AC-12 | A temporal item carrying `smt_encoding` `{max_constants: 7}` is admitted, and its encoder stop settles as FR-363-AC-11 states; two requests that differ only in `smt_encoding.max_constants` (7 and 8) have different request identities and cache keys, and a result recorded for one is not reused for the other; an empty `smt_encoding`, an unknown member, a `max_constants` of 0 and one of 1.5 are each refused before consumption. A completed result produced under `max_solver_time` 5 and reused for a request that sets 10 states 5. | Test (TC-302) |
| FR-368-AC-13 | The `item_kind` of a temporal item is one of `formula`, `deadlock-freedom`, `time-lock-freedom`, `race-freedom` and `refinement`; each of the five is admitted, and `time-lock`, `race`, `deadlock`, `refine` and `Formula` are each refused before consumption. | Test (TC-431) |
| FR-368-AC-14 | The claim of a `deadlock-freedom`, `time-lock-freedom` or `race-freedom` item names its subject's anchor node and that node's `declaration` occurrence at ordinal 0, or its `expression` occurrence at ordinal 0 when it has no `declaration` occurrence: the protocol clause node for a protocol subject, the `model` node of the alias for any other model subject, and the refinement node for a `refinement` item. The deadlock-freedom and time-lock-freedom items of one model subject name the same node and occurrence. | Test (TC-431) |
| FR-368-AC-15 | A `deadlock-freedom`, `time-lock-freedom` or `race-freedom` item whose `fairness` holds a constraint is refused before consumption; a `refinement` item carries the concrete fairness set. | Test (TC-431) |
| FR-368-AC-16 | A derived or `refinement` item classifies `unbounded` with `finite_bound_available` false, an `infinite-trace` domain keyed by the claim's node with the empty path, and no `extent.bounds`; the same item with a `bounded` extent, or with no `infinite-trace` domain, is refused. | Test (TC-431) |
| FR-368-AC-17 | In a request, the caller's items come first in the caller's order and the derived items follow, grouped by subject in order of the subject's first occurrence among the caller's items and, within a subject, as `deadlock-freedom`, `time-lock-freedom`, `race-freedom`; `request_index` is the position. Exchanging two derived items of one subject, or moving a derived item before a caller's item, breaks the order. | Test (TC-302, TC-431) |
| FR-368-AC-18 | A derived or `refinement` item is a `temporal-satisfaction` item with no `hybrid` and no `requirements`, routed by FR-290's tables; a request that gains a derived item changes no caller's item's `request_index`. | Test (TC-302) |
| FR-368-AC-19 | A `refinement` item carries `abstract_subject` (`package_id`, `initial_states`) beside its concrete `subject`, and an item of any other kind carrying one is refused; a refinement item without one, one whose `abstract_subject` has an extra member such as `universes` or `proof_bounds`, and one whose `package_id` is not a package id are refused, and a `package_id` that no entry of `request.dependencies` carries is refused by the reader (the schema cannot relate the two members); two refinement items that differ only in `abstract_subject` have different obligation identities. | Test (TC-431) |
| FR-368-AC-20 | Every temporal item of a request, authored, derived and `refinement`, carries the same `limits`, `max_depth` and `smt_encoding`; `transfer` is not an `item_kind`. | Test (TC-431) |

## Dependencies

- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) owns the
  capability kinds, the advertised-mode table and the candidate table this
  requirement adds the fairness-kind check to.
- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) owns
  the envelope that carries these members.

## References

- quire-spec-language ADR-018 §3 Placement in negotiation and QS-9; ADR-019
  SV-1, SV-5, BE-1 and QS-4; Linear STD-131 and STD-132.
