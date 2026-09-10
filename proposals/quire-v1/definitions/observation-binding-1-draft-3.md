# Observation binding definition candidate

Selection identity: `quire.observation.binding/v1`; revision: `1-draft.3`.
**Candidate only: not adopted, serialized, canonicalized, or implemented.**

This successor retains `1-draft.2`'s role-sharing, correlation, availability,
static-versus-assessment-input, and independent-result-axis meanings. It retains
the package closure compatible with the selected temporal/protocol roots and
adds the explicit all-clock observation-range correspondence. Earlier candidates
remain byte-exact and do not acquire these dependencies.

## Required immutable dependencies

| Dependency | Identity | Revision/version | Exact bytes or source revision |
| --- | --- | --- | --- |
| Linked package interpretation | `quire.package.composed/v1` | `1-draft.1` | `sha256:0df176dbb831117bd27443f7ef8075d9a130bcbc225022b440acbce111c62b9d` |
| Observation-range correspondence | `quire.observation.range/v1` | `1-draft.1` | `sha256:9ed75412559fdd34d1501f88427d68f1583510845b509845849224a56596c1fc` |
| Observation binding rules | `quire.observation.contract/v1` | reviewed range-mapping snapshot | `sha256:5a9b86cc623140b13cca1a691ee9465c101381c176f0238fcb5fb49c82d24466` |
| Temporal lifecycle/result facet | `quire.temporal.bounded-facet/v1` | `1-draft.3` | `sha256:e97b0065361736289c7a37fbdcd1fd0b9af73c8af38a8de1863066179c99e197` |
| Protocol result/conformance root | `quire.protocol.finite-global/v1` | `1-draft.4` | `sha256:721e1c0541eea01db4119c3ee990494b58af140ad90adec63ef6e4b2f6fc5270` |
| Producer interface | FCD Producer interface | `1.2.0` at `cb740adc80fdc44aaf2c6cce0553344228c18784` | `sha256:05258d455c78d6b72d1fa4c063078ac5277d4ebca627cfe965ae7fee5c31a570` |
| Shared result rules | B FR-061 | reviewed `2d8372c0ce22f43b313dba139880d00890f02984` | `sha256:f56b11b7ee602a5d77ace8f3ac945661042d171f4935a82c1f7da184b6661c42` |

An assessment selects every listed dependency by exact identity, revision and
bytes. Missing, substituted, stale, or mutually inconsistent selections refuse
the dependent assessment. The observation-rule snapshot is an immutable rule
selection, not a mutable link to current working-tree prose.

The selected temporal and protocol roots still require the earlier shared
package/edition closure. Combining this candidate with package `1-draft.2`
refuses rather than silently mixing revisions. A future successor may select
the corrected package only after compatible temporal and protocol roots exist.

## Retained binding boundary and range selection

Roles retain their owning declaration/source-region/kind identity. One explicit
binding may serve compatible roles, but equal local names never merge roles.
Each applied binding retains the selected producer relationship/population/window,
source/schema, anchor, unit, capture, scope, and range identity.

The selected range names exactly one clock family. Event-position records retain
sequence/position identity without an invented timestamp; fixed samples retain
exact epoch/period/unit/index; timestamped records retain their timestamp anchor.
Member-object identity, record identity, and position/sample/timestamp-anchor
identity remain independently readable. Start is included once and end is
excluded in every selected producer range. A timestamped native inclusive
deadline at the producer end needs a separately selected covering range; no
closure label, membership list, or adjacent range may substitute.

The selected result interface retains assessment execution, independent
decision-scope and surrounding-execution closure, truth, settlement basis, exact
decision support, activation, participation, completeness, adequacy, provenance,
and linked supersession. A witness inside exact admitted support may settle truth
while unrelated coverage remains incomplete; this definition neither closes the
subject nor manufactures missing coverage.

## Adoption boundary

A must index this successor's exact digest before a linked package selects it.
The prior `1-draft.1` and `1-draft.2` bytes remain recoverable and cannot be
rewritten to mean this dependency closure.
