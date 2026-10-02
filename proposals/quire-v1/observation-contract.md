# Observation binding and assessment-input contract

FS04-accepted F-owned semantic input to OB01–03 and the composed `1-draft`
baseline. This defines
the semantic content an observation source and its temporal/protocol consumers
must exchange. It does not register a wire encoding, canonicalizer, model schema,
result store, or clause authority. A owns source/package/profile identities; S3
owns the model view, including subject kind, relationship and population
declarations; E owns clock, history, and closure meaning; B owns obligations and
result dispositions.

## Contract selections

Every assessment input selects immutable identities for the native linked package,
the observation binding-contract definition, each observation
source/schema, and the applied binding. A compatible field shape, signal name,
trace ID, endpoint, or adapter version does not substitute for any selected
identity. Changing the selected binding-contract definition, including
any change to its binding meaning, requires a newly linked subject. Changing only
the concrete mapping, admitted records, or declared resource limits under the
same selected definition changes assessment-input identity, not
static meaning.

This accepted contract uses the [current shared specification](definitions/README.md)
and its [range correspondence](definitions/observation-range.md).
Runtime binding/progress identities are supplied by the linked package; this
rule document does not select itself through a copied candidate/digest table.

## Domain declaration selection

Subject kinds, relationship declarations and population declarations are the
effective domain declarations of the checked package. A spec declares them in
its own artifacts ([AD-006](../../spec/assurance/AD-006-model-graph-state-contracts.md)).
Each one is keyed by domain package identity, IR node identity and digest
domain `sha256-jcs`. The checked package's `model_selections` name the domain
package. Closed population extents are declared in spec artifacts.

Runtime observation records are assessment inputs. Their subject, relationship
and population references are typed by those declaration keys.

Each assessment input retains the selected domain package identity and digest,
the declaration key of every subject kind it uses, the relationship declaration
key where required, the population declaration key, `snapshotIdentity` or
`windowIdentity`, `configurationIdentity`, and every selected closure digest.
A declaration key outside the checked package, a missing required identity, a
digest mismatch, or an ambient environment, working-directory, current-time, or
network default refuses the dependent assessment input. A relationship display
name does not replace its declaration key.

## Observation-range correspondence

Every temporal or choreography-aware assessment additionally selects one exact
`quire.observation.range/v1` definition and one of its clock-family coverage
rows. The selected range retains the selected window identity, ordered
record-set identity, clock authority, temporal profile identity, and all
family-specific endpoints. The range is required at assessment,
not for static template checking.

Event-position coverage binds explicit sequence positions without inventing a
timestamp or elapsed time. Fixed-sample coverage binds exact epoch, positive
rational period, unit, and sample indexes without interpolation or binary
floating point. Timestamp coverage remains half-open: a witness at native
inclusive deadline 30 is outside window `[0,30)` and is admitted only by a
separately selected range that contains 30. A witness at 29 may decide an
obligation from its exact support without manufacturing unrelated coverage or
closure. Member-object identity never substitutes for record identity or the
selected position/sample/timestamp anchor.

## Observation binding

Each declared observation role has the identity of its owning native declaration,
original source region, and role kind; its local name is diagnostic text and never
a global lookup key. An assessment maps every required role exactly once to a
binding admitted under one source/schema contract. One explicit binding may be
referenced by several compatible role mappings, but equal local names never merge
roles. Each binding retains all of the following semantic values:

| Value | Required meaning |
| --- | --- |
| Binding identity | Exact binding-contract definition identity; applied binding identity; and source location of the authored role. |
| Source selection | Observation source identity, schema identity, declared source authority, and internal/external visibility. |
| Subject mapping | Exact declaration key of the model, workflow, participant, relationship, population, delivery, attempt, or effect subject selected by the role; no textual lookup fallback. |
| Signal and value mapping | Declared signal identity, typed value declaration, unit, missing-value policy, and required/optional status. |
| Anchor | The selected current, invocation pre/post, temporal origin/activation, protocol occurrence, or compensation registration/activation anchor. |
| Scope premises | Selected population declaration key and snapshot/window, membership rule/digest, configuration/closure identity and digest, exact observation-range definition/coverage row, clock/progress authority, and history premise when the role needs them. |
| Capture dependencies | Immutable capture identities and their authored anchors; a binding cannot retag a capture or import another workflow's values. |

Repeated mappings for one required role, incompatible mappings, selected identities
outside the linked package, and source/schema or unit drift refuse the dependent
request. A valid mapping with no required record remains an assessment input; it
does not manufacture a value, relationship, or future event.

## Admitted records and progress

An admitted record retains its record identity, selected binding, source/schema
identity, selected subject handle, typed value/presence state, event time and
ingestion time. The record does not establish causal order solely from timestamps.
It is classified as internal or externally observed without treating either class
as complete evidence by default.

A progress or closure assertion additionally retains its authority identity, the
same declared scope as the affected population/window, the E-owned clock meaning,
and its asserted boundary. A progress record for O2 or another window cannot
settle O1. EOF, a quiet transport, or a record timestamp is not progress/closure
authority.

The current [progress definition](definitions/observation-progress.md),
`quire.observation.progress/v1`, preserves the assertion, binding, scope,
E-owned temporal, record, and restoration selections, including
half-open window coverage `[start, end)`: `start` participates once and
`end` belongs only to a later selected window. This is part of the current shared
contract; no further definition catalog publication gates its review. Runtime
assertions retain the exact accepted definition selection independently from
their own assertion identity.

## Completeness and assessment handoff

For every requested obligation, F's handoff supplies the selected binding and
admitted-record identities plus one explicit availability state:

| State | Meaning |
| --- | --- |
| available | All required observations and selected scope/progress premises for this handoff are present and structurally admitted. This is not a truth or execution claim. |
| incomplete | A required observation, relationship, population member, external response, history, progress, or closure premise is unavailable; retain each missing identity/reason. |
| refused | Binding, source/schema, subject, unit, role, scope, or resource admission failed; retain the typed cause and affected identity. |

The handoff never collapses incomplete/refused to `false`, optional-none, an
untriggered obligation, or a Boolean result. It passes B's result adapter only
independent execution, activation, participation, completeness, provenance, and
logical facts; it does not create a second result/evidence family. A replay or
incremental assessment may settle only under E/B semantics and declared authority.

## Consumer obligations and planned controls

The linked-package binder consumes this contract through the exact role selection
in [FR-035](../../spec/functional/foundation/FR-035-bind-ecosystem-subjects.md). The
observation family requirements [FR-110](../../spec/functional/observation/FR-110-bind-typed-observations.md)
through [FR-116](../../spec/functional/observation/FR-116-map-observation-results-to-consumers.md)
govern its admission, completeness, replay, result, and output effects.

Planned controls use concurrent O1/O2, split shipments, a payment attempt/effect,
a refund, a duplicate receipt, missing provider evidence, cross-bound progress,
quiet deadline, late record, missing history, and exhausted retention. They are
specified in [TM-007](../../spec/observation/tests.md), not executed evidence.
