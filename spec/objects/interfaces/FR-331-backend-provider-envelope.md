---
id: FR-331
title: "Backend provider request and result envelope"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_013
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-195
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-029
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-193
    type: references
  - target: ix://agent-ix/quire-specification/FR-306
    type: references
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
  - target: ix://agent-ix/quire-specification/FR-305
    type: references
  - target: ix://agent-ix/quire-specification/FR-368
    type: references
  - target: ix://agent-ix/quire-specification/FR-439
    type: references
  - target: ix://agent-ix/quire-specification/FR-391
    type: references
  - target: ix://agent-ix/quire-specification/FR-392
    type: references
  - target: ix://agent-ix/quire-specification/FR-399
    type: references
  - target: ix://agent-ix/quire-specification/FR-461
    type: references
  - target: ix://agent-ix/quire-specification/FR-409
    type: references
  - target: ix://agent-ix/quire-specification/FR-410
    type: references
  - target: ix://agent-ix/quire-specification/FR-412
    type: references
  - target: ix://agent-ix/quire-specification/FR-413
    type: references
  - target: ix://agent-ix/quire-specification/FR-418
    type: references
  - target: ix://agent-ix/quire-specification/FR-382
    type: references
  - target: ix://agent-ix/quire-specification/FR-462
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
  - target: ix://agent-ix/quire-specification/FR-272
    type: references
  - target: ix://agent-ix/quire-specification/FR-390
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-338
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-314
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-149
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-163
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-120
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-280
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-185
    type: references
  - target: ix://agent-ix/quire-specification/FR-300
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-179
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-169
    type: references
  - target: ix://agent-ix/quire-specification/FR-412
    type: references
  - target: ix://agent-ix/quire-specification/FR-418
    type: references
---

# FR-331: Backend provider request and result envelope

## Description

When a contract package is submitted to a backend provider, the producer SHALL use the immutable BackendProviderEnvelope contract below rather than an untyped map, display string or implementation-local structure.

## Properties

- `contract_version` — exactly `quire.backend-provider/v1`.
- `capability_vocabulary` — exactly `quire.capability-kind/v1`; it selects the FR-290 vocabulary for every kind in `manifest`, `request` and `results`.
- `manifest` — one backend descriptor per registered backend in the registry snapshot: backend identity, advertised (FR-290 kind, mode) pairs, domains, options, bounds, where the backend declares one its obligation-form set (FR-349), the state-space reductions it applies, a subset of `symmetry`, `partial-order` and `constraint` (FR-382), the state-graph claim forms it decides (`state_graph_forms`, below), and, for `probabilistic-satisfaction`, its advertised evidence kinds (FR-290). A backend that advertises `temporal-satisfaction` also carries its `fairness_kinds` ([FR-368](../../functional/temporal/FR-368-negotiate-temporal-model-check-items.md)). A backend that advertises `temporal-satisfaction` may also carry `state_graph_forms`, a nonempty, duplicate-free subset of `possible`, `always-possible` and `unique-path` (FR-390). Its absence means it decides no state-graph claim form. A descriptor that does not advertise `temporal-satisfaction` carries no `state_graph_forms`. A backend that advertises `temporal-satisfaction` may also carry `memory_models`, the memory models it decides for items over a subject with a weak `parallel`, a nonempty set drawn from `sc`, `tso` and `ra` by the names of the `memory` clause ([FR-439](../../functional/memory/FR-439-bound-replay-and-report-weak-memory-runs.md) routes by it). A descriptor with no `memory_models` advertises `sc` only, so, of the items over a subject with a weak `parallel`, it receives only those resolved to `sc`; a descriptor that does not advertise `temporal-satisfaction` carries none. A backend that advertises `temporal-satisfaction` may also carry `dynamics`, the dynamics it decides, a nonempty set drawn from `discrete` and `hybrid`; a descriptor with no `dynamics` decides `discrete` only, a descriptor whose `dynamics` holds `hybrid` carries both hybrid solver budgets in `bounds`, and negotiation routes by it ([FR-193](../../functional/analysis/FR-193-analyze-continuous-hybrid-models.md) "Routing a hybrid item"). Every descriptor carries `tool` ("Backend tool" below). Backend identities are unique.
- `request` — in the frame form sent to a process provider, the checked package as `package` and `package_id` ("Package bytes" below); in the in-process form ("In-process providers" below), the checked package as a typed value and no `package` or `package_id`; in both forms, the provider deadline `deadline_ms` and the requested items: each item's claim, its proof bounds (the substituted `ProofBound`s of its `extent`, and a temporal item's `subject.proof_bounds`), its `options`, its `requirements` (the hybrid-dynamics requirement of a hybrid item, FR-193 "Routing a hybrid item") and its own limits (`temporal.limits`, `probabilistic.limits`, `hybrid.limits`); there is no request-level `domains` or `limits` member. For each item, its one required FR-290 kind, its extent and #222 extent classification (`bounded` or `unbounded`), the backend the caller names when it names one, its `candidates`, and for a model-check item its `reductions` selection (FR-382); for a `probabilistic-satisfaction` item, its evidence kind (`statistical` or `exact`) and, for `statistical`, its method (`okamoto`, `sprt`, `clopper-pearson` or `regenerative` with `min_cycles`), seed, `max_witnesses`, for a timed subject its grid exponent `q` and `max_sample_steps` (default `2^24`), and `StatisticalLimits{max_samples, max_draws, max_cycle_steps}` ([FR-408](../../functional/analysis/FR-408-settle-probabilistic-claims-statistically.md)), or, for `exact`, `ExactProbLimits` ([FR-411](../../functional/analysis/FR-411-settle-probabilistic-claims-exactly.md)). Every limit has the published default that [FR-461](../../functional/foundation/FR-461-name-the-limit-value-and-setting.md) gives it, and `max_sample_steps` defaults to `2^24`.
- `temporal` (per `temporal-satisfaction` request item over a model subject) — the item's `subject` (initial-state references, universes, proof bounds), `item_kind` (`formula` or `deadlock-freedom`), resolved `fairness` set, `max_depth` for a bounded search and `limits`, as [FR-368](../../functional/temporal/FR-368-negotiate-temporal-model-check-items.md) states. `limits` also holds `max_clocks` for a timed model ([FR-415](../../functional/temporal/FR-415-declare-clocks-and-timed-models.md)), with the setting name `model_check.max_clocks`; a state with more live clocks than `max_clocks` records `incomplete` with `limit-reached` whose `limit` is `max_clocks`. For a state-graph, hyper or step-relation item, `limits` also holds `max_walk_steps` (the step count of one witness-sampling walk; [FR-392](../../functional/analysis/FR-392-sample-possible-witnesses.md)), `max_witness_set` and `max_relation_tuples` ([FR-399](../../functional/analysis/FR-399-settle-hyperproperty-results.md)), with the setting names `model_check.max_walk_steps`, `model_check.max_witness_set` and `model_check.max_relation_tuples` that [FR-461](../../functional/foundation/FR-461-name-the-limit-value-and-setting.md) gives them; the routed provider publishes each default, which applies when the request sets none. `temporal` also holds the sampling settings `witness_samples` and `seed`, which are method parameters with provider-published defaults.
- `candidates` (per request item) — exactly one of: the set of candidate backend identities that the `quire-spec-language` registry computes under FR-290's candidate-set rule, ordered bytewise by identity; or the unknown-backend mark carrying the unregistered backend identity that the request named.
- `dispositions` — supported, requires-bound, unsupported or invalid-request per item.
- `artifacts` — immutable generated bytes with kind, identity and source map.
- `results` — proved, refuted, tested, measured, inconclusive, unsupported, declined, incomplete or failed per item. `measured` is recorded only for a completed statistical run and carries the [FR-409](../protocol/FR-409-measured-decision.md) decision and the [FR-410](FR-410-statistical-verdict.md) `StatisticalVerdict`, with no truth and no settlement basis; it is never `proved`. A `proved` result carries its proof basis: `exact-value` or `value-bounds` with an [FR-412](FR-412-probability-certificate.md) certificate, `zone-certified` with an [FR-418](FR-418-zone-certificate.md) certificate, `closed-form{analysis}` with its recomputed evidence ([FR-419](../../functional/analysis/FR-419-analyze-task-set-schedulability.md)), each settled only after its certificate or evidence is checked; or `bounded-solver{method, horizon, jumps, exhaustive}` or `inductive-solver{method}` ([FR-193](../../functional/analysis/FR-193-analyze-continuous-hybrid-models.md)), a solver's own result with neither, labelled `uncertified`. Every `proved` result also carries exactly one `certification`, which says who stands behind the proof ("Certification" below). `unsupported` is recorded for a supported, routed item whose tool probe finds the tool missing, finds a version other than the descriptor's `tool.version` ("Backend tool"), errors or exceeds its limit, naming the capability kind and the backend; and, separately, for a supported item whose Kani run ends `Unavailable` (the solver or backend is absent after negotiation), naming the capability kind, the backend and the absent solver or backend identity. `declined` is recorded for a supported item whose Kani run ends `Refused`, `InvalidInput` or `IncompleteInput`, and names the typed refusal cause (QSL ADR-013 O-16 refusal category, QC-9); it keeps the item's `supported` disposition and is the item's one terminal accounting record, the same as a tool-absence `unsupported` result. A Kani proof run whose obligation has zero SUCCESS checks records `inconclusive` with a typed vacuity cause, never `proved`; this adds no result value (QSL ADR-013 FND-005, O-16 proof column). An item routed to a process provider over a checked package that `package` refuses to emit settles `unsupported` with a diagnostic ("In-process providers").
- `counterexamples` — canonical assignments/traces plus originating run identity. A hybrid refutation carries an FR-193 `HybridCounterexample` (`type` `hybrid-counterexample`, `kind` `formula` or an `undefined-evaluation` envelope, and its `HybridTrace`), recorded only once exact replay reproduces it (FR-193 "Trace replay"); a hybrid solver's inconclusive trace is not a counterexample but the `trace` member of the item's `solver-inconclusive` cause (FR-193 "Hybrid trace"). A temporal refutation over a model subject or a supplied trace is an [FR-364](../../functional/temporal/FR-364-replay-temporal-counterexamples.md) `TemporalCounterexample`; a timed trace's steps carry their `delay` and a time-lock counterexample carries `kind` `time-lock` and the member `final_delay`, the exact rational delay that reaches the time-locked state (FR-181, [FR-417](../../functional/temporal/FR-417-settle-time-lock-freedom.md)); a probabilistic refutation carries an [FR-413](FR-413-probabilistic-counterexample.md) `ProbabilisticCounterexample`. Each entry names exactly one of: the transcript's `artifacts` entry, whose assignments are its decode and are never separately stored, or the canonical assignments of a counterexample with no transcript; never both (QSL ADR-013 O-25, QC-6).
- `accounting` — exactly one terminal record for every requested item. A temporal item's terminal record carries its [FR-363](../../functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md) proof basis and depth or cause, and any [FR-369](../../functional/temporal/FR-369-name-excluding-strong-fairness.md) `fairness.strong-would-exclude` warnings. The terminal record of a state-graph, hyper or step-relation item also states its settlement method ([FR-391](../../functional/analysis/FR-391-settle-state-graph-claims.md), [FR-399](../../functional/analysis/FR-399-settle-hyperproperty-results.md)), its search horizon, and the seed and limits the run used.

### Certification

Every `proved` result entry carries the member `certification` with exactly
one of three values. A reader tells from it alone whether the proof was
checked, unchecked or taken on trust:

| `certification` | Meaning | Proofs that carry it |
| --- | --- | --- |
| `certified` | The qualified core produced or checked the proof: a core checker accepted its certificate, or the core's own prove path produced it | an accepted certificate or recomputed evidence (`exhaustive` with no state constraint and its closure or component certificate, `reduced` with copy-swap as its only reduction and its product-closure certificate ([FR-403](../../functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md)), `bounded-complete` or `inductive` whose SMT certificate the core verified ([FR-354](../../functional/ir-proof/FR-354-record-proof-trust-basis.md)), `exact-value`, `value-bounds`, `zone-certified`, `closed-form`, `witnessed`); a Kani `bounded-proof{checks}` |
| `uncertified` | A first-party engine or solver outside the core proved it, and no core checker re-checked it | `reduced` with a `symmetry` or `partial-order` reduction, and `exhaustive` under a state constraint ([FR-385](../../functional/analysis/FR-385-report-reduced-verdicts.md)); `bounded-solver` and `inductive-solver` ([FR-193](../../functional/analysis/FR-193-analyze-continuous-hybrid-models.md)); an SMT proof whose certificate is absent or cannot be verified (FR-354); any proof whose method has no core certificate checker |
| `trusted` | A third-party plugin proved it, and the core runs no check of it | every plugin `proved` result ([FR-305](../../functional/tooling/FR-305-load-explicit-rust-plugins.md)) |

Each value settles `proved`, and `certification` belongs to `proved` alone.
A `proved` result of the SMT backend whose Z3 cross-check ran (FR-349) also
carries `cross_check: {tool, verdict}` beside `certification`: `tool` the
cross-check tool's name and `verdict` the result its run alone settles for
the same query, `proved`, `refuted` or `inconclusive`. A `verdict` other than
`proved` records a disagreement over a proof the core certified, which
FR-349 keeps `proved`, `certified`; the cross-check never changes
`certification` (FR-354). No other result carries `cross_check`. The reader
SHALL refuse before consumption a `cross_check` on any result other than
`proved`; on a `proved` result whose `basis` is not `bounded-complete` or
`inductive`, the SMT backend's proof bases, or whose `certification` is
`trusted`; and a `verdict` other than `proved` on a proof that is not
`certified`, since FR-349 settles that disagreement `inconclusive`. A
disagreement that FR-349 settles `inconclusive` is the cause
`cross-check-disagreement{tool, primary, cross_check}`, `primary` the
primary solver's result and `cross_check` the cross-check's, both result
values. A cross-check that did not run is FR-349's `cross-check-absent{tool}`
warning, never a `cross_check` member.
A certificate a core checker rejects settles the item `inconclusive` with
cause `certificate-rejected{rule, at}`: `rule` the check rule that failed and
`at` where it failed, for every core checker.

`rule` is a closed value: the name of the check rule, lower-case words
joined by `-`, so QSL FR-338's `CertificateRule::QueryMismatch` is written
`query-mismatch`. `at` is the locus the checker names, in its one wire form
below. The owning requirement states what each locus means and which rule
each failure takes; this requirement owns the spelling. Each rule is
admitted only at the loci listed with it, and the schema checks the rule,
the locus form and their pairing, so a reader SHALL refuse an unknown rule,
an unknown locus form and a rule at a locus not listed with it before
consumption.

Before consuming a rejection, the reader SHALL bind `request_index` to
the originating request and its admitted checked claim in the request's
package, subject and obligation context. It SHALL resolve the applicable
checker from that claim's classified form and the requested evidence and
method: QSL FR-314 for an SMT certificate; FR-338/339 for EN-1 temporal
closure/components; FR-163 for a hyper product closure; FR-169 for a
state-graph certificate; FR-149 for a refinement simulation certificate;
FR-412 for exact probabilistic evidence; and FR-418 for a zone
certificate. A capability kind or backend label alone does not identify
that checker. The reader SHALL refuse before consumption when the request
binding or checker context is absent, conflicting or ambiguous, or when
the rule and locus are not applicable to the resolved checker and claim
form. This is a reader obligation beyond the global schema union. A
rejection does not carry a successful proof's `basis`, and the reader
SHALL NOT invent one to select a checker.

| Locus | `at` | Meaning owned by | Rules |
| --- | --- | --- | --- |
| Query part | `"unrolling"`, `"base"` or `"step"`; for a refinement certificate's step row `r`, `"step/<r>"`, `r` the zero-based row as a `U64String`, so `"step/3"` is the fourth row and `"step"` alone is an `Inductive` proof's step query | QSL FR-338's `Query { part }`, QSL FR-314; for the step row, `step/<r>` is the spelling QSL FR-338 specifies and FR-468's per-step refinement certificate supplies the row | `query-mismatch`, `shape-mismatch`, `malformed`, `not-refutation` |
| Proof step | `{part, index}`: `part` a query part spelling, `step/<r>` included, and `index` the `U64String` of QSL FR-314's command number, the command's position from 0 in the proof's flattened text order whatever its `:id` | QSL FR-314's `ProofStep { part, index }` | `proof-step-invalid`, `not-refutation` |
| Product state | `{model_state, automaton_state}`: the model state's digest record and the bytes of the automaton state's canonical key | QSL FR-338's `ProductState`, QSL FR-339 | `initial-missing`, `successor-missing`, `bad-state`, `not-partition`, `backward-edge`, `witness-fails`, `shape-mismatch` |
| Hyper product state | `{states, automaton_state, witness_set}`: each component's `StateKey` in variable order; the automaton key's bytes, absent for an HP-1 member; for HP-3, the witness set's pairs `{states, automaton_state}` in their sorted order | QSL FR-163's `HyperProductState` | `initial-missing`, `successor-missing`, `not-partition`, `backward-edge`, `witness-fails`, `malformed`, `undefined-member`, `violating-member` |
| State | `{domain, digest}`, the model state's digest record: `domain` is `quire.simulation.state-key/v1` and `digest` the state's `StateKey`; under `malformed` the record is that of a state-graph closure entry, whose `domain` may be any FR-201 digest domain, since FR-169 rejects an entry outside the state-key domain at that entry | QSL FR-169's `ModelState`; FR-418 (the initial discrete state). FR-412's complete probabilistic product key is a separate identity, not this model key. | `initial-missing`, `successor-missing`, `bad-state`, `predicate-undefined`, `rank-missing`, `rank-broken`, `order-broken`, `count-broken`, `malformed` (any FR-201 domain; every other rule, the state-key domain only) |
| State-graph entry | `{entry}`, the `U64String` closure index that names no closure state | QSL FR-169's `Entry { index }` | `malformed` |
| Probabilistic product state | `{product_key}`, the unpadded base64url spelling of the complete canonical QSL FR-196 product-key octets defined below | FR-411, FR-412 | `inequality-fails`, `rank-missing`, `rank-broken`, `threshold-missed` |
| Hyper transition tuple | `{transitions}`: the ordered, nonempty tuple of labelled edges `{pre_state, transition, post_state, result}` defined below | QSL FR-163, FR-179, FR-120 | `undefined-member`, `violating-member` |
| Simulation step | `{position, edge, verdict}`, every member present: `position` a `U64String`; `edge` a `U64String` for `post-differs` and `step-fails` and `null` for `initial-mismatch`, `successors-differ`, `bad-state` and `malformed`; `verdict` the step verdict below for `step-fails` and `null` for every other rule | QSL FR-149's `SimulationStep` | `initial-mismatch`, `successors-differ`, `bad-state`, `malformed`, `post-differs`, `step-fails` |
| Zone node | `{node}`, its position in the certificate's `nodes` | FR-418 | `successor-missing`, `bad-state`, `witness-fails` |
| Zone edge | `{edge}`, its position in the certificate's `edges` | FR-418 | `backward-edge` |
| Zone component | `{component}`, its component index | FR-418 | `witness-fails` |
| Certificate member | `{member}`, the name of a member of the FR-412 or FR-418 certificate: `identity`, `objective`, `kind`, `values`, `ranking`, `policy`, `components`, `scale`, `bounds`, `nodes`, `edges`, `reasons` or `divergence`; QSL FR-163 also owns `components` for an absent hyper component structure | FR-412, FR-418, QSL FR-163 | `malformed` |

A model state's digest record is `{"domain": "quire.simulation.state-key/v1",
"digest": <StateKey>}`, in a State locus, as a product state's `model_state`
and as an undefined evaluation's `state_key`. Only a State locus under rule
`malformed` carries another FR-201 digest domain, with a lower-case hexadecimal
digest; the reader SHALL refuse a `domain` that FR-201 does not list, and a
State locus in another domain under any other rule, before consumption. A query part that names a step
row is the text `step/`, then the row's `U64String`; `step/02`, `step/` and a
row after `unrolling` or `base` are not query parts.

A simulation step's `verdict` is the `check_step` verdict that judges the
relation, an object tagged by `kind` with `U64String` integers:

| `kind` | Members |
| --- | --- |
| `InitialNotAbstract` | `initial`, the concrete initial state's index |
| `StutterChanged`, `NoAbstractMatch` | `position` |
| `AbstractStepRejected` | `position`; `transition`, the transition identity; `cause`, `{kind: "Precondition", clause}`, `{kind: "Frame", code}` or `{kind: "Postcondition"}` |
| `UndefinedEvaluation` | `where`, `{position, state_key, locus}` with `locus` one of `atom`, `activation`, `contract`, `terminal`, `constraint`, `protocol`, `mapping` and `claim-predicate`; `cause`, `kernel`, `family` or `insufficient-data` |
| `Undetermined` | `cause`, `MappingUndetermined` or `UndecidedSuccessor`; `evaluation`, `{outcome: "refused"}` or `{outcome: "incomplete", limit}`, `limit` `evaluation-meter` or the evaluation counter's snake-case name |

A verdict of another kind, an FR-379 `Undefined` failure (which `check_step`
never returns), a `type`-tagged cause or a JSON-number integer is not a step
verdict.

For HP-3, the hyper locus's `states` is the universal tuple and may be
`[]` when the admitted claim has no universal variable (ADR-023 HC-2).
The required tuple's absence is invalid; an empty tuple is a present
value. Its `witness_set` is required and retains every actual existential
tuple/automaton-key pair, including the empty set. HP-1 state-member
failures have no automaton key. An HP-1 failure on a labelled transition
tuple SHALL preserve QSL FR-163's complete `HyperTransitionTuple`, not
project it onto `states`. FR-412 state failures SHALL preserve the complete
probabilistic product key in every keyed certificate member and rejection.
For a zero-universal HP-3 claim, every supplied witness tuple has at least
two existential state keys (`n = 0`, `n + m >= 2`, ADR-023 HC-2). When the universal and existential tuple arities, variable bindings or
HP-3 witness set differ from those of the originating checked claim, the
reader SHALL refuse the rejection before consumption; matching this global
shape does not authorize another form.

An HP-1 `HyperTransitionTuple` SHALL carry exactly `transitions`, with one
edge for each execution-variable binding in binding order. Each edge
SHALL carry exactly `pre_state` and `post_state` as the actual model
`StateKey`s, `transition` as the existing ordinary `TransitionIdentity`
(`type: "transition"`, operation, receiver and typed arguments), and
`result` as the actual result in the existing typed canonical value form,
or JSON `null` when the operation has no result. A typed result whose
value is null is not the absent-result marker. FR-120 owns the result or
none, FR-179 owns its retained labelled edges, and FR-163 owns this
rejection identity. The tuple SHALL NOT be sorted independently of the
variable bindings or reduced to the pre-state tuple; missing edge
members, protocol transitions and extra members SHALL be refused. No
automaton key is attached to this HP-1 tuple.

`AutomatonStateKey` SHALL travel as unpadded base64url of exactly QSL
FR-338 / ADR-018 PC-3's RFC 8785 UTF-8 octets of
`["quire.automaton-state-key/v1", context, state]`. When the key's canonical sets, occurrence
paths, past memory, windows or acceptance obligations differ from those the
checked property derives, the reader SHALL refuse the rejection before
consumption. Neither a materialization index nor an opaque residual body is a
substitute.

`ProbabilityProductKey` SHALL travel as unpadded base64url of exactly QSL
FR-196 / ADR-028 PR-4's RFC 8785 UTF-8 octets of
`["quire.probability-product-key/v1", model, monitor, accumulator, pending]`.
All five members are required. `model` retains the full typed canonical
simulation state (including digital clock, deadline and tail control when
applicable), not its digest. The remaining members retain their actual
FR-196 closed forms, including activation, the accumulator,
actual units, and the complete scheduled identity and declaration-ordered
drawn vector. Actual None is JSON null in that canonical body, not an
omitted member or an invented empty/default component. A formula monitor
embeds the parsed complete FR-338 array, not its base64url spelling.

The certificate's existing row member name `state` in `values`, `ranking`,
`policy` and each component's `bias` SHALL carry this same
`ProbabilityProductKey`; a rejection uses `{product_key}` to remain
disjoint from the model-only `{domain, digest}` State locus. When a decoded
key differs from the canonical re-encoding of the key that the checked claim
and model derive under QSL FR-196 PR-4, or has an absent, extra or
inadmissible component, the reader SHALL refuse the certificate or rejection
before consumption. Equality and sorting use complete decoded unsigned octets,
never the base64url text's order, a local index or a digest projection.

Base64url only transports these owner-defined bytes and does not create a
digest domain. Padded/aliased transport, non-JCS bodies, unknown tags,
missing/extra members and contextually invalid components SHALL refuse
before consumption.

A `U64String` is the canonical decimal string of an unsigned 64-bit
integer, `0` to `18446744073709551615`, so RFC 8785 keeps every value
exact.

### Wire spellings of result causes and proof bases

Every proof basis and cause below is a closed wire value, spelled exactly as
written: lower-case words joined by `-`. (AD-016's Kani cause
`kani_vacuous_proof` is outside this table and keeps its AD-016 spelling; it
is the one closed cause not spelled with `-`.) Its payload members, in braces, are
named members of the result entry's `basis` or `cause` object ("Result
entries" below). A producer SHALL emit no other spelling,
and a reader SHALL refuse any other spelling of a result cause or proof basis
for these items with `invalid_result`/`unknown-cause`.

| FR-331 result | Closed values |
| --- | --- |
| `proved` (proof basis) | `exact-value{value, reductions}`, `value-bounds{lower, upper, method, reductions}` with `method` `backward-induction{precision_bits}` or `interval-iteration{precision_bits, sweeps}`, `zone-certified`, `exhaustive`, `closed-form{analysis}` with `analysis` `fixed-priority`, `edf` or `amc-rtb`, `bounded-solver{method, horizon, jumps, exhaustive}` and `inductive-solver{method}`, each `method` from FR-193's closed method table ("Solver methods") |
| `refuted` (cause) | `undefined-evaluation{where, cause}` |
| `measured` (FR-409 cause) | `undefined-evaluation{where, cause}` for `rejected`; `indifference-region` and `undecided-successor` for `undecided` |
| `inconclusive` (cause) | `certificate-rejected{rule, at}`, `cross-check-disagreement{tool, primary, cross_check}` (FR-349), `replay-parity`, `undecided-successor`, `no-initial-state`, `no-instance`, `sufficient-test-failed`, `solver-inconclusive{trace}`, whose `trace` is the optional FR-193 `HybridTrace`, `insufficient-horizon{horizon, jumps, exhaustive}` (FR-193 "Proof coverage"), `lasso-not-concretized`, `no-admitted-behaviour`, `well-definedness-unchecked`, `stale-dependency{requested, recompiled}`, `refuted-ignoring-constraints` (QSL FR-127): a liveness lasso found only after the request explicitly omitted memory fairness constraints; it carries no proof basis and is not a `refuted` result or a standard counterexample |
| `incomplete` (cause) | `limit-reached{limit, value, setting}`, naming the limit, its value and the request setting that raises it, the one limit-stop cause of every engine; `timed-out`, the backend's own wall-clock limit, as a hybrid solver's `max_solver_time` or a provider's overrun of `deadline_ms` ("Hybrid solver budgets" below); `precision-budget{lower, upper}`; `cancelled{source}` for a cancelled run, `source` the cancellation cause, `requested` when the caller cancels and `deadline` when the caller's deadline ends the run |
| `unsupported` (cause) | `missing-confidence`, `every-scheduler`, `exact-only-form`, `confidence-beyond-precision`, `not-markov{state, transition, post_states}`, `not-stochastic{state, identity}`, `time-locked-sample{state}`, `total-above-draw-range{total}`, `zero-weight-component{component}`, `delay-distribution{operation}`, `strict-clock-constraint{locus}`, `zero-delay-cycle{states}`, `timed-form-shape{locus}`, `punctual-interval{locus}`, `stopwatch-required` |

A disposition `invalid-request` with `invalid_capability`/`not-statistical`
is a negotiation outcome ([FR-408](../../functional/analysis/FR-408-settle-probabilistic-claims-statistically.md))
and carries no result.

## In-process providers

An in-process provider is one whose engine runs in the producer's own
process over the producer's checked package, such as a QSL `analyze` engine
that `prove` routes to (QSL FR-280). The producer SHALL hand an in-process
provider the request's `manifest`, `deadline_ms` and items as typed values,
together with its checked package and the subject binding (QSL FR-120) that
`prove` and `analyze` take, as typed values with no `package`, `package_id`,
`dependencies` or `provision` member: the checked package and its resolved
dependencies are already in that process. The provider trait (QSL ADR-029
PV-3) also gives such a provider `emitted: Option<&EmittedPackage>`, which the
producer passes as `None` when `package` refuses to emit and as `Some`
otherwise, and an in-process provider ignores it (QSL FR-280). Such a request is never written as
a request frame, so the "Package bytes" rules do not apply to it, and a
request frame that lacks `package` or `package_id` still refuses before
consumption (FR-331-AC-18).

The in-process form exists because some checked packages have no v2 bytes:
the `package` operation refuses a package that declares a random parameter,
a workload, a reward or a probabilistic claim (QSL FR-185) while its checked
package remains, and a statistical engine decides the probabilistic claim
over that checked package. An in-process provider's results, dispositions,
counterexamples and accounting are the members this envelope defines for any
provider.

An in-process request carries no `package` member, so it has no request
identity: the producer SHALL NOT reuse the result of one in-process request
for another, and SHALL run the engine once for each in-process request.

An item routed to a process provider over a checked package that `package`
refuses to emit has no package bytes to send. The producer SHALL settle it
`unsupported` with the diagnostic `unsupported_projection`/`unsupported-requested-capability`,
whose `detail` names the item's capability kind, the backend and that the
package is not emitted, and SHALL write no request frame for it.

## Backend tool

Every backend descriptor SHALL carry `tool`, the closed object
`{name, version, build}` that names the code which decides the backend's
results: `name` and `version` are nonempty exact UTF-8 strings, and `build`
is the content identity of the build inputs of the provider code that
implements the backend.

- A backend **fronts an external tool** when any result it gives depends on
  an executable that is not part of the producer's build: a tool binary the
  backend runs, such as Kani's `cargo kani` with the CBMC it ships, or an SMT
  solver binary; and, for a process provider, the provider's own
  executable. Its `name` is the executable the backend invokes and its
  `version` the version that executable reports. A process provider whose
  engine is its own code and that runs no other executable names its own
  executable and its version.
- A backend is a **pure in-process engine** when every result it gives is
  computed by code linked into the producer's build, in the producer's
  process, and it runs no executable; the QSL engines `qsl-model-check` and
  `qsl-prob` are such engines. Its `name` is the crate that implements the
  engine and its `version` that crate's package version in the producer's
  build.

`build` is a digest record `{domain, algorithm, digest}` (the FR-322
`PackageId` shape) with `domain` `quire.build-inputs/v1`, `algorithm`
`sha256` and `digest` 64 lowercase hexadecimal digits: SHA-256 over the
RFC 8785 bytes of the build-input record of the provider code that
implements the backend, that is, the pure engine's crate, a linked
provider's adapter code, or a process provider's executable. The build that
compiles that code SHALL compute it. Equal inputs give an equal `build`, a
change of any input changes it, and a specification or repository holds no
copy of any `build` value: each build derives its own.

The build-input record is the closed object
`{sources, dependencies, compiler, target, profile, options}`, every string
exact UTF-8:

| Member | Value |
| --- | --- |
| `sources` | One `{path, sha256}` per file the build reads as source of that code and of every package of its own workspace or project that it links: `path` relative to the workspace or project root with `/` separators, `sha256` 64 lowercase hexadecimal digits over the file's bytes. Ordered bytewise by `path`; each `path` once. Nonempty. |
| `dependencies` | One `{name, version, source}` per third-party package the build resolves, as its lock file or package manager records them (`source` the registry, repository or path it comes from). Ordered bytewise by `name`, then `version`, then `source`; each triple once. Empty when the build resolves none. |
| `compiler` | The compiler, or for interpreted code the interpreter, that builds or runs the code, with its version as it reports it, such as `rustc 1.98.0`. |
| `target` | The platform the code is built for, such as a target triple. |
| `profile` | The build profile or configuration name, such as `release`. |
| `options` | A map from each build setting that can change the produced code (enabled features, optimisation level, code-generation flags) to its value, as strings. Empty when the build sets none. |

A Cargo build fills the members from its workspace: the crate's and its
workspace path dependencies' source files, the `Cargo.lock` packages, `rustc`,
the target triple, the profile and the enabled features and code-generation
settings. A build by any other toolchain fills the same members from its own
inputs. A process provider whose build inputs are not available to it, such
as one shipped only as a prebuilt executable, records that executable as its
one `sources` entry, with the executable's file name as `path` and the
digest of its bytes; `dependencies` and `options` are empty, `compiler` and
`profile` are the string `prebuilt`, and `target` is the platform it runs
on. A change to the executable's bytes then changes its `build`.
A pure in-process engine's code change therefore changes its descriptor
even when its crate version stays the same.

The schema refuses a descriptor without `tool` at every stage, and a
registry snapshot carries each descriptor exactly as its provider gave it,
so its `tool` is the provider's.

A provider that fronts an external tool SHALL obtain `tool.version` by
querying the executable it will run, each time it builds its descriptor; a
pure in-process engine's provider takes `version` and `build` from the
producer's build. A
producer builds its registry, and so every descriptor, afresh at each
invocation; no external tool's `tool.version` is fixed at compile time or
kept from one invocation to the next. The FR-290 tool probe of a backend
that fronts an external tool, after routing and before each run, SHALL read
the version of the executable it is about to run; when that version differs
from the descriptor's `tool.version`, the probe fails and the item settles
`unsupported` with `unsupported_projection`/`tool-unavailable`, whose
`detail` names the capability kind, the backend, the descriptor's version and
the probed version. The code that runs therefore always has the version the
descriptor, its `ManifestDigest` and the FR-306 cache key carry.

## Wire schema

The normative wire of this value is the closed JSON schema
[`proposals/backend-provider-v1/schema.json`](../../../proposals/backend-provider-v1/schema.json)
(`urn:quire:backend-provider:v1`), with its positive and adverse vectors in
[`vectors.json`](../../../proposals/backend-provider-v1/vectors.json). Each
schema member names its owning requirement. A reader converts from this wire
into its own types (QSL ADR-013 T-7); no shared Rust crate carries these
types, and no consumer holds a copy of the schema. The schema references the
checked-package schema for `PackageId`, `NodeId`, `Occurrence`, `Rational`
and decimal strings.

### Stages

One envelope has three stages. Each is a closed object carrying
`contract_version` and `capability_vocabulary`:

| Stage | Members | FR-462 frame |
| --- | --- | --- |
| manifest | `manifest`, with exactly one descriptor: the provider's own, carrying `tool` | `hello` body |
| request | `manifest` (the registry snapshot the candidates are checked against) and `request` | `request` body |
| outcome | `dispositions`, `artifacts`, `results`, `counterexamples`, `accounting` | `result` body |

### Package bytes

`request.package` SHALL be the RFC 8785 byte string of the
`quire.checked-package/v2` document
([FR-322](FR-322-checked-package-artifact.md)), spelled as RFC 4648 section 5
base64url with no padding and zero unused trailing bits, so that one byte
string has exactly one spelling and the frame keeps one RFC 8785 encoding.
`request.package_id` SHALL be that package's `package_id`. A reader decodes
`package`, admits it with the v2 reader and refuses a recomputed `package_id`
that differs from `request.package_id` with
`stale_dependency`/`content-mismatch`, before any item runs. A padded or
standard-alphabet spelling, nonzero trailing bits, a length of 1 modulo 4, or
a `package` that is a JSON value rather than a string refuses before
consumption. The bytes travel as bytes, not as an embedded JSON value,
because the v2 reader admits a byte string: it checks canonical bytes and
byte limits itself, and FR-305's IR reader revalidates the `package_id` of
exactly the bytes it was given.

`request.dependencies` carries, in the same spelling, the v2 bytes of each
package in the proved package's recursively derived dependency closure,
including transitive-only packages, with each package's `package_id`,
ordered by `package_id` digest. `request.provision` is the byte provision
FR-364 and FR-368 read: each document with its `sha256-jcs` digest, ordered
by digest, each digest once. `request.deadline_ms` is the provider deadline
in milliseconds (FR-462).

### Items

- `request_index` is the item's zero-based position. Dispositions, results,
  counterexamples, artifacts and accounting records join on it; every item
  has exactly one disposition and one accounting record.
- `claim` names the requested claim by its checked-package node and
  occurrence `{node, occurrence}`. The same pair is the `locus` of every
  `where` and payload in this envelope; the package source map gives its
  regions.
- `extent` is `{classification: "bounded", bounds}`, `bounds` the
  substituted `ProofBound`s (empty for an item whose domains are all finite),
  or `{classification: "unbounded", domains, finite_bound_available}`, each
  domain a `DomainKey` with its domain kind, as the boundedness architecture
  (quire-spec-language#222, QSL ADR-014 §4) defines them: a `DomainKey` is
  `{type: "node", node, path}` or `{type: "population", member_type,
  ordinal}` (`path` entries and `ordinal` unsigned 32-bit), and a
  `FiniteBound` is one of ADR-014's three substitution kinds:
  `cardinality{maximum}`, `integer-range{lower, upper}` or `depth{maximum}`
  with `maximum` non-zero. QSL FR-358's `variants` is a harness bound, never
  a B-4 substitution kind, so it is not an `extent` bound.
- Every `ProofBound`, in `extent.bounds` and in a temporal subject's
  `proof_bounds`, is `{domain, kind, bound}`: `kind` is the bounded domain's
  ADR-014 domain kind, from the same vocabulary as an unbounded extent's
  `domains[].kind` (`collection`, `population`, `integer`, `recursive`,
  `quantity`, `loop`, `infinite-trace`), written by QSL from its own
  classification. Only a boundable kind takes a bound, and the bound matches
  it: `cardinality` for `collection` and `population`, `integer-range` for
  `integer`, `depth` for `recursive`; any other pairing, a non-boundable kind
  and an absent `kind` refuse before consumption. FR-290's domain-kind check
  compares these kinds, and the boundable kinds among an unbounded extent's
  `domains[].kind`, with the single candidate's manifest `domains`, so
  negotiation never reclassifies the package; a non-boundable kind is never
  compared. A bounded extent with empty `bounds` has only finite domains, so
  it has no kinds and passes that check.
- A temporal item's `subject.proof_bounds` are the proof-bound substitutions
  for the subject's own populations and are independent of `extent`. A
  substitution for the subject may be supplied while the item's `extent`
  stays `unbounded` (an `infinite-trace` domain, for instance): the subject
  bound bounds the populations the trace is read over, and does not bound the
  extent, so it moves no entry into `extent.bounds` and changes neither the
  extent's classification nor its `finite_bound_available`. The checked
  requirement carries no proof bounds; the request's caller supplies them
  (ADR-014 B-4), and a consumer that produces the request owns the
  substitution and its kind.
- `candidates` is `{type: "set", backends}` or
  `{type: "unknown-backend", backend}`.
- `options` is a map of backend option names to string values (FR-196).
- `requirements` is the nonempty, repeat-free set of the item's routing requirements, from the closed vocabulary {`hybrid-dynamics`}, carried only by a `temporal-satisfaction` item and by every item that carries `hybrid` (FR-193 "Routing a hybrid item"); an item that breaks either rule refuses before consumption.
- An item with an absent `kind`, a `kind` outside the vocabulary, an absent
  `extent`, or an absent or unknown probabilistic evidence kind is admitted on
  the wire, because FR-290 settles each such item `invalid-request` on its
  own (`absent-kind`, `unknown-kind`, `absent-extent`,
  `absent-evidence-kind`, `unknown-evidence-kind`). A present `extent` is
  closed: an unknown classification, bound kind or domain kind, a
  `ProofBound` without `kind`, or an unbounded extent
  without `finite_bound_available`, refuses before consumption, as does an
  item without `candidates` (FR-331-AC-4).

A manifest descriptor's `domains` lists the domain kinds (`collection`,
`population`, `integer`, `recursive`) for which the backend accepts a finite
bound. FR-290 owns which descriptors carry it and how a missing or malformed
`domains` is refused; the manifest schema enforces that rule. A descriptor's `options` are its published option defaults; its `bounds` are the
published default of each limit it applies, keyed by limit name, which an
item's absent limit takes (FR-461).

### Hybrid solver budgets

A hybrid item (FR-193) carries its budgets in `hybrid.limits`: `max_solver_steps`,
a whole positive count of solver steps as the provider counts them, and
`max_solver_time`, whole seconds of wall-clock time, each a `PositiveCount`; `hybrid` and its `limits` are never empty. Their
setting names are `hybrid.max_solver_steps` and `hybrid.max_solver_time`, and
their defaults are the descriptor's `bounds` keyed by limit name (FR-461). A
solver that exhausts the step budget records `incomplete` with
`limit-reached{limit, value, setting}` whose `limit` is `max_solver_steps`
and whose `setting` is `hybrid.max_solver_steps`. A solver that exhausts
`max_solver_time`, and a provider that overruns `deadline_ms` (FR-462),
record `incomplete` with `timed-out`, never `limit-reached`; a run the
caller's frontend ends at its own deadline records `cancelled{source:
deadline}`. The request writer rounds a fractional time budget up to the next
whole second before writing it; a fractional value on the wire is refused
before consumption. `max_solver_steps` is part of request identity;
`max_solver_time` is a wall-clock setting and is outside it (FR-306). A
descriptor whose `dynamics` holds `hybrid` carries both limits in its
`bounds`; one that lacks either refuses before consumption.

### Result entries

A result entry carries `request_index`, `backend` (the identity its manifest
names, FR-462), `kind` and `result`, and:

- `certification` — on `proved` only, `certified`, `uncertified` or
  `trusted` ("Certification" above);
- `cross_check` — on an SMT `proved` only, when its cross-check ran,
  `{tool, verdict}` ("Certification" above);
- `basis` — on `proved` only, the proof basis as an object whose `type` is
  the spelling and whose other members are its payload, for example
  `{"type": "value-bounds", "lower": …, "upper": …, "method": …, "reductions": []}`;
- `cause` — the closed cause as an object of the same form. The
  undefined-evaluation cause is FR-364's envelope
  `{"type": "undefined-evaluation", "where": …, "cause": <reason>}`; its
  payload has a member named `cause`, which is why every payload is nested in
  the `cause` object rather than spread over the entry;
- `diagnostic` — an [FR-272](../foundation/FR-272-typed-diagnostic-cause.md)
  typed cause `{code, cause, detail}`, `detail` keyed by FR-272 payload
  names;
- `certificate` (FR-412 for `exact-value` and `value-bounds`, FR-418 for
  `zone-certified`), `evidence` (FR-419 for `closed-form`, the FR-393 witness
  arm for `well-definedness-unchecked`), `verdict` (FR-410 for `measured`) and
  `counterexample` (the position of the item's `counterexamples` entry, for
  `refuted`).

| Result | Carries |
| --- | --- |
| `proved` | `basis` and `certification`, and `cross_check` when FR-349's cross-check ran; no `cause` or `diagnostic` |
| `refuted` | `counterexample`; `cause` only for `undefined-evaluation` |
| `tested` | nothing further |
| `measured` | `verdict`; no `basis`, `cause` or `diagnostic` (FR-409) |
| `inconclusive` | `cause` |
| `unsupported` | exactly one of `cause` and `diagnostic` |
| `declined` | `cause` (`refused`, `invalid-input` or `incomplete-input`) and `diagnostic` |
| `incomplete` | `cause` (`limit-reached`, `precision-budget`, `cancelled{source}` or `timed-out`) |
| `failed` | exactly one of `cause` (`tool-failure{defect}`, with `limit` and `value` for `frame-length` and `total-bytes`, FR-462) and `diagnostic` |

The closed proof bases are those of FR-363 (`exhaustive`,
`bounded-complete{depth}`, `inductive{depth}`), FR-385
(`reduced{reductions}`, each reduction `{reduction, parameters, row}`), the
table above, FR-391 (`witnessed{sources}`, the FR-393
witness source of each initial state) and the Kani basis
`bounded-proof{checks}`. The closed inconclusive causes are those of the
table above and of FR-363 (`bound-reached{depth}`,
`induction-not-closed{depth}`), FR-385, FR-399 (`match-undetermined`,
`vacuous-match`, `replay-refused`), FR-379
(`mapping-undetermined{row, state_key, prefix}`), FR-468
(`hypotheses-not-established{hypothesis, result}`, `hypothesis` the invariant
clause's node and `result` the result of its unproved item), FR-380
(`transfer-not-preserving{reason}` with `reason` `fragment` and its
`operator`, `fairness` and its `constraint`, or `stutter-closure`;
`premise-unproved{premise, result}`, `premise` the premise item's
`request_index`), FR-439 (`memory-bound-reached{limit, value, states}`, `limit` the request
member that raises the budget, `max_store_buffer` or `max_messages`), FR-428
(`instance-bound-reached{limit, value, states}`, `limit` always
`max_live_instances`, the member that raises it) and AD-016's
`kani_vacuous_proof`. FR-439 adds the unsupported cause
`weak-access-shape{site, transition, shape}`. FR-427 adds the unsupported cause
`recovery-view{predicate}`, `predicate` the qualified name of the compensation
template whose recovery predicate reads its recovery binder.

A reader SHALL refuse a result entry, or the `counterexamples` entry a
`refuted` result names, before consumption with the FR-272 `invalid_result`
cause that names its first defect in the reading order below, retaining the
result item index and the offending member's RFC 6901 pointer relative to
that entry (the empty pointer names the whole entry).

A plugin's entry (a backend loaded as a plugin, FR-305) carries a subset of
the closed values. FR-305 gives a plugin's `proved` result no in-core
certificate checker and makes its counterexample canonical assignments, so:

- A **core-only basis** is a proof basis whose settlement is a core check or
  the core's own prove path: `bounded-proof` (the Kani check count, AD-016),
  `closed-form` (evidence the core recomputes, FR-419), `witnessed` (a witness
  the core replayed, FR-391), and `exact-value`, `value-bounds` and
  `zone-certified` (certificates a core checker accepted, FR-412, FR-418). A
  plugin's entry carries none.
- A **core-only cause** is an inconclusive cause that only the core's own
  replay or Kani run settles: `replay-parity` (AD-016 arrow 7), `replay-refused`
  (FR-399), `stale-dependency` and `kani_vacuous_proof` (AD-016). A plugin's
  entry carries none.
- A plugin's `counterexamples` entry carries a `counterexample` whose `type`
  is `assignments`, or `hybrid-counterexample` on the entry of a hybrid
  item (FR-193 "Solver result on the wire"), and no `transcript`. A
  `hybrid-counterexample` settles the item only after the reader's exact
  replay (FR-193 "Trace replay"); on the entry of an item that is not a
  hybrid claim it is refused `invalid-value` at `/counterexample/type`.
- An `insufficient-horizon` cause belongs to a hybrid item (FR-193 "Proof
  coverage"); on the entry of an item that is not a hybrid claim, from any
  provider, it is refused `invalid-value` at `/cause/type`.

`stale-dependency{requested, recompiled}` settles an item in exactly these
cases: a model-check counterexample replay or a state-graph evidence replay
whose recompiled package's own `package_id` differs from the one the replay
request names; and a state-graph certificate check (QSL FR-169), an EN-1
closure certificate check (QSL FR-338) or an EN-1 component certificate
check (QSL FR-339) whose recompiled `package_id` differs from the one the
certificate names. Each of these certificates names the `package_id` of the
package it was produced over, and its checker compares it with the
recompiled one before any other rule, so a stale certificate settles
`stale-dependency`, never `certificate-rejected`. A component certificate
names its `package_id` once, for its closure and its components. `requested` is that
named `package_id` and `recompiled` the recompiled one, each a
`quire.package.semantic/v2` `PackageId`, so the record names both
identities. A stale dependency's own `package_id`, and a stale
`package_id` in a scalar-parity or plugin replay, settle `replay-refused`.

This table lists the same closed causes as FR-272's `invalid_result` row:

| Cause | Defect | Pointer and payload |
| --- | --- | --- |
| `missing-member` | A member the entry's result or spelling requires is absent, such as a `proved` entry without `certification` | The first absent member in member-name order (RFC 8785) |
| `unknown-member` | A member the entry's result or spelling does not define is present, such as a `certification` on a result other than `proved` | The first such member in member-name order |
| `wrong-value-kind` | A member's JSON kind is not the kind its domain takes, such as a `certification` that is not a string, or an entry that is not an object | The member |
| `invalid-value` | A member of the right JSON kind lies outside its domain here, such as a `certification` other than `certified`, `uncertified` and `trusted`, a count outside 0 to 2^53 − 1, or a plugin's counterexample `type` other than `assignments` and, on a hybrid item, `hybrid-counterexample` | The member |
| `certification-mismatch` | A `proved` entry's `certification` is one of the three values but not the one the Certification table fixes for that proof | `/certification`; `expected` the fixed value, when there is one, and `actual` the received one |
| `unknown-cause` | A result cause or proof basis spelling outside the closed set ("Wire spellings of result causes and proof bases"), or outside the subset its backend may send: a core-only basis or core-only cause in a plugin's entry | The `type` member; the received spelling |
| `measured-with-truth` | A `measured` entry carries a truth or a basis (FR-409) | The truth or `basis` member |

The reader reads an entry in this order and reports the first defect:

1. The entry is not an object: `wrong-value-kind` at `""`.
2. A result entry's `result` is absent (`missing-member`), not a string
   (`wrong-value-kind`) or not an FR-331 result (`invalid-value`).
3. A `measured` entry carries a truth or a basis: `measured-with-truth`.
4. A member the entry requires is absent: `missing-member`.
5. A member the entry does not define is present: `unknown-member`. A
   `transcript` in a plugin's `counterexamples` entry is one.
6. A `proved` entry's `certification` is not a string
   (`wrong-value-kind`), is not one of the three values (`invalid-value`),
   or is the wrong one of them (`certification-mismatch`).
7. The `basis` or `cause`, then each remaining member in member-name order,
   is read as a value.

A value is read in this order: its JSON kind (`wrong-value-kind`); for an
object with a `type` member, that member's absence (`missing-member`), kind
(`wrong-value-kind`) and spelling (`unknown-cause` for a cause or basis,
`invalid-value` otherwise); then the object's absent required members
(`missing-member`) and undefined members (`unknown-member`), each first in
member-name order; then each member's value in member-name order, by the same
rule; and for a scalar, its domain (`invalid-value`). So a `refuted` entry
carrying `certification` `7` is refused `unknown-member` at
`/certification`, and a plugin's `proved` entry carrying `certified` and a
core-only basis is refused `certification-mismatch`.

The Certification table fixes one value from the entry's `basis` and
`backend` for these proofs: `trusted` for a proof from a backend loaded as a
plugin (FR-305); `uncertified` for a `bounded-solver` or `inductive-solver` proof and a `reduced`
proof with a `symmetry` or `partial-order` reduction; and `certified` for a
Kani `bounded-proof` and an `exact-value`, `value-bounds`, `zone-certified`,
`closed-form`, `witnessed` or copy-swap-only `reduced` proof. That value is
`expected`. An `exhaustive`, `bounded-complete` or `inductive` proof from a
first-party backend is `certified` or `uncertified` according to a
certificate check or state constraint that the entry does not show, so the
reader admits either; it carries `trusted` only as a `certification-mismatch`
with no `expected`.

A value inside the subset its backend may send that a reader's own types
cannot hold is not an invalid result. The reader refuses it before consumption with
`unknown_required_feature`/`unsupported-feature`, the feature being the
received spelling and the scope the reader, at the point in the reading order
where it reads that value.

### Kani results

A Kani item settles exactly one result: from its AD-016 `KaniOutcomeKind`,
or from a failure around the run:

| Kind | Result | Cause or basis |
| --- | --- | --- |
| `Proved`, one or more SUCCESS checks | `proved` | basis `bounded-proof{checks}`, the SUCCESS count; certification `certified` |
| `Proved`, zero SUCCESS checks | `inconclusive` | cause `kani_vacuous_proof` |
| `Counterexample` | `refuted` | its `counterexamples` entry names the transcript artifact |
| `Refused`, `InvalidInput`, `IncompleteInput` | `declined` | cause `refused`, `invalid-input` or `incomplete-input`, with the run's typed `diagnostic` |
| `Unavailable` | `unsupported` | diagnostic `unsupported_projection`/`tool-unavailable`, `detail` naming `kind`, `backend` and the `absent` solver or backend |
| `TimedOut` | `incomplete` | cause `timed-out`, the backend's own time limit; a caller's deadline is `cancelled{source}` with `source` `deadline` |
| `ResourceExhausted` | `incomplete` | cause `limit-reached{limit, value, setting}`, the setting `kani.<limit>` |
| `Cancelled` | `incomplete` | cause `cancelled{source}` |
| `Inconclusive` | `inconclusive` | its typed cause, such as `replay-parity` (a replay disagreement at AD-016 arrow 7) |
| a backend success that is not proof (AD-016: `tested` keeps its value) | `tested` | none; never proof evidence |
| a plugin-wire failure (FR-462) | `failed` | cause `tool-failure{defect}`, with `limit` and `value` for `frame-length` and `total-bytes` |
| the tool becomes unavailable after a passing probe (FR-290) | `failed` | diagnostic `unsupported_projection`/`tool-unavailable` naming `kind` and `backend` |
| an IR invariant breaks while mapping (AD-016) | `failed` | diagnostic `runtime_invariant`/`established-invariant-broken` |

The Kani engine name in FR-461 setting names is `kani`; the exact
probabilistic engine's is `exact_prob`.

### Artifacts, counterexamples and accounting

- An artifact is `{request_index, kind, identity, bytes, source_map}`:
  `kind` names what it is, such as `kani-harness` or `kani-transcript`;
  `identity` is the producer's name for it, derived from the claim identity
  (FR-196) and unique per item and kind; `bytes` uses the package spelling;
  each `source_map` entry maps the byte range `[start, end)` to a node
  occurrence. Artifacts and counterexamples are named by their zero-based
  position.
- A `counterexamples` entry is `{request_index, backend}` with exactly one of
  `transcript`, the position of the transcript artifact, and
  `counterexample`, the canonical counterexample: FR-197 canonical
  assignments `{type: "assignments", values}`, one FR-181 typed value per
  parameter node; an FR-364 `TemporalCounterexample`, whose timed steps carry
  `delay` and which carries `final_delay` for kind `time-lock` (FR-417) and
  `refinement_failure` for a refinement item (FR-379); an FR-413
  `ProbabilisticCounterexample`; an FR-393 `GraphEvidence`; or an FR-400
  `HyperCounterexample`. `backend` and `request_index` are the originating
  run identity.
- An accounting record is the item's terminal record:
  `{request_index, settled_by}`, `settled_by` `disposition` when its
  disposition settles it and no result entry exists, or `result` when its one
  result entry does. The proof basis, depth and cause FR-363 and FR-391 put in
  the terminal record are the ones in that result entry; the accounting
  record adds the run's `method` (`exploration`, `reduced-exploration` or
  `induction`), `max_depth`, `seed`, `witness_samples`, the effective
  `limits` it ran under, the limits it `reached`, its state `constraint`,
  `warnings` (FR-369 `fairness.strong-would-exclude`,
  `memory.load-buffering-shape{load, store}` under FR-439 (each member a
  transition identity), and `cross-check-absent{tool}` for FR-349's absent cross-check) and the FR-196
  `tool` record (name, version, target, solver, bounds, options).
- An `unsupported` or `invalid-request` disposition carries its FR-272
  `diagnostic`; the warning an unsupported disposition shows is that
  diagnostic.
- Where FR-410, FR-412 and FR-418 state a member's content without a
  spelling, the schema's spelling is normative: a statistical verdict's
  per-test provenance carries `first_trace_index` and `last_trace_index`, its
  `estimate` is `{value, quantile}` with `quantile` only for a quantile
  claim, and a sampled-witness step carries `scheduled`, `random` and `delay`;
  a probability certificate keys `values`, `ranking`, `policy` and each
  component's `bias` by product state key; a zone certificate's DBM entry is
  `{bound, strict}` or `"infinity"`, and its Büchi extension is `components`
  (a component index per node), `reasons` (per component) and `divergence`
  (per node, the time-divergent target node).

## Identity and validation

Request identity includes the capability vocabulary, manifest, package, each item's claim, extent with its proof bounds, options and limits, each item's kind, extent classification, `requirements`, named backend and candidates, each temporal item's subject, item kind, fairness set, `max_depth`, `limits` and `smt_encoding` (FR-368), the `witness_samples` and `seed` of a state-graph item, and each item's symmetry declarations, partial-order selection and state constraint; the state constraint is also recorded in the item's terminal record. A different temporal horizon or deterministic budget can produce a different result, so a result is reused only for a request with the same request identity; a wall-clock setting, a hybrid item's `hybrid.limits.max_solver_time` or a temporal item's `temporal.limits.max_solver_time` (FR-368 "SMT solver budgets"), is outside it. Result identity includes artifacts and results.
Unknown contract versions, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption; in a result entry or a `counterexamples` entry, a missing or unknown member and a value of the wrong kind or outside its domain refuse with the `invalid_result` causes of "Result entries". A `capability_vocabulary` other than `quire.capability-kind/v1`, or none, refuses the envelope as `invalid_capability`/`unknown-vocabulary`.

A backend's manifest identity, its `ManifestDigest`, is the SHA-256 digest in
the FR-201 domain `quire.tool-manifest.jcs/v1` over the RFC 8785 bytes of its
complete admitted descriptor: for a process provider, the descriptor at
`/manifest/0` of its `hello` body; for a linked or in-process provider, the
descriptor its manifest gives. Every member the descriptor carries, `tool`
included, is in that preimage, and the enclosing array, the stage, the
registry snapshot and the request are not. The backend identity, the
`BackendId`, is the `backend` member alone, so a change to any other member,
`tool.version` included, changes the `ManifestDigest` and leaves the
`BackendId` unchanged.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-331-AC-1 | Negotiation completes before artifact emission. | Analysis |
| FR-331-AC-2 | An item settled `unsupported`, `requires-bound` or `invalid-request` emits no artifact; a tool-absence item keeps the artifacts it emitted before the probe, and they carry no verdict. | Analysis |
| FR-331-AC-3 | Every returned counterexample replays through the native owner with the same verdict. | Analysis |
| FR-331-AC-4 | `negotiate_*` settles each item from its `candidates` and extent classification under FR-290: unknown-backend, inconsistent candidates, an absent extent classification, and more than one candidate with no named backend settle `invalid-request`; an empty set settles `unsupported` with a warning naming the item's kind; a single candidate settles through its negotiation arm. A request item without `candidates` refuses before consumption. | Test (TC-271) |
| FR-331-AC-5 | Two requests whose items differ only in `candidates` have different request identities, and candidates serialize ordered by identity under every registration order. | Test (TC-271) |
| FR-331-AC-6 | A tool-absence `unsupported` result keeps the item's `supported` disposition, is the item's one terminal accounting record, and names the capability kind and backend. | Test (TC-271) |
| FR-331-AC-7 | An envelope whose `capability_vocabulary` is not exactly `quire.capability-kind/v1`, or is absent, refuses as `invalid_capability`/`unknown-vocabulary` with none of its kinds read. | Test (TC-271) |
| FR-331-AC-8 | A Kani proof run whose obligation has zero SUCCESS checks records the item's result as `inconclusive` with a typed vacuity cause, never as `proved`, and this cause is not a new result value. | Test (TC-274) |
| FR-331-AC-9 | A `counterexamples` entry names exactly one of the transcript's `artifacts` entry or the canonical assignments of a counterexample with no transcript, never both; a transcript-backed entry's assignments are its decode of that entry and are not separately stored. | Test (TC-275) |
| FR-331-AC-10 | A supported item whose Kani run ends `Refused`, `InvalidInput` or `IncompleteInput` records exactly one terminal result `declined` with a typed refusal cause; a supported item whose Kani run ends `Unavailable` records exactly one terminal result `unsupported` with a typed unavailability cause naming the capability kind, the backend and the absent solver or backend identity. | Test (TC-276) |
| FR-331-AC-11 | A temporal item over a model subject carries `subject`, `item_kind`, `fairness`, `max_depth` for a bounded search and `limits`, and its terminal record carries its proof basis or cause; two requests differing only in `subject`, `item_kind`, `fairness`, `max_depth` or one `limits` member have different request identities, and a result recorded for one is not reused for the other. | Test (TC-302) |
| FR-331-AC-13 | A completed statistical item records `measured` with its decision and `StatisticalVerdict` and no truth or basis; no `measured` result is counted as `proved`. | Test (TC-358) |
| FR-331-AC-14 | A `probabilistic-satisfaction` request item carries exactly one evidence kind with the settings and limits of that kind, and each limit it omits takes its published default. | Test (TC-353) |
| FR-331-AC-15 | A `proved` result with proof basis `exact-value`, `value-bounds` or `zone-certified` carries its certificate, and one with `closed-form` its evidence; each is recorded only after the checker accepts it. | Test (TC-363, TC-371, TC-372) |
| FR-331-AC-16 | Each proof basis and cause of a probabilistic, timed or real-time result is emitted with exactly its spelling in the wire-spelling table, and a result carrying another spelling is refused `invalid_result`/`unknown-cause`. | Test (TC-358) |
| FR-331-AC-17 | A state-graph item's `temporal` member carries `witness_samples` and `seed` and its `limits` carry `max_walk_steps`, each taking the provider's published default when the request sets none; two requests differing only in `seed`, `witness_samples` or `max_walk_steps` have different request identities; a hyper item's `limits` carry `max_witness_set` and `max_relation_tuples`. | Test (TC-336, TC-339) |
| FR-331-AC-18 | A request's `package` is the unpadded base64url spelling of the RFC 8785 bytes of its checked package and decodes to exactly those bytes, whose `package_id` equals the request's `package_id`; a padded or standard-alphabet spelling, nonzero trailing bits, a length of 1 modulo 4, an absent `package` or `package_id`, a `package` that is a JSON value, and a `package_id` outside `quire.package.semantic/v2` each refuse before consumption. | Test (TC-431) |
| FR-331-AC-19 | Every positive vector of `proposals/backend-provider-v1/vectors.json` is admitted by its schema, each owned member it lists exists, and every adverse vector, a single-member change of a positive one, is refused: an unknown contract version or vocabulary, an extra or missing member, an out-of-vocabulary value, a cause or basis outside its result's closed set, and a counterexample entry naming both or neither arm. | Test (TC-431) |
| FR-331-AC-20 | The Kani vectors carry an admitted result entry for each of the Kani results table's thirteen rows, each `declined` cause separately, so together they hold each of the eight results a Kani item can settle; changing a row's cause or diagnostic to one outside its result's closed set is refused. | Test (TC-431) |
| FR-331-AC-21 | Every `ProofBound` carries the `kind` of the domain it bounds, from the ADR-014 domain-kind vocabulary shared with an unbounded extent's `domains`, and a bound matching that kind; a `ProofBound` without `kind`, with a kind outside the vocabulary, with a non-boundable kind or with a bound of another kind refuses before consumption: of the seven kinds and three bound types, exactly the pairs `collection` and `population` with `cardinality`, `integer` with `integer-range` and `recursive` with `depth` are admitted. | Test (TC-431) |
| FR-331-AC-22 | The manifest schema applies FR-290's `domains` registration rule to every descriptor: one whose `domains` is present and empty, holds a non-boundable or unknown kind or repeats a kind is refused whatever modes it advertises, one that advertises a `bounded` mode without `domains` is refused, and one that advertises only `unbounded` modes is admitted without `domains`. | Test (TC-431) |
| FR-331-AC-23 | Every `proved` result entry carries exactly one `certification`, `certified`, `uncertified` or `trusted`: a Kani `bounded-proof`, a checked `value-bounds` and a checked `witnessed` proof, and an SMT `bounded-complete` proof whose certificate the core verified, carry `certified`, a symmetry-`reduced` proof (copy-swap with symmetry), a partial-order-`reduced` proof and an SMT `inductive` proof with no verified certificate `uncertified`, and a plugin proof `trusted`; a `proved` entry without `certification`, a `reduced` proof with a `symmetry` or `partial-order` reduction or a `bounded-solver` proof carrying `certified`, one with another value, and a `certification` on any other result are refused. | Test (TC-431) |
| FR-331-AC-24 | A `certificate-rejected` cause is admitted with each locus form of the Certification table: each query part, `unrolling`, `base`, `step` and `step/<r>` with `r` `"2"`, `"6"` or `"18446744073709551615"`; a proof step `{part, index}` with index `"0"`, `"41"` or `"18446744073709551615"`, and one whose part is `step/4`; a product state; a hyper product state with and without its automaton key and with a witness set; a state, and under `malformed` a state record in another FR-201 domain such as `package-fingerprint`; a state-graph entry; each simulation-step form, with `null` `edge` and `verdict`, with an `edge` and `null` `verdict`, and `step-fails` carrying each verdict kind; a zone node, edge and component; and a certificate member. An `at` that is an unknown part, an empty string or an empty object; a query part `step/`, `step/02`, `step/18446744073709551616`, `unrolling/2` or `base/0`; a proof step with an unknown part, a part `step/-1`, without `part` or `index`, with another member, or whose `index` is a JSON number, `"-1"`, `"18446744073709551616"` or `"041"`; a product state whose automaton key is padded or whose model state is a bare `state` member; a hyper product state with no states; a state whose `digest` is not a `StateKey`, whose `domain` is absent or empty, or that is a bare `state` member; a state record in another domain under any rule but `malformed`; an entry whose index is a JSON number; a simulation step without its `null` `edge` or `verdict`, a `post-differs` with a `null` `edge` and a `step-fails` with a `null` `verdict`; a negative zone node; an unknown member; and a `step-fails` verdict that is another inconclusive cause, a `type`-tagged cause, a JSON-number position, an `Undefined` failure, an `Undetermined` verdict with another cause spelling or an incomplete evaluation without `limit`, or an undefined evaluation without `state_key`, are each refused. Across every digit position of `18446744073709551615`, and lengths up to 21 digits, an `index` is admitted exactly when it is the canonical decimal spelling of an unsigned 64-bit integer. | Test (TC-431) |
| FR-331-AC-25 | A `certificate-rejected` cause is admitted with each rule at a locus listed with it, `not-refutation` and `malformed` at a query part included. An independent finite inventory enumerates all 24 owned rule spellings and equals both the schema's rule set and the positive-vector rule set; removing a rule from both fails that comparison. A rule outside the closed set, such as `state-missing`; a rule written as its variant name, `QueryMismatch`; and a rule at a locus not listed with it are each refused: `query-mismatch` at a proof step, `proof-step-invalid` at a query part, `bad-state` at a hyper product state, `witness-fails` at a state, `rank-broken` at a zone node, `bad-state` at a zone component, `rank-missing` at a certificate member, `bad-state` at a state-graph entry, `initial-mismatch` with an `edge`, `post-differs` without an `edge`, and `step-fails` without a `verdict`. | Test (TC-431) |
| FR-331-AC-26 | A reader consuming a `certificate-rejected` cause binds it to the request's admitted checked claim and the checker that claim's classified form resolves, and refuses before consumption a rejection whose rule and locus do not belong to that checker, such as an `inequality-fails` cause on an EN-1 temporal claim or a model-only `successor-missing` for the FR-338 product-state checker, and one whose request, claim or checker context is absent, conflicting or ambiguous. A rejection whose rule and locus belong to the resolved checker is consumed as that certificate rejection and keeps its rule and locus. The global schema admits each such pair on its shape alone, so the schema run does not discharge this criterion. | Inspection (TC-431) |
| FR-331-AC-27 | The schema keeps the identities the checkers name distinct: an HP-3 key for zero universal variables carries a present empty `states` tuple with both empty and nonempty witness sets, and absent `states` or an absent HP-3 `witness_set` is refused; an HP-1 failing transition tuple carries every edge's pre-state, transition identity, post-state and result in binding order, and a missing edge member, a protocol transition and a state-only projection are refused; an FR-412 rejection and each keyed certificate member carry the complete probabilistic product key, so two product states with equal model states and different monitor, accumulator or pending components stay distinct. | Test (TC-431, TC-363) |
| FR-331-AC-28 | A model-subject item with no initial state, and one with zero instances, each record `inconclusive` with the cause `no-initial-state` and `no-instance` respectively, spelled exactly so; the temporal-outcome vector carries a result entry of each, and a `no-instances` spelling is refused. | Test (TC-431) |
| FR-331-AC-29 | A hybrid item's solver that exhausts `hybrid.limits.max_solver_steps` records `incomplete` with `limit-reached` whose `limit` is `max_solver_steps` and whose `setting` is `hybrid.max_solver_steps`; one that exhausts `max_solver_time` records `incomplete` with `timed-out`; a `limit-reached{limit: max_solver_time}` or a `time-limit` cause on a hybrid item is refused `invalid_result`/`unknown-cause`; a fractional `max_solver_time`, a zero budget, an unknown `hybrid.limits` member and an empty `hybrid` or `limits` are refused before consumption; two requests differing only in `max_solver_time` have equal request identity. | Test (TC-373) |
| FR-331-AC-30 | The descriptor schema admits `memory_models` as a nonempty duplicate-free subset of `sc`, `tso` and `ra` on a descriptor that advertises `temporal-satisfaction`, and refuses an empty `memory_models`, a repeated or unknown model, and `memory_models` on a descriptor that does not advertise `temporal-satisfaction`. | Test (TC-431) |
| FR-331-AC-31 | An accounting record whose `warnings` hold `{code: memory.load-buffering-shape, load, store}` with `load` and `store` transition identities is admitted; one missing `store`, one with an extra member and one with a `load` that is not a transition identity are refused. | Test (TC-431) |
| FR-331-AC-32 | A temporal request item whose `extent` is `unbounded` with an `infinite-trace` domain, `finite_bound_available` false and no `bounds`, and whose `subject.proof_bounds` holds a `population` `cardinality` bound, is admitted by the request schema; the same item with `subject.proof_bounds` empty is also admitted. That the bound moves nothing into `extent.bounds` and changes neither the classification nor `finite_bound_available` is an obligation of the request's producer, which this wire criterion does not check. | Test (TC-431) |
| FR-331-AC-33 | An in-process request holds the manifest, `deadline_ms`, items, checked package and subject binding as typed values; its results carry the members a process provider's do; two in-process requests of the same items run the engine twice and the second result comes from the second run; an item of a process provider over a checked package that `package` refuses to emit settles `unsupported` with `unsupported_projection`/`unsupported-requested-capability` naming the kind, the backend and that the package is not emitted, with no frame written; and a request frame of the same items without `package` and `package_id` refuses before consumption. | Test (TC-220) |
| FR-331-AC-34 | A reader refuses each of these before consumption with the `invalid_result` cause given, naming the pointer given: `certification-missing`, `missing-member` at `/certification`; `certification-on-refuted`, `unknown-member` at `/certification`; `certification-other-value`, `invalid-value` at `/certification`; a `proved` entry whose `certification` is `7`, `wrong-value-kind` at `/certification`; `certification-reduced-certified`, `certification-bounded-solver-certified` and `certification-symmetry-certified`, `certification-mismatch` at `/certification` with `expected` `uncertified` and `actual` `certified`; a plugin's `proved` entry carrying `certified`, `certification-mismatch` with `expected` `trusted`, also when its basis is core-only; a `refuted` entry carrying `certification` `7`, `unknown-member` at `/certification`; a plugin's `proved` entry carrying `trusted` and a `closed-form` basis, `unknown-cause` at `/basis/type`; a plugin's `inconclusive` entry with cause `replay-parity`, `unknown-cause` at `/cause/type`; a plugin's `inconclusive` entry with cause `stale-dependency`, `unknown-cause` at `/cause/type`; a plugin's `counterexamples` entry whose `counterexample` is a `TemporalCounterexample`, `invalid-value` at `/counterexample/type`; a plugin's `hybrid-counterexample` on a hybrid item admitted and passed to FR-193 exact replay, and the same on an FR-468 refinement item, `invalid-value` at `/counterexample/type`; and a plugin's `counterexamples` entry carrying canonical assignments and a `transcript`, `unknown-member` at `/transcript`. A first-party `exhaustive` proof carrying `uncertified` is admitted. A plugin's proof basis or inconclusive cause that a plugin may send but that is outside the reader's types is refused `unknown_required_feature`/`unsupported-feature`, not `invalid_result`. The schema refuses each vector on its shape alone, so the schema run does not discharge the cause. | Inspection (TC-431) |
| FR-331-AC-35 | An `inconclusive` entry whose cause is `stale-dependency` with `requested` and `recompiled` two `quire.package.semantic/v2` `PackageId`s is admitted; the same cause without `recompiled` is refused by the schema. | Test (TC-431) |
| FR-331-AC-36 | The `temporal-satisfaction` entry of a safety item settled by a stale EN-1 closure certificate (QSL FR-338), and that of a TP-4 liveness item settled by a stale EN-1 component certificate (QSL FR-339), each `inconclusive` with cause `stale-dependency{requested, recompiled}`, two `quire.package.semantic/v2` `PackageId`s, are admitted; the same cause without `requested` is refused by the schema. The checkers' comparison before any other rule and the settlement are verified by QSL FR-338-AC-11, FR-339-AC-4, FR-282-AC-10 and FR-127-AC-13. | Test (TC-431) |
| FR-331-AC-37 | The schema refuses a descriptor without `tool` at the manifest and request stages, including a linked Kani snapshot descriptor (`snapshot-kani-without-tool`) and a pure engine's snapshot descriptor (`snapshot-engine-without-tool`); refuses a `tool` without `name`, `version` or `build`, with an empty `name` or `version`, with another member, or that is a string or `null`; refuses a `build` in another domain, with another algorithm, with a digest that is not 64 lowercase hexadecimal digits, with another member, or that is a bare digest string; and admits a pure in-process engine's descriptor whose `tool` names its implementing crate, version and build. A request-stage snapshot carries the kani descriptor with the `tool` its `hello` gives. Two admitted descriptors that differ only in `tool.version`, or only in `tool.build` with the same `tool.version`, have different `ManifestDigest` records in `quire.tool-manifest.jcs/v1` and the same `BackendId`. | Test (TC-431) |
| FR-331-AC-38 | A provider fronting an external tool reports, in the descriptor it builds at an invocation, the version the executable reports at that invocation. When the executable is replaced by another version after the descriptor is built and before an item's run, the item's FR-290 probe fails and the item settles `unsupported` with `unsupported_projection`/`tool-unavailable` naming the capability kind, the backend, the descriptor's version and the probed version, and no run starts. | Test (TC-226) |
| FR-331-AC-39 | A `proved` result with `cross_check` `{"tool": "z3", "verdict": "proved"}` and one with `{"tool": "z3", "verdict": "refuted"}`, each `certified`, are admitted; `cross_check` on a `refuted`, `inconclusive` or `failed` result, a `cross_check` missing `tool` or `verdict`, with a `verdict` outside `proved`, `refuted` and `inconclusive`, or with another member, is refused before consumption. `cross_check` on a plugin's `trusted` `bounded-complete` proof (`cross-check-on-trusted`, refused by the `trusted` rule alone), on a certified `reduced` proof (`cross-check-on-non-smt-basis`), and verdict `refuted` on an `uncertified` `inductive` proof (`cross-check-disagreement-on-uncertified`) are each refused at `/results/<i>/cross_check`. An `inconclusive` result with cause `{"type": "cross-check-disagreement", "tool": "z3", "primary": "proved", "cross_check": "refuted"}` is admitted, so is one with `primary` `inconclusive`, and the cause missing `primary` is refused. | Test (TC-431) |
| FR-331-AC-40 | A `failed` result with cause `{"type": "tool-failure", "defect": "frame-length", "limit": 1048576, "value": 1048577}` and one with `defect` `total-bytes`, `limit` 4194304 and `value` 4194305 are admitted; either defect without `limit` or `value` is refused, and `limit` or `value` on any other defect, such as `crash`, is refused, before consumption. | Test (TC-431) |
| FR-331-AC-41 | The schema admits a hybrid request item carrying `"requirements": ["hybrid-dynamics"]` and `hybrid`, a temporal descriptor with `dynamics` `["discrete", "hybrid"]` and `bounds` holding `max_solver_steps` and `max_solver_time`, a `proved` entry with `inductive-solver{method: inductive-enclosure}` labelled `uncertified`, and an `inconclusive` entry with `insufficient-horizon{horizon: {numerator: "1", denominator: "1"}, jumps: 1, exhaustive: ["time"]}`; it refuses before consumption `requirements` `[]` or holding another label, `requirements` on an item of another kind, `hybrid` without `requirements`, a `dynamics` that is empty, repeats a label, holds another label or sits on a descriptor advertising no `temporal-satisfaction`, a hybrid descriptor whose `bounds` lack either limit, a `bounded-solver` `method` outside `delta-complete`, `flowpipe-outward-rounded`, `flowpipe-unrounded` and `unclassified`, an `inductive-solver` `method` other than `inductive-enclosure`, an `inductive-solver` proof carrying `certified`, an `insufficient-horizon` without `jumps`, a `bounded-solver` without `exhaustive`, and an `exhaustive` that repeats a dimension or holds a label other than `time` and `jumps`; and it refuses `requirements` `[]` and `["stochastic-dynamics"]` on an item that carries no `hybrid`. A reader refuses an `insufficient-horizon` cause on an item whose claim is not a hybrid claim `invalid_result`/`invalid-value` at `/cause/type`. | Test (TC-431) |
| FR-331-AC-42 | The manifest schema admits `state_graph_forms` as a nonempty, duplicate-free array of `possible`, `always-possible` and `unique-path` on a descriptor advertising `temporal-satisfaction`, and admits it absent as no state-graph forms; it refuses an empty, duplicate or unknown form list and refuses `state_graph_forms` on a descriptor that does not advertise `temporal-satisfaction`, with registration cause `invalid_capability`/`invalid-state-graph-forms` keyed by backend identity. | Test (TC-968) |
| FR-331-AC-43 | An `inconclusive` temporal result carrying only cause `refuted-ignoring-constraints` is admitted with no proof basis, certification or counterexample; the same cause on a `refuted` or `proved` result is refused before consumption, so the diagnostic is never readable as a standard refutation or proof. | Test (TC-431) |

## Dependencies

In AC-24, a hyper product state "with no states" means the required
`states` member is absent. It does not mean HP-3's valid empty universal
tuple; AC-27 pins that distinction. The closed HP-1 transition tuple,
HP-3's present empty universal tuple with its required automaton and
witness-set members, and FR-163's `components` member ownership are bound by
the schema. The model-only State locus cannot losslessly bind FR-412's
complete product key, so the separate `ProbabilityProductStateLocus` and the
certificate row bindings above carry the QSL FR-196 forms.

- [interface_013](./interface_013-backend-provider.md) owns the operation boundary.
- FR-195 owns the behavior that produces or consumes this value.
- [FR-382](../../functional/analysis/FR-382-select-state-space-reductions.md)
  owns the `reductions` request member, its identity rule and the reduction
  advertisement.
- [FR-290](../protocol/FR-290-protocol-claim-kind.md) owns the capability
  kinds, their vocabulary identity, advertised modes, candidate-set rule and
  tool-absence result.
- [FR-462](../../functional/tooling/FR-462-speak-the-plugin-wire.md) owns the
  frames that carry each stage and the `tool-failure` defects.
- [FR-322](FR-322-checked-package-artifact.md) owns the checked package that
  `package` carries.

## Status

Accepted complete-V1 boundary value contract.
