# Quire v1 capability decisions and inspected foundations

Proposed design baseline, 2026-09-09. Owning issue:
[standard #10](https://github.com/agent-ix/quire-specification/issues/10).
This is the language design matrix requested by the owner. The cross-stream
[functional ledger #9](https://github.com/agent-ix/quire-specification/issues/9)
remains the coordination index; this document does not replace its ownership.

**V1** means required in the complete v1 language, not implemented today.
**Later** means a named subsequent capability, refused when requested of v1.
**Refuse** means the proposed v1 contract deliberately rejects the behavior; no
implementation is inferred or scheduled merely from its mention. It is not a
permanent language exclusion unless the row identifies a source-authority,
side-effect or invalid-claim boundary. Broader semantic domains can be proposed
through new versioned profiles. A bounded
implementation stage is not permission to remove a required family from v1.

## Evidence and precedence

The current owner handoff and ruling take precedence over older research and
roadmaps. Source inspection establishes only the named component behavior; no
new Cargo tests or integration qualification were run for this inventory.

| Key | Inspected source | What it establishes and limits |
| --- | --- | --- |
| OWN | Owner handoff `/tmp/quire-spec-design-handoff.md`; [#10](https://github.com/agent-ix/quire-specification/issues/10) | All three families are v1 requirements; architecture and review precede implementation. |
| RULE | [Native-source ruling](https://github.com/agent-ix/quire-contract-ir/issues/53#issuecomment-5611950216) and [reference correction](https://github.com/agent-ix/quire-contract-ir/issues/53#issuecomment-5612031500) | One source language, Rust production/qualification, output-only external mappings, state admission and retained reference design gate. |
| PLAN | [Research plan](https://github.com/agent-ix/quire-research/blob/3df57c91773d8d64b1431649aeafc422a87d6578/implementation-plan.md) | FS01–06, LC01–05, observation OB01–03, protocol PT01/PT02 and qualification owners. Its old scheduling of the entire protocol family as research is superseded by OWN. |
| ARCH | [Maintained architecture](https://github.com/agent-ix/quire-research/blob/3df57c91773d8d64b1431649aeafc422a87d6578/architecture-detail.md) §§4–9 | State, temporal/protocol, relational and verification-plan subjects; clocks, correlation, compensation and distinct conformance claims. |
| DISC | [Original choreography research](https://github.com/agent-ix/quire-research/blob/3df57c91773d8d64b1431649aeafc422a87d6578/discovery/2026-09-07-choreography-research/architecture.md) | Product examples and proposed features only. FRETish authoring, implied solver capabilities, endpoint-as-role and automatic monitor qualification claims are superseded or unestablished. |
| STATE | [Native compiler](https://github.com/agent-ix/quire-spec-language/tree/dd34599e7e3232b9b3b3effd0cecb51b55da6803), especially `src/checking`, `src/runtime`, `src/linking` and `examples/config-version` | Landed parser/linker/checker, state/operation evaluation and explicit runtime correspondence. Their old profile semantics do not prove the new admission rules or any choreography support. Open compiler #26–34 are separate unlanded work. |
| ADOPT | [Historical state definition](../state-core/profile.md), [semantics](../state-core/state-semantics.md), [internal adoption](../../docs/internal-implementation-adoption.md) | Actual state foundation and its preserved definition identity; not the full official language. |
| TL | [Current syntax source](https://github.com/agent-ix/tl-syntax/blob/7fb0fe32f6ba4ef2606d120ddbe5858965d4fb5a/src/syntax.rs) | Boolean propositions and bounded Future/Globally/Until/Release; closed-trace and online-prefix profiles. The inspected enum has no past-time nodes. |
| TLE | [Current evaluator tree](https://github.com/agent-ix/tl-mltl/tree/a8ad227473d58c2d2ed42411b80d1e09b9c4cdf0/src); inspected earlier evaluator [f0a9a0f](https://github.com/agent-ix/tl-mltl/blob/f0a9a0fdfbdc56bc727c9d397aec1befd4f82363/src/evaluate.rs) | Existing finite-trace/pending-aware evaluation is a reusable foundation. Tree presence at the current revision is not a rerun of its checks or proof of native correspondence. |
| TIME | [FS04](https://github.com/agent-ix/quire-specification/issues/5), [OB01](https://github.com/agent-ix/quire-research/issues/44), [OB02](https://github.com/agent-ix/quire-research/issues/45) | Required clocks, captured trigger values, completeness, silent deadlines, late-data policy and timed-refund cases. |
| PROTO | [PT01](https://github.com/agent-ix/quire-research/issues/47), [PT02](https://github.com/agent-ix/quire-research/issues/48), [choreography specification #12](https://github.com/agent-ix/quire-specification/issues/12) | Roles, channels, branching, parallelism, iteration, compensation and separate replay/conformance/projection obligations. |
| BRIDGE | [Contract IR #64](https://github.com/agent-ix/quire-contract-ir/issues/64), [#63](https://github.com/agent-ix/quire-contract-ir/issues/63), [TL past-time #33](https://github.com/agent-ix/tl-syntax/issues/33) | Native-to-TL correspondence is missing design work; the old FRETish source wording must be retargeted. Past-time is existing owned roadmap work, not existing runtime support. |
| MODEL | [FCD #95](https://github.com/agent-ix/filament-core-data/issues/95), [model contract](../state-core/model-contract.md) | Field presence has a separate shared-model owner; it cannot be invented as native syntax to satisfy FCD #93. |
| EXPORT | [FS06](https://github.com/agent-ix/quire-specification/issues/7), contract IR #55/#56/#57 | OCL, SysML/KerML and FRETish are desired generated mappings with per-obligation loss/refusal evidence. |

## Shared language and model capabilities

| ID | Capability and selected boundary | Decision | Evidence / owner | Existing foundation or gap |
| --- | --- | --- | --- | --- |
| V1-001 | One native source, explicit edition and exact profile bundle; state, temporal and choreography clauses in one linked package | V1 | OWN, ARCH; A / #10 | Profile composition missing; current single-profile parser is a foundation. |
| V1-002 | Byte/revision/span identity, exact model imports, typed bindings, semantic versus artifact versus run identities | V1 | RULE, STATE; A/B/C | State implementations exist; common cross-profile contract and consumers must be extended. |
| V1-003 | One shared declared type authority; roles/messages reference model declarations without making another domain schema | V1 | MODEL, ARCH; D/C supply, A consumes | Existing typed model APIs; role/event contract needs specification. |
| V1-004 | Total typed Boolean predicates; local bindings, explicit units and guarded optional access | V1 | ADOPT, STATE; A | Existing state language; define reuse as typed leaves in temporal/protocol nodes. |
| V1-005 | Distinct parse/type/admission/lowering/execution/observation failures with typed context and located source | V1 | STATE, RULE; A/B/C | Partial existing codes; cross-profile catalog and stage behavior missing. |
| V1-006 | Requested backend capability checked separately from language admission; preserve other supported clause assessments | V1 | ARCH §8; A/B | State lowering currently bounded; no generic composed request/result contract. |
| V1-007 | Acyclic named predicate/function libraries | Later | ARCH, ADOPT; A / FS01 | Reuse through linked typed predicates is required internally; user-defined calls are outside the selected v1 source subset. |
| V1-008 | General recursion, reflection, I/O or execution of user code during clause evaluation | Refuse | RULE; A | No implicit escape hatch around total/bounded semantics. |

## State and modeling semantics

| ID | Capability and selected boundary | Decision | Evidence / owner | Existing foundation or gap |
| --- | --- | --- | --- | --- |
| V1-009 | Invariants and operation pre/postconditions with exact current/pre/post anchors, immutable captures, results and frames | V1 | ADOPT, STATE, RULE; A / FS03 | Implemented under old definition; anchor migration and original source correspondence remain required. |
| V1-010 | Inclusive finite signed-64 integer domains and checked arithmetic; exact rationals normalized before authored bounds | V1 | RULE; A / compiler #30 | Integer foundation exists; rational type/syntax/runtime/lowering missing. |
| V1-011 | Minimum-only unbounded integer declarations, including the original `ConfigVersion` minimum-only example | Refuse | RULE; A/D design gate | Cannot invent a maximum. Preserve the original example and explicit unresolved domain decision. |
| V1-012 | Integer division/remainder, floating point, NaN, infinity, saturation or implicit integer-to-rational conversion | Refuse | RULE; A | Existing historical integer division/remainder must not silently enter the new state profile. |
| V1-013 | Independently authored absence, explicit null, required presence and multiplicity preserved distinctly or explicitly refused | V1 | MODEL, RULE; D decides, C exposes, A consumes | Source presence decision remains FCD #95. No new native field-presence grammar is presumed. |
| V1-014 | Typed acyclic value containment; enums, text and exact units where their selected scalar contract admits them | V1 | MODEL, STATE; A/D | Existing state/model foundation; cross-profile payload admission needs agreement. |
| V1-015 | Finite ordered duplicate-preserving sequences; bounded quantification and collection operations needed by authored state rules | V1 | RULE, ADOPT; A | Foundation exists; declared maximum ≤10,000 and exact supported operation catalog must be retained. |
| V1-016 | Sets, bags, implicit collection conversions and recursive value containment | Later | RULE; A/D | Explicit v1 refusal; no name-based emulation with sequences. |
| V1-017 | Finite object populations, explicit references, identity comparison and exact reachability as a separately versioned state-graph facet | V1 | ARCH, ADOPT, STATE; A/D | Existing native behavior is a foundation. This is an explicit proposed promotion beyond the ruling's first state admission subset, requiring review and distinct profile identity; it does not silently amend that subset. |
| V1-018 | Record structural equality and ambient object lookup through opaque scalar IDs | Refuse | RULE; A/D | Identity references require an explicit typed, closed environment. Ordinary scalar equality never authorizes navigation. |
| V1-019 | Full OCL/KerML/SysML metamodels, unrestricted inheritance/redefinition and dynamic dispatch | Later | OWN, MODEL; A/D | Adopt only declared model features with specified correspondence; v1 is not an upstream language implementation. |

## Temporal and observation semantics

| ID | Capability and selected boundary | Decision | Evidence / owner | Existing foundation or gap |
| --- | --- | --- | --- | --- |
| V1-020 | Native bounded future operators, including eventuality, always, until and release | V1 | TL, BRIDGE; A/B/TL | TL representation exists; native grammar, typed-predicate bridge and source correspondence missing. |
| V1-021 | Native bounded past operators and explicit history boundary | V1 | TIME, TL #33; A/TL | Past-time specification is owned; no past nodes in the inspected current syntax enum. |
| V1-022 | Event-position and fixed-period sampled clocks with explicit units and no conversion of event counts into elapsed time | V1 | TIME; A / FS04, observation owners | Cross-layer clock binding missing. |
| V1-023 | Timestamped exact-clock deadlines, trigger-value capture and timeout progress during silence | V1 | TIME, PROTO; A / FS04 and #12, OB02 | Observation/replay work is planned; sparse events cannot be passed to MLTL as elapsed-time samples without a justified binding. |
| V1-024 | Closed execution, valid open prefix and incomplete observations distinguished; separate trigger activation and verdict | V1 | TIME, TLE; A/B/TL | Existing TL pending result is a foundation; missing data is not a false proposition. |
| V1-025 | Explicit late-data cutoff, completeness authority, history and retained revision/progress identity | V1 | TIME; observation owners / OB01–03 | Native binding and supersession contracts missing. |
| V1-026 | Arbitrary clock uncertainty, out-of-order repair and partial-valuation inference | Later | TIME; A / FS04 | V1 reports unavailable/ambiguous inputs without inventing a Boolean. |
| V1-027 | Infinite-trace guarantees or unbounded liveness proved from a finite prefix | Refuse | ARCH, TIME; A/B/TL | A finite observation may witness a violation; absence of one is not the claimed proof. |
| V1-028 | Native temporal source lowered through a versioned TL bridge, with typed predicates evaluated under native model rules | V1 | BRIDGE; A/B/TL | No competing TL, FRETish or numeric-to-Boolean authoring path. |

## Protocol/choreography semantics

| ID | Capability and selected boundary | Decision | Evidence / owner | Existing foundation or gap |
| --- | --- | --- | --- | --- |
| V1-029 | Finite named participant roles, runtime role instances and explicit role/component bindings | V1 | PROTO, ARCH; A / #12 | Missing native language contract; an endpoint is not necessarily a role. |
| V1-030 | Typed messages and channels, distinct send/receive, message/delivery identity and business correlation | V1 | PROTO, TIME; A/C/D | Missing native contract; a queue is a channel, not automatically a participant. |
| V1-031 | Authored finite sequence, owned labeled choice, parallel branches, join and bounded iteration | V1 | PROTO; A / #12 | Requires protocol representation and conformance semantics, not if/else dispatch in the state expression parser. |
| V1-032 | Causal edges, choice visibility and explicit delivery/duplication assumptions | V1 | ARCH §8, PROTO; A / #12 | Cannot infer causality from timestamp sorting or assume exactly-once delivery. |
| V1-033 | Per-instance obligations, immutable captured values, state/operation guards and inclusive timeout branches | V1 | TIME, PROTO; A | Cross-profile semantics missing. |
| V1-034 | Compensation registration after a successful effect, separate effect/retry/action identities, commit and recovery relation | V1 | PROTO; A / #12, PT01 | Missing native/runtime contract; refund success is not necessarily restoration of the original state. |
| V1-035 | Failed, timed-out, retried and untriggered compensation kept distinct, including duplicate business effects | V1 | PROTO, ARCH; A/B | Must have independent adverse cases and source-bound evidence. |
| V1-036 | Global replay and conformance of supplied finite traces to the selected bounded protocol | V1 | PROTO; A native integration, PT02 implementation | No qualified protocol engine is established by the inspected sources. |
| V1-037 | Sound local projection, realizability, assume-guarantee composition and implementation refinement | Later | PT02, ARCH §8; protocol/analysis owners | Separate requested claims with explicit premises. A monitorable but unprojectable protocol remains usable for v1 replay. |
| V1-038 | Dynamic role spawning, unbounded queues/loops, arbitrary cross-instance properties and distributed consensus semantics | Later | OWN, PROTO; A / #12 | Bounded per-instance choreography is v1; these expansions require later profiles. |
| V1-039 | Executing business actions or compensation from the specification evaluator | Refuse | ARCH, RULE; A | V1 observes/checks recorded actions; it is not a workflow orchestrator with external side effects. |

## Composition, evidence and mappings

| ID | Capability and selected boundary | Decision | Evidence / owner | Existing foundation or gap |
| --- | --- | --- | --- | --- |
| V1-040 | One package relates state predicates, temporal obligations and choreography nodes through typed references and shared source/model identities | V1 | OWN, ARCH; A / #10 | Cross-profile semantic package and admission rules missing. |
| V1-041 | Independent execution, truth, activation, participation, assumptions, completeness and provenance dimensions | V1 | ARCH, PROTO; A produces, B contracts, Quoin retains | Existing evidence infrastructure must be reused; no new evidence store. |
| V1-042 | State workflow, timed refund, and branching compensated choreography with fan-out/join as release examples | V1 | PLAN, TIME, PROTO; A/B/C/TL | Existing state example is useful; full composed workflows are unimplemented. |
| V1-043 | OCL, SysML/KerML and FRETish output mappings with source-bound preserved/conditional/unrepresented/refused accounting | V1 | EXPORT, RULE; mapping owners | Select bounded mapping coverage per target; no requirement that every v1 feature is expressible in every target. |
| V1-044 | External source-language imports or Java/Node/Electron semantic dependencies in production/qualification | Refuse | OWN, RULE; all owners | Generated text does not acquire source authority; existing grants remain intact. |
| V1-045 | General relational/hyperproperty language and a verification-plan DSL inside native clauses | Later | ARCH §4; A/B | Subjects retain linked identities; B's method/plan/result contracts are not duplicated in the clause AST. |
| V1-046 | Reproducible exact profile/dependency manifests and explicit migration from historical definitions | V1 | ADOPT, RULE; A/B/C | Old IDs and evidence must never be relabeled as v1. |
| V1-047 | Executable reference evaluation, reproducible finite replay and incremental monitoring of admitted temporal/protocol obligations | V1 | ARCH, TIME, technical brief; A / FS04 and #12, observation/TL owners | A monitor consumes bound observations and progress; deployment integration and external monitor qualification remain separately scoped. |
| V1-048 | Qualification-oriented conformance vectors and exact implementation/binding/tool provenance for executable specifications | V1 | ARCH §§10–12, evaluation plan; A/B and QA01 | Language conformance, monitor correctness and qualification of a consuming system are distinct claims. |

## Scope audit and long-term architecture

The owner's later scope clarification requires evaluation of inherited deferrals,
not merely transcription of the other agent's list. The v1 cut is chosen by
workflow need and semantic dependencies, not by the operators the current backend
happens to implement. The full-language roadmap stays open to later capabilities.

Three independent questions apply to every feature: what its native meaning is,
which language release admits it, and which execution/analysis/export capability
can consume it. A feature can be well-defined and useful for one method even
when another backend refuses it. Unknown syntax is not accepted as an opaque
"supported" extension.

The external family census uses primary sources: [OCL 2.4, §§7 and 11](https://www.omg.org/spec/OCL/2.4/PDF),
[KerML 1.0, §7](https://www.omg.org/spec/KerML/1.0/PDF),
[SysML 2.0 language, §7](https://www.omg.org/spec/SysML/2.0/Language/PDF),
and the [NASA FRET documentation](https://github.com/NASA-SW-VnV/fret/tree/master/fret-electron/docs).
The following Quire choices are design judgments against the program's workflows,
not claims of equivalence to those languages or promises to implement them in full.

| Family being assessed | V1 decision and reason | What must be designed now so later work does not require replacement |
| --- | --- | --- |
| OCL-like Boolean, value and contract expressions | Core v1: guards, state assertions, captured amounts and operation conditions depend on them. | One reusable typed predicate layer, with definedness separate from temporal truth and observation availability. |
| Collection queries and transformations | V1 needs more than quantifiers: specify size, membership, count, ordered filtering/mapping and exact bounded sum for aggregate and participant obligations. These are new work, not implied by existing `forall`/`exists`. | Distinguish container kind, element type, occurrence order, multiplicity and output bounds; later collection kinds cannot be aliases for sequences. |
| Tuples, derived values and named query definitions | Fixed typed records and local bindings in v1; reusable user-defined queries remain a subsequent source feature. | Stable declaration/call identities and a future effect/termination contract. Do not make every predicate an untyped string. |
| Type conformance, specialization, subsetting and redefinition | Explicitly resolved model declarations in v1; richer inherited or redefined declarations need a producer-owned normalization/correspondence contract before use. | Preserve original versus effective declaration identity; refuse ambiguity instead of copying a flattened type with lost provenance. This is a shared-model task, not a reason to fork the type system. |
| Associations, references and containment | Explicit finite graph facet proposed for v1; recursive value containment remains separate. | Keep object identity, links, containment and universe closure distinct; model navigation does not become ambient lookup. |
| Parts, ports, interfaces, connections and allocations | V1 consumes typed participant/channel/payload/component references needed by choreography. Broader systems-model authoring remains in the model producer. | Maintain role, component, endpoint, channel, build and deployment identities independently; reference producer capabilities instead of embedding a SysML metamodel in the clause parser. |
| Actions, state transitions, control/data flow | Finite action structure, choice, concurrency and iteration in v1; general execution/simulation engines are separate consumers. | Distinct behavior nodes and event contracts, with typed state predicates attached to actions and transitions. |
| Temporal scopes, triggers, response windows and history | V1 requires explicit scope boundaries, captured triggers, bounded future/past obligations and deadlines. | A clock/history/completeness contract must precede surface shorthand and lowering; do not equate end-of-file with execution closure. |
| Physical quantities and dimensional models | Exact declared units needed for amounts and time in v1; continuous/hybrid dynamics and extensive domain libraries are later. | Value representation, dimension, unit and conversion rules stay separate. A duration cannot silently become an event count. |
| Requirements, assumptions, verification links and evidence | Native obligations bind to existing requirement identities and evidence contracts in v1. | Preserve semantic subject versus verification method versus result. Do not make a clause-language feature responsible for Quoin's assurance store. |
| General local projection, refinement, realizability and hyperproperties | Later methods/subjects with explicit interfaces; v1 supports useful global execution and monitoring. | Retain owners, assumptions, visibility, trace identities and derivation edges now. Adding an analyzer must not require reauthoring the choreography or rewriting historical evidence. |
| Model metadata, views, rendering, tool APIs and library ecosystems | Consume relevant authoritative model metadata; complete upstream tool ecosystems are not a v1 clause-language deliverable. | Versioned model/package interfaces and output mappings; no foreign-runtime semantic dependency hidden in an adapter. |

Effort decisions belong at feature-family and consumer boundaries. Exact type,
identity, time, event and composition contracts are costly to retrofit and are
specified up front. Surface conveniences and independent analysis algorithms can
follow through owned tickets. No calendar or staffing estimate is invented from
these classifications. Each implementation ticket must expose its actual
dependencies and a useful independently assessable result.

## Design consequences and retained decisions

The normative specification must cover all V1 rows together before implementation
tasking is finalized. V1-017 is a deliberate proposal to admit a separately named
graph facet in the wider language; it is not a claim that the previous ruling
already admitted object equality or closure. The original minimum-only
`ConfigVersion` remains an explicit design gate under V1-011. Neither removing
its parent relationship nor manufacturing an integer maximum closes that gate.

The temporal bridge must retain separate native, TL, clock and observation
profile identities. Existing closed TL semantics may extend false propositions
beyond closure; native finite/end-of-execution semantics must select that rule
explicitly or use a different profile. Backend acceptance is not correspondence.

The implementation plan must expose the missing rational, graph-admission,
native temporal, observation, protocol and composition work. It must not use a
green state suite to mark any of those rows delivered. Broader analysis and
projection tasks remain owned roadmap work, with explicit v1 capability refusal.
