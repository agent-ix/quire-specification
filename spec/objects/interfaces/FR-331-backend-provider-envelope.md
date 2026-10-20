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
  - target: ix://agent-ix/quire-specification/FR-368
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
---

# FR-331: Backend provider request and result envelope

## Description

When a contract package is submitted to a backend provider, the producer SHALL use the immutable BackendProviderEnvelope contract below rather than an untyped map, display string or implementation-local structure.

## Properties

- `contract_version` — exactly `quire.backend-provider/v1`.
- `capability_vocabulary` — exactly `quire.capability-kind/v1`; it selects the FR-290 vocabulary for every kind in `manifest`, `request` and `results`.
- `manifest` — one backend descriptor per registered backend in the registry snapshot: backend identity, advertised (FR-290 kind, mode) pairs, domains, options, bounds, where the backend declares one its obligation-form set (FR-349), the state-space reductions it applies, a subset of `symmetry`, `partial-order` and `constraint` (FR-382), and, for `probabilistic-satisfaction`, its advertised evidence kinds (FR-290). A backend that advertises `temporal-satisfaction` also carries its `fairness_kinds` ([FR-368](../../functional/temporal/FR-368-negotiate-temporal-model-check-items.md)). A backend that advertises `temporal-satisfaction` and decides weak-memory subjects may also carry `memory_models`, the memory models it decides, a nonempty set drawn from `sc`, `tso` and `ra` by the names of the `memory` clause (FR-439; QSL FR-226 routes by it); a descriptor that advertises another kind carries none. Backend identities are unique.
- `request` — the checked package as `package` and `package_id` ("Package bytes" below), the provider deadline `deadline_ms` and the requested items: each item's claim, its proof bounds (the substituted `ProofBound`s of its `extent`, and a temporal item's `subject.proof_bounds`), its `options` and its own limits (`temporal.limits`, `probabilistic.limits`, `hybrid.limits`); there is no request-level `domains` or `limits` member. For each item, its one required FR-290 kind, its extent and #222 extent classification (`bounded` or `unbounded`), the backend the caller names when it names one, its `candidates`, and for a model-check item its `reductions` selection (FR-382); for a `probabilistic-satisfaction` item, its evidence kind (`statistical` or `exact`) and, for `statistical`, its method (`okamoto`, `sprt` or `regenerative` with `min_cycles`), seed, `max_witnesses`, for a timed subject its grid exponent `q` and `max_sample_steps` (default `2^24`), and `StatisticalLimits{max_samples, max_draws, max_cycle_steps}` ([FR-408](../../functional/analysis/FR-408-settle-probabilistic-claims-statistically.md)), or, for `exact`, `ExactProbLimits` ([FR-411](../../functional/analysis/FR-411-settle-probabilistic-claims-exactly.md)). Every limit has the published default that [FR-461](../../functional/foundation/FR-461-name-the-limit-value-and-setting.md) gives it, and `max_sample_steps` defaults to `2^24`.
- `temporal` (per `temporal-satisfaction` request item over a model subject) — the item's `subject` (initial-state references, universes, proof bounds), `item_kind` (`formula` or `deadlock-freedom`), resolved `fairness` set, `max_depth` for a bounded search and `limits`, as [FR-368](../../functional/temporal/FR-368-negotiate-temporal-model-check-items.md) states. For a state-graph, hyper or step-relation item, `limits` also holds `max_walk_steps` (the step count of one witness-sampling walk; [FR-392](../../functional/analysis/FR-392-sample-possible-witnesses.md)), `max_witness_set` and `max_relation_tuples` ([FR-399](../../functional/analysis/FR-399-settle-hyperproperty-results.md)), with the setting names `model_check.max_walk_steps`, `model_check.max_witness_set` and `model_check.max_relation_tuples` that [FR-461](../../functional/foundation/FR-461-name-the-limit-value-and-setting.md) gives them; the routed provider publishes each default, which applies when the request sets none. `temporal` also holds the sampling settings `witness_samples` and `seed`, which are method parameters with provider-published defaults.
- `candidates` (per request item) — exactly one of: the set of candidate backend identities that the `quire-spec-language` registry computes under FR-290's candidate-set rule, ordered bytewise by identity; or the unknown-backend mark carrying the unregistered backend identity that the request named.
- `dispositions` — supported, requires-bound, unsupported or invalid-request per item.
- `artifacts` — immutable generated bytes with kind, identity and source map.
- `results` — proved, refuted, tested, measured, inconclusive, unsupported, declined, incomplete or failed per item. `measured` is recorded only for a completed statistical run and carries the [FR-409](../protocol/FR-409-measured-decision.md) decision and the [FR-410](FR-410-statistical-verdict.md) `StatisticalVerdict`, with no truth and no settlement basis; it is never `proved`. A `proved` result carries its proof basis: `exact-value` or `value-bounds` with an [FR-412](FR-412-probability-certificate.md) certificate, `zone-certified` with an [FR-418](FR-418-zone-certificate.md) certificate, `closed-form{analysis}` with its recomputed evidence ([FR-419](../../functional/analysis/FR-419-analyze-task-set-schedulability.md)), each settled only after its certificate or evidence is checked; or `bounded-solver{method, horizon, jumps}` ([FR-193](../../functional/analysis/FR-193-analyze-continuous-hybrid-models.md)), a solver's own result with neither, labelled `uncertified`. Every `proved` result also carries exactly one `certification`, which says who stands behind the proof ("Certification" below). `unsupported` is recorded for a supported, routed item whose tool probe finds the tool missing, errors or exceeds its limit, naming the capability kind and the backend; and, separately, for a supported item whose Kani run ends `Unavailable` (the solver or backend is absent after negotiation), naming the capability kind, the backend and the absent solver or backend identity. `declined` is recorded for a supported item whose Kani run ends `Refused`, `InvalidInput` or `IncompleteInput`, and names the typed refusal cause (QSL ADR-013 O-16 refusal category, QC-9); it keeps the item's `supported` disposition and is the item's one terminal accounting record, the same as a tool-absence `unsupported` result. A Kani proof run whose obligation has zero SUCCESS checks records `inconclusive` with a typed vacuity cause, never `proved`; this adds no result value (QSL ADR-013 FND-005, O-16 proof column).
- `counterexamples` — canonical assignments/traces plus originating run identity. A temporal refutation over a model subject is an [FR-364](../../functional/temporal/FR-364-replay-temporal-counterexamples.md) `TemporalCounterexample`; a timed trace's steps carry their `delay` and a time-lock counterexample carries `kind` `time-lock` and the member `final_delay`, the exact rational delay that reaches the time-locked state (FR-181, [FR-417](../../functional/temporal/FR-417-settle-time-lock-freedom.md)); a probabilistic refutation carries an [FR-413](FR-413-probabilistic-counterexample.md) `ProbabilisticCounterexample`. Each entry names exactly one of: the transcript's `artifacts` entry, whose assignments are its decode and are never separately stored, or the canonical assignments of a counterexample with no transcript; never both (QSL ADR-013 O-25, QC-6).
- `accounting` — exactly one terminal record for every requested item. A temporal item's terminal record carries its [FR-363](../../functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md) proof basis and depth or cause, and any [FR-369](../../functional/temporal/FR-369-name-excluding-strong-fairness.md) `fairness.strong-would-exclude` warnings. The terminal record of a state-graph, hyper or step-relation item also states its settlement method ([FR-391](../../functional/analysis/FR-391-settle-state-graph-claims.md), [FR-399](../../functional/analysis/FR-399-settle-hyperproperty-results.md)), its search horizon, and the seed and limits the run used.

### Certification

Every `proved` result entry carries the member `certification` with exactly
one of three values. A reader tells from it alone whether the proof was
checked, unchecked or taken on trust:

| `certification` | Meaning | Proofs that carry it |
| --- | --- | --- |
| `certified` | The qualified core produced or checked the proof: a core checker accepted its certificate, or the core's own prove path produced it | an accepted certificate or recomputed evidence (`exhaustive` with no state constraint and its closure or component certificate, `reduced` with copy-swap as its only reduction and its product-closure certificate ([FR-403](../../functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md)), `bounded-complete` or `inductive` whose SMT certificate the core verified ([FR-354](../../functional/ir-proof/FR-354-record-proof-trust-basis.md)), `exact-value`, `value-bounds`, `zone-certified`, `closed-form`, `witnessed`); a Kani `bounded-proof{checks}` |
| `uncertified` | A first-party engine or solver outside the core proved it, and no core checker re-checked it | `reduced` with a `symmetry` or `partial-order` reduction, and `exhaustive` under a state constraint ([FR-385](../../functional/analysis/FR-385-report-reduced-verdicts.md)); `bounded-solver` ([FR-193](../../functional/analysis/FR-193-analyze-continuous-hybrid-models.md)); an SMT proof whose certificate is absent or cannot be verified (FR-354); any proof whose method has no core certificate checker |
| `trusted` | A third-party plugin proved it, and the core runs no check of it | every plugin `proved` result ([FR-305](../../functional/tooling/FR-305-load-explicit-rust-plugins.md)) |

Each value settles `proved`, and `certification` belongs to `proved` alone.
A certificate a core checker rejects settles the item `inconclusive` with
cause `certificate-rejected{rule, at}`: `rule` the check rule that failed and
`at` where it failed, for every core checker.

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
| `proved` (proof basis) | `exact-value{value, reductions}`, `value-bounds{lower, upper, method, reductions}` with `method` `backward-induction{precision_bits}` or `interval-iteration{precision_bits, sweeps}`, `zone-certified`, `exhaustive`, `closed-form{analysis}` with `analysis` `fixed-priority`, `edf` or `amc-rtb`, `bounded-solver{method, horizon, jumps}` |
| `refuted` (cause) | `undefined-evaluation{where, cause}` |
| `measured` (FR-409 cause) | `undefined-evaluation{where, cause}` for `rejected`; `indifference-region` and `undecided-successor` for `undecided` |
| `inconclusive` (cause) | `certificate-rejected{rule, at}`, `replay-parity`, `undecided-successor`, `no-initial-state`, `no-instance`, `sufficient-test-failed`, `solver-inconclusive`, `lasso-not-concretized`, `no-admitted-behaviour`, `well-definedness-unchecked` |
| `incomplete` (cause) | `limit-reached{limit, value, setting}`, naming the limit, its value and the request setting that raises it, the one limit-stop cause of every engine; `timed-out`, the backend's own wall-clock limit, as a hybrid solver's `max_solver_time` or a provider's overrun of `deadline_ms` ("Hybrid solver budgets" below); `precision-budget{lower, upper}`; `cancelled{source}` for a cancelled run, `source` the cancellation cause, `requested` when the caller cancels and `deadline` when the caller's deadline ends the run |
| `unsupported` (cause) | `missing-confidence`, `every-scheduler`, `exact-only-form`, `not-markov{state, transition, post_states}`, `not-stochastic{state, identity}`, `time-locked-sample{state}`, `total-above-draw-range{total}`, `zero-weight-component{component}`, `delay-distribution{operation}`, `strict-clock-constraint{locus}`, `zero-delay-cycle{states}`, `timed-form-shape{locus}`, `punctual-interval{locus}`, `stopwatch-required` |

A disposition `invalid-request` with `invalid_capability`/`not-statistical`
is a negotiation outcome ([FR-408](../../functional/analysis/FR-408-settle-probabilistic-claims-statistically.md))
and carries no result.

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
| manifest | `manifest`, with exactly one descriptor: the provider's own | `hello` body |
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
package the lock's `dependency_selections` names, with its `package_id`,
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
- `candidates` is `{type: "set", backends}` or
  `{type: "unknown-backend", backend}`.
- `options` is a map of backend option names to string values (FR-196).
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
`max_solver_time` is a wall-clock setting and is outside it (FR-306).

### Result entries

A result entry carries `request_index`, `backend` (the identity its manifest
names, FR-462), `kind` and `result`, and:

- `certification` — on `proved` only, `certified`, `uncertified` or
  `trusted` ("Certification" above);
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
| `proved` | `basis` and `certification`; no `cause` or `diagnostic` |
| `refuted` | `counterexample`; `cause` only for `undefined-evaluation` |
| `tested` | nothing further |
| `measured` | `verdict`; no `basis`, `cause` or `diagnostic` (FR-409) |
| `inconclusive` | `cause` |
| `unsupported` | exactly one of `cause` and `diagnostic` |
| `declined` | `cause` (`refused`, `invalid-input` or `incomplete-input`) and `diagnostic` |
| `incomplete` | `cause` (`limit-reached`, `precision-budget`, `cancelled{source}` or `timed-out`) |
| `failed` | exactly one of `cause` (`tool-failure{defect}`, FR-462) and `diagnostic` |

The closed proof bases are those of FR-363 (`exhaustive`,
`bounded-complete{depth}`, `inductive{depth}`), FR-385
(`reduced{reductions}`, each reduction `{reduction, parameters, row}`), the
table above, FR-391 (`witnessed{sources}`, the FR-393
witness source of each initial state) and the Kani basis
`bounded-proof{checks}`. The closed inconclusive causes are those of the
table above and of FR-363 (`bound-reached{depth}`,
`induction-not-closed{depth}`), FR-385, FR-399 (`match-undetermined`,
`vacuous-match`, `replay-refused`), FR-379
(`mapping-undetermined{row, state_key, prefix}`), FR-380
(`transfer-not-preserving{reason}` with `reason` `fragment` and its
`operator`, `fairness` and its `constraint`, or `stutter-closure`;
`premise-unproved{premise, result}`, `premise` the premise item's
`request_index`), FR-439 (`memory-bound-reached{limit, value, states}`, `limit` the request
member that raises the budget, `max_store_buffer` or `max_messages`), FR-428
(`instance-bound-reached{limit, value, states}`, `limit` always
`max_live_instances`, the member that raises it) and AD-016's
`kani_vacuous_proof`. FR-439 adds the unsupported cause
`weak-access-shape{site, transition, shape}`.

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
| a plugin-wire failure (FR-462) | `failed` | cause `tool-failure{defect}` |
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
  `memory.load-buffering-shape{load, store}` for a weak-memory item, QSL
  FR-226, and `cross-check-absent{tool}` for FR-349's absent cross-check) and the FR-196
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

Request identity includes the capability vocabulary, manifest, package, each item's claim, extent with its proof bounds, options and limits, each item's kind, extent classification, named backend and candidates, each temporal item's subject, item kind, fairness set, `max_depth` and `limits`, the `witness_samples` and `seed` of a state-graph item, and each item's symmetry declarations, partial-order selection and state constraint; the state constraint is also recorded in the item's terminal record. A different temporal horizon or deterministic budget can produce a different result, so a result is reused only for a request with the same request identity; a wall-clock setting, a hybrid item's `hybrid.limits.max_solver_time`, is outside it. Result identity includes artifacts and results.
Unknown contract versions, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption. A `capability_vocabulary` other than `quire.capability-kind/v1`, or none, refuses the envelope as `invalid_capability`/`unknown-vocabulary`.

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
| FR-331-AC-28 | A model-subject item with no initial state, and one with zero instances, each record `inconclusive` with the cause `no-initial-state` and `no-instance` respectively, spelled exactly so; the temporal-outcome vector carries a result entry of each, and a `no-instances` spelling is refused. | Test (TC-431) |
| FR-331-AC-29 | A hybrid item's solver that exhausts `hybrid.limits.max_solver_steps` records `incomplete` with `limit-reached` whose `limit` is `max_solver_steps` and whose `setting` is `hybrid.max_solver_steps`; one that exhausts `max_solver_time` records `incomplete` with `timed-out`; a `limit-reached{limit: max_solver_time}` or a `time-limit` cause on a hybrid item is refused `invalid_result`/`unknown-cause`; a fractional `max_solver_time`, a zero budget, an unknown `hybrid.limits` member and an empty `hybrid` or `limits` are refused before consumption; two requests differing only in `max_solver_time` have equal request identity. | Test (TC-373) |
| FR-331-AC-30 | The descriptor schema admits `memory_models` as a nonempty duplicate-free subset of `sc`, `tso` and `ra` on a descriptor that advertises `temporal-satisfaction`, and refuses an empty `memory_models`, a repeated or unknown model, and `memory_models` on a descriptor that does not advertise `temporal-satisfaction`. | Test (TC-431) |

## Dependencies

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

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
