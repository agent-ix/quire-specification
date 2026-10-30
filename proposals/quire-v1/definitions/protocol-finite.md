# Native finite global protocol definition

Profile identity: `quire.protocol.finite-global/v1`.
Language: `ix:native` / `1-draft`. **The profile role is accepted by FS01; these
semantics remain proposed and is not adopted or fully implemented.**

Local rule and dependency links resolve within this checkout. Runtime references
select each definition by its immutable identity.

## Required definitions and interfaces

| Identity | Definition artifact | Consumed interface |
| --- | --- | --- |
| `quire.state.graph/v1` | [State/query/graph](state-graph.md) | Checked shared values, guards, captures and predicate references |
| `quire.temporal.bounded-facet/v1` | [Temporal meaning](temporal-common.md) | Explicit activation, bounded clock/history/progress and temporal-reference contract |

The source root admits protocol declarations. State and temporal declarations
select their own profiles; an imported declaration retains that exact selection.
Each authored timed obligation additionally selects its concrete temporal
clock/boundary profile through its `using` alias or referenced temporal clause.
The common temporal facet alone supplies no default clock interpretation.

## Admitted forms and obligations

Admit finite typed roles/relationships/channels; distinct send, receive,
operation-attempt, effect and domain-event nodes; checks; explicit causal
sequence; role-owned labeled choice with total visible exclusive/exhaustive
guards; bounded parallel branches with explicit all-branch join; and bounded
repetition with progress and an explicit exhausted path. No implicit race,
cancellation, message order, reliable delivery or exactly-once business effect
is assumed.

Admit exact related-instance annotations, finite snapshot/window inputs, awaited
events with inclusive deadlines, compensation templates with separate forward
registration and eligible activation captures, causal retries, achieved recovery
relations, commit boundaries and explicit workflow closure. Definite value
availability follows the native causal graph; a sibling or optional branch does
not supply a value before its required join/path. Repeated delivery, invocation
attempt and successful effect remain distinct identities.

All value conditions use the shared expression rules and their supplied anchors;
they cannot inspect pending monitor status, retag captures, infer population
membership or perform business actions. Forward success creates a modeled
compensation obligation, not an external compensator. A successful refund call
alone cannot establish achieved recovery or complete global conformance.

Global conformance checks the exact declared causal, channel, visibility,
correlation, activation, recovery and closure premises. All permitted branch
interleavings remain admissible. Open/incomplete histories cannot establish
complete global success. Finite replay and settled incremental results must
agree for equal admitted inputs/premises.

Projection, realizability, refinement and composition remain separately requested
claims with their own premises and backend dispositions; a finite passing trace
does not prove them. No result grants testing adequacy or evidence sufficiency
from observed conformance. C's assurance completion is not a semantic dependency.

## Checked-package state transition carrier

The profile role `protocol_profile` of the checked-package catalog also law-binds
the `quire.op.state.transition` application of a Boolean model-field
`state`/`transition` node
([FR-341](../../../spec/objects/interfaces/FR-341-checked-package-state-clause-body.md)
"State transition body"). The profile defines for it exactly one operator,
`transition`, so a state transition is not a protocol and adds no protocol
semantics: only the catalog law role requires this profile. A producer that
emits a `state`/`transition` node SHALL write the `protocol_profile` selection
of this profile into the package lock even when the package declares no
protocol, and a reader refuses a transition under a lock without it as
`operation-law-unselected`.

## Selected protocol rules

The native surface fixes concrete syntax/binding details; B's requirements own
protocol/conformance/result meaning. A disagreement is an integration defect,
not permission to choose a favorable interpretation or overwrite either owner.

| Rule | Current normative file |
| --- | --- |
| FR-049 | [Native choreography binding](../../../spec/functional/foundation/FR-049-bind-native-choreography-syntax.md) |
| FR-050 | [Protocol instances](../../../spec/functional/protocol/FR-050-bind-protocol-instances.md) |
| FR-051 | [Communication identities](../../../spec/functional/protocol/FR-051-preserve-communication-identities.md) |
| FR-052 | [Bounded control](../../../spec/functional/protocol/FR-052-represent-bounded-control.md) |
| FR-053 | [Choice visibility](../../../spec/functional/protocol/FR-053-enforce-choice-visibility.md) |
| FR-054 | [Channel premises](../../../spec/functional/protocol/FR-054-bind-channel-premises.md) |
| FR-055 | [Obligation activation](../../../spec/functional/protocol/FR-055-activate-protocol-obligations.md) |
| FR-056 | [Compensation registration](../../../spec/functional/protocol/FR-056-register-compensation.md) |
| FR-057 | [Commit and recovery](../../../spec/functional/protocol/FR-057-enforce-commit-recovery.md) |
| FR-058 | [Retry and partial recovery](../../../spec/functional/protocol/FR-058-preserve-retry-and-partial-recovery.md) |
| FR-059 | [Finite global conformance](../../../spec/functional/protocol/FR-059-assess-finite-global-conformance.md) |
| FR-060 | [Independent claims](../../../spec/functional/protocol/FR-060-separate-protocol-claims.md) |
| FR-061 | [Orthogonal results](../../../spec/functional/protocol/FR-061-report-orthogonal-results.md) |
| NFR-020 | [Protocol resource bounds](../../../spec/non-functional/NFR-020-bound-protocol-processing.md) |
| NFR-021 | [Protocol reproducibility](../../../spec/non-functional/NFR-021-reproduce-protocol-results.md) |
| Native surface | [Choreography surface](../choreography-surface.md) |
| Protocol meaning | [Protocol contract](../protocol-contract.md) |

## Result settlement and compatibility

This draft consumes the current canonical result contract, the temporal facet
and the state dependency. It retains assessment execution,
decision-scope progress, decision-scope closure, surrounding-execution progress,
surrounding-execution closure, truth, settlement basis and exact decision support
as independent facts. A completed
assessment may settle an obligation from a decisive witness or counterexample
while the decision scope and surrounding execution remain open, only when the
selected temporal meaning preserves that truth under every admitted continuation.
Missing support makes that decision unavailable; missing unrelated facts remain
visible without erasing independently settled truth. A later contradiction
creates a linked superseding or invalidating result with a corrected input
identity and preserves the prior result bytes.

This does not weaken FR-059's complete global conformance rule. A satisfied
payment obligation cannot certify the still-open workflow, an unknown population
or an unsupported independently requested claim.

The complete-global-conformance closure record is a fifth independent fact:
global claims retain `closed`, `open`, `incomplete` or `contradicted` plus the
exact required execution, branch and workflow premises. Other claims retain
`not-required` with no global premise set. Missing, duplicated, cross-wired or
scope-mismatched required records/premises refuse; none of the other four
progress/closure axes substitutes for this record.

Runtime requests retain their exact selected definition identities. Missing,
substituted or incompatible dependencies refuse their dependent request. A
changed semantic selection changes the linked subject; an existing result cannot
be relabelled under a different selection.

The canonical result now carries exact assumptions, separate observation and
protocol adequacy, and decision-support oracle dependence/lineage. Its
`resultId` identifies the canonical result under FR-286. The outgoing portable
verification projection remains
a separate requested adapter under
[FR-062](../../../spec/functional/protocol/FR-062-project-results-to-portable-verification.md).
It does not change source meaning or make E02 support a condition of native
protocol admission. An outgoing projection must retain its canonical source
result and preserve all independent target findings under the selected adapter.

The [observation result contribution](../../../spec/functional/observation/FR-115-report-activation-participation-and-adequacy.md)
and [output mapping](../observation-output-mapping-contract.md) preserve these
independent progress/closure axes, settlement, support and linked supersession.
The current [observation contract](../observation-contract.md) supplies the
producer selection and range correspondence. Draft normalization establishes no
implementation, conformance or adoption claim.
