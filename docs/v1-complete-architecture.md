# Complete Quire V1 architecture

This is the architecture for every row of the
[atomic capability inventory](v1-capability-inventory.md). It reconciles the
accepted QSpec architecture with the current Rust component graph.

## 1. Architectural drivers

The architecture has nine non-negotiable drivers:

1. Native Quire is the sole editable formal-clause language. Mappings, IR,
   generated Rust and proof inputs are derived artifacts.
2. Complete language meaning is independent from any backend's supported
   fragment. A backend refuses an unsupported request exactly and never
   approximates it.
3. One semantic object or fact has one normative owner. Consumers may validate
   and bind an owned value but cannot reconstruct, strengthen or rename it.
4. Source, semantic, model, artifact, execution and evidence identities remain
   distinct and domain-separated.
5. Definition, execution, truth, activation, participation, settlement,
   completeness, assumption and provenance states remain independent axes.
6. Every public operation is bounded or accepts an explicit caller-owned budget.
   Exhaustion is data, not false, success or panic.
7. Extensions add versioned descriptors, providers or adapters. They do not
   replace the parser, type system, result model or evidence store.
8. First-party executable production and qualification logic is Rust. A spec's
   artifacts declare its domain model, and Quire owns that model's meaning;
   Quoin remains the evidence retention/audit consumer behind its Rust-port gate.
9. Every capability is V1 scope. Delivery can be dependency-ordered, but cannot
   be closed through deferral, a failed experiment or one backend's limitation.

### Current seams retained

The design extends implemented boundaries rather than inventing a replacement
stack:

| Implemented seam at the inventory revision | Architectural use |
| --- | --- |
| QSL `source`, `parser`, `linking/composed`, `checking/composed`, `package`, `runtime/evaluation`, `temporal` and `protocol_artifact` modules | S1-S5 native compiler and reference-evaluation foundation; expand their type/declaration families rather than add a peer frontend. |
| QSL `linking/composed/requests` family/capability/disposition report | Generalize into the common exact capability-negotiation pattern. |
| Cycle-free `quire-contract-model` package inside Contract IR | Lowest shared representation dependency; keep the full IR adapters out of it. |
| Full Contract IR `predicate`, `temporal`, `kani`, `bridge` and `ecosystem_model` modules | S8 native/provider bridges, not native semantic authority. |
| Codegen oracle, strategy, Kani, bound and replay modules plus the small `quire-contract-runtime` | Exact generated-backend foundation; extend profiles and keep the helper distinct from S6. |
| QObs `authority/*` record/population/position/clock/progress/completeness views | S4 observation authority; add uncertainty/partial/repair views without moving truth evaluation into QObs. |
| Protocol intake/link/compensation/conformance/claims/result modules | S5 bounded foundation; add new graph/analyzer families behind the same owner boundaries. |
| Verification planner, native process, qualification profile/corpus/oracle and report modules | S7/S10 plan/provider/result foundation; add method providers rather than a second planner. |
| Existing `quire-spec` and `quire` Rust binaries | Separate native-language lifecycle from specification/assurance coordination while sharing typed provider envelopes. |

The missing complete-V1 capabilities recorded by the inventory remain missing;
module names are evidence of seams, not evidence that the expanded behavior is
already implemented.

## 2. Four planes and ten subsystems

The ten subsystems are organized into four planes. The planes constrain
dependency direction; they are not new products or scope buckets.

### Definition plane

| ID | Subsystem | Owns | Primary Rust owners |
| --- | --- | --- | --- |
| S1 | Language kernel, packages and evolution | Source/CST-to-AST admission, editions, profiles, declarations, packages, dependencies, feature discovery, source identity and migration | `quire-spec-language`; normative QSpec AD-003 |
| S2 | Values, types, expressions and collections | Scalar/value algebra, exact numeric domains, functions and termination, records, collection kinds, queries, equality, definedness and pure evaluation | `quire-spec-language`; cycle-free descriptors in `quire-contract-model` |
| S3 | Model, graph and state-contract semantics | Model intake from domain packages, object identity/populations/references, inheritance/redefinition/dispatch, graph traversal, state anchors, frames and operation contracts | `quire-spec-language` model intake; normative QSpec AD-006 |

### Time and interaction plane

| ID | Subsystem | Owns | Primary Rust owners |
| --- | --- | --- | --- |
| S4 | Temporal and observation semantics | Temporal operators, clocks/history, uncertainty, partial valuations, progress, completeness, out-of-order repair and replay/incremental agreement | QSL native temporal artifact; `tl-*` independent temporal implementation; `quire-observation` authority views |
| S5 | Protocol and choreography semantics | Roles/instances, messages/channels/queues, control graphs, dynamic topology, guards/obligations, compensation/recovery, conformance, projection, realizability, composition and refinement | QSL source compiler; `quire-protocol` representation/runtime/analyzers |

### Execution and analysis plane

| ID | Subsystem | Owns | Primary Rust owners |
| --- | --- | --- | --- |
| S6 | Composed execution, monitoring and workflow | Cross-domain request coordination, state/temporal/protocol replay, incremental monitoring, population aggregation, simulation façade and authorized external workflow decisions/receipts | new Rust workspace in the already-planned `quire-integration` repository; component evaluators remain QSL/TL/QObs/Protocol |
| S7 | Verification methods and plans | Model-based testing, counterexample shrinking/replay, relational/hyperproperty analysis, continuous/hybrid analysis, bounded synthesis, method planning and non-conclusive result semantics | `quire-verification`, `quire-analyze`, method-provider crates |
| S8 | Semantic IR and exact backends | Cycle-free target-neutral IR, lowering requests, oracle/proptest/Kani generation, solver/codegen providers, capability negotiation, source maps and backend replay parity | `quire-contract-model`, `quire-contract-ir`, `quire-contract-codegen`, `quire-contract-runtime`, `quire-analyze` |

### Product and assurance plane

| ID | Subsystem | Owns | Primary Rust owners |
| --- | --- | --- | --- |
| S9 | Mappings, authoring and tooling | OCL/SysML/KerML/FRETish outputs, deterministic packages, formatter, lossless CST/editor/LSP, CLI, code/model views, plugins, JIT and reusable libraries | Contract IR mapping adapters, QSL, `quire-cli`, `quire-code-rs` |
| S10 | Integration, evidence and qualification | Conformance corpora, verification-result composition, independent qualification, evidence intake/retention and release decisions | `quire-integration`, `quire-verification`, future Rust Quoin interface, human release owner |

The complete primary allocation is exact:

- S1: all 15 `V1-SRC-*` rows.
- S2: `V1-TYPE-001..020`, `V1-TYPE-030..034`, and `V1-EXPR-001..027`
  (52 total).
- S3: `V1-TYPE-021..029` (9).
- S4: all 26 `V1-TEMP-*` rows.
- S5: all 31 `V1-PROT-*` rows.
- S6: `V1-RUN-001..012` and `V1-RUN-020` (13).
- S7: `V1-RUN-013..019` and `V1-EXPR-028..029` (9).
- S8: `V1-BACK-001..006` and `V1-BACK-014..024` (17).
- S9: `V1-BACK-007..013` and all 12 `V1-TOOL-*` rows (19).
- S10: all 9 `V1-EVID-*` rows.

The totals are 200 exactly. A cross-subsystem dependency does not change an
item's primary owner or duplicate its normative clause.

## 3. Authority model

QSpec owns language meaning and shared domain-object definitions. Implemented
authority is divided as follows:

| Concern | Authoritative producer | Consumer rule |
| --- | --- | --- |
| Authored source and source spans | QSL source reader/parser | No consumer re-parses source text or fabricates a span. |
| Edition/profile/package admission | QSL linker/checker | Downstream tools consume one immutable checked package and its capability report. |
| Domain model declarations and declaration keys | Spec artifacts, extracted by quire-rs and lifted by the FCD extraction-frontend into a `sha256-jcs` domain package | QSL model intake builds and checks Quire model declarations; it never invents declaration identities. |
| Native value/type/expression meaning | QSpec semantics implemented by QSL reference evaluator | IR/backends demonstrate correspondence; they never redefine native meaning. |
| Temporal authored meaning | QSpec/QSL native temporal artifact | TL supplies an independent selected-profile implementation and correspondence target. |
| Observation, population, clock progress and completeness facts | QObs owner artifacts | Protocol/temporal consumers validate the exact owner contract and do not infer missing facts. |
| Protocol representation and conformance facts | QSL checked protocol artifact plus Protocol assessment | Verification/mapping consumers preserve Protocol result axes and lineage. |
| Target-neutral derived contract model | Cycle-free `quire-contract-model` crate | It is a representation contract, not an authored-language or execution authority. |
| Domain execution results | The selected native Rust producer | Verification can compare/compose results but cannot relabel their domain outcomes. |
| Evidence retention and audit | Quoin after its Rust boundary is available | Producers do not acquire a Quoin runtime dependency; Quoin does not execute them. |
| Human release/acceptance decision | Attributed human/ix-flow record | No test, proof, score or agent promotes itself. |

## 4. Shared semantic kernel

### 4.1 Identity domains

All shared identities are opaque validated newtypes. Equality is allowed only
within the same identity domain. At minimum the kernel distinguishes:

- source bytes, source digest, source unit and lexical span;
- edition, semantic profile, wire profile and requested feature bundle;
- package, dependency manifest, declaration, type and semantic definition;
- domain package, model declaration key, effective specialization and
  relationship;
- runtime object, population, membership snapshot and state anchor;
- operation invocation, capture, result, created identity and deleted identity;
- clock, temporal formula, activation, observation and observation revision;
- protocol, role definition, role instance, message, send, receipt, delivery,
  attempt, effect, obligation, compensation, commit and recovery;
- requirement, clause, lowering request, backend run and counterexample;
- generated artifact, tool/toolchain/options set, evidence record and human
  decision.

A digest is always computed over a named canonical domain and version. Source
byte digests, semantic digests, artifact digests and evidence digests cannot be
compared as bare bytes. An
effective model declaration never replaces its original declaration identity.

### 4.2 Selection axes

Four independently selected axes keep unrelated meaning apart:

1. **Edition** selects source grammar and language meaning.
2. **Semantic profile bundle** selects named optional/parameterized semantic
   families within that edition.
3. **Artifact/wire contract** selects serialized representation.
4. **Provider profile** selects a temporal engine, analyzer, backend, plugin or
   mapping's supported correspondence and options.

### 4.3 Type and value algebra

The common type descriptor is a versioned algebra rather than a bag of Rust
types. Complete V1 includes:

- Boolean and mathematical integer;
- bounded signed integers;
- normalized rational;
- exact decimal as coefficient/scale, distinct from binary float;
- explicit IEEE binary floating profile with width, rounding and exceptional
  bit-level values;
- text and declaration-qualified enumeration;
- quantity as numeric value plus declared dimension/unit;
- independently represented presence, absence and explicit null;
- fixed records and tuples with type-owned structural equality;
- sequence, set, bag and ordered-set collection kinds;
- named recursive value types whose runtime values remain finite;
- object references separate from value containment.

Collections share bounded traversal interfaces but retain kind-specific
equality, order, uniqueness and multiplicity. Conversion is an explicit
operation with loss accounting. Iteration order is deterministic where meaning
depends on order and irrelevant where the mathematical type does not define it.

Mathematical values are not narrowed for a backend. A finite backend receives
an explicit model domain or returns `requires-bound`/`unsupported`; it cannot
invent maxima, round exact numbers or encode missing as zero.

### 4.4 Expressions and functions

One typed expression graph is reused by state clauses, temporal atoms,
protocol guards, analyzers and mappings. It contains source occurrences and
resolved definition references. It supports immutable locals, predicates,
functions, checked arithmetic, navigation, equality, collection operations and
state anchors.

Pure functions declare typed parameters/results, effects=`none`, a termination
obligation and a resource contract. Recursive functions are admitted only when
a selected termination checker validates a well-founded decrease. Runtime fuel
and depth remain independent safety bounds; exhaustion produces an explicit
non-result even for a statically terminating function. Ambient callbacks,
reflection, mutation and I/O are rejected.

Undefined arithmetic, incompatible units, absent/missing values, ambiguous
dispatch and resource exhaustion are typed results. Short-circuit logical
operators preserve their definedness rules and never turn an unevaluated error
into an executed Boolean.

### 4.5 Model and state semantics

The model universe is a closed selection of one or more domain packages, each
lifted from spec artifacts, selected by identity and pinned in the lock by the `sha256-jcs` digest the resolver derives. QSL model intake
constructs a read-only model view containing original
and effective declaration IDs, inheritance/conformance edges, redefinitions,
parts, ports, interfaces, connections, allocations, operations and
relationships. Dynamic dispatch resolves only inside the selected closed method
set.

Runtime state is an explicit finite environment: typed values, object IDs,
object-reference edges, a population/membership identity and named anchors.
Pre/current/post/result/capture values are never inferred from map position or
wall-clock time. Frames enumerate permitted updates plus created/deleted
identities. Graph traversal uses an explicit root, relationship set, direction,
positive-length/zero-length rule and resource budget.

## 5. Temporal and observation model

QSL publishes a checked native temporal artifact whose atoms reference the same
typed expression graph. The TL bridge projects that artifact to an exact
selected TL profile and records every correspondence decision. Native and TL
evaluators remain independent parity oracles rather than sharing one evaluator
and calling agreement proven.

S4 contains both finite/bounded temporal profiles and an explicit
infinite-trace profile for unbounded properties. A finite-prefix monitor can
return pending/indeterminate information about an unbounded property, but it
cannot prove infinite-trace satisfaction. A selected sound infinite-trace
provider consumes I06 through I13/I15 and returns proved, refuted,
inconclusive, unsupported or failed with its premises and counterexample. This
positive capability is V1-TEMP-023; V1-TEMP-021 remains the independent
integrity rule forbidding a finite-evidence proof claim.

QObs publishes immutable owner views for observation records, revisions,
population membership, positions, clock progress, closure, availability,
captures and completeness. A later record supersedes an earlier record only by
an explicit revision relation. Ingest order and timestamp order do not create
authority.

Partial valuation is represented as `known(value)`, `missing(reason)` or
`conflicting(candidates, reason)`. Temporal truth therefore carries the set of
possible truth values under the selected finite prefix and clock-uncertainty
interval, not a guessed midpoint or false default. Definite true, definite
false, indeterminate/pending, incomplete, refused and failed remain
distinguishable through the common result axes.

Replay and incremental monitoring invoke the same semantic kernel through
different state-transition adapters. A revisioned repair invalidates the
affected suffix/window, recomputes it under the same package and budget, and
links old/new results. Silent deadline progress comes only from an authoritative
clock-progress fact.

## 6. Protocol and choreography model

The checked protocol graph contains typed roles and role instances, messages,
channels, delivery premises, causal edges and control nodes. Control nodes are
sequence, owned choice, parallel fork/join, loop and bounded dynamic spawn.
Queues declare ordering, duplication, capacity/bound mode and overflow meaning.
Dynamic roles declare a finite population/lifetime contract; symbolic bounds
must be discharged before finite execution.

State guards and temporal obligations reference shared expressions/formulas.
Attempt, effect, delivery and compensation identities stay separate.
Compensation registration follows a matching successful effect; commit,
retry/recovery and partial outcome transitions follow authored relations only.

One pure conformance kernel backs batch replay and incremental assessment.
Projection, realizability, assume-guarantee composition and refinement are
separate pure analyzers over the same immutable protocol graph. Each analyzer
publishes premises, capability/profile, proof or counterexample, and
inconclusive/refusal outcomes. It cannot silently weaken the graph to obtain a
result.

Workflow execution is outside the conformance kernel. Protocol may emit a typed
recommended decision. Only S6's authorized adapter can request an external
effect, using an idempotency identity and returning a signed/attributed receipt.
The resulting observation re-enters through QObs; successful transport never
manufactures a business effect.

## 7. Composed execution and workflow subsystem

The existing component graph has no cycle-safe home for a composed runtime
facade:

- QSL is already a dependency of Protocol and cannot depend back on Protocol;
- Protocol must not absorb state/temporal compiler authority;
- `quire-contract-runtime` is intentionally a small `no_std` helper for
  generated oracles; and
- Verification plans and compares producers but does not become the product
  runtime.

The already-planned `quire-integration` repository (Research #28/#34 and #39)
therefore becomes a Rust workspace containing the S6 coordinator and adapter
contracts. This is not a new top-level epic or semantic engine.

The coordinator accepts one `ExecutionRequest` with exact package, model,
profile, dependency, observation/population, bound and requested-claim
identities. It validates the lock, dispatches independent state, temporal,
protocol or analysis producers, and emits an ordered `ExecutionBundle`. It
never parses source, evaluates a clause itself or collapses producer outcomes.

The same facade supports:

- one-shot state/temporal/protocol replay;
- incremental sessions with explicit checkpoint and supersession identity;
- finite population aggregation;
- deterministic simulation using an explicit seed, scheduler model and bound;
- workflow decisions through separately authorized effect adapters; and
- replay of a backend/model-test counterexample through the native producers.

An adapter declares capability, required authority, side-effect class,
idempotency rules and resource limits. Read-only/decision-only adapters are
distinct from effecting adapters. No default adapter can execute external
business effects.

## 8. Verification methods and planning

`quire-verification` owns a versioned plan language over references to authored
requirements/clauses; it does not copy those clauses. A plan selects method
providers, exact domains, preconditions, bounds, tools/options, evidence
obligations and comparison rules. Its dependency graph is acyclic and each task
has explicit runnable, refused, blocked and completed states.

Every method implements a typed `MethodProvider` contract through an in-process
Rust trait or the same versioned bounded process protocol. Providers consume an
immutable checked artifact and return native `MethodResult` records. Required
providers include:

- model-based trace generation and assumption-preserving shrinking;
- state/temporal/protocol simulation and replay;
- relational and finite-trace hyperproperty analysis;
- SMT consistency, implication, contradiction and vacuity analysis;
- continuous/hybrid analysis under a declared numeric/time model;
- bounded synthesis that labels candidates separately from proof; and
- mutation/differential methods used to qualify sensitivity, not define truth.

No aggregate score proves a requirement. A result names the exact admitted
domain and distinguishes proved, refuted, witnessed, candidate, inconclusive,
unsupported, exhausted and failed. Counterexamples carry replay inputs and are
accepted as evidence only when the native runtime reproduces the same semantic
verdict or records an explicit replay disagreement.

## 9. Target-neutral IR and exact backend protocol

The cycle-free `quire-contract-model` crate remains the lowest shared Rust
representation package. It contains versioned data contracts, identity
newtypes, expression/type/protocol descriptors, requests/results and bounded
readers; it contains no parser, native evaluator, solver, code generator,
monitor, evidence store or release policy.

QSL lowers checked native artifacts into this model. Full
`quire-contract-ir` owns adapters to QSL, QObs, Protocol and TL plus canonical
derived packages. Keeping those adapters in the full crate preserves the
existing acyclic dependency direction:

```text
Domain package ─────> QSL ──────────────┐
       └────────────> QObs ──> Protocol ├──> full Contract IR
contract-model ─────> QSL/Protocol ─────┘          │
TL syntax/evaluator ───────────────────────────────┤
                                                   ├──> Codegen ──> generated code
                                                   └──> Analyze / other providers
contract-runtime <──────────────────────── generated code
```

A `BackendRequest` selects exact source clauses, provider/profile version,
finite domains/bounds, options and requested artifacts. Before execution the
provider returns one capability disposition per clause/feature:

- `supported-exactly`;
- `requires-explicit-bound`;
- `unsupported` with a stable source-located cause; or
- `invalid-request` for an incoherent selection.

Execution returns independent per-obligation outcomes and accounting. One
unsupported or exhausted item cannot erase supported sibling outputs. Generated
code embeds source maps. Kani, proptest, SMT, JIT and generated
Rust use the same protocol and must replay counterexamples through S6/native
execution. Approximation, implicit numeric narrowing, omitted obligations and
success on an unexecuted precondition are forbidden.

`quire-contract-runtime` remains permissively licensed, small and `no_std`
where its selected features allow. It implements generated-oracle value/result
primitives only; it does not become the native Quire runtime.

## 10. Tooling, mappings and extension boundary

QSL retains both a lossless CST and a typed semantic graph. Formatting and
editor transformations operate on the CST; checking/execution operate on the
semantic graph; source correspondence connects them. Recovery nodes may support
editor feedback but can never enter a checked package.

The native lifecycle API and `quire-spec` binary expose parse, format, check,
compile, lower, execute and inspect. The general `quire` CLI remains the
specification/assurance coordinator and invokes versioned native provider APIs;
it does not duplicate the language parser or evaluator. Replay, monitor,
analysis and workflow commands route through S6/S7. Both CLIs publish stable
machine envelopes, stdout/stderr/exit contracts and exact engine provenance.

OCL 2.4, SysML 2.0/KerML 1.0 and FRETish 3.1 adapters consume a target-neutral
mapping request and emit an immutable package plus one disposition per source
obligation: preserved, conditional, unrepresented or refused. Generated text is
never admitted as native source without a separately authorized external-source
frontend—which complete V1 does not define.

`quire-code-rs` and model renderers publish derived views with source/model
identity and no authority over the input graph.

Two plugin forms are supported:

1. trusted compile-time Rust provider crates implementing versioned traits; and
2. isolated Rust executables implementing the bounded provider wire protocol.

There is no unstable in-process dynamic-library ABI. A plugin declares its
semantic role, capabilities, purity/effect class, resource needs and
conformance corpus. Unknown or conflicting extensions refuse. A plugin
cannot add source meaning without a registered edition/profile extension.

JIT is an S8 backend provider. Its artifact identity includes checked IR,
provider/toolchain/options and target. It must agree with native reference
execution on the conformance corpus and cannot be the sole qualification oracle.
Reusable domain libraries are immutable Quire packages that select an exact
edition and profile, not arbitrary runtime callbacks.

## 11. Common request, result and failure contracts

Every cross-subsystem operation uses the same envelope structure while retaining
domain-specific payloads:

### Request envelope

- operation;
- exact subject/package/dependency identities;
- requested capabilities and output ordering;
- finite domains, resource budget and cancellation policy;
- provider and options;
- caller and authorization context where effects are possible; and
- source/object loci for each selected obligation.

### Result bundle

- request and producer/run identities;
- one result or refusal for every requested item, in request order;
- independent execution, semantic truth/value, activation, participation,
  settlement, completeness, assumption and provenance axes;
- resource usage and cancellation state;
- source maps, counterexamples and replay material; and
- canonical content identity.

### Failure taxonomy

| Class | Meaning | Required behavior |
| --- | --- | --- |
| Invalid | Input is malformed or internally contradictory. | Typed located rejection; no semantic execution. |
| Unsupported | The selected edition/profile/provider does not implement an exact requested correspondence. | Per-item refusal; supported siblings continue. |
| Incomplete | Required authoritative input, membership, progress or premise is absent/unknown. | Preserve missing owner/reason; never coerce false. |
| Conflicting | Two authoritative inputs cannot both govern the same selected subject. | Preserve both sources and refuse affected items. |
| Undefined | Admitted expression has a specified undefined result for these values. | Domain result distinct from tool failure. |
| Exhausted | A named resource budget was reached. | Usage plus bound; no success/truth inference. |
| Cancelled | Caller cancellation was observed at a defined boundary. | Partial independent results retained; no implicit rollback claim. |
| Failed | Producer/tool/internal/external execution failed. | Attributed failure and captured process/adapter status; never unsupported or false. |
| Unauthorized | A side-effecting adapter lacks exact authority. | No effect request; typed security refusal. |

Each cause carries phase, stable owner-local code, subject, source/object locus,
human message, structured details and cause chain. A common envelope does not
flatten owner code catalogs into one ambiguous global enum.

## 12. Interface catalog

These interfaces are specified as objects plus FR/IT/TC rows.
Names are architectural roles; Rust names may follow repository idioms.

| Interface | Producer -> consumer | Contract |
| --- | --- | --- |
| I01 Source document | editor/filesystem -> QSL | Exact bytes, unit identity and bounded read outcome. |
| I02 Parsed source | QSL parser -> formatter/linker/editor | Lossless CST, recoveries, tokens/spans and edition; not semantically admitted. |
| I03 Model selection | FCD extraction-frontend lift -> QSL/QObs | Domain package identity, lock-recorded `sha256-jcs` digest, declaration keys, declarations and relationships lifted from spec artifacts. |
| I04 Checked package | QSL -> all semantic consumers | Immutable semantic graph, package lock, feature report, source map and diagnostics. |
| I05 Native runtime input/result | caller/S6 <-> QSL | Typed state anchors, objects, frames, bounds and independent results. |
| I06 Native temporal artifact/result | QSL/S6 <-> TL/QSL | Typed atoms/formula, clocks/history/captures, partial valuations and progress. |
| I07 Observation authority bundle | QObs -> temporal/Protocol/S6 | Records, revisions, populations, positions, clocks, completeness and authority. |
| I08 Checked protocol artifact | QSL -> Protocol/S6 | Roles/instances, messages/channels, control/queue graph, guards, obligations and bounds. |
| I09 Protocol assessment | Protocol -> S6/Verification/IR | Conformance/projection/refinement outcomes, premises, axes and lineage. |
| I10 Execution bundle | S6 -> CLI/Verification/Quoin intake | Ordered cross-domain native results without semantic flattening. |
| I11 Effect decision/receipt | S6 <-> authorized adapter | Decision, authority, idempotency, request and external receipt/observation link. |
| I12 Target-neutral contract package | QSL/full IR -> backend providers | Cycle-free types/expressions/clauses/protocol refs and source map. |
| I13 Backend capability/request/result | S8 coordinator <-> provider | Per-item exact support, bounds/options, artifact accounting and counterexamples. |
| I14 Verification plan | author/Verification -> method providers | Referenced clauses, methods, domains, dependencies, bounds and evidence obligations. |
| I15 Method result | provider -> Verification/S10 | Domain-native result, non-conclusive state, provenance and replay material. |
| I16 Output mapping package | mapping adapter -> consumer | Target text/schema plus per-obligation preservation/loss/refusal records. |
| I17 Tool/plugin manifest | tool/plugin -> S9 registry | Capabilities, purity/effects, platform and conformance identity. |
| I19 Conformance corpus/result | independent corpus -> producer/Qualification | Feature/refusal vectors, expected domains and independently produced outcomes. |
| I20 Evidence intake | native producers/Verification -> Quoin | Immutable structured result references and artifacts; no producer execution. |
| I21 Human decision | ix-flow/human -> release record | Attributed scope, inputs, disposition, rationale and supersession. |

## 13. Extension scenarios

The following scenarios are architectural acceptance tests. Each must be served
without replacing an existing subsystem or creating a second authority:

1. **Add a scalar or collection kind.** Add a versioned S2 type descriptor,
   parser/checker/evaluator handlers and conformance vectors. Existing editions
   remain unchanged; old backends return unsupported until they add exact
   correspondence.
2. **Add an expression operator.** Register syntax/typing/definedness/evaluation
   in S1/S2 once. Temporal atoms, guards and mappings reuse the expression node;
   they do not add private evaluators.
3. **Add a declaration family.** Add an edition/profile feature descriptor,
   namespace/linking rule and checked artifact member. Existing packages remain
   readable under their wire profiles.
4. **Add a backend or analyzer.** Implement I13/I15 and publish capabilities,
   options, bounds and conformance. No grammar or native semantic amendment is
   required unless the language itself changes.
5. **Add a mapping or consumer.** Consume I12/I16 and account for every selected
   obligation. The consumer never becomes a source frontend.
6. **Add a model feature.** Declare it in an artifact section, add its quire-rs
   extraction rule, its semantic IR member and its module manifest section, then
   give it Quire meaning in model intake. QSL's model semantics do not bind to
   extraction implementation structs.
7. **Add dynamic protocol behavior.** Extend S5 graph nodes plus bounds and
   analyzers. S6/Protocol consume the same graph; bounded kernels may refuse an
   undischargeable symbolic bound without deleting the source construct.
8. **Add a verification method.** Register an I14 provider and I15 result
   vocabulary. Quoin accepts the structured result through I20 without new
   domain execution code.
9. **Add a plugin or JIT.** Use I17 plus I13/process isolation; exact
   toolchain/options and artifact identity prevent a plugin from changing
   meaning invisibly.
