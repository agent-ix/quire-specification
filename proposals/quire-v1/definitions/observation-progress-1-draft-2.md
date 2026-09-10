# Observation progress and restoration definition candidate

Selection identity: `quire.observation.progress/v1`; revision: `1-draft.2`.
**Candidate only: not adopted, serialized, canonicalized, or implemented.**

This successor retains the scoped assertion, contradiction and restoration
meanings of [1-draft.1](observation-progress.md). It adds the exact dependency
closure required to exchange assertions with the selected B/E lifecycle result
semantics. It is not a clock profile, event schema, replay engine, result store,
or executable monitor.

## Required immutable dependencies

| Dependency | Identity | Revision/version | Exact bytes or source revision |
| --- | --- | --- | --- |
| Observation binding | `quire.observation.binding/v1` | `1-draft.2` | `sha256:5f454277722e7f4816f226361fc3204cfc1a0083f10fe442ef6e93084202f63b` |
| Temporal lifecycle/result facet | `quire.temporal.bounded-facet/v1` | `1-draft.3` | `sha256:e97b0065361736289c7a37fbdcd1fd0b9af73c8af38a8de1863066179c99e197` |
| Shared result rules | B FR-061 | reviewed `2d8372c0ce22f43b313dba139880d00890f02984` | `sha256:f56b11b7ee602a5d77ace8f3ac945661042d171f4935a82c1f7da184b6661c42` |
| Producer interface | FCD Producer interface | `1.2.0` at `06a62c7624ea56bbe30120e8987d2ec4f551e77d` | `sha256:de838deae48b2b11c79019749b91057049e9ca5e80a483d9281cb1a7d2ab96ae` |

An assertion or restoration request selects every dependency by exact identity,
revision and bytes. Missing, stale, substituted, or inconsistent dependencies
refuse the assertion; they cannot default to a current clock/profile/result or
begin a fresh complete stream.

## Half-open window coverage

For the selected FCD `1.2.0` half-open window `[start, end)`, a progress or
closure assertion covers a member only when the member's selected observation
instant is `start <= instant < end` and its exact object identity is listed or
covered by the window's membership digest. An event at `end` belongs only to a
subsequent selected window; it cannot complete, contradict, or be double-counted
in this one. An event at `start` participates once. A snapshot remains distinct
from a window and uses its immutable snapshot membership digest.

The assertion retains its authority, binding, scope, temporal, record/revision
and restoration selections. O2 progress cannot settle O1. A late contradiction
identifies the prior assertion/result and emits linked supersession under the
corrected input identity without rewriting historical bytes.

## Adoption boundary

A must index this successor's exact digest before selection. The prior
`1-draft.1` bytes remain historical and do not acquire these dependencies.
