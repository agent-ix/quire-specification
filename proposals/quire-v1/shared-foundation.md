# Shared v1 drafting foundation

Draft input for the owner-approved initial specification cycle. This checkpoint
enables parallel specification authoring; it is not an accepted profile, a full
grammar, an implementation instruction, or a reviewed baseline. The
[capability inventory](capabilities.md) and
[cycle allocation](https://github.com/agent-ix/quire-research/blob/roadmap/native-language-foundation/initial-spec-cycle.md)
govern scope and artifact ownership.

## Common enterprise example

An order service accepts orders O1 and O2. Inventory reserves stock. A payment
provider, external to the enterprise, records attempts and successful effects.
Fulfillment splits O1 into shipments S1 and S2. A failed branch can create a
refund obligation for the matching payment effect. An observed refund is an
effect with its own identity; a retry or repeated receipt is not automatically
another charge/refund. Evaluators check observations and do not execute actions.

Related-instance examples connect order → payment attempt → payment effect,
order → shipment and refund → compensated effect. Selected population examples
check total active reservations in a declared snapshot and total related refund
amounts in a declared event-time window. Neither example assumes that one trace
ID identifies all business relationships or establishes complete membership.

Use independently specified expected outcomes for: wrong-order correlation;
missing external-provider observations; two concurrent orders sharing a provider;
split fulfillment with a failed branch; duplicate delivery versus duplicate
business effect; absent refund; failed/timed-out/untriggered compensation;
unknown population membership; and replay versus incremental settled results.
Numeric domains, deadlines and population bounds in each concrete case must be
authored explicitly. Do not infer a bound from example data.

## Vocabulary and authorities

| Term | Shared meaning / authority |
| --- | --- |
| System/component | An internal or external participant implementation, distinct from its protocol role and deployed version. D owns configuration bindings. |
| Role / role instance | An authored protocol responsibility / a concrete participant bound to it. B owns protocol meaning; D supplies configuration identity. |
| Workflow instance | A concrete instance of an authored process, with identity distinct from a role, message, operation invocation or telemetry trace. B owns protocol binding. |
| Relationship | An explicitly typed relation between identified instances, with source/model authority. D defines domain relationships; F establishes observed correlation. |
| Population scope | A declared finite snapshot or event-time window with membership criteria and bound identity. D owns model/configuration; F supplies completeness/progress evidence. |
| Observation | A supplied record interpreted under a separately identified binding. F owns transport-independent binding and provenance. |
| Message / delivery / attempt / effect | Distinct authored/observed subjects. Repeated delivery alone cannot prove another attempt or successful business effect. B defines obligations; F binds records. |
| Capture / anchor | Immutable values and the exact evaluation instant/environment of a predicate or obligation. A owns shared bindings; E/B define temporal/protocol use. |
| Clock / progress / closure | Time interpretation / an explicit advancement assertion / a declared execution-history boundary. E owns temporal meaning; F owns observation authority and late-data behavior. |
| Result | Scoped facts about a requested assessment, with execution, truth, activation, completeness and provenance kept distinct. B owns shared result contracts. |

Native Quire owns the editable clause. Domain declarations remain owned by the
existing model producer; no family declares a parallel model schema. Quoin
retains evidence authority. C's engineering-assurance library is a reusable
consumer dependency, not the source of language meaning or an evidence store.

## Interfaces to agree together

These are semantic contract boundaries, not a frozen serialization layout or
permission to create new registries. Existing versioned interfaces are reused
where they preserve the required distinctions.

| Boundary | Required identity and information | Editor / consumer |
| --- | --- | --- |
| Source to native syntax | Exact bytes/revision/digest, byte spans, clause identities, one language/edition | A; all families consume |
| Model to linker | Exact declaration/package closure, declared types, source correspondence; explicit presence/refusal | D producer; A consumes |
| Linked package | Exact profile definitions, typed declarations and family dependency graph; source/model identity; no mutable monitor state | A; B/E/F consume |
| Package to assessment | [Static components and typed binding roles](package-contract.md); full configuration retained separately from concrete observations, clocks, progress and resource controls | A consumer contract; D/F/B supply exact interfaces |
| Predicate reference | Declaration and type, arguments, model closure, evaluation anchor and immutable captures | A; B/E consume |
| Scope binding | Workflow/role/relationship/population identities, exact snapshot/window and membership/progress dependencies | D/B meaning; F binding; A common interface |
| Native temporal to TL | Profile meaning, typed predicate valuations, clock/history/closure, original native correspondence | E; A/F consume |
| Native protocol to conformance | Roles, messages/effects, causal structure, related instances, obligations, selected claim/assumptions | B; A/F consume |
| Observation to assessment | Exact binding/input identities, correlation, clock/progress/closure, completeness and resource controls | F; B/E consume |
| Concrete observation handoff | [F's binding contract](observation-contract.md): explicit role/schema/subject/anchor mapping, availability and scoped progress | F; through `921993b`, successor immutable closure tracked by SR-403 |
| Assessment to result | Requested subject/capability and per-obligation disposition, failed prerequisites, completeness/provenance | B contract; A/E/F produce |
| Native diagnostic interpretation | [Retained codes and typed causes](definitions/native-diagnostics.md), actual producer stage, original loci and independent response-catalog identity | A; consumes existing compiler Code vocabulary and result envelopes |
| Export | Selected mapping/profile and source-bound preserved/conditional/unrepresented/refused obligations | F coordinates; D/E contribute; A identity |

Partial execution is allowed only for dependency-closed admitted subjects. A
missing shared model/profile prevents every dependent clause from executing; an
unsupported independent backend request does not erase a valid state result.
Nothing converts temporal pending or missing observation into an ordinary Boolean.

## Architecture and evolution

Retain the existing lexer and structured parser. Publish one native grammar;
parse shared expressions once, then retain typed family ASTs, source spans and
family-specific checking. Resolution/type checking produce shared predicates
and specialized state/temporal/protocol representations. Backend admission is
separate from language admission. Formatting uses the parsed syntax structures.

Separate language edition, profile meaning, wire/canonicalization definition,
backend capability and implementation release identity. Exercise addition of
an operator, declaration and semantic family without changing the meaning of an
old selected profile. No speculative plugin system, CST rewrite or parser
generator is selected by this checkpoint.

## Required decisions during drafting

| Decision | Owner | Required disposition before affected implementation |
| --- | --- | --- |
| Composed grammar, lexical rules, precedence and predicate declaration spelling | A / L1 | [One composed grammar](shared-grammar.md) now includes all three families, common query leaves and capture scopes; reconcile its model/observation bindings and examples before acceptance. No textual pattern parsing in adapters. |
| Exact integer/rational spelling and normalized bounds; anchors and alias/composite pre-state correspondence | A / FS03 | [State contract](state-contract.md) now proposes exact arithmetic, queries and capture/anchor rules; preserve carried-forward admission and explicitly adopt extensions. |
| Presence independent of multiplicity and explicit null; related-instance/population types | D / FCD #95, FS02, IN01 | D's FR-100–103 and baseline 1.2 at `d6ac501` are delivered; resolve the concrete stage/digest/window findings below without inventing native model semantics. |
| Current TL false-padding versus the old finite-window draft; clocks and past/history | E / FS04 and IR #64 | [FR-090–095](../../spec/functional/FR-090-select-temporal-profile-and-clock.md) now distinguish false-extension and timestamped finite-window meanings; current TL refuses unsupported timestamped/past mappings. Native syntax is drafted; final definition identities and review remain. |
| Protocol structure, owned choices, parallel join, bounded repetition, compensation and conformance | B / PT01–02 with A / standard #12 | [Protocol contract](protocol-contract.md) and revised results are integrated through B's PR #21 (`2d8372c`); A's [native surface](choreography-surface.md) supplies control, value scope and registration/activation rules. SR-401/402/406 identify remaining cross-family conflicts. Global conformance is not a realizability proof. |
| Related-instance correlation, population/window membership, silent deadlines and late data | F / OB01–03 with D/E | FR-110–116 and [observation cases](../../spec/observation/tests.md) are integrated through F's `921993b`. SR-401/403/404/406 identify the remaining replay, selection, evidence-plan and window-mapping corrections. |

The original ConfigVersion minimum-only integer and parent relation stay visible
as an admission/design gate. A graph/predicate extension receives an explicit
new reviewed profile identity; it never restamps the first state profile or its
old evidence. All decisions above are specification work, not claims of coverage.

## Authoring integration and review disposition

B/D/E/F own their family artifacts; A owns the shared grammar, architecture,
master index, catalog and composed integration. Reserved IDs are not a quota.
D's externally owned FR-100–103 are indexed directly instead of being duplicated
under reserved standard FR-070–089.

The [complete QUOIN review](../../spec/reviews/composed-v1/base.md) evaluates
`788b161bd1b7fd5d632cc42c949f497914df3ce6` with the exact inputs below.
Base plus all seven analyses are complete; **changes are required before
baseline acceptance**. Review artifacts are retained at
`6e589dbf2fb8b2349903cbb4530ec17d78d2ef60`. Their findings replace blanket
“undelivered family” gates.

| Remaining correction | Owner / review |
| --- | --- |
| Static type-checking versus runtime population/window prerequisites | D/A; SR-402 FND-001 |
| Preserve decisive truth with complete exact support despite unrelated unavailable data | D with B/E/F; SR-401 FND-001 |
| Preserve decisive satisfied and violated results on an open subject in replay/incremental assessment | Resolved in F `03bda33`; SR-401 FND-002, FR-114 and TC-144 |
| Exact canonical numeric domain and producer-object/native-byte correspondence | D/A; SR-401 FND-003 and SR-406 FND-002 |
| Map producer population/window coverage to each selected temporal clock, preserving endpoint and record identity | F with D/E/A; SR-406 FND-001 |
| Publish successor observation selections with immutable reachable rules | F/A after the affected corrections; SR-403 FND-001 |
| Reconcile existing case/metric/method bindings and temporal wording/prerequisite | B/E/F; SR-403 FND-002, SR-404, SR-407 |

SR-400 FND-001's obsolete delivery status is corrected in this navigation and
the capability crosswalk. A accepts SR-404 FND-012/013's unit-testing dispositions
for the existing TC-039/042 controls; the authored Test class remains valid.
Those dispositions select planned methods, not executed evidence.

B's `57ddca2dddd637789a50b4e78977cf87c939da20` is integrated. SR-404 FND-001
is resolved at the specification level by NFR-020's inspection method, the
independent source-site inventory in TC-065 and its TM-004 binding. The added
case brings the four matrices to 59 planned TC artifacts; no inventory has
yet been executed against a new protocol implementation.

A's [package correction](package-contract.md#prerequisites-by-processing-stage)
now supplies the stage table and explicit producer/native identity correspondence,
with FR-030-AC-9/10, FR-035-AC-9, TC-030/035 and IT-010-SC-07/08. This completes
the concrete native consumer proposal for SR-402 FND-001 and SR-406 FND-002;
matching D producer semantics remain required before those shared findings close.

F's `03bda33af53ea28c2badb21f22d3d119fd32e6c9` is integrated exactly.
FR-114/TC-144 resolve SR-401 FND-002's settled-open replay contradiction.
NFR-050/051/052 and TC-142–146 resolve SR-404 FND-006/007/008/010/011/014/015
through independent resource measurements, catalog methods and explicit consumer
controls. SR-404 FND-009 is partially addressed: TC-145/146 now omit individual
provenance identities, but TM-007's NFR-051 row still needs their explicit binding.
These are specification/case dispositions; no replay, resource or provenance
implementation evidence is claimed.

The [capability crosswalk](capabilities.md#initial-specification-crosswalk)
links all 51 approved decisions to draft requirements and planned cases.
Twenty-two versioned definition candidates and all earlier candidate bytes are
preserved. State remains `1-draft.2`; temporal `1-draft.3` selects E's
`19bbf7a` rules, and protocol `1-draft.4` selects B's `2d8372c` results.
Outgoing verification projection under FR-062 is independent of native admission.

B/E/F now preserve the shared distinction: `eventually[0,30] holds(p)` may
settle true at an admitted witness while its subject remains open. Execution,
truth, exact support, progress, closure, completeness and adequacy remain
separate; complete global conformance has its own closure premises. The review
identifies the remaining FR-114 and D blanket-rule contradictions to that meaning.

Integrate owner corrections and validate the affected artifacts, then close
successor definition selections and refine existing compiler #35–39 and
ecosystem tickets. Do not repeat the full review on each edit. C's
engineering-assurance completion is independent.

## Composed review input

F's result-consumer correction through `921993bbbef92a8010aec46d46277498de5e0288`
is integrated. FR-115/116, TC-145/146, IT-051 and the output mapping retain
settlement/support, immutable selections and linked historical supersession.
D has delivered the standard-facing
[model/configuration/population/window binding contract](https://github.com/agent-ix/filament-core-data/blob/d6ac501dc4f7514aaa9973d83b049eb2e4b15e7f/docs/semantic-data-system/baseline-1-2.md)
at `d6ac501dc4f7514aaa9973d83b049eb2e4b15e7f`, exact document SHA-256
`d4923f378a7019998844f1b9124d5b0242f46c76060218b2d9c05af7b68c3ed8`.

F's subsequent `08166a28b72b5a1d87d68a54cbdc25d41d976245` is also integrated.
Both 1-draft.2 observation successors are indexed and their exact digests are
verified; FR-112/113, TC-142/143 and TM-007 retain its half-open-boundary controls.
This partially addresses SR-403/406: inherited observation-rule snapshot
selection, the explicitly retained older D pin, and correspondence to all native
clocks/inclusive intervals still need resolution. The frozen review is not
rewritten to claim it evaluated this later contribution.

D's producer-owned FR-100/101/102/103 supply presence, relationship, population
and configuration requirements. The master index references those exact artifacts.
All families have review input; delivery alone does not establish agreement
or adoption. F's older producer pin at `06a62c7` and its 1-draft.1 definition
files stay unchanged until an explicit successor selection. D's canonical object
digests remain distinct from A's raw model/document and definition-byte digests;
an equal `sha256:` prefix does not identify the domain.
