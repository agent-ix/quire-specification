# Shared v1 drafting foundation

Working draft for the owner-approved initial specification cycle. The initial
composed review and owner corrections are integrated; acceptance remains at the
PR boundary. The
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
| Concrete observation handoff | [F's binding contract](observation-contract.md): explicit role/schema/subject/anchor mapping, availability and scoped progress | F; range and result corrections integrated through `b7381fd` |
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

## Current integration and review disposition

The [current draft index](definitions/README.md) identifies the working grammar,
requirements and contracts. Use Git history while drafting; freeze one coherent
baseline at acceptance. Earlier candidate checksum tables are historical notes,
not an integration gate.

| Contribution | Integrated input |
| --- | --- |
| A | Shared grammar, predicates, package stages, producer/native correspondence and choreography surface |
| B | Protocol/results and resource-inventory correction through `57ddca2`; explicit global-conformance closure at `fe31710` |
| D | External FR-100–105, producer contract and Plan-016 at [6259d3a](https://github.com/agent-ix/filament-core-data/tree/6259d3a5b99088740df9bcc8e8d60f3720aaa603); capability ledger through `199adc3` |
| E | Temporal requirement/case/method corrections through `c6a8905` |
| F | Result preservation, resource/provenance controls and all-clock range mapping through `b7381fd` |

The [QUOIN base-plus-seven review](../../spec/reviews/composed-v1/base.md) remains
the record of the evaluated `788b161` baseline and its external inputs. The
following are subsequent correction dispositions for PR review:

| Finding | Correction |
| --- | --- |
| SR-400 FND-001 | Delivered families and current ownership are reflected in the draft and master index. |
| SR-401 FND-001/002 | D FR-102 and F FR-114 retain decisive truth with exact support despite unrelated incomplete observations. |
| SR-401 FND-003; SR-402 FND-001; SR-406 FND-002 | Exact producer decimal semantics, separate static/runtime prerequisites and explicit producer/native digest correspondence. |
| SR-406 FND-001 | F's range correspondence and TC-142/143 preserve all three clocks, record/member identity and half-open versus inclusive boundaries. |
| SR-403 FND-001 | Current unsuffixed definitions contain the latest corrected meaning and link their current dependencies; the observation contract explicitly uses D's corrected producer input. Immutable runtime selections remain required; the draft has no copied checksum selectors. |
| SR-403 FND-002; SR-404; SR-407 | E's prerequisite, deadline-case, method and wording fixes join the earlier A/B/F evidence-plan corrections. |

### PR correction disposition

[SR-408](../../reviews/2026-09-10-composed-v1-pr15.md) records the independent
review at `3972783`; its original FAIL verdict remains unchanged. The following
corrections are submitted for recheck:

| Finding | Current correction |
| --- | --- |
| SR-408 FND-001/002 | Latest corrected definition text and revision labels are promoted to the unsuffixed files. Superseded copies are removed. Protocol, temporal and state dependencies link the matching current definitions, including state graph and temporal common at draft revision `1-draft.3`. |
| SR-408 FND-003 | Per-rule and internal-definition draft digest pins are replaced with same-checkout links. Explicit external producer/compiler selections and runtime digest-domain requirements remain. |
| SR-408 FND-004 | The historical A/F integration findings are dispositioned below. Their original review observations are retained; they no longer impose checksum publication or missing-consumer-text gates. |
| SR-408 FND-005 | The capability ledger links the same current temporal/protocol files and draft revisions as the definition index. |

B's independently reviewed [global-conformance closure correction](../../spec/reviews/global-conformance-closure/base.md)
is also integrated. Its fifth closure record, including `not-required` for
non-global claims, is preserved by the current definition and observation
consumer contracts and their planned controls. It does not replace either
decision-scope or surrounding-execution progress/closure.

### Historical integration findings

These dispositions apply to the earlier B/E review sets at their recorded
revisions; they do not claim executed consumer qualification or baseline adoption.

| Earlier findings | Current disposition |
| --- | --- |
| SR-200 FND-004; SR-203 FND-003; SR-300 FND-007; SR-303 FND-004; SR-306 FND-004 | Definition integration is corrected in the current files. Old checksum/successor publication instructions are superseded by the owner's one-draft workflow. |
| SR-200 FND-005; SR-202 FND-003; SR-203 FND-002; SR-206 FND-003; SR-300 FND-006; SR-302 FND-004; SR-303 FND-003; SR-306 FND-003 | The current FR-115/116 and observation output contract preserve B/E result dimensions, exact support and supersession. The correction at `8eea9d0` additionally names each decision-scope and surrounding-execution progress/closure axis independently. |
| SR-204 FND-003; SR-304 FND-004 | TC-145/146 and IT-051 now specify consumer preservation controls, including independent omitted/cross-wired axes and complete-global-conformance closure premises. Execution of those controls remains planned; the missing test-specification gap is corrected. |

The [master index](../../spec/spec.md) includes D's inventory and mixed-version
impact requirements; the [capability crosswalk](capabilities.md#initial-specification-crosswalk)
links the 51 approved scope decisions to requirements and planned cases.
D's producer plan and FCD PR #97 need their own acceptance; #93 remains parked
until its affected contracts and plan are accepted.

Next: review the coherent PR correction set, resolve substantive findings, then
refine existing implementation tickets. No new full review campaign for each
edit, repetitive progress log, or per-document version cascade is required.
The four native matrices contain 59 planned TC artifacts, not executed evidence.
Historical accepted profiles and the original ConfigVersion admission case keep
their separate meanings. C's assurance campaign remains independent.
