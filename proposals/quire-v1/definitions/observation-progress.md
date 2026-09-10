# Observation progress and restoration definition candidate

Selection identity: `quire.observation.progress/v1`; revision: `1-draft.1`.
**Candidate only: not adopted, serialized, canonicalized, or implemented.**

This definition selects the F-owned assertion and restoration boundary used with
the [observation binding definition](observation-binding.md). It is not a clock
profile, event schema, replay engine, result store, or executable monitor. E
owns the meaning of selected clocks, history, and closure; B owns the resulting
dispositions; D owns the selected subjects, relationships, and populations.

## Required assertion identity

Every progress, completeness, closure, late-contradiction, or restoration
assertion retains all of the following immutable selections:

| Value | Required meaning |
| --- | --- |
| Assertion identity | Assertion kind, unique assertion identity, and exact `quire.observation.progress/v1` identity/revision/definition bytes. |
| Binding selection | Selected `quire.observation.binding/v1` identity, revision, definition bytes, and applied role/binding identities. |
| Scope selection | Exact selected workflow/subject/relationship handles, population or event-time-window identity, membership rule, and required source set. |
| Temporal selection | E-owned clock/profile identity, revision, definition bytes, inclusive asserted boundary, and history/closure premise. |
| Record and revision | Admitted-record identities, ingestion/order information, and the retained observation/history revision to which this assertion applies. |
| Restoration selection | When continuing from retained state, exact predecessor assessment/checkpoint identity, retained-state revision, resource-limit identity, and replay basis. |

No local clock name, timestamp, EOF marker, quiet transport, trace identifier,
or matching field shape can substitute for these selections. A progress assertion
for O2, another source set, or another window cannot settle O1.

## Validity, contradiction, and restoration boundary

A valid assertion is an observation input, not a result. It can establish no
truth, activation, participation, completeness, causality, or execution beyond
what the selected E/B contracts permit. A late record contradicting an assertion
identifies the affected assertion and historical result revision; it requests
supersession or invalidation through E/B without rewriting prior result bytes.

Restart or incremental continuation without the selected predecessor/history
and restoration basis is incomplete or refused. It cannot begin a fresh complete
stream, silently replace prior history, or infer a progress boundary. Resource
limits and record-set changes remain identified assessment inputs under the same
definition selection; changing this definition's identity, revision, or bytes
requires a new linked subject.

## Adoption boundary

A must index this candidate and its exact byte digest in the shared definition
catalog before a composed baseline can select it. This candidate has no wire
layout and does not authorize replay, restoration, native parsing, or a monitor
implementation.
