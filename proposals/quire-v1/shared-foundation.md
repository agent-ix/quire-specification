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
| Predicate reference | Declaration and type, arguments, model closure, evaluation anchor and immutable captures | A; B/E consume |
| Scope binding | Workflow/role/relationship/population identities, exact snapshot/window and membership/progress dependencies | D/B meaning; F binding; A common interface |
| Native temporal to TL | Profile meaning, typed predicate valuations, clock/history/closure, original native correspondence | E; A/F consume |
| Native protocol to conformance | Roles, messages/effects, causal structure, related instances, obligations, selected claim/assumptions | B; A/F consume |
| Observation to assessment | Exact binding/input identities, correlation, clock/progress/closure, completeness and resource controls | F; B/E consume |
| Assessment to result | Requested subject/capability and per-obligation disposition, failed prerequisites, completeness/provenance | B contract; A/E/F produce |
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
| Composed grammar, lexical rules, precedence and predicate declaration spelling | A / L1 | [Common fragment](shared-grammar.md) now drafted; integrate family productions, query/type catalogs and positive/negative cases before acceptance. No textual pattern parsing in adapters. |
| Exact integer/rational spelling and normalized bounds; anchors and alias/composite pre-state correspondence | A / FS03 | Preserve all carried-forward admission rules; explicitly scope any extension. |
| Presence independent of multiplicity and explicit null; related-instance/population types | D / FCD #95, FS02, IN01 | Name the producer contract or refuse; never invent native field-presence semantics to repair producer loss. |
| Current TL false-padding versus the old finite-window draft; clocks and past/history | E / FS04 and IR #64 | Select exact native semantics, distinguishing vectors and bridge support/refusal; no equivalence assumption. |
| Protocol structure, owned choices, parallel join, bounded repetition, compensation and conformance | B / PT01–02 with A / standard #12 | Define observable meaning and claim premises; monitoring is not general realizability proof. |
| Related-instance correlation, population/window membership, silent deadlines and late data | F / OB01–03 with D/E | Name completeness/progress authority and incomplete/refused outcomes; no inferred success from missing data. |

The original ConfigVersion minimum-only integer and parent relation stay visible
as an admission/design gate. A graph/predicate extension receives an explicit
new reviewed profile identity; it never restamps the first state profile or its
old evidence. All decisions above are specification work, not claims of coverage.

## Authoring integration

Use the reserved IDs in the cycle allocation. B/D/E/F own only their assigned
new artifact files and family matrix; A owns this document, shared requirements,
AD-001 and the master index. Submit changes to another editor's artifact as a
proposal with one concrete distinguishing case. Maintain one shared meaning.

The initial A requirements and planned acceptance cases are linked from the
[shared matrix](../../spec/composed-foundation/tests.md). Family contributions
and final grammar are still required before the complete baseline review. Run
structural validation while drafting; reserve formal reviews for coherent PR
readiness. C continues engineering-assurance independently.
