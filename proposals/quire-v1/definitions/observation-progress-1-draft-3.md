# Observation progress and restoration definition candidate

Selection identity: `quire.observation.progress/v1`; revision: `1-draft.3`.
**Candidate only: not adopted, serialized, canonicalized, or implemented.**

This successor retains `1-draft.2`'s scoped assertion, contradiction and
restoration meanings. It selects the range-aware binding and the exact
observation-rule snapshot required for all-clock progress/coverage checks.
Earlier candidates remain byte-exact.

## Required immutable dependencies

| Dependency | Identity | Revision/version | Exact bytes or source revision |
| --- | --- | --- | --- |
| Observation binding | `quire.observation.binding/v1` | `1-draft.3` | `sha256:63988fb5f38101d4b7ee785b21828be2c7800b7d874ada2b1b84353a743bd239` |
| Observation-range correspondence | `quire.observation.range/v1` | `1-draft.1` | `sha256:9ed75412559fdd34d1501f88427d68f1583510845b509845849224a56596c1fc` |
| Observation binding rules | `quire.observation.contract/v1` | reviewed range-mapping snapshot | `sha256:5a9b86cc623140b13cca1a691ee9465c101381c176f0238fcb5fb49c82d24466` |
| Temporal lifecycle/result facet | `quire.temporal.bounded-facet/v1` | `1-draft.3` | `sha256:e97b0065361736289c7a37fbdcd1fd0b9af73c8af38a8de1863066179c99e197` |
| Shared result rules | B FR-061 | reviewed `2d8372c0ce22f43b313dba139880d00890f02984` | `sha256:f56b11b7ee602a5d77ace8f3ac945661042d171f4935a82c1f7da184b6661c42` |
| Producer interface | FCD Producer interface | `1.2.0` at `cb740adc80fdc44aaf2c6cce0553344228c18784` | `sha256:05258d455c78d6b72d1fa4c063078ac5277d4ebca627cfe965ae7fee5c31a570` |

An assertion or restoration request selects every dependency by exact identity,
revision and bytes. Missing, stale, substituted, or inconsistent dependencies
refuse the assertion; they cannot default to a current clock/profile/result or
begin a fresh complete stream.

## Range-aware progress and closure

Every assertion retains its authority, binding, scope, temporal profile,
selected range, record/revision and restoration selections. O2 progress cannot
settle O1. A late contradiction identifies the prior assertion/result and emits
linked supersession under the corrected input identity without rewriting
historical bytes.

For an event-position range `[start,end)`, progress covers only the selected
integer positions and silence supplies no new position. For a fixed-sample range,
progress covers exact indexed samples under the selected epoch/period/unit,
including silent samples only when their required valuations are supplied. For a
timestamp range, a native timestamp tick is covered only when it lies in the
producer's half-open interval. A timestamped native inclusive endpoint equal to
the producer end is not covered or closed by that range. Start participates once;
end belongs only to a separately selected range that contains it.

Member identity does not substitute for record identity, and record identity
does not substitute for a position/sample/timestamp anchor. Missing required
coverage remains incomplete; wrong authority, family, endpoint, range, or
identity refuses. A decisive record at 29 may preserve its exact support while
unrelated coverage remains incomplete.

## Adoption boundary

A must index this successor's exact digest before selection. The prior
`1-draft.1` and `1-draft.2` bytes remain historical and do not acquire these
dependencies.
