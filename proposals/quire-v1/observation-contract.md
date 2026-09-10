# Observation binding and assessment-input contract

Draft F-owned input to OB01–03 and the composed `1-draft` baseline. This defines
the semantic content an observation producer and its temporal/protocol consumers
must exchange. It does not register a wire encoding, canonicalizer, model schema,
result store, or clause authority. A owns source/package/profile identities; D
owns model, relationship, population, and configuration identities; E owns clock,
history, and closure meaning; B owns obligations and result dispositions.

## Contract selections

Every assessment input selects immutable identities for the native linked package,
the observation binding contract, each observation source/schema, and the applied
binding. A compatible field shape, signal name, trace ID, endpoint, or adapter
version does not substitute for any selected identity. A change to binding meaning
requires a new linked subject; changing admitted records or declared resource
limits changes assessment input identity, not static meaning.

## Observation binding

One binding maps one declared observation role to records admitted under one
source/schema contract. It retains all of the following semantic values:

| Value | Required meaning |
| --- | --- |
| Binding identity | Exact binding contract identity, binding revision, and source location of the authored role. |
| Source selection | Observation source identity, schema identity/revision, declared producer, and internal/external visibility. |
| Subject mapping | Exact D/B-owned workflow, participant, relationship, population, delivery, attempt, or effect handle selected by the role; no textual lookup fallback. |
| Signal and value mapping | Declared signal identity, typed value declaration, unit, missing-value policy, and required/optional status. |
| Anchor | The selected current, invocation pre/post, temporal origin/activation, protocol occurrence, or compensation registration/activation anchor. |
| Scope premises | Selected snapshot/window, membership rule, clock/progress authority, and closure/history premise when the role needs them. |
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
in [FR-035](../../spec/functional/FR-035-bind-ecosystem-subjects.md). The
observation family requirements [FR-110](../../spec/functional/FR-110-bind-typed-observations.md)
through [FR-116](../../spec/functional/FR-116-map-observation-results-to-consumers.md)
govern its admission, completeness, replay, result, and output effects.

Planned controls use concurrent O1/O2, split shipments, a payment attempt/effect,
a refund, a duplicate receipt, missing provider evidence, cross-bound progress,
quiet deadline, late record, missing history, and exhausted retention. They are
specified in [TM-007](../../spec/observation/tests.md), not executed evidence.
