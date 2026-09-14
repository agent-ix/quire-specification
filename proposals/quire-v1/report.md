# Quire language foundation — architecture findings and approved roadmap

Owner-approved roadmap: 2026-09-10. This report records the completed assessment
and its selected ticket changes. It is not a claim that detailed language
specifications have passed review or that new runtime capabilities exist.

## Recommendation

Keep native Quire and extend the existing compiler into a composed state,
temporal and choreography language. The error was treating the first finite-state
development profile as the product boundary. The inspected compiler already has
a Logos lexer, structured parser, linking/checking stages, immutable packages and
independent state evaluation; those are useful foundations.

There is **one ecosystem roadmap**, owned by
[research program #36](https://github.com/agent-ix/quire-research/issues/36).
Its [implementation plan](https://github.com/agent-ix/quire-research/blob/roadmap/native-language-foundation/implementation-plan.md)
is the ticket map. This report supplies language/architecture findings to that map.
Existing protocol, observation, integration, verification and qualification owners
retain their work; no parallel roadmap or competing engine is introduced.

## Evidence and architecture assessment

The [capability matrix](capabilities.md) retains the source inventory and individual
decisions. The assessment used maintained quire-research architecture at
3df57c91773d8d64b1431649aeafc422a87d6578, native compiler source at
f444d03c06539a6cd0ada6be4ae099b54466d9d9 (including unlanded work), and the pinned
TL syntax/evaluator sources linked there. GitHub ticket states were reread during
the 2026-09-10 roadmap amendment. Source inspection is not executed qualification.

| Finding | Selected response |
| --- | --- |
| A real lexer and structured parser already exist. | Retain Logos and recursive-descent/precedence parsing; publish the grammar and share expression parsing across family modules. No parser-generator rewrite is justified by this assessment. |
| The parser admits one hard-coded state profile. | Separate grammar recognition, semantic profile admission and requested backend support through a composed package. |
| Existing linking, checking, source spans and immutable packages are useful. | Extend their typed contracts; preserve original bytes, historical semantic identities and explicit migrations. |
| Future-time TL infrastructure exists; native correspondence and required past-time support are gaps. | Use the owned native-to-TL bridge contracts and existing TL work. Do not infer native support from a TL enum or successful parse. |
| TL closure semantics differ from the earlier native finite-window draft. | Preserve each exact profile meaning; a different interpretation requires reviewed correspondence and distinguishing examples. |
| Ecosystem behavior crosses system and workflow-instance boundaries. | Make explicit related-instance and selected population/window scopes v1; a universal trace ID is not an adequate semantic model. |
| Observation quality determines the meaning of timed/protocol results. | Specify correlation, captures, causal order, clocks, progress, membership and completeness before relying on monitor verdicts. |
| State truth, temporal pending and protocol state are different. | Keep specialized typed representations/evaluators while sharing predicates and identity. |
| Protocol monitoring and compositional proof are different claims. | Deliver supported conformance/monitoring; preserve visibility, ownership and assumptions for later scoped analysis. |
| Existing verification contracts are completed. | Reuse VP02/VP03 without reopening their historical implementation; track new consumer integration through its existing expansion epic #13. |

Lexer, parser, AST and IR are successive stages, not alternatives. The lexer
recognizes tokens; the parser applies grammar and records syntax with locations.
Linking resolves names and checking establishes types, units and definedness.
State, temporal and protocol representations then support their respective
consumers. A temporal or choreography guard references a checked shared predicate,
rather than carrying another arithmetic language.

The existing foundations are partly implemented; the cross-family and ecosystem
interfaces remain specification/implementation work. The assessment supports the
architecture direction, not a claim that extensibility has already been proven.

## Selected v1 and evolution boundary

V1 contains exact bounded integer/rational values, explicit units/definedness,
operation contracts, bounded sequence queries and acyclic named Boolean predicates.
An explicitly adopted graph profile supplies finite references, identity and
reachability. Temporal clauses include bounded future/past obligations and distinct
event-position, sampled and timestamped semantics. Protocols include roles,
messages, owned choices, parallelism/joins, bounded repetition, deadlines and
compensation with separate attempt/delivery/effect identities.

Enterprise scope includes internal/external systems and relationships among orders,
payment attempts, split shipments and partial refunds. Selected population
obligations use declared finite snapshots or event-time windows. Reservation totals
and related refund totals are acceptance examples. Unknown membership or incomplete
observations cannot establish a successful aggregate.

FS01 and FS05 specify evolution now: edition/profile/wire identities, keyword and
syntax compatibility, typed family boundaries, explicit capability discovery,
migration and preservation of historical evidence. Extension scenarios cover a new
operator, declaration and family. This groundwork does not require speculative
plugins, a JIT, an editor CST or every future analysis algorithm.

Future pure functions/collection kinds, richer model/numeric domains and broader
dynamic/history queries have named research tickets in the existing program.
Related-instance choreography is not deferred. General projection, realizability,
synthesis and relational experiments retain their existing owners and do not
blanket-block v1.

OCL contributes value/contract concepts; FRET contributes temporal requirement
structure; KerML/SysML contribute model and interaction concepts. Quire selects
coherent semantics against its ecosystem use cases rather than cloning their
whole tooling or metamodels. Primary sources:
[OCL 2.4](https://www.omg.org/spec/OCL/2.4/PDF),
[NASA FRET](https://github.com/NASA-SW-VnV/fret),
[KerML 1.0](https://www.omg.org/spec/KerML/1.0/PDF),
[SysML 2.0](https://www.omg.org/spec/SysML/2.0/Language/PDF).

## Routed work and acceptance

The approved new tickets are standard #13 (grammar), compiler #35–39 (composition,
predicates/queries, graph, temporal and choreography compilation), and research
#65–67 (three future studies). Their exact parents and acceptance prerequisites
are recorded in the single roadmap, not duplicated here.

Standard #12 owns authored choreography meaning; research PT01 (#47) owns protocol
representation/consumer contracts. OB01–03 own bindings, replay, incremental
monitoring and adequacy. IN01/IN02 own configuration and compatibility. LC05 and
QA01 retain integration/conformance ownership. The original compiler diagnostic,
CLI and admission tickets remain reused work. Public IR #52/#55–57/#63/#64 consistently describe native integration and output-only
mappings, without private roadmap links. QA01–03 cover composed conformance,
full-v1 release completeness and related-instance/population resource limits;
verification expansion #13 retains new consumer qualification ownership.

Specify the shared semantic baseline together and run the selected QUOIN
base-plus-seven review set at coherent PR readiness. Then implement packages and
shared types, family compilation, consumers and integration in dependency order.
Only actual dependencies gate engineering; future research is not a delivery gate.

The common fixture is order/payment/inventory/fulfillment with an external
provider, concurrent orders, split shipments, retries, refunds and compensation.
Independent expected cases distinguish valid/violating/refused/incomplete/pending,
wrong correlation/membership, missing provider observations, duplicate effects,
aggregate constraint failures, causal ordering, silent deadlines, failed/untriggered
recovery, replay/streaming agreement and mixed-version impacts.

## Retained limitations

The minimum-only ConfigVersion integer cannot be admitted by inventing a maximum
or removing its parent relation. FCD #95 has accepted the producer-owned static
presence decision; FCD #93 retains its Semantic IR/frontend delivery and no
native presence syntax is inferred. Graph and reusable-predicate additions
require explicit reviewed profile amendments; roadmap approval does not rewrite
historical semantics.

Existing draft architecture and requirements remain proposed. Their detailed
temporal/profile choices must be reconciled with this roadmap in the specification
tickets. This amendment did not run /spec-review or a Cargo qualification campaign,
implement grammar/runtime features, deploy monitors or execute business actions.

All new first-party production/qualification paths remain Rust. Local testing is
resource-conscious and hosted workflows remain manual-dispatch only. Private
research and historical source licenses remain preserved.
